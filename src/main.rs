use std::{collections::HashMap, path::PathBuf, str::FromStr};

use clap::Parser;
use fehler::throws;
use anyhow::{Context, Error, Result};
use serde_derive::Deserialize;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	// /// The epub book to edit on
	// #[clap(short, long, value_parser)]
	// input: String,

	/// Sets a custom config file
	#[clap(short, long, value_parser, value_name = "CONFIG")]
	config: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct Config {
	replace: HashMap<String, Vec<String>>,
}

#[throws]
fn main() {
	let args = Args::parse();
	let config = if let Some(config) = args.config {
		config
	} else {
		PathBuf::from_str("./config.toml").with_context(|| "Could not load './config.toml'.")?
	};
	parse_toml(config)?;
}

#[throws]
fn parse_toml(config: PathBuf) {
	// let config = std::fs::read_to_string("./resouces/config.toml")?;
	let config = std::fs::read_to_string(&config)
		.with_context(|| format!("Could not load '{}'.", config.display()))?;
	let config: Config = toml::de::from_str(&config)?; 
	println!("{config:#?}");
}
