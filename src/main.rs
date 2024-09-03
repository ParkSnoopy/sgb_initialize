// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2020 Tobias Hunger <tobias.hunger@gmail.com>

#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::path::Path;
use eyre::Result;

use downloader::Downloader;
use homedir::{ my_home as home };

pub mod config;
mod config_builder;


fn main() -> Result<()> {
    let home_download_path = {
        let mut tmpdir = home()?.unwrap();
        tmpdir.push("Downloads");
        tmpdir
    };
    let to_downloads = config_builder::get_to_download(); // Vec of (download url, download filename)


    let mut downloader = Downloader::builder()
        .download_folder( &home_download_path )
        .parallel_requests( config::PARALLEL )
        .build()
        .unwrap();


    let mut downloader_prebuild = Vec::new();

    for (url, filename) in to_downloads.iter() {
        downloader_prebuild.push(
            downloader::Download::new(url)
                .file_name( Path::new(filename) )
        )
    }


    let result = downloader.download( downloader_prebuild.as_slice() )?;

    for r in result {
        match r {
            Err(e) => println!("[-] {}", e.to_string().trim()),
            Ok (s) => println!("[+] {}", &s.file_name.display()),
        };
    }


    { // IDLE config::POST_IDLE second(s)
        println!("\n\n[i] All Done!");
        std::thread::sleep(
            std::time::Duration::new(config::POST_IDLE, 0)
        );
    }
    Ok(())
}
