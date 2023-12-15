use std::process::exit;

use clap::{arg, Parser, Subcommand};
use console::{style, Term};

use crate::rendering::display::Display;
use crate::document::Document;
use crate::prune::prune;

mod dependencies;
mod document;

mod rendering;
mod prune;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Features(FeaturesArgs),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct FeaturesArgs {
    #[arg(long, short)]
    dependency: Option<String>,

    #[command(subcommand)]
    sub: Option<FeaturesSubCommands>,
}

#[derive(Subcommand)]
enum FeaturesSubCommands {
    Prune {
        #[arg(long, short)]
        dry_run: bool,
    },
}

fn main() {
    let CargoCli::Features(args) = CargoCli::parse();

    if let Err(err) = run(args) {
        print!("{} : {}", style("error").red().bold(), err);
    }
}

fn run(args: FeaturesArgs) -> anyhow::Result<()> {
    let document = Document::new("./Cargo.toml")?;

    if let Some(sub) = args.sub {
        match sub {
            FeaturesSubCommands::Prune { dry_run } => {
                prune(document, dry_run)?;
            }
        }
    } else {
        let mut display = Display::new(document)?;

        if let Some(name) = args.dependency {
            display.set_selected_dep(name)?
        }

        let _ = ctrlc::set_handler(|| {
            let term = Term::stdout();
            term.show_cursor().unwrap();

            exit(0);
        });

        display.start()?;
    }

    Ok(())
}
