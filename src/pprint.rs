use ansi_term::{ Color };


#[allow(unused)]
pub fn enable_ansi() {
	#[cfg(windows)]
	{
		print!("[i] Enabling ANSI...\n\n");
		use ansi_term::enable_ansi_support;
		enable_ansi_support();
	}
}

pub fn info(msg: &str) {
	println!("{} {}",
		Color::Fixed( 14).paint("[i]"),
		Color::Fixed( 87).paint(msg)
	);
}

pub fn success(msg: &str) {
	println!("{} {}",
		Color::Fixed( 10).paint("[+]"),
		Color::Fixed(120).paint(msg)
	);
}

pub fn failure(msg: &str) {
	println!("{} {}",
		Color::Fixed(172).paint("[-]"),
		Color::Fixed( 11).paint(msg)
	);
}

pub fn failure_fatal(msg: &str) {
	println!("{} {}",
		Color::Fixed(  9).paint("[X]"),
		Color::Fixed( 11).paint(msg)
	);
}
