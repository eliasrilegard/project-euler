use clap::{Parser, Subcommand};

mod problems;

/// My collection of solved Project Euler problems
#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
  /// The subcommand to execute
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// Run a problem
  Solve { number: usize },
  /// Run all problems
  All,
}

pub fn run() {
  let args = Args::parse();

  match args.command {
    Commands::Solve { number } => {
      let answer = problems::solve(number);
      println!("Solution to problem {}: {}", number, answer);
    }
    Commands::All => {
      println!("Solving all problems...");
      problems::solve_all();
    }
  }
}
