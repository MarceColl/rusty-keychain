extern crate clap;
extern crate log;
extern crate git2;
extern crate gpgme;
extern crate clipboard;

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
    info!("Init module.");

	let key_input = matches.value_of("gpgkey").unwrap();
	info!("Using key given by user: {}", key_input);

	let mut key = get_key(key_input);

	match key {
		Ok(k) => info!("Key found. Key fingerprint: {}", k.fingerprint().unwrap()),
		Err(e) => error!("Error while looking for key: {}", e.description()),
	}

	create_rk_folder_structure();
}

fn get_key(key_id: &str) -> Result<gpgme::keys::Key, gpgme::Error> {
	let proto = gpgme::PROTOCOL_OPENPGP;

	let mut mode = gpgme::ops::KeyListMode::empty();

	let mut ctx = match gpgme::create_context() {
		Ok(ctx) => {
			info!("gpg context created.");
			ctx
		},
		Err(why) => { 
			error!("Error creating context: {}", why.description());
			return Err(why);
		},
	};

    match ctx.set_protocol(proto) {
    	Ok(_) => info!("Protocol set to OpenPGP"),
    	Err(why) => {
    		error!("Failed setting the protocol to OpenPGP: {}", why.description());
    		return Err(why);
    	},
    };

    match ctx.set_key_list_mode(mode) {
    	Ok(_) => info!("Key List mode set."),
    	Err(why) => {
    		error!("Failed setting key list mode: {}", why.description());
    		return Err(why);
    	},
    }

    ctx.find_key::<String>(key_id.to_string())
}

fn create_rk_folder_structure() {
	info!("Creating ~/.rusty-keychain.");
	let mut home_path: PathBuf = env::home_dir().expect("could not determine a home directory");

	info!("Using {:?} as HOME.", home_path.to_str().unwrap());

	home_path.push(".rusty-keychain");
	let folder_create_result = fs::create_dir(home_path.as_path());

	match folder_create_result {
		Ok(_) => info!("~/.rusty-keychain created successfully"),

		Err(why) => match why.kind() {
			io::ErrorKind::AlreadyExists => info!("~/.rusty-keychain already exists. No need to create new."),
			_ => error!("Couldn't create ~/.rusty-keychain: {}", why),
		},
	}
}
