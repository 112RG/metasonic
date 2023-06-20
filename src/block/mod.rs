use byteorder::{ReadBytesExt, BE};

use crate::{file::FlacError, from_u8};
use std::{
    io::{BufReader, Read},
    path::PathBuf,
};

use self::stream_info::StreamInfo;
pub(crate) mod stream_info;
pub(crate) mod vorbis_comment;
#[derive(Clone)]
pub struct FlacMeta {
    /// The path from which the blocks were loaded.
    /// The metadata blocks contained in this tag.
    pub blocks: Vec<Block>,
}

impl FlacMeta {
    pub fn new() -> FlacMeta {
        FlacMeta { blocks: Vec::new() }
    }
    /// Adds a block to the tag.
    pub fn push_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
/// The parsed content of a metadata block.
#[derive(Clone, Debug)]
pub enum Block {
    /// A value containing a parsed streaminfo block.
    StreamInfo(StreamInfo),
    /*     /// A value containing a parsed application block.
    Application(Application),
    /// A value containing a parsed cuesheet block.
    CueSheet(CueSheet),
    /// A value containing the number of bytes of padding.
    Padding(u32),
    /// A value containing a parsed picture block.
    Picture(Picture),
    /// A value containing a parsed seektable block.
    SeekTable(SeekTable),
    /// A value containing a parsed vorbis comment block.
    VorbisComment(VorbisComment),
    /// An value containing the bytes of an unknown block.
    Unknown((u8, Vec<u8>)), */
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BlockType {
    StreamInfo,
    Padding,
    Application,
    SeekTable,
    VorbisComment,
    CueSheet,
    Picture,
    Reserved,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct BlockHeader {
    pub block_type: BlockType,
    pub block_size: u32,
}
pub(crate) fn read_block_header(reader: &mut dyn Read) -> Result<BlockHeader, FlacError> {
    let mut header = [0; 4];
    reader
        .read_exact(&mut header)
        .map_err(|e| FlacError::IoError(e.to_string()))?;

    log::debug!("Header: {:?}", header);

    let block_type = match header[0] {
        0 => BlockType::StreamInfo,
        1 => BlockType::Padding,
        2 => BlockType::Application,
        3 => BlockType::SeekTable,
        4 => BlockType::VorbisComment,
        5 => BlockType::CueSheet,
        6 => BlockType::Picture,
        _ => BlockType::Reserved,
    };
    let block_size = header[3];
    Ok(BlockHeader {
        block_type,
        block_size: block_size.into(),
        // ... other block header fields ...
    })
}
pub(crate) fn process_block(reader: &mut dyn Read) -> Block {
    let block_header = read_block_header(reader).unwrap();
    log::debug!(
        "Reading block {:?} with {} bytes",
        block_header.block_type,
        block_header.block_size
    );

    let mut data = Vec::new();
    reader
        .take(block_header.block_size as u64)
        .read_to_end(&mut data)
        .unwrap();

    return match block_header.block_type {
        BlockType::StreamInfo => {
            Block::StreamInfo(crate::block::stream_info::StreamInfo::from_bytes(&data[..]))
        }
        BlockType::Padding => todo!(),
        BlockType::Application => todo!(),
        BlockType::SeekTable => todo!(),
        BlockType::VorbisComment => todo!(),
        BlockType::CueSheet => todo!(),
        BlockType::Picture => todo!(),
        BlockType::Reserved => todo!(),
    };
}

pub(crate) fn skip_block(reader: &mut dyn Read, block_size: u32) -> Result<(), FlacError> {
    let mut buffer = vec![0; block_size as usize];
    reader
        .read_exact(&mut buffer)
        .map_err(|e| FlacError::IoError(e.to_string()))?;
    Ok(())
}
