#![no_std]
#![no_main]

use atapi::{inw, outb, ATAPIOCommands, DMAOrPIO, LBAOrCHS, MasterOrSlave, PrimaryOrSecondary, ATAPI};

// example, how you can to read sectors from cd using atapi
// for master channel lba 48
fn read_pio_lba48(atapi: &ATAPI, sectors: u16, lba: u64, mut buffer: *mut u16) {
    atapi.wait_drq_and_busy().expect("error while read kernel");

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


fn main(){
    // creating a variable for manipulating ATAPI
    let atapi = ATAPI::new(PrimaryOrSecondary::Primary);
    if !atapi.is_has_device(){
        panic!("has not device");
    }
    atapi.set_flags(MasterOrSlave::Master, LBAOrCHS::LBA);
    atapi.set_dma_or_pio(DMAOrPIO::PIO);

    let buffer = 0 as *mut u16;
    read_pio_lba48(&atapi, 1, 0, buffer);
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> !{
    loop { unsafe { core::arch::asm!("hlt") } }
}