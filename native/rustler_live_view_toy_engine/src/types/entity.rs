use bytes::{Bytes, BytesMut};
use futures::sync::mpsc;
use std::str;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::io;
use tokio::prelude::*;

use super::channel::Rx;
use super::client::Client;
use super::session::Session;

use crate::constants::MESSAGE_BATCH_SIZE;

/// The state of an entity in the game.
pub struct Entity {
    addr: SocketAddr,
    client: Client,
    id: Bytes,
    is_master: bool,
    rx: Rx,
    session: Arc<Mutex<Session>>,
}

impl Entity {
    /// Create a new `Entity` given all required fields.
    pub fn new(id: BytesMut, session: Arc<Mutex<Session>>, client: Client) -> Entity {
        // Cache the `SocketAddr` and create a channel to communicate with.
        let addr = client.peer_addr().unwrap();
        let entity_key = id.clone().freeze();
        let entity_id = id.clone().freeze();
        let (tx, rx) = mpsc::unbounded();

        // Add an entry for this `Entity` in the shared session map.
        let is_master = session.lock().unwrap().add_entity(addr, &entity_key, tx);

        Entity {
            addr,
            client,
            id: entity_id,
            is_master: is_master,
            rx,
            session,
        }
    }
}

/// The main message loop for a `Entity`. This future is responsible
/// for reading and writing messages using the entity's `Client` as
/// well as publishing events to the game simulation thread.
impl Future for Entity {
    type Item = ();
    type Error = io::Error;

    // Poll for incoming messages and buffer them as needed. This
    // defines a bounded loop that reads up to `MESSAGE_BATCH_SIZE`
    // messages every tick.
    fn poll(&mut self) -> Poll<(), io::Error> {
        for i in 0..MESSAGE_BATCH_SIZE {
            // Polling an `UnboundedReceiver` cannot fail, so `unwrap`
            // here is safe.
            match self.rx.poll().unwrap() {
                Async::Ready(Some(v)) => {
                    // Buffer the message.
                    self.client.buffer(&v);

                    // If `MESSAGE_BATCH_SIZE` is hit, there are still
                    // messages left to read on the last iteration of the
                    // loop before it breaks. Notify the executor of this
                    // task to schedule it again.
                    if i + 1 == MESSAGE_BATCH_SIZE {
                        task::current().notify();
                    }
                }
                _ => break,
            }
        }

        // Flush the write buffer to the socket.
        let _ = self.client.poll_flush()?;

        // Read new messages from the socket.
        while let Async::Ready(message) = self.client.poll()? {
            if let Some(raw_message) = message {
                let _s = match str::from_utf8(&raw_message) {
                    Ok(v) => {
                        let entity_key = self.id.clone();

                        &self.session.lock().unwrap().handle_message(entity_key, String::from(v));
                        ()
                    }
                    Err(_e) => (),
                };
            } else {
                // If EOF was reached, the client has disconnected and there
                // is nothing left to do.
                return Ok(Async::Ready(()));
            }
        }

        // This should only return `NotReady` if an inner future also
        // returned `NotReady`. At this point we received a `NotReady`
        // from either `self.rx` or `self.client` and this future can
        // end.
        Ok(Async::NotReady)
    }
}

impl Drop for Entity {
    // Clean up on the way out.
    fn drop(&mut self) {
        let entity_id = self.id.clone();

        self.session.lock().unwrap().remove_entity(&entity_id)
    }
}
