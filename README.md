# rustler_live_view_toy

A small experiment demonstrating bidirectional communication between [phoenix_live_view](https://github.com/phoenixframework/phoenix_live_view) and Rust threads (via [rustler](https://github.com/rusterlium/rustler)) using TCP sockets.

Note: This is just for fun / learning, there are better ways to do this.

## Running

```bash
asdf install    # or manually install dependencies in .tool-versions
mix deps.get
cd assets && npm install
cd .. && mix phx.server
```

Open any number of browser windows on [localhost:4000](http://localhost:4000) to create objects on the screen. Press WASD in a browser window to update the state for that object across all sessions.

## License

Apache License 2.0
