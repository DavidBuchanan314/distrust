# distrust
A very very WIP type-2 hypervisor in Rust, which uses the Linux KVM API.

The eventual goal is to be able to emulate "Legacy" hardware safely, for example
Cirrus VGA graphics, which has been the source of numerous security bugs in QEMU.

This is my first time using Rust, so there will be a lot of terrible design
decisions which will hopefully later be refactored.

### Current Status:

Basically everything is stubbed out. It's capable of partially booting a SeaBIOS
ROM.

For example, here's the serial debug output from SeaBIOS:

```
Changing serial settings was 0/0 now 3/0
SeaBIOS (version rel-1.11.0-49-gbf8e4f9)
BUILD: gcc: (GCC) 8.2.1 20180831 binutils: (GNU Binutils) 2.31.1
enabling shadow ram
Unable to unlock ram - bridge not found
Add to e820 map: 00000000 ffff0000 1
Add to e820 map: fffc0000 00040000 2
RamSize: 0xffff0000 [cmos]
malloc preinit
Add to e820 map: 000a0000 00050000 -1
Add to e820 map: 000f0000 00010000 2
Add to e820 map: fff80000 00040000 2
phys_alloc zone=0x000e9ad4 size=82368 align=20 ret=fff6bda0 (detail=0xfff6bd70)
Relocating init from 0x000d59e0 to 0xfff6bda0 (size 82368)
malloc init
init ivt
init bda
Add to e820 map: 0009fc00 00000400 2
init bios32
init PMM
init PNPBIOS table
init keyboard
init mouse
init pic
math cp init
pci setup
=== PCI bus & bridge init ===
Detected non-PCI system
No apic - only the main cpu is present.
init PIR table
phys_alloc zone=0xfff7fe9c size=128 align=10 ret=f4ba0 (detail=0xfff6bce0)
Copying PIR from 0xfff7fc60 to 0x000f4ba0
init MPTable
phys_alloc zone=0xfff7fe94 size=32768 align=10 ret=fff63ce0 (detail=0xfff63cb0)
phys_alloc zone=0xfff7fe9c size=208 align=10 ret=f4ad0 (detail=0xfff63c80)
Copying MPTABLE from 0x00006e80/fff63ce0 to 0x000f4ad0
phys_free fff63ce0 (detail=0xfff63cb0)
init SMBIOS tables
phys_alloc zone=0xfff7fe94 size=32768 align=10 ret=fff63ce0 (detail=0xfff63cb0)
phys_alloc zone=0xfff7fe9c size=263 align=10 ret=f49c0 (detail=0xfff63c50)
phys_alloc zone=0xfff7fe9c size=31 align=10 ret=f49a0 (detail=0xfff63c20)
Copying SMBIOS entry point from 0x00006e80 to 0x000f49a0
phys_free fff63ce0 (detail=0xfff63cb0)
load ACPI tables
init ACPI tables
init timer
WARNING - Timeout at wait_reg8:81!
WARNING - Timeout at wait_reg8:81!
Scan for VGA option rom
No VGA found, scan for other display
Turning on vga text mode console
SeaBIOS (version rel-1.11.0-49-gbf8e4f9)
init usb
init ps2port
phys_alloc zone=0xfff7fe94 size=4096 align=1000 ret=fff6a000 (detail=0xfff6bcb0)
/fff6a000\ Start thread
|fff6a000| i8042_flush
|fff6a000| i8042_command cmd=ad
|fff6a000| i8042_wait_write
|fff6a000| i8042_command cmd=a7
|fff6a000| i8042_wait_write
|fff6a000| i8042_flush
|fff6a000| i8042_command cmd=1aa
|fff6a000| i8042_wait_write
|fff6a000| i8042_wait_read
|fff6a000| WARNING - Timeout at i8042_wait_read:38!
|fff6a000| i8042 command 1aa failed
\fff6a000/ End thread
phys_free fff6a000 (detail=0xfff6bcb0)
All threads complete.
init floppy drives
init hard drives
phys_alloc zone=0xfff7fe9c size=20 align=10 ret=f4980 (detail=0xfff6bcb0)
ATA controller 1 at 1f0/3f4/0 (irq 14 dev ffffffff)
phys_alloc zone=0xfff7fe94 size=4096 align=1000 ret=fff6a000 (detail=0xfff6bc80)
/fff6a000\ Start thread
|fff6a000| powerup iobase=1f0 st=0
|fff6a000| powerup iobase=1f0 st=0
|fff6a000| ata_detect ata0-0: sc=0 sn=0 dh=0
|fff6a000| powerup iobase=1f0 st=0
|fff6a000| powerup iobase=1f0 st=0
|fff6a000| ata_detect ata0-1: sc=0 sn=0 dh=0
\fff6a000/ End thread
phys_free fff6a000 (detail=0xfff6bc80)
All threads complete.
phys_alloc zone=0xfff7fe9c size=20 align=10 ret=f4960 (detail=0xfff6bc80)
ATA controller 2 at 170/374/0 (irq 15 dev ffffffff)
phys_alloc zone=0xfff7fe94 size=4096 align=1000 ret=fff6a000 (detail=0xfff6bc50)
/fff6a000\ Start thread
|fff6a000| powerup iobase=170 st=0
|fff6a000| powerup iobase=170 st=0
|fff6a000| ata_detect ata1-0: sc=0 sn=0 dh=0
|fff6a000| powerup iobase=170 st=0
|fff6a000| powerup iobase=170 st=0
|fff6a000| ata_detect ata1-1: sc=0 sn=0 dh=0
\fff6a000/ End thread
phys_free fff6a000 (detail=0xfff6bc50)
All threads complete.
init ahci
init virtio-blk
init virtio-scsi
init lsi53c895a
init esp
init megasas
init pvscsi
init MPT
init nvme
init lpt
Found 0 lpt ports
init serial
Found 0 serial ports
Scan for option roms

Press ESC for boot menu.

Checking for bootsplash
```
