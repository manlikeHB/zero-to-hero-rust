use chat_server::error::ChatError;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::result::Result;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Receiver, Sender};

#[tokio::main]
async fn main() -> Result<(), ChatError> {
    let users = Arc::new(Mutex::new(HashMap::<SocketAddr, String>::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("✈️ server listening on 127.0.0.1:8080");

    let (tx, _rx) = broadcast::channel::<String>(100);

    loop {
        let (stream, address) = listener.accept().await?;
        println!("New connection from: {}", address);

        let tx = tx.clone();
        let rx = tx.subscribe();
        let users = users.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, address, tx, rx, users).await {
                eprintln!("Error handling client {}: {}", address, e);
            }
        });
    }
}

async fn handle_client(
    mut stream: TcpStream,
    address: SocketAddr,
    tx: Sender<String>,
    mut rx: Receiver<String>,
    users: Arc<Mutex<HashMap<SocketAddr, String>>>,
) -> Result<(), ChatError> {
    let mut buf = vec![0; 1024];
    let mut username = String::new();

    while username.trim().is_empty() {
        stream.write_all(b"Enter your Username: ").await?;

        match stream.read(&mut buf).await {
            Ok(0) => {
                println!("Connection closed by client: {}", address);
                return Ok(());
            }
            Ok(n) => {
                username = String::from_utf8_lossy(&buf[..n]).trim().to_string();
            }
            Err(e) => {
                eprintln!("Error reading username from {}: {}", address, e);
                return Err(ChatError::Io(e));
            }
        };
    }

    {
        let mut users_list = users.lock().unwrap();
        users_list.insert(address, username.clone());
    }

    let join_msg = format!("*** {} has joined the chat ***\n", username);
    tx.send(join_msg)?;

    println!("User '{}' connected!", username);

    loop {
        tokio::select! {
            result = stream.read(&mut buf) => {
                match result {
                    Ok(0) => {
                        println!("Connection closed by client: {}", username);
                        break;
                    },
                    Ok(n) => {
                        let message = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                        if message.starts_with("/") {
                            match message.as_str() {
                                "/users" => {
                                    let response = {
                                        let users_list = users.lock().unwrap();
                                        let usernames: Vec<&str> = users_list.values().map(|x| x.as_str()).collect();
                                        format!("Connected users: {}\n", usernames.join(", "))
                                    };
                                    stream.write_all(response.as_bytes()).await?;
                                },
                                "/quit" => {
                                    stream.write_all(b"Goodbye!\n").await?;
                                    break;
                                },
                                _ => {
                                    let response = format!("Unknown command: {}\n", message);
                                    stream.write_all(response.as_bytes()).await?;
                                }
                            }
                        } else {
                            println!("Received message: {}", message);

                            let formatted_message = format!("{}: {}\n", username, message);
                            if let Err(_) = tx.send(formatted_message) {
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from {}: {}", username, e);
                        break;
                    }
                }
            }

            result = rx.recv() => {
                match result {
                    Ok(msg) => {
                        if let Err(e) = stream.write_all(msg.as_bytes()).await {
                            eprintln!("Error writing message to {}: {}", username, e);
                            break;
                        }
                    },
                    Err(broadcast::error::RecvError::Closed) => {
                        println!("Broadcast channel closed");
                        break;
                    },
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        println!("Client {} lagged behind", username);
                    }

                }
            }

        }
    }

    {
        let mut users_list = users.lock().unwrap();
        users_list.remove(&address);
    }

    let leave_msg = format!("*** {} has left the chat ***\n", username);
    let _ = tx.send(leave_msg);

    Ok(())
}
