#[derive(Debug)]
pub struct Settings {
    pub custom_lock: bool,
    pub bottom: u32,
    pub top: u32,
    pub modulo: u32,
    pub threads: u32
}

#[derive(Debug)]
pub enum Mode {
    Count,
    List,
    Search(Box<[u8; 20]>),
}
