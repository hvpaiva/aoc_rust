use std::process::Command;

use aocr::runner::Part;
use clap::{Parser, Subcommand};
use config::StateFile;

mod config;

#[derive(Debug, Parser)]
#[command(name = "aocr-cli", version, author, about, long_about)]
struct AocCli {
    #[command(subcommand)]
    command: CommandCli,
}

#[derive(Subcommand, Debug)]
enum CommandCli {
    InitYear {
        #[arg(short, long)]
        year: u16,
    },
    CreateDay {
        #[arg(short, long)]
        year: Option<u16>,
        #[arg(short, long)]
        day: u8,
    },
    Run {
        #[arg(short, long)]
        year: Option<u16>,
        #[arg(short, long)]
        day: Option<u8>,
        #[arg(short, long)]
        part: Option<Part>,
        #[arg(short, long)]
        name: Option<String>,
    },
    Test {
        #[arg(short, long)]
        year: Option<u16>,
        #[arg(short, long)]
        day: Option<u8>,
        #[arg(short, long)]
        name: Option<String>,
    },
    Set {
        #[arg(short, long)]
        year: u16,
        #[arg(short, long)]
        day: u8,
    },
}

fn main() {
    let cli = AocCli::parse();
    match cli.command {
        CommandCli::InitYear { year } => init_year(year),
        CommandCli::CreateDay { year, day } => create_day(year, day),
        CommandCli::Run {
            year,
            day,
            part,
            name,
        } => run_solution(year, day, part, name),
        CommandCli::Test { year, day, name } => test_solution(year, day, name),
        CommandCli::Set { year, day } => set(year, day),
    }
}

fn init_year(year: u16) {
    let mut state = StateFile::load().unwrap();
    if state.has_initialized_year(&year) {
        panic!("Year {} is already initialized", year);
    }
    state.set_current_year(year).unwrap();
    Command::new("cargo")
        .arg("generate")
        .arg("--name")
        .arg(format!("aoc_{}", year))
        .arg("--path")
        .arg("./aocr_template/year")
        .arg("--define")
        .arg(format!("year={}", year))
        .status()
        .unwrap();

    create_day(Some(year), 1);

    Command::new("cargo")
        .arg("add")
        .arg("-p")
        .arg(format!("aoc_{}", year))
        .args(["anyhow", "aocr", "aocr_macro", "ctor"])
        .status()
        .unwrap();

    Command::new("cargo")
        .arg("add")
        .arg("-p")
        .arg(format!("aoc_{}", year))
        .arg("--dev")
        .args(["predicates", "pretty_assertions", "rstest"])
        .status()
        .unwrap();
}

fn create_day(year: Option<u16>, day: u8) {
    let mut state = StateFile::load().unwrap();
    let year = year.or(state.current_year).unwrap();
    if state.has_initialized_day(&year, &day) {
        panic!("Day {} is already initialized for year {}", day, year);
    }
    state.set_current_day(day, year).unwrap();

    Command::new("cargo")
        .arg("generate")
        .arg("--name")
        .arg(format!("{:02}", day))
        .arg("--path")
        .arg("./aocr_template/day")
        .arg("--destination")
        .arg(format!("./aoc_{}/src/bin", year))
        .arg("--define")
        .arg(format!("year={}", year))
        .arg("--define")
        .arg(format!("day={:02}", day))
        .status()
        .unwrap();

    Command::new("mkdir")
        .arg("-p")
        .arg(format!("inputs/{}", year))
        .status()
        .unwrap();

    Command::new("aoc")
        .arg("d")
        .arg("-I")
        .arg("-i")
        .arg(format!("inputs/{}/{:02}.txt", year, day))
        .arg("-d")
        .arg(format!("{}", day))
        .arg("-y")
        .arg(format!("{}", year))
        .status()
        .unwrap();
}

fn run_solution(year: Option<u16>, day: Option<u8>, part: Option<Part>, name: Option<String>) {
    let state = StateFile::load().unwrap();
    let day = day.or(state.current_day).unwrap();
    let year = year.or(state.current_year).unwrap();

    let mut runner = Command::new("cargo");
    runner
        .arg("run")
        .arg("-p")
        .arg(format!("aoc_{}", year))
        .arg("--bin")
        .arg(format!("{:02}", day))
        .arg("--");

    if let Some(name) = name {
        runner.arg("-n").arg(name);
    }

    if let Some(part) = part {
        runner.arg("-p").arg(part.as_str());
    }

    runner.status().unwrap();
}

fn test_solution(year: Option<u16>, day: Option<u8>, name: Option<String>) {
    let state = StateFile::load().unwrap();
    let day = day.or(state.current_day).unwrap();
    let year = year.or(state.current_year).unwrap();

    let mut runner = Command::new("cargo");
    runner
        .arg("test")
        .arg("-q")
        .arg("-p")
        .arg(format!("aoc_{}", year))
        .arg("--bin")
        .arg(format!("{:02}", day));

    if let Some(name) = name {
        runner.args([name]);
    }

    runner.status().unwrap();
}

fn set(year: u16, day: u8) {
    let mut state = StateFile::load().unwrap();
    state.set_current_day(day, year).unwrap();
}
