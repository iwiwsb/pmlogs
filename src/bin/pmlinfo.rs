use std::fs::File;

use pmlogs::Header;

fn main() {
    let mut procmon_log =
        File::open(".\\test_files\\test_logfile.pml").expect("File should be openable");
    let header = Header::read_from(&mut procmon_log, 0);
    println!("{:#?}", header);
}
