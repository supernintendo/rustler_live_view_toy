use Mix.Config

# Configure your database
config :rustler_live_view_toy, RustlerLiveViewToy.Repo,
  username: "postgres",
  password: "postgres",
  database: "rustler_live_view_toy_test",
  hostname: "localhost",
  pool: Ecto.Adapters.SQL.Sandbox

# We don't run a server during test. If one is required,
# you can enable the server option below.
config :rustler_live_view_toy, RustlerLiveViewToyWeb.Endpoint,
  http: [port: 4002],
  server: false

# Print only warnings and errors during test
config :logger, level: :warn
