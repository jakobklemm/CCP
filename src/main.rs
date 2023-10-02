mod arguments;
mod entry;
mod config;
mod id;

use clap::Parser;
use polodb_core::Database;
use crate::{arguments::Args, config::Config};
use lazy_static::lazy_static;

lazy_static! {
    static ref DATABASE: Database = Database::open_file("dcp-devel.db").unwrap();
    static ref CONFIG: Config = Config::default();
}

fn main() {
    match Args::parse() {
        Args::Ingest(args) => {},
        Args::Process(args) => {},
        Args::Find(args) => {},
        Args::View(args) => {},
        Args::Export(args) => {},
        Args::Edit(args) => {},
        Args::Delete(args) => {},
        Args::Publish(args) => {},
    }

    println!("Hello, world!");
}
