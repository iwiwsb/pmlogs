use std::{
    fs::File,
    io::{Read, Seek},
};

fn main() {
    let mut procmon_log =
        File::open(".\\test_files\\test_logfile.pml").expect("File should be openable");

    let mut signature_buf = [0u8; 4];
    procmon_log
        .read_exact(&mut signature_buf)
        .expect("File should be readable");
    let signature: String = signature_buf.into_iter().map(|x| char::from(x)).collect();
    println!("Signature: {}", signature);

    let version = get_u32_le(&mut procmon_log);
    println!("Version: {}", version);

    let system_bitness = get_u32_le(&mut procmon_log);
    if system_bitness == 1 {
        println!("System bitness: 64-bit");
    } else {
        println!("System bitness: 32-bit");
    }

    let mut computer_name_buf = [0u8; 32];
    procmon_log
        .read_exact(&mut computer_name_buf)
        .expect("File should be readable");
    let computer_name: String = computer_name_buf
        .chunks(2)
        .map(|x| {
            char::from_u32(u32::from_le_bytes([x[0], x[1], 0, 0])).expect("Should be valid unicode")
        })
        .collect();
    println!("Computer name: {}", computer_name);

    let mut system_root_buf = [0u8; 520];
    procmon_log
        .read_exact(&mut system_root_buf)
        .expect("File should be readable");
    let system_root: String = system_root_buf
        .chunks(2)
        .map(|x| {
            char::from_u32(u32::from_le_bytes([x[0], x[1], 0, 0])).expect("Should be valid unicode")
        })
        .collect();
    println!("System root: {}", system_root);

    let total_number_of_events = get_u32_le(&mut procmon_log);
    println!("Total number of events: {}", total_number_of_events);

    procmon_log
        .seek(std::io::SeekFrom::Current(8))
        .expect("File should be seekable");

    let events_array_start = get_u64_le(&mut procmon_log);
    println!(
        "File offset to the start of the events array: {}",
        events_array_start
    );

    let events_offsets_array_start = get_u64_le(&mut procmon_log);
    println!(
        "File offset to an array of offsets to all the events: {}",
        events_offsets_array_start
    );
}

// fn get_u32_le_at_offset<R: Read + Seek>(reader: &mut R, offset: u64) -> u32 {
//     reader
//         .seek(std::io::SeekFrom::Start(offset))
//         .expect("File should be seekable");
//     get_u32_le(reader)
// }

fn get_u64_le<R: Read>(reader: &mut R) -> u64 {
    let mut buf = [0u8; 8];
    reader
        .read_exact(&mut buf)
        .expect("File should be readable");
    u64::from_le_bytes(buf)
}

fn get_u32_le<R: Read>(reader: &mut R) -> u32 {
    let mut buf = [0u8; 4];
    reader
        .read_exact(&mut buf)
        .expect("File should be readable");
    u32::from_le_bytes(buf)
}
