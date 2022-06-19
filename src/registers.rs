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
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
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

    pub fn set_l(&mut self, val: u8) -> u8 {
        self.l = val;
        self.f
    }

    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xFF) as u8;
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0xFF) as u8;
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val & 0xFF) as u8;
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0xFF) as u8;
    }

    pub fn set_sp(&mut self, val: u16) {
        self.sp = val;
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

    pub fn read_h(&self) -> u8 {
        self.h
    }

    pub fn read_l(&self) -> u8 {
        self.l
    }

    pub fn read_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    pub fn read_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn read_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn read_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn read_sp(&self) -> u16 {
        self.sp
    }
}

#[cfg(test)]
mod tests {
    use crate::Registers;

    #[test]
    fn check_16bit_registers() {
        let mut r = Registers::new();

        assert_eq!(r.read_af(), 0x0000);
        r.set_af(0xD240);
        assert_eq!(r.read_a(), 0xD2);
        assert_eq!(r.read_f(), 0x40);
        assert_eq!(r.read_af(), 0xD240);

        assert_eq!(r.read_bc(), 0x0000);
        r.set_bc(0xAF22);
        assert_eq!(r.read_b(), 0xAF);
        assert_eq!(r.read_c(), 0x22);
        assert_eq!(r.read_bc(), 0xAF22);

        assert_eq!(r.read_d(), 0x0000);
        r.set_de(0x09F1);
        assert_eq!(r.read_d(), 0x09);
        assert_eq!(r.read_e(), 0xF1);
        assert_eq!(r.read_de(), 0x09F1);

        assert_eq!(r.read_hl(), 0x0000);
        r.set_hl(0x0110);
        assert_eq!(r.read_h(), 0x01);
        assert_eq!(r.read_l(), 0x10);
        assert_eq!(r.read_hl(), 0x0110);
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

    #[test]
    fn test_sp_register() {
        let mut r = Registers::new();

        assert_eq!(r.read_sp(), 0x0000);
        r.set_sp(0xF94A);
        assert_eq!(r.read_sp(), 0xF94A);
    }
}
