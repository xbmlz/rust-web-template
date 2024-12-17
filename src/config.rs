use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[clap(long = "host", env = "SERVER_HOST", default_value = "127.0.0.1")]
    pub host: String,

    #[clap(long = "port", env = "SERVER_PORT", default_value = "8080")]
    pub port: u16,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
        .field("host", &self.host)
        .field("port", &self.port)
        .finish()
    }
}
