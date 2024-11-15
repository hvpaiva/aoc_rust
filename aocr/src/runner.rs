use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use std::{collections::HashMap, fmt::Display, path::PathBuf, sync::RwLock};

use anyhow::Result;
use clap::{Parser, ValueEnum};

/// Runner arguments for Advent of Code problems.
///
/// This struct is used to parse command line arguments for the `aocr` binary.
#[derive(Debug, Parser)]
pub struct AocRunnerArgs {
    /// The part of the Advent of Code problem to run.
    #[arg(short, long)]
    part: Part,
    /// The optional name of the solution function.
    ///
    /// This is helpful when running multiple solutions for the same problem.
    #[arg(short, long, default_value = "solution")]
    name: String,
}

/// Represents the part of the Advent of Code problem.
#[derive(ValueEnum, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Part {
    /// Part one of the problem.
    One,
    /// Part two of the problem.
    Two,
}

impl Part {
    pub fn as_str(&self) -> &'static str {
        match self {
            Part::One => "one",
            Part::Two => "two",
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

type SolutionFn = fn(&str) -> Result<i64>;
type FunctionRegistry = HashMap<String, SolutionFn>;

lazy_static::lazy_static! {
    static ref FUNCTION_REGISTRY: RwLock<FunctionRegistry> = RwLock::new(HashMap::new());
}

/// Registers a solution function for the given part and name.
///
/// Intended to be used by a proc macro to automatically register solution functions.
pub fn register_function(part: &str, name: &str, func: fn(&str) -> Result<i64>) {
    let func_name = format!("{}_{}", name, part);
    FUNCTION_REGISTRY
        .write()
        .expect("Failed to acquire write lock")
        .insert(func_name, func);
}

/// Runs the Advent of Code problem solution.
///
/// This function reads the input file, parses the arguments, and runs the solution function.
pub fn run(input_path: PathBuf, args: AocRunnerArgs) -> Result<()> {
    let input = std::fs::read_to_string(&input_path)
        .map_err(|e| anyhow::anyhow!("Failed to read input file: {:?}", e))?;

    let registry = FUNCTION_REGISTRY
        .read()
        .expect("Failed to acquire read lock");
    let func = registry
        .get(format!("{}_{}", args.name, args.part).as_str())
        .ok_or_else(|| anyhow::anyhow!("Unable to find a function for the given part and name"))?;

    println!(
        "Running part {} solution:\n",
        args.part.as_str().cyan().bold()
    );
    let output = func(&input)?;

    println!(
        "{} {}",
        "Answer:".italic(),
        format!("{output}").green().bold()
    );

    if let Ok(mut ctx) = ClipboardContext::new() {
        if let Err(e) = ctx.set_contents(output.to_string()) {
            eprintln!("Failed to copy to clipboard: {:?}", e);
        } else {
            println!("{}", "\nOutput copied to clipboard.".blue().italic());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    #[test]
    fn test_part_as_str() {
        assert_eq!(Part::One.as_str(), "one");
        assert_eq!(Part::Two.as_str(), "two");
    }

    #[test]
    fn test_part_display() {
        assert_eq!(format!("{}", Part::One), "one");
        assert_eq!(format!("{}", Part::Two), "two");
    }

    fn sample_solution(input: &str) -> Result<i64> {
        Ok(input.parse().unwrap_or(0))
    }

    #[test]
    fn test_register_function() {
        let name = "solution";
        register_function(Part::One.as_str(), name, sample_solution);

        let registry = FUNCTION_REGISTRY.read().unwrap();
        let func_key = format!("{}_{}", name, Part::One.as_str());
        assert!(registry.contains_key(&func_key));
    }

    #[test]
    fn test_run_registered_function() {
        let name = "solution";
        register_function(Part::One.as_str(), name, sample_solution);

        let registry = FUNCTION_REGISTRY.read().unwrap();
        let func = registry
            .get(&format!("{}_{}", name, Part::One.as_str()))
            .expect("Function not found in registry");

        let result = func("42").unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_run_with_mocked_input() {
        let input_path = PathBuf::from("test_input_mocked.txt");
        std::fs::write(&input_path, "123").expect("Failed to create test input file");

        register_function(Part::One.as_str(), "solution", sample_solution);
        let args = vec!["binary_name", "--part", "one", "--name", "solution"];
        let aoc_args = AocRunnerArgs::parse_from(args);

        let result = run(input_path, aoc_args);

        assert!(result.is_ok());

        // Cleanup: removing test file
        std::fs::remove_file("test_input_mocked.txt").unwrap();
    }

    #[test]
    fn test_run_with_missing_function() {
        let input_path = PathBuf::from("test_input.txt");
        std::fs::write(&input_path, "123").expect("Failed to create test input file");

        let args = vec!["binary_name", "--part", "one", "--name", "missing_function"];
        let aoc_args = AocRunnerArgs::parse_from(args);

        let result = run(input_path, aoc_args);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e
                .to_string()
                .contains("Unable to find a function for the given part and name"));
        }

        // Cleanup
        std::fs::remove_file("test_input.txt").unwrap();
    }

    #[test]
    fn test_run_with_missing_input_file() {
        let input_path = PathBuf::from("non_existent_file.txt");
        let args = vec!["binary_name", "--part", "one", "--name", "solution"];
        let aoc_args = AocRunnerArgs::parse_from(args);

        let result = run(input_path, aoc_args);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Failed to read input file"));
        }
    }
}
