use crate::config;


pub fn get_to_download() -> Vec<(&'static str, &'static str)> {
	let mut target = Vec::new();

	for level in 1..=3 {
		if level > config::DL_LEVEL {
			break;
		}

		match level {
			1 => { target.extend_from_slice( config::DL_1 ) },
			2 => { target.extend_from_slice( config::DL_2 ) },
			3 => { target.extend_from_slice( config::DL_3 ) },
			_ => { eprintln!("`level` out of bound: {}", level); break; },
		}
	}

	target
}
