use std::io::Read;

use byteorder::{ReadBytesExt, BE};

use crate::file::FlacError;

#[derive(Debug, Clone, Eq, PartialEq)]
/// A structure representing a STREAMINFO block.

pub struct StreamInfo {
    pub min_block_size: u16, // Minimum block size in samples
    pub max_block_size: u16, // Maximum block size in samples
    pub min_frame_size: u32, // Minimum frame size in bytes
    pub max_frame_size: u32, // Maximum frame size in bytes
    pub sample_rate: u32,    // Sample rate in Hz
    pub num_channels: u8,    // Number of channels
    pub bit_depth: u8,       // Bit depth per sample
    pub total_samples: u64,  // Total number of samples in the stream
                             //  pub md5: Vec<u8>,
}

impl StreamInfo {
    pub fn new() -> StreamInfo {
        StreamInfo {
            min_block_size: 0,
            max_block_size: 0,
            min_frame_size: 0,
            max_frame_size: 0,
            sample_rate: 0,
            num_channels: 0,
            bit_depth: 0,
            total_samples: 0,
            // md5: Vec::new(),
        }
    }
    /// Parses the bytes as a StreamInfo block.
    pub fn from_bytes(bytes: &[u8]) -> StreamInfo {
        let mut streaminfo = StreamInfo::new();

        streaminfo.min_block_size = u16::from_be_bytes(bytes[0..2].try_into().unwrap());
        streaminfo.max_block_size = u16::from_be_bytes(bytes[2..4].try_into().unwrap());
        streaminfo.min_frame_size = (&bytes[4..7]).read_uint::<BE>(3).unwrap() as u32;
        streaminfo.max_frame_size = (&bytes[7..10]).read_uint::<BE>(3).unwrap() as u32;

        let sample_first = u16::from_be_bytes(bytes[10..12].try_into().unwrap());
        let sample_channel_bps = bytes[12];
        streaminfo.sample_rate = (sample_first as u32) << 4 | (sample_channel_bps as u32) >> 4;
        streaminfo.num_channels = ((sample_channel_bps >> 1) & 0x7) + 1;

        let bps_total = (&bytes[13..18]).read_uint::<BE>(5).unwrap();
        streaminfo.bit_depth = ((sample_channel_bps & 0x1) << 4 | (bps_total >> 36) as u8) + 1;
        streaminfo.total_samples = bps_total & 0xF_FF_FF_FF_FF;

        // streaminfo.md5 = bytes[18..34].to_vec();

        streaminfo
    }
}

/* #[cfg(test)]
mod test_stream_info {
    use std::{fs::File, io::BufReader};

    use crate::block::{stream_info::StreamInfo, BlockType};

    #[test]
    fn test_stream_info_read() {
        let file = File::open("./test_assets/full_test.flac").expect("Failed to open file");
        let mut reader = BufReader::new(file);

        // We dont really want to test for a valif flac marker but it progresses the reader. And all tests should fail if the valid_flac_marker is failing
        crate::file::valid_flac_marker(&mut reader).unwrap();

        let block_header = crate::block::read_block_header(&mut reader).unwrap();
        assert_eq!(block_header.block_type, BlockType::StreamInfo);
        assert_eq!(block_header.block_size, 34);

        let block = crate::block::stream_info::read_block(&mut reader).unwrap();

        let mut base_block = StreamInfo::new();
        base_block.min_block_size = 4608;
        base_block.max_block_size = 4608;
        base_block.min_frame_size = 783;
        base_block.max_frame_size = 4744;
        base_block.sample_rate = 48000;
        base_block.num_channels = 2;
        base_block.bit_depth = 16;
        base_block.total_samples = 68546;
        assert_eq!(block, base_block);
    }
}
 */
