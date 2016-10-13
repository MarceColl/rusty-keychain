extern crate libc;

use std::{thread, time};

pub enum DaemonResult {
    Parent,
    Daemon,
}

pub unsafe fn daemonized_timeout(timeout: u64) -> DaemonResult {
	let pid = libc::fork();

	if pid > 0 {
		return DaemonResult::Parent;
	}

	libc::setsid();
	libc::signal(libc::SIGCHLD, libc::SIG_IGN);
	libc::signal(libc::SIGHUP, libc::SIG_IGN);

	// let pid = libc::fork();

	// if pid > 0 {
	// 	return DaemonResult::Parent;
	// }

	let timout_seconds = time::Duration::from_secs(timeout);
	thread::sleep(timout_seconds);

	return DaemonResult::Daemon;
}