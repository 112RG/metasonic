use std::collections::HashMap;

// VorbisComment {{{
/// A structure representing a VORBIS_COMMENT block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VorbisComment {
    /// The vendor string.
    pub vendor_string: String,
    /// A map of keys to a list of their values.
    pub comments: HashMap<String, Vec<String>>,
}
impl VorbisComment {
    /// Returns a new `VorbisComment` with an empty vendor string and no comments.
    pub fn new() -> VorbisComment {
        VorbisComment {
            vendor_string: String::new(),
            comments: HashMap::new(),
        }
    }

    /// Attempts to parse the bytes as a vorbis comment block. Returns a `VorbisComment` on
    /// success.
    pub fn from_bytes(bytes: &[u8]) -> Result<VorbisComment, ()> {
        let mut vorbis = VorbisComment::new();
        let mut i = 0;

        let vendor_length = u32::from_le_bytes((&bytes[i..i + 4]).try_into().unwrap()) as usize;
        i += 4;

        vorbis.vendor_string = String::from_utf8(bytes[i..i + vendor_length].to_vec()).unwrap();
        i += vendor_length;

        let num_comments = u32::from_le_bytes((&bytes[i..i + 4]).try_into().unwrap());
        i += 4;

        for _ in 0..num_comments {
            let comment_length =
                u32::from_le_bytes((&bytes[i..i + 4]).try_into().unwrap()) as usize;
            i += 4;

            let comments = String::from_utf8(bytes[i..i + comment_length].to_vec()).unwrap();
            i += comment_length;

            let comments_split: Vec<&str> = comments.splitn(2, '=').collect();
            let key = comments_split[0].to_ascii_uppercase();
            let value = comments_split[1].to_owned();

            vorbis
                .comments
                .entry(key)
                .or_insert_with(|| Vec::with_capacity(1))
                .push(value);
        }

        Ok(vorbis)
    }
}
