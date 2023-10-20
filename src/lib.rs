pub struct Header {
    pub signature: [u8; 4],
    pub version: u32,
    pub system_bitness: u32,
    pub computer_name: [u16; 16],
    pub system_root: [u16; 256],
    pub total_number_of_events: u32,
    pub unused: u64,
    pub events_array_offset: u64,
    pub processes_array_offset: u64,
    pub array_of_strings_offset: u64,
    pub icons_array_offset: u64,
    pub max_user_address: u64,
    pub os_version_info: OSVersionInfoEx,
    pub number_of_processors: u32,
    pub physical_memory: u32,
    pub events_array_offset_2: u64,
    pub hosts_and_ports_array_offset: u64,
}

#[allow(non_snake_case)]
pub struct OSVersionInfoEx {
    pub os_version_info_size: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub build_number: u32,
    pub platform_id: u32,
    pub CSDVersion: [u16; 256],
    pub service_pack_major: u16,
    pub service_pack_minor: u16,
    pub suite_mask: u16,
    pub product_type: u8,
    pub reserved: u8,
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
