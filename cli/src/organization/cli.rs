use clap::{Command, ArgMatches};
use reqwest::Url;

pub fn cli() -> Command<'static> {
    Command::new("organization")
        .about("Made actions on Organiaztion resources")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(super::action::cli())
}

pub fn matches(matches: &ArgMatches, server_url: Url) {
    let endpoint_url = server_url.join("/organizations/");
    match matches.subcommand() {
        Some(("list", sub_matches)) => {
            super::action::list(endpoint_url.unwrap());
        }
        Some(("get", sub_matches)) => {
            super::action::get(endpoint_url.unwrap());
        }
        _ => unreachable!(),
    }
}
