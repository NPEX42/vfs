use vfs_lib::*;

fn main() {
    let mut disk = Disk::new();
    disk.write(0x00, 0xFF);
    assert_eq!(disk.read(0), 255);
}


#[cfg(test)]
mod integration {
    use vfs_lib::*;
    #[test]
    pub fn init_fs() {
        let mut disk = Disk::new();
        disk.write(0x00, 0xFF);
        assert_eq!(disk.read(0), 255);
    }
}