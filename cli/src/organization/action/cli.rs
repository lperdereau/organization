use clap::{Arg, Command};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    TableBuilder, TableStyle
};

use super::requests::Organiaztion;

pub fn cli() -> Vec<Command<'static>> {
    vec![cli_list(), cli_get(), cli_delete(), cli_create()]
}

fn cli_list() -> Command<'static> {
    Command::new("list")
        .arg(
            Arg::new("page")
                .short('p')
                .long("page")
                .value_name("LIST_PAGE")
                .help("Pagination index of organization ressource.")
                .takes_value(true),
        )
}

fn cli_get() -> Command<'static> {
    Command::new("get")
        .arg(
            Arg::new("id")
                .short('i')
                .long("id")
                .value_name("ID")
                .help("organization identifier.")
                .takes_value(true),
        )
        .arg_required_else_help(true)
}

fn cli_delete() -> Command<'static> {
    Command::new("delete")
        .arg(
            Arg::new("id")
                .short('i')
                .long("id")
                .value_name("ID")
                .help("organization identifier.")
                .takes_value(true),
        )
        .arg_required_else_help(true)
}
fn cli_create() -> Command<'static> {
    Command::new("create")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("organization name.")
                .takes_value(true),
        )
        .arg_required_else_help(true)
}

fn build_table(organizations: Vec<Organiaztion>) -> String {
    let raws = vec![Row::new(vec![
        TableCell::new_with_alignment("id", 2, Alignment::Left),
        TableCell::new_with_alignment("name", 2, Alignment::Left),
    ])];
    let mut table = TableBuilder::new()
        .style(TableStyle::blank())
        .rows(raws)
        .build();
    organizations.iter().for_each(|org| {
        table.add_row(
            Row::new(vec![
                TableCell::new_with_alignment(org.id, 2, Alignment::Left),
                TableCell::new_with_alignment(&org.name, 2, Alignment::Left),
            ])
        );
    });
    table.render()
}

pub fn list(endpoint_url: reqwest::Url) {
    match super::requests::list(endpoint_url) {
        Ok(organizations) => {
            print!("{}", build_table(organizations.results));
        }
        Err(_) => (),
    }
}

pub fn get(endpoint_url: reqwest::Url, id: uuid::Uuid) {
    match super::requests::get(endpoint_url, id) {
        Ok(organizations) => {
            print!("{}", build_table(vec!(organizations)));
        }
        Err(_) => (),
    }
}
