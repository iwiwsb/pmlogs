pub struct Header {
    pub signature: [u8; 4],
    pub version: [u8; 4],
    pub system_bitness: [u8; 4],
    pub computer_name: [u8; 32],
    pub system_root: [u8; 512],
    pub total_number_of_events: [u8; 4],
    pub unused: [u8; 8],
    pub events_array_offset: [u8; 8],
    pub processes_array_offset: [u8; 8],
    pub array_of_strings_offset: [u8; 8],
    pub icons_array_offset: [u8; 8],
    pub max_user_address: [u8; 8],
    pub os_version_info: OSVersionInfoEx,
    pub number_of_processors: [u8; 4],
    pub physical_memory: [u8; 8],
    pub events_array_offset_2: [u8; 8],
    pub hosts_and_ports_array_offset: [u8; 8],
}

#[allow(non_snake_case)]
pub struct OSVersionInfoEx {
    pub os_version_info_size: [u8; 4],
    pub major_version: [u8; 4],
    pub minor_version: [u8; 4],
    pub build_number: [u8; 4],
    pub platform_id: [u8; 4],
    pub CSDVersion: [u8; 128],
    pub service_pack_major: [u8; 2],
    pub service_pack_minor: [u8; 2],
    pub suite_mask: [u8; 2],
    pub product_type: [u8; 1],
    pub reserved: [u8; 1],
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
