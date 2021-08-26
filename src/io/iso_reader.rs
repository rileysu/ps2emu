use std::ops::Index;
use std::{convert::TryInto, fs::File};
use std::io::{self, Read, Seek};
use std::collections::BTreeMap;
use std::error::Error;

pub const SECTOR_SIZE: u64 = 2 * 1024;

pub enum VolumeType {
    BootRecord,
    PrimaryVolumeDescriptor,
    SuppVolumeDescriptor,
    VolumePartitionDescriptor,
    Unknown(i8),
    VolumeDescriptorSetTerminator,
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct bi_u16 {
    le: u16,
    be: u16
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct bi_u32 {
    le: u32,
    be: u32
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct BasicVolume {
    vtype: u8,
    identifier: [u8; 5],
    version: u8,
    data: [u8; 2041], 
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct BootRecordVolume {
    vtype: u8,
    identifier: [u8; 5],
    version: u8,
    boot_system_identifier: [u8; 32],
    boot_identifier: [u8; 32],
    data: [u8; 1977],
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct PrimaryVolume {
    vtype: u8,
    identifier: [u8; 5],
    version: u8,
    _unused1: u8,
    system_identifier: [u8; 32],
    volume_identifier: [u8; 32],
    _unused2: [u8; 8],
    volume_space_size: bi_u32,
    _unused3: [u8; 32],
    volume_set_size: bi_u16,
    volume_sequence_number: bi_u16,
    logical_block_size: bi_u16,
    path_table_size: bi_u32,
    type_l_path_table_location: u32,
    opt_type_l_path_table_location: u32,
    type_m_path_table_location: u32,
    opt_type_m_path_table_location: u32,
    root_directory: BaseDirectoryRecord,
    _unused4: u8,
    pub volume_set_identifier: [u8; 128],
    pub publisher_identifier: [u8; 128],
    pub data_preparer_identifier: [u8; 128],
    pub application_identifier: [u8; 128],
    copyright_file_identifier: [u8; 37],
    abstract_file_identifier: [u8; 37],
    bibliographic_file_identifier: [u8; 37],
    volume_created_datetime: [u8; 17],
    volume_modification_datetime: [u8; 17],
    volume_expiration_datetime: [u8; 17],
    volume_effective_datetime: [u8; 17],
    file_structure_version: u8,
    _unused5: u8,
    application_data: [u8; 512],
    reserved_data: [u8; 653],
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct DescriptorSetTerminatorVolume {
    vtype: i8,
    identifier: [u8; 5],
    version: i8,
    data: [u8; 2041], 
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct BaseDirectoryRecord {
    length: u8,
    extended_attribute_length: u8,
    extent_location: bi_u32,
    extent_length: bi_u32,
    recording_date: [u8; 7],
    file_flags: u8,
    file_unit_size_int: u8,
    int_gap_size: u8,
    volume_seq_number: bi_u16,
    file_identifier_length: u8,
}

#[derive(Clone, Debug)]
pub struct DirectoryRecord {
    base: BaseDirectoryRecord,
    file_identifier: Vec<u8>
}

#[derive(Copy, Clone, Debug)]
pub enum Volume {
    Basic(BasicVolume),
    BootRecord(BootRecordVolume),
    Primary(PrimaryVolume),
    DescriptorSetTerminator(DescriptorSetTerminatorVolume),
}

#[derive(Copy, Clone, Debug)]
pub struct FileLocation {
    offset: u64,
    length: u64,
}

type PathLocationFinder = BTreeMap<Vec<String>, FileLocation>;

#[derive(Debug)]
pub struct ISOFileReader {
    file: File,
    pub primary_volume: PrimaryVolume,
    pub path_locations: PathLocationFinder,
}

fn read_bytes(file: &mut File, offset: u64, length: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    file.seek(io::SeekFrom::Start(offset))?;

    let mut data: Vec<u8> = vec![0; length];
    
    let bytes_read = file.read(&mut data)?;

    if bytes_read == length {
        Ok(data)
    } else {
        Err(Box::new(io::Error::new(io::ErrorKind::UnexpectedEof, "could not read full length of bytes!")))
    }
}

fn get_volume_type(data: &[u8; SECTOR_SIZE as usize]) -> u8 {
    data[0]
}

fn read_primary_volume(file: &mut File, offset: u64) -> Result<PrimaryVolume, Box<dyn Error>> {
    let length: usize = std::mem::size_of::<PrimaryVolume>();

    let data = read_bytes(file, offset, length)?;
    
    Ok(
        unsafe {
            //Struct should be aligned in memory even if attributes arent
            //The function will fail if the buffer was not entirely filled
            std::ptr::read(data.as_ptr() as *const PrimaryVolume)
        }
    )
}

fn read_directory_record(file: &mut File, offset: u64) -> Result<DirectoryRecord, Box<dyn Error>> {
    let base_record_length: usize = std::mem::size_of::<BaseDirectoryRecord>();

    let data = read_bytes(file, offset, base_record_length)?;

    let base = unsafe {
        std::ptr::read(data.as_ptr() as *const BaseDirectoryRecord)
    };

    let identifier_length = base.file_identifier_length;

    let file_identifier = read_bytes(file, offset + base_record_length as u64, identifier_length as usize)?;
    
    Ok(
        DirectoryRecord {
            base,
            file_identifier,
        }
    )
}

fn read_volume(data: &[u8; SECTOR_SIZE as usize]) -> Volume {
    println!("{:?}", std::str::from_utf8(&data[1..6]));

    match get_volume_type(data) {
        v => Volume::Basic(BasicVolume {
            vtype: v,
            identifier: data[1..6].try_into().unwrap(),
            version: 0,
            data: [0; 2041]
        })
    }
}

fn read_directory_children(file: &mut File, dir: &BaseDirectoryRecord) -> Result<Vec<DirectoryRecord>, Box<dyn Error>> {
    let mut final_records: Vec<DirectoryRecord> = vec![];
    
    let children_offset = dir.extent_location.le;
    let children_length = dir.extent_length.le;

    let mut bytes_read: u64 = 0;
    let mut wasted_bytes: u64 = 0;

    while bytes_read < children_length as u64 {
        let new_dir = read_directory_record(file, children_offset as u64 * SECTOR_SIZE + bytes_read + wasted_bytes)?;

        //Spec will waste bytes if a directory record crosses a sector boundary
        //Thankfully it is guaranteed to be zero padded so the length will be zero in this case
        //Unless I was lied to >:(
        if new_dir.base.length != 0 {
            bytes_read += new_dir.base.length as u64;
            final_records.push(new_dir);
        } else {
            wasted_bytes += SECTOR_SIZE - (bytes_read + wasted_bytes) % SECTOR_SIZE;
            println!("Crossed boundary");
        }
    }

    Ok(final_records)
}

fn is_file_string(file_identifier: &String) -> bool {
    file_identifier.contains(";")
}

fn is_special_file_string(file_identifier: &String) -> bool {
    file_identifier.eq("\u{0}") || file_identifier.eq("\u{1}")
}

fn parse_identifier(identifier: &Vec<u8>) -> Result<String, Box<dyn Error>> {
    Ok(std::str::from_utf8(identifier)?.to_string())
}

fn parse_file_identifier(identifier: &Vec<u8>) -> Result<String, Box<dyn Error>> {
    Ok(parse_identifier(identifier)?.trim_end_matches(|c: char| c == ';' || c.is_numeric()).to_string())
}

fn generate_path_location_finder(file: &mut File, root_dir: &BaseDirectoryRecord) -> Result<PathLocationFinder, Box<dyn Error>> {
    let mut path_location_finder: PathLocationFinder = BTreeMap::new();
    
    let init_dir = DirectoryRecord {
        base: *root_dir,
        file_identifier: vec![],
    };
    let mut search_dirs: Vec<(DirectoryRecord, Vec<String>)> = vec![(init_dir, vec![])];

    while !search_dirs.is_empty() {
        let (search_dir, path) = search_dirs.pop().unwrap();

        let children_dirs = read_directory_children(file, &search_dir.base)?;
        
        for child_dir in children_dirs {
            if !is_special_file_string(&parse_identifier(&child_dir.file_identifier)?) {


                if is_file_string(&parse_identifier(&child_dir.file_identifier)?) {
                    let mut new_path = path.clone();
                    new_path.append(&mut vec![parse_file_identifier(&child_dir.file_identifier)?]);

                    path_location_finder.insert(new_path, FileLocation {
                        offset: child_dir.base.extent_location.le as u64 * SECTOR_SIZE,
                        length: child_dir.base.extent_length.le as u64,
                    });
                } else {
                    let mut new_path = path.clone();
                    new_path.append(&mut vec![parse_identifier(&child_dir.file_identifier)?]);

                    search_dirs.push((child_dir, new_path));
                }
            }
        }
    }
    
    Ok(path_location_finder)
}

impl ISOFileReader {
    fn new(path: &str) -> Result<ISOFileReader, Box<dyn Error>> {
        let file = File::open(path).unwrap();
        let primary_volume = read_primary_volume(&mut file, 16 * SECTOR_SIZE)?;
        let path_locations = generate_path_location_finder(&mut file, &primary_volume.root_directory)?;

        Ok(ISOFileReader {
            file,
            primary_volume,
            path_locations,
    })
    }

    fn read_file(&mut self, path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let conv_path: Vec<String> = String::from(path).split("/").map(|string| string.to_string()).collect();

        match self.path_locations.get(&conv_path) {
            Some(file_location) => {
                Ok(read_bytes(&mut self.file, file_location.offset, file_location.length as usize)?)
            },
            None => Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "could not find file in iso!")))
        }
    }
}