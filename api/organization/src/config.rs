use lazy_static::lazy_static;
use clap::Parser;

const APP_TITLE: &str = "Organization";

#[derive(Debug, Clone, Parser)]
#[clap(author, about, version, name = APP_TITLE)]
pub struct Config {
    /// IP address on which to start the HTTP server
    #[clap(long, env, default_value = "127.0.0.1")]
    pub ip: String,

    /// Port on which to start the HTTP server
    #[clap(long, env, default_value = "8080")]
    pub port: String,

    /// Database URL (with credentials)
    #[clap(long, env, hide_env_values = true)]
    pub database_url: String,

    /// Max elements per page
    #[clap(long, env, default_value = "50")]
    pub page_limit: u8,

    /// Allowed Orgins separate by comma
    #[clap(long, env, default_value = "*")]
    pub allowed_origins: String,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::parse();
}
