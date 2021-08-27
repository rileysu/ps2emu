use std::fs::File;
use std::error::Error;
use std::io::{self, Read, Seek};

pub fn create_io_error(kind: io::ErrorKind, error: &str) -> Box<io::Error> {
    Box::new(io::Error::new(kind, error))
}

pub fn read_bytes(file: &mut File, offset: u64, length: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    file.seek(io::SeekFrom::Start(offset))?;

    let mut data: Vec<u8> = vec![0; length];
    
    let bytes_read = file.read(&mut data)?;

    if bytes_read == length {
        Ok(data)
    } else {
        Err(create_io_error(io::ErrorKind::UnexpectedEof, "could not read full length of bytes!"))
    }
}