//! # Executor

use crate::{arguments::*, command::Command};
use std::path::PathBuf;

pub fn exec_ingest(mut pwd: PathBuf, args: IngestArgs) {
    pwd.push(args.path);
    println!("{:?}", pwd);
}

fn exec_ffmpeg(cmd: Command) {}
