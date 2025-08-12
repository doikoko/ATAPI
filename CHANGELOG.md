# [0.1.2] 12.08.2025

# remove:
``` rust
fn set_master_or_slave(&self, master_or_slave: MasterOrSlave);
```

# add:
1) 
``` rust
fn set_flags(&self, master_or_slave: MasterOrSlave, lba_or_chs: LBAOrCHS);
```

2) 
``` rust
#[repr(u8)]
pub enum LBAOrCHS{
    LBA = 1 << 6,
    CHS = 0,
}
```

# edit:
example function has been edited
``` rust
fn read_pio_lba48(atapi: &ATAPI, sectors: u16, lba: u64, mut buffer: *mut u16);
```

## NOTE
you can send your offers, ideas, meaning, chatting with other devs and so on in our Discord server:
https://discord.gg/cwXhbFXm