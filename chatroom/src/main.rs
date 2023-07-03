use std::net::SocketAddr;

use crate::models::Message;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{tcp::WriteHalf, TcpListener},
    sync::broadcast::{self, error::RecvError, Sender},
};

pub mod models;

/// Processes the sending of a message over a tokio::sync::broadcast::Sender
/// Sends a struct Messages
/// Caveats:
///   - If the legth of the message is 0 byes, the message will not be sent
/// 
async fn process_message_send(tx: &Sender<Message>, message: Message) {
    if message.0.as_bytes().len() == 0 {
        return;
    }

    // Send the contents of that buffer with the address
    tx.send(message).unwrap();
}

/// Processes the receiving of messages on a broadcast
///   - Wites all bytes using a WriteHalf of the TcpStream
///   - Message sent by self will not be written
async fn process_message_receive(
    received: Result<Message, RecvError>,
    writer: &mut WriteHalf<'_>,
    addr: SocketAddr,
) {
    match received {
        Ok(message) => {
            let line = message.0;
            let sender_addr = message.1;

            // Don't write if message is from self
            if addr == sender_addr {
                return;
            }

            writer.write_all(line.as_bytes()).await.unwrap();
        }
        Err(e) => {
            println!("Error receiving message {:?}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    // Start tcp server
    let listener = TcpListener::bind("localhost:8000")
        .await
        .expect("Server could not start");

    // Create a channel to pass Messages on
    let (tx, _rx) = broadcast::channel::<Message>(10);

    loop {
        // Continually wait and listen for a new connection
        let (mut socket, addr) = listener.accept().await.unwrap();

        // Make clone of channel sender and receiver
        let tx = tx.clone();

        // Get a reader to the channel by subscribing
        let mut rx = tx.subscribe();

        println!("User has joined the chat");

        // For the accepted connection, spawn this process
        tokio::spawn(async move {
            // Split the socket into read half and write half
            let (read, mut writer) = socket.split();

            let mut reader = BufReader::new(read);

            let mut line = String::new();

            // Contiually loop for reads or writes
            loop {
                // Multiple async at same time, act on first
                tokio::select! {

                    // Sending a message read in from the client
                    _ = reader.read_line(&mut line) => {
                        let message: Message =  Message(line.clone(), addr);

                        process_message_send(&tx, message).await;

                        line.clear();
                    }

                    // Receiving messages
                    received = rx.recv() => {
                        process_message_receive(received, &mut writer, addr).await;
                    }
                };
            }
        });
    }
}
