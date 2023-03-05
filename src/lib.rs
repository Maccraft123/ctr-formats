pub mod smdh;

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Language {
    Japanese = 0,
    English = 1,
    French = 2,
    German = 3,
    Italian = 4,
    Spanish = 5,
    SimplifiedChinese = 6,
    Korean = 7,
    Dutch = 8,
    Portugese = 9,
    Russian = 10,
    TraditionalChinese = 11,
}
