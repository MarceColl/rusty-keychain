extern crate rand;
extern crate clap;
extern crate log;

use clap::{Arg, App, SubCommand, ArgMatches};
use rand::Rng;

pub fn define_subcommand<'a>() -> App<'a, 'a> {
	SubCommand::with_name("manage")
        .about("Manage a site")
        .version("0.1")
        .arg(Arg::with_name("SITE")
            .required(true)
            .index(1)
            .help("Site to manage"))
}

pub fn exec<'a>(matches: &ArgMatches<'a>) {
	println!("Manage!");
	println!("{}", generate_password(50));
}

fn generate_password(length: usize) -> String {
	rand::thread_rng()
	    .gen_ascii_chars()
	    .take(length)
	    .collect()
}

// TESTING
#[test]
fn test_generate_password() {
	let pwd0 = generate_password(0);
	assert_eq!(pwd0, "");
	let pwd30 = generate_password(30);
	assert_eq!(pwd30.len(), 30);
	let pwd60 = generate_password(60);
	assert_eq!(pwd60.len(), 60);
}