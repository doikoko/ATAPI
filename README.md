# atapi 10.08.2025

`atapi` `#![no_std]` ATAPI driver, used to bare-metal and OS dev or other drivers. It providing 
comfortable API for work with this protocol

## NOTE
you can send your offers, ideas, meaning, chatting with other devs and so on in our Discord server:
https://discord.gg/cwXhbFXm

## example

Example of work ATAPI with this crate [example/sectors](./example/sectors).

```rust
fn read_pio_lba48(atapi: &ATAPI, sectors: u8, lba: u64, mut buffer: *mut u16) {
    atapi.wait_drq_and_busy().expect("error while read kernel");

    // high bytes
    outb(atapi.io_registers.sector_count_rw_w, sectors);
    outb(atapi.io_registers.lba_low_rw_w,  (lba >> 24) as u8);
    outb(atapi.io_registers.lba_mid_rw_w,  (lba >> 32) as u8);
    outb(atapi.io_registers.lba_high_rw_w, (lba >> 40) as u8);

    // low bytes
    outb(atapi.io_registers.sector_count_rw_w, sectors as u8);
    outb(atapi.io_registers.lba_low_rw_w,  lba as u8);
    outb(atapi.io_registers.lba_mid_rw_w,  (lba >> 8) as u8);
    outb(atapi.io_registers.lba_high_rw_w, (lba >> 16) as u8);

    outb(atapi.io_registers.command_w_or_status_r_b, 
        ATAPIOCommands::ReadSectorsExtW as u8);
    
    atapi.clear_cache();
    for _ in 0..sectors{
        atapi.wait_drq_and_busy().expect("error while read kernel from register");
        
        for _ in 0..256{
            unsafe{ 
                *buffer = inw(atapi.io_registers.data_register_rw_w); 
                buffer = buffer.add(1);
            };
        }
    }
    atapi.wait_drq_and_busy().expect("error after read kernel");
}