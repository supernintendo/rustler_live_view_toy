use futures::future::{self, Either};
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio::prelude::*;

use crate::types::Client;
use crate::types::Entity;
use crate::types::Session;

/// Spawn a task to manage the socket. The first line received
/// is treated as the ID of the client at which point the client
/// is added as an `Entity` to the global `Session`.
pub fn process(socket: TcpStream, session: Arc<Mutex<Session>>) {
    // Decorate the socket with a `Client` that contains a buffered
    // channel.
    let client = Client::new(socket);

    // Wait for the first item from the client stream and extract it.
    // Futures return an error of type `()` so `map_err` must be used
    // to allow passing through successful values.
    let connection = client
        .into_future()
        .map_err(|(e, _)| e)
        .and_then(|(id, client)| {
            // We need to return one of two different future types from
            // this closure depending on whether or not `id` is valid.
            // To allow this, the `Either` enum is used to allow wrapping
            // two concrete future types into a single return type.
            let id = match id {
                Some(id) => id,
                None => {
                    // If no data was sent at this point, the remote client
                    // connection has closed and we return `future::ok()` to
                    // allow the future to end.
                    return Either::A(future::ok(()));
                }
            };
            // Instantiate a `Entity` and return a `Entity` future that will
            // handle it until the client socket connection ends.
            let entity = Entity::new(id, session, client);

            Either::B(entity)
        })
        .map_err(|e| {
            // TODO: Better error handling
            println!("connection error = {:?}", e);
        });

    // Spawn the task.
    tokio::spawn(connection);
}
