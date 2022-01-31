use crate::memory::MainMemory;

pub struct Chip8 {
    ram: MainMemory,
}

#[allow(dead_code)]
impl Chip8 {
    /// Create a Chip8 object with ram set to zeros for all 4096 bytes.
    /// 
    /// # Example
    /// 
    /// ```
    /// let chip8 = Chip8::new();
    /// let test = [0; 4096];
    /// 
    /// assert_eq!(&chip8.ram.mem, &test);
    /// ```
    pub fn new() -> Chip8 {
        Chip8 { ram: MainMemory::new() }
    }

    /// Copy a CHIP-8 ROM to the RAM starting from address 0x200.
    /// 
    /// # Example
    /// ```
    /// let test_data: Vec<u8> = vec![123, 0, 211, 100, 8, 23];
    /// let chip8 = Chip8::new();
    /// chip8.load_rom(&test_data);
    /// assert_eq!(&[123, 0, 211, 100, 8, 23], &chip8.ram.mem[512..518]); 
    pub fn load_rom(&mut self, data: &Vec<u8>) {
        let mut offset: u16 = 0x200;
        for eight_bytes in data {
            self.ram.write_bytes(offset, eight_bytes.clone());
            offset += 1;
        }
    }
}