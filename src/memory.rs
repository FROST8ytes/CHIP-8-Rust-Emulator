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
        MainMemory { mem: [0; 4096] }
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