extern crate clipboard;

use std::{thread, time};
use clap::{Arg, App, SubCommand, ArgMatches};

pub fn define_subcommand<'a>() -> App<'a, 'a> {
	SubCommand::with_name("copy")
        .about("Copy a database value to clipboard")
        .version("0.1")
    	.arg(Arg::with_name("site")
            .required(true)
            .index(1)
            .help("site name"))
}

fn to_timed_clipboard(pwd: String, timeout: u64) {
	let timout_seconds = time::Duration::from_secs(timeout);

	let mut ctx = clipboard::ClipboardContext::new().unwrap();
    ctx.set_contents(pwd);

    println!("Password copied to clipboard, in {} seconds it will be removed from clipboard.", timeout);
	thread::sleep(timout_seconds);

	ctx.set_contents("".to_string());
	println!("Clipboard reset");
}

pub fn exec<'a>(matches: &ArgMatches<'a>) {
	let site = matches.value_of("site").unwrap();
	
	to_timed_clipboard(site.to_string(), 5);
}