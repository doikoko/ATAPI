# [0.1.1] 12.08.2025

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
## NOTE
you can send your offers, ideas, meaning, chatting with other devs and so on in our Discord server:
https://discord.gg/cwXhbFXm