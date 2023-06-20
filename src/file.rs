use std::io::Read;

use crate::FLAC_MARKER;

#[derive(Debug, PartialEq)]
pub enum FlacError {
    InvalidMarker,
    IoError(String),
}

pub fn valid_flac_marker(file: &mut dyn Read) -> Result<(), FlacError> {
    let mut ident = [0; 4]; // Create a buffer to store the file identifier
    file.read_exact(&mut ident)
        .map_err(|err| FlacError::IoError(err.to_string()))?; // Convert the io::Error to a FlacError with a custom message    log::debug!("Checking flac file");

    if ident == FLAC_MARKER {
        log::debug!("Valid flac file found");
        Ok(()) // FLAC marker is valid
    } else {
        log::debug!("Invalid flac file found");
        Err(FlacError::InvalidMarker)
    }
}

#[cfg(test)]
mod test_marker {
    use super::*;
    use std::{fs::File, io};
    // Mock struct that implements the Read trait and always returns an error
    struct MockReader;

    impl Read for MockReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "Mock I/O error"))
        }
    }
    #[test]
    fn test_valid_flac_marker() {
        let mut file = File::open("./test_assets/full_test.flac").expect("Failed to open file");
        assert_eq!(valid_flac_marker(&mut file), Ok(()));
    }

    #[test]
    fn test_valid_flac_marker_invalid_marker() {
        let mut file = File::open("./test_assets/invalid.flac").expect("Failed to open file");
        assert_eq!(valid_flac_marker(&mut file), Err(FlacError::InvalidMarker));
    }

    #[test]
    fn test_valid_flac_marker_io_error() {
        let mut reader = MockReader;
        let result = valid_flac_marker(&mut reader);
        assert_eq!(result, Err(FlacError::IoError("Mock I/O error".to_owned())));
    }
}
