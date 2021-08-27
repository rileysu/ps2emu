mod emotion_engine;
mod io;

fn main() {
    let mut bios_file = io::BiosFileReader::new("/Users/riley/Downloads/ps2_bios/SCPH-70004_BIOS_V12_PAL_200.BIN").unwrap();

    println!("{:?}", bios_file.get_all_rom_dir_identifiers());

    println!("{:?}", std::str::from_utf8(&bios_file.read_rom_dir_data("ROMVER").unwrap()));
    println!("{:?}", bios_file.get_bios_version());

    /*let mut iso_file = io::ISOFileReader::new("/Users/riley/Downloads/rc3.iso").unwrap();

    let data = iso_file.read_file("CNF/SYS_NET.ICO;1").unwrap();

    println!("{:?}", &data);*/

    /*println!("{:?}", iso_file);

    println!("{:?}", std::str::from_utf8(&iso_file.primary_volume.volume_set_identifier).unwrap());
    println!("{:?}", std::str::from_utf8(&iso_file.primary_volume.publisher_identifier).unwrap());
    println!("{:?}", std::str::from_utf8(&iso_file.primary_volume.data_preparer_identifier).unwrap());
    println!("{:?}", std::str::from_utf8(&iso_file.primary_volume.application_identifier).unwrap());*/
}
