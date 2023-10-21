//use rex::prelude::*;

use std::{path::PathBuf, env};

use anyhow::Result;
use clap::{Parser, Subcommand};
use const_format::formatcp;

use rex::prelude::*;

const EXT: &str = "rx";
const EXTI: &str = "rxi";

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const ABOUT: &str = formatcp!("\n\
  Rex {VERSION} \n\
  {DESCRIPTION}"
);

#[derive(Parser)]
#[command(name="rex", author="rex-lang", version, about=ABOUT, arg_required_else_help = true)]
struct Cli {
  #[arg(short, long)]
  output: Option<PathBuf>,
  #[command(subcommand)]
  command: Option<Commands>
}

// Potentially make subcommand not have args, and use flags for input/output

#[derive(Subcommand)]
enum Commands {
  /// Compiles file at relative path
  Compile{
    /// relative path of file to compile
    path: PathBuf
  },
  /// Interprets file at relative path
  Run{
    /// relative path of file to interpret
    path: PathBuf
  }
}

fn main() -> Result<()> {
  let cli = Cli::parse();

  match &cli.command {
    Some(Commands::Compile {path}) => {
      let file_name = path.file_name().unwrap().to_str().unwrap();
      let source = read_file(path, EXT)?;

      let _ = Scanner::new(file_name, &source[..]);
      println!("{}", source);
    },
    Some(Commands::Run {path}) => {
      let file_name = path.file_name().unwrap().to_str().unwrap();
      let source = read_file(path, EXTI)?;

      let _ = Scanner::new(file_name, &source[..]);
      println!("{}", source);
    },
    _ => {}
  }

  Ok(())
}
