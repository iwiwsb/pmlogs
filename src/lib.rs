use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct Header {
    pub signature: String,
    pub version: u32,
    pub system_bitness: u32,
    pub computer_name: String,
    pub system_root: String,
    pub total_number_of_events: u32,
    pub events_array_offset: u64,
    pub processes_array_offset: u64,
    pub strings_array_offset: u64,
    pub icons_array_offset: u64,
    pub max_application_address: u64,
    pub os_version_info: OSVersionInfoEx,
    pub number_of_processors: u32,
    pub total_physical_memory: u64,
    pub hosts_and_ports_array_offset: u64,
}

impl Header {
    pub fn read_from<R: Read + Seek>(reader: &mut R, at_offset: u64) -> Self {
        let mut signature_buf = [0u8; 4];
        reader
            .read_exact(&mut signature_buf)
            .expect("File should be readable");
        let signature: String = signature_buf.into_iter().map(|x| char::from(x)).collect();
        let version = get_u32_le(reader);
        let system_bitness = get_u32_le(reader);
        let mut computer_name_buf = [0u8; 32];
        reader
            .read_exact(&mut computer_name_buf)
            .expect("File should be readable");
        let computer_name: String = computer_name_buf
            .chunks(2)
            .map(|x| {
                char::from_u32(u32::from_le_bytes([x[0], x[1], 0, 0]))
                    .expect("Should be valid unicode")
            })
            .collect();
        let mut system_root_buf = [0u8; 520];
        reader
            .read_exact(&mut system_root_buf)
            .expect("File should be readable");
        let system_root: String = system_root_buf
            .chunks(2)
            .map(|x| {
                char::from_u32(u32::from_le_bytes([x[0], x[1], 0, 0]))
                    .expect("Should be valid unicode")
            })
            .collect();
        let total_number_of_events = get_u32_le(reader);
        reader
            .seek(std::io::SeekFrom::Current(8))
            .expect("File should be seekable");
        let events_array_offset = get_u64_le(reader);
        let events_offsets_array_offset = get_u64_le(reader);
        let processes_array_offset = get_u64_le(reader);
        let strings_array_offset = get_u64_le(reader);
        let icons_array_offset = get_u64_le(reader);
        let max_application_address = get_u64_le(reader);
        let os_ver_info_offset = reader.seek(std::io::SeekFrom::Current(0)).unwrap();
        let os_version_info = OSVersionInfoEx::read_from_at_offset(reader, os_ver_info_offset);
        let number_of_processors = get_u32_le(reader);
        let total_physical_memory = get_u64_le(reader);
        let _ = reader.seek(std::io::SeekFrom::Current(8));
        let hosts_and_ports_array_offset = get_u64_le(reader);

        Self {
            signature,
            version,
            system_bitness,
            computer_name,
            system_root,
            total_number_of_events,
            events_array_offset,
            processes_array_offset,
            strings_array_offset,
            icons_array_offset,
            max_application_address,
            os_version_info,
            number_of_processors,
            total_physical_memory,
            hosts_and_ports_array_offset,
        }
    }
}

#[derive(Debug)]
pub struct OSVersionInfoEx {
    pub os_version_info_size: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub build_number: u32,
    pub platform_id: u32,
    pub csd_version: String,
    pub service_pack_major: u16,
    pub service_pack_minor: u16,
    pub suite_mask: u16,
    pub product_type: u8,
}

impl OSVersionInfoEx {
    pub fn read_from<R: Read + Seek>(reader: &mut R) -> Self {
        let os_version_info_size = get_u32_le(reader);
        let major_version = get_u32_le(reader);
        let minor_version = get_u32_le(reader);
        let build_number = get_u32_le(reader);
        let platform_id = get_u32_le(reader);
        let mut csd_version_buf = [0u8; 256];
        reader
            .read_exact(&mut csd_version_buf)
            .expect("File should be readable");
        let csd_version: String = csd_version_buf
            .chunks(2)
            .map(|x| char::from_u32(u32::from_le_bytes([x[0], x[1], 0, 0])).unwrap())
            .collect();
        let service_pack_major = get_u16_le(reader);
        let service_pack_minor = get_u16_le(reader);
        let suite_mask = get_u16_le(reader);
        let product_type = get_u8(reader);
        reader
            .seek(SeekFrom::Current(1))
            .expect("File should be seekable");
        Self {
            os_version_info_size,
            major_version,
            minor_version,
            build_number,
            platform_id,
            csd_version,
            service_pack_major,
            service_pack_minor,
            suite_mask,
            product_type,
        }
    }

    pub fn read_from_at_offset<R: Read + Seek>(reader: &mut R, at_offset: u64) -> Self {
        reader
            .seek(SeekFrom::Start(at_offset))
            .expect("File should be seekable");
        Self::read_from(reader)
    }
}

pub struct SuiteMask {
    backoffice: bool,
    blade: bool,
    compute_server: bool,
    datacenter: bool,
    enterprise: bool,
    embedded_nt: bool,
    personal: bool,
    single_user_ts: bool,
    small_business: bool,
    small_business_restricted: bool,
    storage_server: bool,
    terminal: bool,
    wh_server: bool,
    multi_user_ts: bool,
}

pub const VER_NT_WORKSTATION: u8 = 1;
pub const VER_NT_DOMAIN_CONTROLLER: u8 = 2;
pub const VER_NT_SERVER: u8 = 3;

pub enum ProductType {
    Workstation,
    DomainController,
    Server,
}

pub fn get_u8<R: Read>(reader: &mut R) -> u8 {
    let bytes: [u8; 1] = get_array(reader);
    bytes[0]
}

pub fn get_u16_le<R: Read>(reader: &mut R) -> u16 {
    let bytes = get_array(reader);
    u16::from_le_bytes(bytes)
}

pub fn get_u32_le<R: Read>(reader: &mut R) -> u32 {
    let bytes = get_array(reader);
    u32::from_le_bytes(bytes)
}

// fn get_u32_le_at_offset<R: Read + Seek>(reader: &mut R, offset: u64) -> u32 {
//     reader
//         .seek(std::io::SeekFrom::Start(offset))
//         .expect("File should be seekable");
//     get_u32_le(reader)
// }

pub fn get_u64_le<R: Read>(reader: &mut R) -> u64 {
    let bytes = get_array(reader);
    u64::from_le_bytes(bytes)
}

fn get_array<R: Read, const N: usize>(reader: &mut R) -> [u8; N] {
    let mut buf = [0u8; N];
    reader
        .read_exact(&mut buf)
        .expect("File should be readable");
    buf
}
