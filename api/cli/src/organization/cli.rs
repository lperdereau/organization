use clap::{Command, ArgMatches};
use reqwest::Url;

use super::action;

pub fn cli() -> Command<'static> {
    Command::new("organization")
        .about("Made actions on Organiaztion resources")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(action::cli())
}

pub fn matches(matches: &ArgMatches, server_url: Url) {
    let endpoint_url = server_url.join("/organizations/");
    match matches.subcommand() {
        Some(("list", _sub_matches)) => {
            action::list(endpoint_url.unwrap());
        }
        Some(("get", sub_matches)) => {
            action::get(endpoint_url.unwrap(), &sub_matches.value_of_t::<String>("id").unwrap_or_else(|e| e.exit()));
        }
        _ => unreachable!(),
    }
}
