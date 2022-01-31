#[allow(dead_code)]
pub struct MainMemory {
    mem: [u8; 4096],
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl MainMemory {
    /// Create new MainMemory object with 0 as the default value
    /// for the memory of 4096 bytes.
    /// 
    /// # Example
    /// 
    /// ```
    /// let ram = MainMemory::new();
    /// assert_eq!(&ram.mem, &[0; 4096]);
    /// ```
    pub fn new() -> MainMemory {
        let mut ram = MainMemory { mem: [0; 4096] };

        ram.copy_sprite_to_memory();
        ram
    }

    /// Copy predefined CHIP-8 sprites to the memory from address 0x000 to 0x050.
    /// Specifications are defined [here](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#font).
    fn copy_sprite_to_memory(&mut self) {
        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        let mut idx: usize = 0;
        for sprite in sprites.iter() {
            for ch in sprite {
                self.mem[idx] = *ch;
                idx += 1;
            }
        }

    }
    /// Write bytes into memory at the given address.
    /// 
    /// # Example
    /// 
    /// ```
    /// let ram = MainMemory::new();
    /// ram.write_bytes(0x1F, 255);
    /// 
    /// let mut test = [0; 4096];
    /// test[31] = 255;
    /// assert_eq!(&ram.mem, &test);
    /// ```
    pub fn write_bytes(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    /// Read bytes that are stored in the memory at the given address.
    /// 
    /// # Example
    /// 
    /// let ram = MainMemory::new();
    /// ram.write_bytes(3, 255);
    /// assert_eq!(255, ram.read_bytes(3));
    /// ```
    pub fn read_bytes(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }
}