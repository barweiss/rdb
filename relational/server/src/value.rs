pub enum Value {
    Int(i64),
    UInt(u64),
    Float(f64),
    Bit(bool),
    String(String),
    Buffer(Vec<u8>),
}
