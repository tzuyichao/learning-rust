use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
  name: Option<String>,
  #[arg(short, long, value_name = "FILE")]
  config: Option<PathBuf>,

  #[arg(short, long, action = clap::ArgAction::Count)]
  debug: u8,
  
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  Test {
    #[arg(short, long)]
    list: bool,
  },
}

fn main() {
    let cli = Cli::parse();
    if let Some(name) = cli.name.as_deref() {
      println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
      println!("Value for config: {}", config_path.display());
    }

    match cli.debug {
      0 => println!("Debug mode is off"),
      1 => println!("Debug mode is kind of on"),
      2 => println!("Debug mode is on"),
      _ => println!("Don't be crazy"),
    }

    match &cli.command {
      Some(Commands::Test {list}) => {
        if *list {
          println!("Printing testing lists...");
	} else {
	  println!("Not printing testing lists...");
	}
      }
      None => {}
    }

    println!("Hello, World!");
}
