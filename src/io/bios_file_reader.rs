use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Seek};
use std::ptr::read;
use super::utils::{create_io_error, read_bytes};
use std::collections::BTreeMap;

#[repr(C, packed)]
pub struct RomDirEntry {
    name: [u8; 10],
    ext_info_size: u16,
    file_size: u32,
}

#[derive(Debug)]
struct RomDirLocation {
    offset: u64,
    file_offset: u64,
    file_length: usize,
}

pub struct BiosFileReader {
    file: File,
    rom_dirs: BTreeMap<String, RomDirLocation>,
}

#[derive(Copy, Clone, Debug)]
enum BiosZone {
    T10K,
    Test,
    Japan,
    USA,
    Europe,
    HK,
    Free,
    China,
    Unknown(char),
}

#[derive(Copy, Clone, Debug)]
enum BiosType {
    Console,
    Devel,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct BiosVersion {
    zone: BiosZone,
    major_version: String,
    minor_version: String,
    day: String,
    month: String,
    year: String,
    bios_type: BiosType,
}

fn read_rom_dir(file: &mut File, offset: u64) -> Result<RomDirEntry, Box<dyn Error>> {
    file.seek(io::SeekFrom::Start(offset));

    let length = std::mem::size_of::<RomDirEntry>();

    let data = read_bytes(file, offset, length)?;
    
    unsafe {
        Ok(std::ptr::read(data.as_ptr() as *const RomDirEntry))
    }
}

fn find_first_rom_dir(file: &mut File) -> Result<u64, Box<dyn Error>> {
    let mut offset: u64 = 0;

    //The assumption is no rom_dirs can be found after this many iterations
    //PCSX2 makes this assumption so I'll trust them
    for _ in 0..(512 * 1024) {
        //If the file isn't a BIOS file it will eventually error
        //TODO fix the error so it explains the file was not a BIOS file
        let potential_rom_dir = read_rom_dir(file, offset)?;



        let potential_rom_dir_identifier = std::str::from_utf8(&potential_rom_dir.name);

        match potential_rom_dir_identifier {
            Ok(identifier) => if identifier.starts_with("RESET") { 
                return Ok(offset);
            },
            Err(_) => {},
        };

        offset += std::mem::size_of::<RomDirEntry>() as u64;
    }

    Err(create_io_error(io::ErrorKind::InvalidData, "bios file seems to be incorrect or corrupted!"))
}

fn read_all_rom_dirs(file: &mut File) -> Result<BTreeMap<String, RomDirLocation>, Box<dyn Error>> {
    let mut rom_dirs: BTreeMap<String, RomDirLocation> = BTreeMap::new();
    
    let mut offset = find_first_rom_dir(file)?;

    let mut file_offset: u64 = 0;

    loop {
        let rom_dir = read_rom_dir(file, offset)?;

        //Although also undocumented, a zeroed rom_dir indicates the end of the list
        if rom_dir.name.iter().all(|x| *x == 0) {
            break;
        }

        rom_dirs.insert(std::str::from_utf8(&rom_dir.name)?.trim_matches('\u{0}').to_string(), RomDirLocation {
            offset,
            file_offset: file_offset,
            file_length: rom_dir.file_size as usize,
        });

        //Again stolen from PCSX2
        //I suppose it means files in the bios are aligned to the nearest 16 byte interval
        //Ext info size seems useless, maybe it's all packed at the end or something
        let additional_file_offset = rom_dir.file_size as u64;
        file_offset += if additional_file_offset % 0x10 == 0 {
            additional_file_offset
        } else {
            (additional_file_offset + 0x10) & 0xfffffff0
        };

        offset += std::mem::size_of::<RomDirEntry>() as u64;
    }

    Ok(rom_dirs)
}

impl BiosFileReader {
    pub fn new(path: &str) -> Result<BiosFileReader, Box<dyn Error>> {
        let mut file = File::open(path)?;
    
        let rom_dirs = read_all_rom_dirs(&mut file)?;

        Ok(BiosFileReader {
            file,
            rom_dirs,
        })
    }

    pub fn get_all_rom_dir_identifiers(&self) -> Vec<&String> {
        self.rom_dirs.keys().collect()
    }

    pub fn read_rom_dir_data(&mut self, rom_dir_identifier: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let rom_dir_opt = self.rom_dirs.get(&rom_dir_identifier.to_string());

        match rom_dir_opt {
            Some(rom_dir) => Ok(read_bytes(&mut self.file, rom_dir.file_offset, rom_dir.file_length)?),
            None => Err(create_io_error(io::ErrorKind::NotFound, "could not find the specified file in the bios!"))
        }
    }

    pub fn get_bios_version(&mut self) -> Result<BiosVersion, Box<dyn Error>> {
        let bios_version_data = self.read_rom_dir_data("ROMVER")?;
        
        Ok(BiosVersion {
            zone: match bios_version_data[4] as char {
                'T' => BiosZone::T10K,
                'X' => BiosZone::Test,
                'J' => BiosZone::Japan,
                'A' => BiosZone::USA,
                'E' => BiosZone::Europe,
                'H' => BiosZone::HK,
                'P' => BiosZone::Free,
                'C' => BiosZone::China,
                _ => BiosZone::Unknown(bios_version_data[4] as char),
            },
            major_version: std::str::from_utf8(&[bios_version_data[0], bios_version_data[1]])?.to_string(),
            minor_version: std::str::from_utf8(&[bios_version_data[2], bios_version_data[3]])?.to_string(),
            day: std::str::from_utf8(&[bios_version_data[12], bios_version_data[13]])?.to_string(),
            month: std::str::from_utf8(&[bios_version_data[10], bios_version_data[11]])?.to_string(),
            year: std::str::from_utf8(&[bios_version_data[6], bios_version_data[7], bios_version_data[8], bios_version_data[9]])?.to_string(),
            bios_type: match bios_version_data[5] as char {
                'C' => BiosType::Console,
                'D' => BiosType::Devel,
                _ => BiosType::Unknown,
            },
        })
    }
}