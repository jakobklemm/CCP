//! # Arguments

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub enum Args {
    Process(ProcessArgs),
    Ingest(IngestArgs),
    Find(FindArgs),
    View(ViewArgs),
    Export(ExportArgs),
    Edit(EditArgs),
    Delete(DeleteArgs),
    Publish(PublishArgs),
}

#[derive(Parser, Debug)]
pub struct ProcessArgs {
    eid: String,
}

#[derive(Parser, Debug)]
pub struct IngestArgs {
    eid: String,
}

#[derive(Parser, Debug)]
pub struct ViewArgs {
    eid: String,
}

#[derive(Parser, Debug)]
pub struct FindArgs {
    #[arg(short, long, default_value_t = String::from("*"))]
    tag: String,
    #[arg(short, long, default_value_t = String::from("*"))]
    game: String,
    title: String
}

#[derive(Parser, Debug)]
pub struct ExportArgs {
    eid: String,
}

#[derive(Parser, Debug)]
pub struct EditArgs {
    eid: String,
}

#[derive(Parser, Debug)]
pub struct DeleteArgs {
    eid: String,
}

#[derive(Parser, Debug)]
pub struct PublishArgs {
    eid: String,
}
