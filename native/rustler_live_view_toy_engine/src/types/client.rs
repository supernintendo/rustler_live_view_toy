use bytes::{BufMut, BytesMut};
use futures::try_ready;
use std::net::SocketAddr;
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

use crate::constants::MESSAGE_DELIMITER;

/// Client interface backed by a TCP socket connection
/// and buffered channel for handling character-delimited
/// messages.
#[derive(Debug)]
pub struct Client {
    pub socket: TcpStream,
    pub rd: BytesMut,
    pub wr: BytesMut,
}

impl Client {
    /// Create a new `Client` for the given `TcpStream`.
    pub fn new(socket: TcpStream) -> Self {
        Client {
            socket,
            rd: BytesMut::new(),
            wr: BytesMut::new(),
        }
    }

    /// Buffer incoming data.
    pub fn buffer(&mut self, message: &[u8]) {
        // Reserve capacity for the message and push it to the
        // end of the write buffer.
        //
        // TODO: Impose bounds to avoid potential allocation.
        self.wr.reserve(message.len());
        self.wr.put(message);
    }

    pub fn peer_addr(&self) -> Result<SocketAddr, io::Error> {
        self.socket.peer_addr()
    }

    /// Flush the write buffer to the socket.
    ///
    /// This will write any buffered data to the socket as it's
    /// available and clear it from the write buffer afterwards.
    pub fn poll_flush(&mut self) -> Poll<(), io::Error> {
        while !self.wr.is_empty() {
            // Try to write to the socket.
            let n = try_ready!(self.socket.poll_write(&self.wr));

            // Discard the bytes that were just written.
            let _ = self.wr.split_to(n);
        }
        Ok(Async::Ready(()))
    }

    /// Read data from the socket.
    ///
    /// This pulls data from the socket into the read buffer
    /// and only returns `Ready` when the socket has closed.
    pub fn poll_read(&mut self) -> Poll<(), io::Error> {
        loop {
            // Reserve capacity for incoming bytes.
            //
            // TODO: Impose bounds to avoid potential allocation.
            self.rd.reserve(1024);

            // Pull data into the buffer.
            let n = try_ready!(self.socket.read_buf(&mut self.rd));

            // If the socket is ready to read from but doesn't return
            // any bytes, the connection has closed and we can stop
            // polling.
            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }
}

impl Stream for Client {
    type Item = BytesMut;
    type Error = io::Error;

    /// Watch for incoming data and return it as messages.
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // Check the client read buffer for new data.
        let socket_closed = self.poll_read()?.is_ready();

        // Look for the presence of a delimiter indicating that
        // a complete message has been sent.
        let pos = self
            .rd
            .windows(1) // A delimiter is one byte
            .enumerate()
            .find(|&(_, bytes)| bytes == MESSAGE_DELIMITER)
            .map(|(i, _)| i);

        if let Some(pos) = pos {
            // Split the message from the read buffer, including
            // the delimiter.
            let mut message = self.rd.split_to(pos + 1);

            // Drop the delimiter from the message.
            message.split_off(pos);

            // Return the message.
            return Ok(Async::Ready(Some(message)));
        }

        // If the socket has closed, stop polling.
        if socket_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}
