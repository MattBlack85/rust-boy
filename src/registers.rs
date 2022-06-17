/// The registers used by the gameboy, following the official spec
/// 
///  16bit Hi   Lo   Name/Function
///  AF    A    -    Accumulator & Flags
///  BC    B    C    BC
///  DE    D    E    DE
///  HL    H    L    HL
///  SP    -    -    Stack Pointer
///  PC    -    -    Program Counter/Pointer
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
	Registers {
	    a: 0,
	    b: 0,
	    c: 0,
	    d: 0,
	    e: 0,
	    f: 0,
	    h: 0,
	    l: 0,
	    sp: 0,
	    pc: 0,
	}
    }

    pub fn af(&self) -> u16 {
	(self.a as u16) << 8 | self.f as u16
    }
}

#[cfg(test)]
mod tests {
    use crate::Registers;
    #[test]
    fn check_16bit_registers() {
	let mut r = Registers::new();
	r.a = 0xD2;
	r.f = 0x40;
	    
        assert_eq!(r.af(), 0xD240);
    }
}
