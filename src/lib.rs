use std::io::{BufReader, Read};

use file::FlacError;

use crate::{
    block::{process_block, BlockType, FlacMeta},
    file::valid_flac_marker,
};

//pub mod blocks;
pub mod block;
pub mod file;
const FLAC_MARKER: [u8; 4] = [0x66, 0x4C, 0x61, 0x43];

pub fn process_flac(file: &mut dyn Read) -> Result<(), FlacError> {
    // Create a bufreader here since it faster then the normal reader
    let mut reader = BufReader::new(file);
    // Check if a valid flac marker can be found if not then return
    valid_flac_marker(&mut reader)?;

    //let mut flac_file = FlacMeta::new();

    loop {
        log::debug!("{:?}", process_block(&mut reader));
    }

    //log::debug!("{:?}", flac_file.blocks);

    Ok(())
}
fn from_u8(n: u8) -> BlockType {
    match n {
        0 => BlockType::StreamInfo,
        1 => BlockType::Padding,
        2 => BlockType::Application,
        3 => BlockType::SeekTable,
        4 => BlockType::VorbisComment,
        5 => BlockType::CueSheet,
        6 => BlockType::Picture,
        n => BlockType::Reserved,
    }
}
