use std::{
    fs::{File, FileType},
    time::Instant,
};

use metasonic::file::valid_flac_marker;
use walkdir::WalkDir;
macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                log::error!("An error occured: {:?}; skipped.", e);
                continue;
            }
        }
    };
}
fn main() {
    env_logger::init();
    // Open the FLAC file
    let mut file: File = File::open("./test_assets/full_test.flac").expect("Failed to open file");
    let start_time = Instant::now();
    let tag = metaflac::Tag::read_from_path("./test_assets/full_test.flac").unwrap();
    let elapsed_time = start_time.elapsed();
    log::debug!("Elapsed time metaflac: {:?}", elapsed_time);
    // Parse the FLAC header and VORBIS_COMMENT block
    //valid_flac_marker(&mut file).unwrap();
    println!("Passed");
    let start_time = Instant::now();
    metasonic::process_flac(&mut file).expect("Failed to open file");
    let elapsed_time = start_time.elapsed();
    log::debug!("Elapsed time: {:?}", elapsed_time);
    /*
    for entry in WalkDir::new(String::from("H:\\aa"))
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|f| {
            !f.path()
                .iter()
                .any(|s| s.to_str().map(|x| x.starts_with('.')).unwrap_or(false))
        })
    {
        if entry.file_type().is_file() {
            if entry.path().extension() == Some(std::ffi::OsStr::new("flac")) {
                println!("{:?}", entry.path());
                let mut file: File = File::open(entry.path()).unwrap();
                skip_fail!(valid_flac_marker(&mut file));
            }
        }
    } */
}
