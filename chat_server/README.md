# 💬 Chat Server (TCP)

A concurrent TCP chat server with usernames, commands, and real-time message broadcasting.

## 🎯 Learning Objectives

- **TCP Sockets** - Working with low-level network programming
- **Async/Await** - Non-blocking I/O with Tokio runtime
- **Concurrency** - Handling multiple clients simultaneously
- **Channels** - Message passing between async tasks with `broadcast`
- **Shared State** - Thread-safe data sharing with `Arc<Mutex<T>>`
- **`tokio::select!`** - Concurrent operations on multiple futures
- **Error Handling** - Graceful connection failures and cleanup

## 💬 Features

- Multiple concurrent client connections
- Username prompts on connect
- Real-time message broadcasting
- Join/leave announcements
- Commands (`/users`, `/quit`)
- Graceful error handling
- Thread-safe user tracking

## 🚀 Running the Server

```bash
cargo run
```

The server listens on `127.0.0.1:8080`

## 📝 Connecting as a Client

### Using netcat (nc)
```bash
nc localhost 8080
```

### Using telnet
```bash
telnet localhost 8080
```

### Multiple clients
Open multiple terminal windows and connect simultaneously to test chat functionality.

## 🎮 Example Session

**Terminal 1: Server**
```bash
$ cargo run
✈️  server listening on 127.0.0.1:8080
New connection from: 127.0.0.1:54321
User 'Alice' connected!
New connection from: 127.0.0.1:54322
User 'Bob' connected!
Received message: Hi everyone!
Connection closed by client: Bob
```

**Terminal 2: Alice**
```bash
$ nc localhost 8080
Enter your Username: Alice
*** Alice has joined the chat ***
*** Bob has joined the chat ***
Bob: Hi everyone!
Hello Bob!
/users
Connected users: Alice, Bob
*** Bob has left the chat ***
/quit
Goodbye!
```

**Terminal 3: Bob**
```bash
$ nc localhost 8080
Enter your Username: Bob
*** Alice has joined the chat ***
*** Bob has joined the chat ***
Hi everyone!
Alice: Hello Bob!
^C
```

## 🔧 Available Commands

- **`/users`** - List all currently connected users
- **`/quit`** - Disconnect gracefully from the chat
- Any other `/command` - Returns "Unknown command" message

## 🔑 Key Concepts Demonstrated

### TCP Server Loop
```rust
let listener = TcpListener::bind("127.0.0.1:8080").await?;

loop {
    let (stream, address) = listener.accept().await?;
    
    // Spawn a task per client
    tokio::spawn(async move {
        handle_client(stream, address, tx, rx, users).await
    });
}
```

### Broadcast Channel for Messages
```rust
// Create broadcast channel
let (tx, _rx) = broadcast::channel::<String>(100);

// Each client gets:
let tx = tx.clone();        // Clone sender to broadcast
let rx = tx.subscribe();     // New receiver to listen
```

### Concurrent Read/Write with `tokio::select!`
```rust
loop {
    tokio::select! {
        // Read from client socket
        result = stream.read(&mut buf) => {
            // Handle incoming messages
            tx.send(formatted_message)?;
        }
        
        // Read from broadcast channel
        result = rx.recv() => {
            // Forward broadcast to this client
            stream.write_all(msg.as_bytes()).await?;
        }
    }
}
```

### Shared State with Arc<Mutex<T>>
```rust
type Users = Arc<Mutex<HashMap<SocketAddr, String>>>;

// Add user
{
    let mut users_list = users.lock().unwrap();
    users_list.insert(address, username.clone());
}  // Lock released here

// Remove user
{
    let mut users_list = users.lock().unwrap();
    users_list.remove(&address);
}
```

### Command Handling
```rust
if message.starts_with("/") {
    match message.as_str() {
        "/users" => {
            let response = {
                let users_list = users.lock().unwrap();
                let usernames: Vec<&str> = users_list.values()
                    .map(|x| x.as_str())
                    .collect();
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
}
```

## 💡 What I Learned

1. **Two Loops Pattern**: 
   - Outer loop: Accept new connections forever
   - Inner loop: Handle messages from each client

2. **Channels for Message Passing**: 
   - `broadcast::channel` allows one-to-many communication
   - Each client gets a sender (to broadcast) and receiver (to listen)

3. **`tokio::select!` Macro**: 
   - Run multiple async operations concurrently
   - Whichever completes first is handled

4. **Shared Mutable State**: 
   - `Arc<Mutex<T>>` for thread-safe shared data
   - Scope locks to avoid holding across `.await` points

5. **Buffer Reuse**: 
   - Same buffer can be reused for multiple reads
   - Slice `[..n]` ensures only valid data is used

6. **Graceful Shutdown**: 
   - Detect client disconnect (`read()` returns 0)
   - Clean up user from shared state
   - Announce departure to other clients

7. **Error Propagation in Spawned Tasks**: 
   - Errors in spawned tasks don't crash the server
   - Use `if let Err(e)` to handle and log errors per client

## 🧪 Testing Strategy

### Basic Functionality
```bash
# Terminal 1: Start server
cargo run

# Terminal 2: Connect first client
nc localhost 8080
# Enter username, send messages

# Terminal 3: Connect second client
nc localhost 8080
# Verify both clients see each other's messages
```

### Commands
```bash
# Test /users command
/users
# Should list all connected users

# Test /quit command
/quit
# Should disconnect gracefully with "Goodbye!"

# Test unknown command
/help
# Should return "Unknown command: /help"
```

### Edge Cases
- **Empty username**: Server keeps prompting until valid username entered
- **Disconnect before username**: Server handles gracefully, no join message
- **Client crash (Ctrl+C)**: Leave announcement sent to other clients
- **Multiple clients**: All messages broadcast to all connected clients

## 🔄 Possible Improvements

- [ ] Private messages (`/whisper user message`)
- [ ] Chat rooms/channels
- [ ] Message history/logging
- [ ] User authentication
- [ ] Rate limiting per user
- [ ] Profanity filter
- [ ] Emoji support
- [ ] Timestamp messages
- [ ] Client connection limits
- [ ] Persistent chat logs to file

## 📚 Relevant Rust Book Chapters

- [Chapter 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) - `Arc`, `Mutex`
- [Chapter 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - Async programming

## 📦 Dependencies

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

## 🐛 Common Issues & Solutions

### Issue: "future cannot be sent between threads safely"
**Cause**: Holding `std::sync::MutexGuard` across `.await` point

**Solution**: Use scoped blocks to release lock before `.await`
```rust
let response = {
    let guard = mutex.lock().unwrap();
    // Use guard here
    create_response()
};  // Guard dropped here
stream.write(response).await?;  // Safe!
```

### Issue: Client sees their own messages
**Cause**: Broadcast channel sends to all subscribers including sender

**Solution**: This is expected behavior for a chat room. To filter, track sender info in message.

## 🏗️ Architecture

### Connection Flow
```
1. Client connects → TcpListener.accept()
2. Server spawns task for this client
3. Client enters username
4. Add to shared user list
5. Send join announcement
6. Enter message loop:
   - Read from client → broadcast to all
   - Read from broadcast → write to this client
7. Client disconnects
8. Remove from user list
9. Send leave announcement
```

### Message Flow
```
Client A sends "Hello"
    ↓
Server reads from Client A's socket
    ↓
Server broadcasts to channel
    ↓
All clients' receivers get message
    ↓
Each client writes to their own socket
    ↓
All clients see "Alice: Hello"
```

---

**Status**: ✅ Completed | **Difficulty**: Intermediate-Advanced