use crate::config;
use crate::pprint;
use crate::serializer::{ FeaturedDownloadTargetList, DownloadTarget };

use std::fs::{ File };
use std::io::{ BufReader };
use std::path::{ Path };

use eyre::{ Result };
use tempfile::{ TempDir };
use downloader::{ Downloader };
use serde_json;


pub fn get_config_local() -> Result<Vec<DownloadTarget>> {
	let file = File::open( config::DEFAULT_CONFIG_JSON_LOCAL )?;
	let reader = BufReader::new( file );

	let fdtl: FeaturedDownloadTargetList = serde_json::from_reader( reader )?;

	// collection vector
	let mut all_dt: Vec<DownloadTarget> = Vec::new();

	// foreach: enabled
	for enabled_feature in fdtl.enabled.iter() {
		// find: enabled - from: list
		for fdt in fdtl.list.iter() {
			// if: found: append
			if fdt.feature == *enabled_feature {
				all_dt.extend( fdt.targets.clone() );
				break;
			}
		}
	}

	Ok(all_dt)
}


pub fn get_config_remote() -> Result<Vec<DownloadTarget>> {

	let tempdir = TempDir::new()?;

	let mut downloader = Downloader::builder()
        .download_folder( tempdir.path() )
        .build()
        .unwrap();

    let json_download = downloader::Download::new(format!("{}{}",
    		config::DEFAULT_CONFIG_JSON_REMOTE_BASE_URL,
    		config::DEFAULT_CONFIG_JSON_REMOTE_FILENAME,
    	).as_str())
        .file_name( Path::new( config::DEFAULT_CONFIG_JSON_REMOTE_FILENAME ) );

    let result = downloader.download( &[json_download] )?;
    for r in result {
        match r {
            Err(_) => {
            	pprint::failure("Failed to fetch default JSON profile");
            	return Ok(Vec::new());
            },
            Ok (_) => {
            	pprint::success("Successfully fetched default JSON profile");
            },
        };
    }

	let file = File::open( tempdir.path().join( config::DEFAULT_CONFIG_JSON_REMOTE_FILENAME ) )?;
	let reader = BufReader::new( file );

	let fdtl: FeaturedDownloadTargetList = serde_json::from_reader( reader )?;

	// collection vector
	let mut all_dt: Vec<DownloadTarget> = Vec::new();

	// foreach: enabled
	for enabled_feature in fdtl.enabled.iter() {
		// find: enabled - from: list
		for fdt in fdtl.list.iter() {
			// if: found: append
			if fdt.feature == *enabled_feature {
				all_dt.extend( fdt.targets.clone() );
				break;
			}
		}
	}

	Ok(all_dt)
}
