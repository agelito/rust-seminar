//! Basic calculator application
//!
//! This application demonstrates building a basic calculator application.
//! - Show examples of using the `clap` crate to parse arguments
//! - Show examples of the pattern matching feature in Rust
//! - Show examples of adding implementation to enum
//! - Show examples of derive macro usage
//! - Show examples of importing functions from external crate
//!

use clap::{Parser, Subcommand};

use my_library::{add, div, mul, sub};

#[derive(Debug, Subcommand)]
pub enum Operator {
    #[command(about = "Add two numbers")]
    Add { left: usize, right: usize },
    #[command(about = "Multiply two numbers")]
    Multiply { left: usize, right: usize },
    #[command(about = "Subtract two numbers")]
    Subtract { left: usize, right: usize },
    #[command(about = "Divide two numbers")]
    Divide { left: usize, right: usize },
}

impl Operator {
    pub fn format_result(&self, result: usize) -> String {
        match self {
            Operator::Add { left, right } =>  format!("{} + {} = {}", left, right, result),
            Operator::Multiply {left, right  } => format!("{} * {} = {}", left, right, result),
            Operator::Subtract {left, right  } => format!("{} - {} = {}", left, right, result),
            Operator::Divide {left, right  } => format!("{} / {} = {}", left, right, result),
        }
    }

    pub fn calculate_result(&self) -> usize {
        match self {
            Operator::Add { left, right } => add(*left, *right),
            Operator::Multiply { left, right } => mul(*left, *right),
            Operator::Subtract { left, right } => sub(*left, *right),
            Operator::Divide { left, right } => div(*left, *right),
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "calculator")]
#[command(about = "A basic calculator app", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub operator: Operator,
}

fn main() {
    let cli = Cli::parse();
    let operator = cli.operator;

    let result = operator.calculate_result();
    let format = operator.format_result(result);

    println!("{}", format);
}
