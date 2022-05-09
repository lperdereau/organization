#[macro_use]
extern crate log;

mod organization;

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use reqwest::Url;

fn cli() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(organization::cli())
}

fn main() {
    env_logger::init();
    let matches = cli()
        .arg(
            Arg::new("server_url")
                .short('s')
                .long("server-url")
                .value_name("SERVER_URL")
                .help("Server url to request endpoint (e.g. http://localhost:8080).")
                .takes_value(true),
        )
        .get_matches();
    let server_url: Url = matches.value_of_t("server_url").unwrap_or_else(|e| e.exit());
    info!("Request will be made on: {server_url}");
    match matches.subcommand() {
        Some(("organization", sub_matches)) => organization::matches(sub_matches, server_url),
        _ => unreachable!(),
    }
}
