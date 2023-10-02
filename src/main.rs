mod arguments;
mod config;
mod entry;
mod executor;
mod command;
mod id;

use crate::{arguments::Args, config::Config};
use clap::Parser;
use lazy_static::lazy_static;
use polodb_core::Database;

use std::env;

lazy_static! {
    static ref DATABASE: Database = Database::open_file(format!("{}/dcp.db", std::env::var("DCP_ROOT").unwrap_or(String::from("/database")))).unwrap();
}

fn main() {
    match Args::parse() {
        Args::Ingest(args) => {
            println!("Starting DCP Ingest Process: {:?}", args);
            println!("PATH: {:?}", arguments::cmd_name());
            println!("EXECUTABLE: {}", env::current_exe().unwrap().display());
            let i = config::Config::getNextID();
            println!("ID: {:?}", i);
            executor::exec_ingest(arguments::pwd_name(), args);
        }
        Args::Process(args) => {}
        Args::Find(args) => {}
        Args::View(args) => {}
        Args::Export(args) => {}
        Args::Edit(args) => {}
        Args::Delete(args) => {}
        Args::Publish(args) => {}
    }

    println!("Hello, world!");
}
