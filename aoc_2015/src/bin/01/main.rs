use clap::Parser;
use std::{path::PathBuf, str::FromStr};

use aocr::runner::{run, AocRunnerArgs};

mod solutions;

fn main() {
    let args = AocRunnerArgs::parse();
    let input_path = PathBuf::from_str("inputs/2015/01.txt").unwrap();
    run(input_path, args).expect("Failed to run solution");
}
