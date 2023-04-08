## tungstenite+mio simple chat server

A super basic implementation of a WebSocket server written in Rust with `tungstenite` and `mio` crates. Using a single thread with event loop for non-blocking I/O without async syntax.

I created it to learn basic Rust without the `async` part, and to get more familiar with WebSockets. It's not really usable in real life, obviously. And by no means it's a _good_ example of how to write in Rust. Likely a bad one.

But in case you, dear reader, see this and want to try it out, to tweak and to learn like I did, be my guest. Mind you, it will only work locally on one machine, and not in Chromium browsers because it doesn't use secure connection (TLS).

If you still want to try, OK, I get it, I've been there. Here's what you do:

1. Install [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you don't have them.

2. Inside repo's root run

```shell
cargo run
```

This will start the server.

3. Serve `fe_example` folder using any tool you choose, or simply open the `fe_example/index.html`. Use FireFox or Safari.

4. Open different browsers or tabs, send message and see it appear everywhere.

Note: FE example is entirely stolen from Brian Holt's [repo](https://github.com/btholt/realtime-exercises/tree/main/websockets/exercise-raw) for his Frontend Masters workshop.