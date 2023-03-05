use ctr_formats::smdh::Smdh;
use ctr_formats::Language;

fn main() {
    let path = std::env::args().skip(1).next()
        .expect("1st argument has to be path to SMDH blob");
    let file = std::fs::read(&path)
        .expect("Failed to read file");
    let smdh = unsafe { Smdh::from_u8_slice(&file) };

    println!("English short name: {}", smdh.short_name(Language::English));
    println!("English long name: {}", smdh.long_name(Language::English));
    println!("English publisher name: {}", smdh.publisher(Language::English));
    println!("Version: {:x}", smdh.version());
    println!("Flags: {:?}", smdh.flags());
    println!("Region: {:?}", smdh.region());
}
