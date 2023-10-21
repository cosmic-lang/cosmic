//use rex::prelude::*;

use std::{path::PathBuf, env};

use anyhow::{Result, anyhow};
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
      let path = env::current_dir()?.join(path);
      let file_name = path.file_name().unwrap().to_str().unwrap();
      let extension = path.extension().unwrap().to_str().unwrap();

      if extension != EXT {
        return Err(anyhow!(format!("Invalid extension: {extension}, expected: {EXT}")))
      }

      let source: String;
      match path.to_str() {
        Some(path) => match std::fs::read_to_string(path) {
          Ok(src) => source = src,
          Err(msg) => return Err(anyhow!(msg))
        },
        None => return Err(anyhow!("Could not convert path to string")) 
      }

      let _ = Scanner::new(file_name, &source[..]);
      println!("{}", source);
    },
    Some(Commands::Run {path}) => {
      let path = env::current_dir()?.join(path);
      let file_name = path.file_name().unwrap().to_str().unwrap();
      let extension = path.extension().unwrap().to_str().unwrap();

      if extension != EXTI {
        return Err(anyhow!(format!("Invalid extension: {extension}, expected: {EXTI}")))
      }

      let source: String;
      match path.to_str() {
        Some(path) => match std::fs::read_to_string(path) {
          Ok(src) => source = src,
          Err(msg) => return Err(anyhow!(msg))
        },
        None => return Err(anyhow!("Could not convert path to string")) 
      }

      let _ = Scanner::new(file_name, &source[..]);
      println!("{}", source);
    },
    _ => {}
  }

  Ok(())
}
