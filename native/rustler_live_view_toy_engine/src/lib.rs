#[macro_use] extern crate rustler;
use bytes::Bytes;
use rustler::{Encoder, Env, Error, Term};
use serde_json::{Value, Map};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::net::TcpListener;
use tokio::prelude::*;

mod constants;
mod tasks;
mod types;

use constants::*;
use tasks::clients;
use types::Config;
use types::Session;

/// Rustler macros
mod atoms {
    rustler_atoms! {
        atom circle;
        atom entity;
        atom pickup;
        atom projectile;
        atom message_batch_size;
        atom message_delimiter;
        atom ok;
        atom rect;
        atom server_fps;
        atom started;
        atom terrain;
    }
}
rustler::rustler_export_nifs! {
    "Elixir.RustlerLiveViewToy.Engine",
    [
        ("start", 1, start)
    ],
    None
}

pub type Avatars = HashMap<Bytes, Map<String, Value>>;

fn start<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    // Decode the resource struct passed to this NIF from the
    // Elixir context. This object contains some configuration
    // constants used by the engine.
    let config: Config = args[0].decode()?;
    let tcp_port = config.tcp_port;

    // Create a `Session` instance to contain the game state.
    // A unique handle to this value is passed to each client
    // thread as well as the game simulation thread to allow
    // communication between threads.
    let avatars = Arc::new(Mutex::new(Avatars::new()));
    let session = Arc::new(Mutex::new(Session::new(config, avatars)));
    let clients_session = session.clone();

    // Spawn the client connection server as a separate thread to
    // avoid blocking the calling thread (in this case, the Erlang VM).
    thread::spawn(move || {
        // Bind a TCP listener to the local socket address with
        // the `tcp_port` defined in `config`. This allows Elixir
        // to pass messages to the game engine while it is running
        // in the background.
        let addr = format!("127.0.0.1:{}", tcp_port).parse().unwrap();
        let listener = TcpListener::bind(&addr).unwrap();

        // Define a server task to asynchronously process incoming
        // connections.
        let server = listener
            .incoming()
            .for_each(move |socket| {
                // Process the connection in a new thread to avoid
                // blocking the TCP listener, passing it a unique
                // handle to the `Session`.
                clients::process(socket, clients_session.clone());

                Ok(())
            })
            .map_err(|err| {
                // TODO: Better error handling for bad connections.
                println!("accept error = {:?}", err);
            });

        tokio::run(server);
    });

    // Spawn a separate background thread for running the game
    // engine simulation. This will execute N times per second
    // where N is the `fps` value defined in `config`.
    thread::spawn(move || {
        let mut clock = fps_clock::FpsClock::new(SERVER_FPS);

        loop {
            clock.tick();
        }
    });

    // Return some info about the native engine to Elixir.
    let info = vec![
        (atoms::message_batch_size(), MESSAGE_BATCH_SIZE.encode(env)),
        (atoms::message_delimiter(), MESSAGE_DELIMITER.encode(env)),
        (atoms::server_fps(), SERVER_FPS.encode(env)),
    ];
    Ok((atoms::ok(), info.encode(env)).encode(env))
}
