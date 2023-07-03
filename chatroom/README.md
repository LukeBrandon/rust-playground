## Chatroom

This is a simple chatroom using `tokio.rs`

### Server
- Run the server by running `cargo run` in for his crate, by default this will run on `localhost:8000`

### Client
- Any TCP client will do, but this was developed using `telnet` as the clients
- To connect to the server once it is running, run `telnet localhost 8000` (this is most useful with multiple clients)
  - Once connected, you will see incoming message from other connected clients and you can send messages to all other connection client as well.
