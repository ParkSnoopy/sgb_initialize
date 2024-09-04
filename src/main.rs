use eyre::{ Result };
use downloader::{ Downloader };
use homedir::{ my_home as home };
use ansi_term::{ Color };

pub mod config;
pub mod pprint;
pub mod serializer;
mod builder;



fn main() -> Result<()> {
    pprint::enable_ansi();
    print!("\n\n");

    let home_download_path = {
        let mut tmpdir = home()?.unwrap();
        tmpdir.push("Downloads");
        tmpdir
    };

    let all_download_target: Vec<serializer::DownloadTarget> = {
        pprint::info("Fetching JSON profile from prebuilt remote...");
        let conf = builder::get_config_remote()?;

        let conf = if conf.is_empty() {
            pprint::info("Reading JSON profile from local...");
            builder::get_config_local()?
        } else {
            conf
        };

        println!();

        if conf.is_empty() {
            pprint::failure_fatal("Couldn't fetch any profile");
            pprint::info("Quiting...");
            return Ok(());
        }

        pprint::info(format!("Successfully fetched {} item(s) from profile", conf.len()).as_str());

        conf
    };


    let mut downloader = Downloader::builder()
        .download_folder( &home_download_path )
        .parallel_requests( config::PARALLEL )
        .build()
        .unwrap();


    let mut downloader_prebuild = Vec::new();

    for download_target in all_download_target.iter() {
        downloader_prebuild.push(
            downloader::Download::new( &download_target.url )
                .file_name( &download_target.make_filename_pathbuf() )
        )
    }


    let result = downloader.download( downloader_prebuild.as_slice() )?;

    println!();
    pprint::info("Download Status --\n");
    for r in result {
        match r {
            Err(e) => println!("  {} {}", Color::Fixed(11).paint("-"), e.to_string().trim()),
            Ok (s) => println!("  {} {}", Color::Fixed(10).paint("+"), &s.file_name.display()),
        };
    }


    { // IDLE config::POST_IDLE second(s)
        print!("\n\n");
        pprint::info("All Done!");
        std::thread::sleep(
            std::time::Duration::new(config::POST_IDLE, 0)
        );
    }
    Ok(())
}
