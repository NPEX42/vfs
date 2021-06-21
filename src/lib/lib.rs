pub const SECTOR_SIZE        : usize = 512;
pub const DISK_SIZE_SECTORS  : usize = 2048;
pub const DISK_SIZE_BYTES    : usize = SECTOR_SIZE * DISK_SIZE_SECTORS;

#[derive(Debug, Clone, Copy)]
pub struct Disk {
    sectors : [Sector; DISK_SIZE_SECTORS]
}

#[derive(Debug, Clone, Copy)]
pub struct Sector {
    data : [u8; SECTOR_SIZE]
}

impl Sector {
    pub fn new() -> Sector {
        Sector {
            data : [0; SECTOR_SIZE]
        }
    }

    pub fn read(&self, offset : usize) -> u8 {
        self.data[offset]
    }

    pub fn write(&mut self, offset : usize, data : u8) {
        self.data[offset] = data
    }
}

impl Disk {
    pub fn new() -> Disk {
        Disk {
            sectors : [Sector::new(); DISK_SIZE_SECTORS]
        }
    }

    pub fn read(&self, address : usize) -> u8 {
        let sector = self.sectors[address / DISK_SIZE_SECTORS];
        sector.read(address % SECTOR_SIZE)
    }

    pub fn write(&mut self, address : usize, data : u8) {
        let mut sector = self.sectors[address / DISK_SIZE_SECTORS];
        sector.write(address % SECTOR_SIZE, data);
    }
}