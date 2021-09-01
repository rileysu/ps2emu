mod emotion_engine;
mod io;

fn main() {
    let bios_data = Box::new([0u8; 4 * 1024 * 1024]);
    let mut cpu = emotion_engine::Cpu::new(&bios_data, 0);

    let rd: u8 = 1;
    let rs: u8 = 2;
    let rt: u8 = 3;

    //cpu.write_ee_register_32(rs, 1);
    //cpu.write_ee_register_32(rt, 2);

    //emotion_engine::instruction_impl::ADD(&mut cpu, rd, rs, rt);

    //println!("{:?}", cpu.read_ee_register_32(rd));
    
    //let mut bios_file = io::BiosFileReader::new("/Users/riley/Downloads/ps2_bios/SCPH-70004_BIOS_V12_PAL_200.BIN").unwrap();

    //println!("{:?}", bios_file.get_all_rom_dir_identifiers());

    //println!("{:?}", std::str::from_utf8(&bios_file.read_rom_dir_data("ROMVER").unwrap()));
    //println!("{:?}", bios_file.get_bios_version());

    /*let mut iso_file = io::ISOFileReader::new("/Users/riley/Downloads/rc3.iso").unwrap();

    let data = iso_file.read_file("CNF/SYS_NET.ICO;1").unwrap();

    println!("{:?}", &data);*/

    /*println!("{:?}", iso_file);

    println!("{:?}", std::str::from_utf8(&iso_file.primary_volume.volume_set_identifier).unwrap());
    println!("{:?}", std::str::from_utf8(&iso_file.primary_volume.publisher_identifier).unwrap());
    println!("{:?}", std::str::from_utf8(&iso_file.primary_volume.data_preparer_identifier).unwrap());
    println!("{:?}", std::str::from_utf8(&iso_file.primary_volume.application_identifier).unwrap());*/
}
