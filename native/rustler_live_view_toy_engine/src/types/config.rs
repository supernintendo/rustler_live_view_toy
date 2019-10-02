/// Configuration struct passed from Elixir to NIF
#[derive(NifStruct)]
#[module = "RustlerLiveViewToy.Engine.Config"]
pub struct Config {
    pub master_key: String,
    pub tcp_port: i32,
}
