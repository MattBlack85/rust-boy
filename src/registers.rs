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
    a: u8,
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

    pub fn set_a(&mut self, val: u8) -> u8 {
        self.a = val;
        self.a
    }

    pub fn set_b(&mut self, val: u8) -> u8 {
        self.b = val;
        self.b
    }

    pub fn set_c(&mut self, val: u8) -> u8 {
        self.c = val;
        self.c
    }

    pub fn set_d(&mut self, val: u8) -> u8 {
        self.d = val;
        self.d
    }

    pub fn set_e(&mut self, val: u8) -> u8 {
        self.e = val;
        self.e
    }

    pub fn set_f(&mut self, val: u8) -> u8 {
        self.f = val;
        self.f
    }

    pub fn read_a(&self) -> u8 {
        self.a
    }

    pub fn read_b(&self) -> u8 {
        self.b
    }

    pub fn read_c(&self) -> u8 {
        self.c
    }

    pub fn read_d(&self) -> u8 {
        self.d
    }

    pub fn read_e(&self) -> u8 {
        self.e
    }

    pub fn read_f(&self) -> u8 {
        self.f
    }

    pub fn read_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }
}

#[cfg(test)]
mod tests {
    use crate::Registers;

    #[test]
    fn check_16bit_registers() {
        let mut r = Registers::new();
        r.set_a(0xD2);
        r.set_f(0x40);

        assert_eq!(r.read_af(), 0xD240);
    }

    #[test]
    fn test_a_register() {
        let mut r = Registers::new();
        r.set_a(0xF9);

        assert_eq!(r.read_a(), 0xF9);
    }

    #[test]
    fn test_b_register() {
        let mut r = Registers::new();
        r.set_b(0xF9);

        assert_eq!(r.read_b(), 0xF9);
    }

    #[test]
    fn test_c_register() {
        let mut r = Registers::new();
        r.set_c(0xF9);

        assert_eq!(r.read_c(), 0xF9);
    }

    #[test]
    fn test_d_register() {
        let mut r = Registers::new();
        r.set_d(0xF9);

        assert_eq!(r.read_d(), 0xF9);
    }

    #[test]
    fn test_e_register() {
        let mut r = Registers::new();
        r.set_e(0xF9);

        assert_eq!(r.read_e(), 0xF9);
    }

    #[test]
    fn test_f_register() {
        let mut r = Registers::new();
        r.set_f(0xF9);

        assert_eq!(r.read_f(), 0xF9);
    }
}
