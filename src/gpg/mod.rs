extern crate gpgme;


pub struct GPGClient {
	context: gpgme::context::Context,
	key: gpgme::keys::Key,
}

impl GPGClient {
	// pub fn new() -> GPGClient {
	// 	GPGClient { context: None, key: None }
	// }

	pub fn init(mut self) {
		// self.context = gpgme::create_context().ok();
		// self.context.map(|mut ctx| ctx.set_protocol(gpgme::PROTOCOL_OPENPGP).unwrap());
	}

	pub fn set_key(mut self, key_id: String) {
		// let mut mode = gpgme::ops::KeyListMode::empty();
		// self.context = self.context.map(|mut ctx| ctx.set_key_list_mode(mode).unwrap());
		// self.context.map(|mut ctx| ctx.find_key::<String>(key_id));
	}
}