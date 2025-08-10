# atapi 10.08.2025

`atapi` `#![no_std]` ATAPI driver, used to bare-metal and OS dev or other drivers. It providing 
comfortable API for work with this protocol

## NOTE
join my Discord Server with low-level developers, WELCOME: https://discord.gg/cwXhbFXm

## example

Example of work ATAPI with this crate [example/sectors](./example/sectors).

```rust
pub fn read_pio_lba48(atapi: &ATAPI, sectors: u16, lba: u64, buffer: *mut u16) {
    unsafe {
        atapi.wait_busy();

        // Device/Head: LBA mode, Master (0x40 | master_bit)
        outb(atapi.io_registers.device_or_head_rw_b, 0x40);

        // high bytes
        outb(atapi.io_registers.sector_count_rw_w, (sectors >> 8) as u8);
        outb(atapi.io_registers.lba_low_rw_w,  (lba >> 24) as u8);
        outb(atapi.io_registers.lba_mid_rw_w,  (lba >> 32) as u8);
        outb(atapi.io_registers.lba_high_rw_w, (lba >> 40) as u8);

        // low bytes
        outb(atapi.io_registers.sector_count_rw_w, sectors as u8);
        outb(atapi.io_registers.lba_low_rw_w,  lba as u8);
        outb(atapi.io_registers.lba_mid_rw_w,  (lba >> 8) as u8);
        outb(atapi.io_registers.lba_high_rw_w, (lba >> 16) as u8);

        outb(atapi.io_registers.command_w_or_status_r_b, ATAPIOCommands::ReadSectorsExtW as u8);

        let mut ptr = buffer;

        for _ in 0..sectors {
            if let Some(_) = atapi.wait_drq_and_busy(){
                panic!("error")
            }

            for _ in 0..256 {
                let w = inw(atapi.io_registers.data_register_rw_w);
                core::ptr::write_volatile(ptr, w);
                ptr = ptr.add(1);
            }
        }
        atapi.clear_cache();
    }
}
