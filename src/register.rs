#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    A,
    B,
    C,
    M,
    Sp,
    Pc,
    Bp,
    Flags,
}

impl Register {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            x if x == Register::A as u8 => Some(Register::A),
            x if x == Register::B as u8 => Some(Register::B),
            x if x == Register::C as u8 => Some(Register::C),
            x if x == Register::M as u8 => Some(Register::M),
            x if x == Register::Sp as u8 => Some(Register::Sp),
            x if x == Register::Pc as u8 => Some(Register::Pc),
            x if x == Register::Bp as u8 => Some(Register::Bp),
            x if x == Register::Flags as u8 => Some(Register::Flags),
            _ => None,
        }
    }
}
