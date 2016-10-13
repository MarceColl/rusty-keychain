extern crate clipboard;

use std::{thread, time};

fn main() {
    println!("Hola");
    thread::sleep(time::Duration::from_secs(5));

    let mut ctx_inner = clipboard::ClipboardContext::new().unwrap();
    ctx_inner.set_contents("test".to_string()).unwrap();
}