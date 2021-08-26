mod emotion_engine;
mod io;

fn main() {
    let mut iso_file = io::read_iso("/Users/riley/Downloads/nfs.iso").unwrap();

    /*println!("{:?}", iso_file);

    println!("{:?}", std::str::from_utf8(&iso_file.primary_volume.volume_set_identifier).unwrap());
    println!("{:?}", std::str::from_utf8(&iso_file.primary_volume.publisher_identifier).unwrap());
    println!("{:?}", std::str::from_utf8(&iso_file.primary_volume.data_preparer_identifier).unwrap());
    println!("{:?}", std::str::from_utf8(&iso_file.primary_volume.application_identifier).unwrap());*/
}
