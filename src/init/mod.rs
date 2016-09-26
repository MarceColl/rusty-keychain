extern crate clap;
extern crate log;
extern crate git2;
extern crate gpgme;

use std::fs;
use std::io;
use std::env;
use std::process::exit;
use std::path::PathBuf;
use clap::{Arg, App, SubCommand, ArgMatches};

pub fn define_subcommand<'a>() -> App<'a, 'a> {
	SubCommand::with_name("init")
        .about("Initialize the password database")
        .version("0.1")
    	.arg(Arg::with_name("git")
    		.short("g")
            .help("Site to manage"))
    	.arg(Arg::with_name("gpgkey")
            .required(true)
            .index(1)
            .help("gpg key id"))
}

pub fn exec<'a>(matches: &ArgMatches<'a>) {
	let key_input = matches.value_of("gpgkey").unwrap();

	let mut key = get_key(key_input);

	match key {
		Ok(k) => println!("Key found. fingerprint: {}", k.fingerprint().unwrap()),
		Err(e) => println!("Key not found."),
	}

	create_rk_folder_structure();
}

fn get_key(key_id: &str) -> Result<gpgme::keys::Key, gpgme::Error> {
	let proto = gpgme::PROTOCOL_OPENPGP;

	let mut mode = gpgme::ops::KeyListMode::empty();
	let mut ctx = gpgme::create_context().unwrap();
    ctx.set_protocol(proto).unwrap();
    ctx.set_key_list_mode(mode).unwrap();

    ctx.find_key::<String>(key_id.to_string())
}

fn create_rk_folder_structure() {
	let mut home_path: PathBuf = env::home_dir().expect("could not determine a home directory");

	println!("{:?}", home_path.to_str());

	home_path.push(".rusty-keychain");
	let folder_create_result = fs::create_dir(home_path.as_path());

	match folder_create_result {
		Ok(_) => println!("Folder created successfully"),
		Err(why) => match why.kind() {
			io::ErrorKind::AlreadyExists => println!("Folder already exists. No need to create new."),
			_ => println!("Couldn't create folder: {}", why),
		},
	}
}
