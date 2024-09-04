use serde::{ Serialize, Deserialize };
use std::path::{ PathBuf };


#[derive(Serialize, Deserialize, Clone)]
pub struct DownloadTarget {
	pub url: String,
	pub filename: String,
	pub filetype: String,
	pub version: String,
	pub description: String,
}

impl DownloadTarget {
	pub fn make_filename_pathbuf(&self) -> PathBuf {
		let mut filename_pathbuf = PathBuf::new();
		filename_pathbuf.push(
			format!("{} {}.{}", self.filename, self.version, self.filetype).as_str()
		);
		filename_pathbuf
	}
}


#[derive(Serialize, Deserialize)]
pub struct FeaturedDownloadTarget {
	pub feature: String,
	pub targets: Vec<DownloadTarget>,
}

#[derive(Serialize, Deserialize)]
pub struct FeaturedDownloadTargetList {
	pub enabled: Vec<String>,
	pub list: Vec<FeaturedDownloadTarget>,
}
