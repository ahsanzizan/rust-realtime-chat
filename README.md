# Realtime Minimal Chat App

## Overview

A simple real-time chat application using TCP sockets in Rust, supporting multiple concurrent clients.

## Prerequisites

- Rust (latest stable version)
- Cargo package manager

## Project Structure

- `src/server.rs`: TCP server implementation
- `src/client.rs`: TCP client implementation
- `Cargo.toml`: Project configuration

## Running the Application

### Start the Server

```bash
cargo run --bin server
```

### Start Clients

In separate terminal windows, run:

```bash
cargo run --bin client
```

## Usage

- Type messages in the client terminal
- Messages are broadcast to all connected clients
- Type `:quit` to exit the client

## Features

- Concurrent client handling
- Non-blocking I/O
- Simple broadcast messaging
- Error handling

## Network Details

- Localhost: 127.0.0.1
- Port: 6000

## Limitations

- No persistent message storage
- Basic error handling
- No authentication

## License

MIT License
