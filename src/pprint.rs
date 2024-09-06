use nu_ansi_term::Color;


#[allow(unused_must_use)]
pub fn enable_ansi() {
	#[cfg(windows)]
	{
		print!("[i] Enabling ANSI...\n\n");
		use nu_ansi_term::enable_ansi_support;
		enable_ansi_support();
	}
}

pub fn info<S: AsRef<str>>(msg: S) {
	let msg = msg.as_ref();
	println!("{} {}",
		Color::Fixed( 14).paint("[i]"),
		Color::Fixed( 87).paint(msg)
	);
}

pub fn success<S: AsRef<str>>(msg: S) {
	let msg = msg.as_ref();
	println!("{} {}",
		Color::Fixed( 10).paint("[+]"),
		Color::Fixed(120).paint(msg)
	);
}

pub fn failure<S: AsRef<str>>(msg: S) {
	let msg = msg.as_ref();
	println!("{} {}",
		Color::Fixed(172).paint("[-]"),
		Color::Fixed( 11).paint(msg)
	);
}

pub fn failure_fatal<S: AsRef<str>>(msg: S) {
	let msg = msg.as_ref();
	println!("{} {}",
		Color::Fixed(  9).paint("[X]"),
		Color::Fixed( 11).paint(msg)
	);
}
