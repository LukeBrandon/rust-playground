use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};


#[tokio::main]
async fn main() {

    // Start tcp server
    let listener = TcpListener::bind("localhost:8000").await.expect("Connection closed");

    let (tx, _) = broadcast::channel(10);

    loop {
        // Wait and listen for a new connection
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone(); // Clone so not borrowed
        let mut rx = tx.subscribe(); // Weird clone so not borrowed, thanks tokio

        // For the accepted connection, spawn this process
        tokio::spawn(async move {
            let (read, mut writer) = socket.split();

            let mut reader = BufReader::new(read);

            let mut line = String::new();

            // Contiually loop for reads or writes
            loop {
                // Multiple async at same time, act on first
                tokio::select! {
                    // Sending out message
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0  {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }

                    // Receiving messages
                    result = rx.recv() => {
                        let (message, other_addr) = result.unwrap();
                        // Don't send to self
                        if addr != other_addr {
                            writer.write_all(message.as_bytes()).await.unwrap();
                        }
                    }

                };
            }
        });
    }
}
