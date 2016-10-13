extern crate clipboard;
extern crate libc;

use utils;
use std::{thread, time, ffi};
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
	let d_result: utils::DaemonResult;
	let mut ctx = clipboard::ClipboardContext::new().unwrap();
	let old_cpb = ctx.get_contents().unwrap();
    ctx.set_contents(pwd);

	unsafe {
		let pid = libc::fork();

		if pid > 0 {
			println!("Forked");
			return;
		} else {
			let program = ffi::CString::new("./rk-clear").unwrap().into_raw();
			let test = ffi::CString::new("cleared").unwrap().into_raw();
			let args = vec!(test).as_ptr() as *const *const i8;
			libc::execv(program, args);
		}
	}
}

pub fn exec<'a>(matches: &ArgMatches<'a>) {
	let site = matches.value_of("site").unwrap();

	to_timed_clipboard(site.to_string(), 5);
}