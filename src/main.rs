use std::fs::File;
use std::io::Read;

mod emotion_engine;

fn main() {
    let mut file = File::open("/Users/riley/Downloads/nfs.iso").unwrap();
    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();
    
    println!("{:?}", buffer.len());
}
