use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::{HashMap};
use std::io::{self};
use tungstenite::{accept, WebSocket, Message};

enum EventResult {
    ReceivedMessage(String),
    ShakenHands,
    ConnectionClosed,
    Noop
}

struct Client {
    stream: Option<TcpStream>,
    websocket: Option<WebSocket<TcpStream>>
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client {stream: Some(stream), websocket: None }
    }

    /// Perform handshake, move stream ownership to WebSocket
    pub fn handshake(&mut self) {
        match self.stream.take() {
            Some(stream) => {
                let websocket = accept(stream).unwrap();
                self.websocket = Some(websocket);
            }
            None => {}
        }
    }

    /// Get mutable reference to underlying stream
    pub fn get_stream(&mut self) -> &mut TcpStream {
        if self.websocket.is_some() {
            return self.websocket.as_mut().unwrap().get_mut();
        } else {
            return self.stream.as_mut().unwrap();
        }
    }

    pub fn send(&mut self, message: &String) {
        self.websocket
            .as_mut()
            .unwrap()
            .write_message(Message::Text(message.to_string()))
            .unwrap();
    }
}

// Setup some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);


/// A WebSocket echo server
fn main () {
    // Create a poll instance.
    let mut poll = Poll::new().unwrap();
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the TCP server socket.
    let addr = "127.0.0.1:9001".parse().unwrap();
    let mut server = TcpListener::bind(addr).unwrap();

    // Register the server with poll we can receive events for it.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE).unwrap();

    let mut clients: HashMap<Token, Client> = HashMap::new();
    // Unique token for each incoming connection.
    let mut unique_token = Token(SERVER.0 + 1);

    

    // start the event loop
    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => loop {
                    // Received an event for the TCP server socket, which
                    // indicates we can accept an connection.

                    let (mut stream, address) = match server.accept() {
                        Ok((stream, address)) => (stream, address),
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            // If we get a `WouldBlock` error we know our
                            // listener has no more incoming connections queued,
                            // so we can return to polling and wait for some
                            // more.
                            break;
                        }
                        Err(e) => {
                            // If it was any other kind of error, something went
                            // wrong and we terminate with an error.
                            // return Err(e);
                            panic!("{}", e);
                        }
                    };

                    println!("Accepted connection from: {}", address);

                    let token = next(&mut unique_token);
                    poll.registry().register(
                        &mut stream,
                        token,
                        Interest::READABLE.add(Interest::WRITABLE),
                    ).unwrap();

                    let client = Client::new(stream);

                    clients.insert(token, client);
                },
                token => {
                    println!("Received a token");
                    // Maybe received an event for a TCP connection.
                    let event_result = if let Some(client) = clients.get_mut(&token) {
                        handle_connection_event(client, event)
                    } else {
                        EventResult::Noop
                    };

                    match event_result {
                        EventResult::ReceivedMessage(message) => {
                            println!("I am supposed to transmit {} message now", &message);

                            // Relay message to all clients
                            for (_, client) in clients.iter_mut() {
                                client.send(&message);
                            }
                        }
                        EventResult::ConnectionClosed => {
                            if let Some(mut client) = clients.remove(&token) {
                                println!("Removed client from map, deregistering from polling");
                                poll.registry().deregister(client.get_stream()).unwrap();
                            }
                        }
                        EventResult::Noop => {}
                        EventResult::ShakenHands => {}
                        
                    }
                }
            }
        }
    }
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

fn handle_connection_event(
    client: &mut Client,
    event: &Event,
) -> EventResult {
    if event.is_writable() {
        // We're not using this event
        println!("Stream is writeable");
    }

    if event.is_readable() {
        println!("Stream is readable");

        match client.stream {
            Some(_) => {
                client.handshake();
                return EventResult::ShakenHands;
            }
            None => {
                let message: String = client.websocket
                    .as_mut()
                    .unwrap()
                    .read_message()
                    .unwrap() // might panic here if connection closed
                    .into_text()
                    .unwrap();

                if message.is_empty() {
                    println!("Message is empty, closing connection");
                    client.websocket.as_mut().unwrap().close(None);
                    return EventResult::ConnectionClosed;
                } else {
                    println!("{}", message);
                    return EventResult::ReceivedMessage(message);
                }
            }
        }
    }

    EventResult::Noop
}