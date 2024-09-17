use eyre::{ Result };
use downloader::{ Downloader };
use homedir::{ my_home as home };
use nu_ansi_term::{ Color };

pub mod config;
pub mod pprint;
pub mod serializer;
mod builder;



fn main() -> Result<()> {
    pprint::enable_ansi();
    println!();

    let home_download_path = {
        let mut tmpdir = home()?.unwrap();
        tmpdir.push("Downloads");
        tmpdir
    };

    let all_download_target: Vec<serializer::DownloadTarget> = {
        pprint::info("Reading JSON profile from local...");
        let conf = builder::get_config_local()?;

        let conf = if conf.is_empty() {
            println!();
            pprint::info("Fetching JSON profile from default remote...");
            builder::get_config_remote()?
        } else {
            conf
        };

        println!();

        if conf.is_empty() {
            pprint::failure("Config vector empty...");
            pprint::failure_fatal("Couldn't fetch any profile");
            println!();
            pprint::info("Exit");
            return Ok(());
        }

        println!();
        pprint::success(format!("Successfully fetched {} item(s) from profile", conf.len()));

        conf
    };


    let mut downloader = Downloader::builder()
        .download_folder( &home_download_path )
        .parallel_requests( config::CONCURRENT_DOWNLOAD )
        .build()
        .unwrap();


    let mut downloader_prebuild = Vec::new();

    for download_target in all_download_target.iter() {
        downloader_prebuild.push(
            downloader::Download::new( &download_target.url )
                .file_name( &download_target.make_filename_pathbuf() )
        )
    }

    pprint::info("Download in progress...");

    let result = downloader.download( downloader_prebuild.as_slice() )?;

    print!("\n\n");
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
        pprint::info( format!("Program will exit in {} second(s)...", config::POST_IDLE) );
        std::thread::sleep(
            std::time::Duration::new(config::POST_IDLE, 0)
        );
    }
    Ok(())
}
