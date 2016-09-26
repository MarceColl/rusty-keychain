extern crate clap;
extern crate log;
extern crate git2;
extern crate gpgme;

use std::env;
use std::io;
use std::io::prelude::*;
use std::process::exit;

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
	let key = matches.value_of("gpgkey").unwrap().to_string();

	println!("{}", key);

	// let proto = gpgme::PROTOCOL_OPENPGP;

	// let mut mode = gpgme::ops::KeyListMode::empty();
	// let mut ctx = gpgme::create_context().unwrap();
 //    ctx.set_protocol(proto).unwrap();
 //    ctx.set_key_list_mode(mode).unwrap();

 //    let mut key = ctx.find_key(key.into_bytes().into()).unwrap();
 //    println!("keyid   : {}", key.id().unwrap_or("?"));
 //    println!("fpr     : {}", key.fingerprint().unwrap_or("?"));
 //    println!("caps    : {}{}{}{}",
 //             if key.can_encrypt() { "e" } else { "" },
 //             if key.can_sign() { "s" } else { "" },
 //             if key.can_certify() { "c" } else { "" },
 //             if key.can_authenticate() { "a" } else { "" });
 //    println!("flags   :{}{}{}{}{}{}",
 //             if key.is_secret() { " secret" } else { "" },
 //             if key.is_revoked() { " revoked" } else { "" },
 //             if key.is_expired() { " expired" } else { "" },
 //             if key.is_disabled() { " disabled" } else { "" },
 //             if key.is_invalid() { " invalid" } else { "" },
 //             if key.is_qualified() { " qualified" } else { "" });
 //    for (i, user) in key.user_ids().enumerate() {
 //        println!("userid {}: {}", i, user.uid().unwrap_or("[none]"));
 //        println!("valid  {}: {:?}", i, user.validity())
 //    }
 //    println!("");
}