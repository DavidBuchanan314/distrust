use portio::PortIO;

pub const CONFIG_ADDRESS: u16 = 0xcf8;
pub const CONFIG_DATA: u16 = 0xcfc;
pub const CONFIG_DATA_HIWORD: u16 = 0xcfe;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
struct DeviceHeader {
	vendor_id: u16,
	device_id: u16,
	command: u16,
	status: u16,
	revid: u8,
	progif: u8,
	subclass: u8,
	class: u8,
	cl_size: u8,
	latency: u8,
	hdr_type: u8,
	bist: u8,
	bar0: u32,
	bar1: u32,
	bar2: u32,
	bar3: u32,
	bar4: u32,
	bar5: u32,
	cis_ptr: u32,
	sub_vendor_id: u16,
	sub_id: u16,
	rom_base: u32,
	cap_ptr: u8,
	reserved: [u8; 7],
	int_line: u8,
	int_pin: u8,
	min_grant: u8,
	max_latency: u8
}

#[repr(C)]
union DeviceHeaderUnion {
	header: DeviceHeader,
	bytes: [u8; 64],
	words: [u16; 32],
	lwords: [u32; 16]
}

pub struct PCI {
	address: u32,
	addr_dev: u32,
	addr_reg: u32,
	rom_len: bool,
	foo: DeviceHeaderUnion
}

impl PCI {
	pub fn new() -> Self {
		let mut foo = DeviceHeaderUnion { bytes: [0u8; 64] };
		unsafe { // TODO: assign before foo is created for safety...
		foo.header.vendor_id = 0x1234;
		foo.header.device_id = 0x1111;
		foo.header.class = 0x03;
		foo.header.rom_base = 0xC0000;
		foo.header.command = 3;
		}
		Self {
			address: 0x80000000,
			addr_dev: 0,
			addr_reg: 0,
			rom_len: false,
			foo
		}
	}
}

impl PortIO for PCI {
		fn inb(&mut self, port: u16) -> u8 {
		eprintln!("PCI inb(0x{:x})", port);
		match (port, self.addr_dev) {
			(CONFIG_ADDRESS, _) => (self.address & 0xFF) as u8,
			(CONFIG_DATA, 1) => unsafe { self.foo.bytes[(self.addr_reg*4) as usize] },
			(CONFIG_DATA_HIWORD, 1) => unsafe { self.foo.bytes[(self.addr_reg*4+2) as usize] },
			_ => 0xFF
		}
	}
	fn inw(&mut self, port: u16) -> u16 {
		eprintln!("PCI inw(0x{:x})", port);
		match (port, self.addr_dev) {
			(CONFIG_ADDRESS, _) => (self.address & 0xFFFF) as u16,
			(CONFIG_DATA, 1) => unsafe { self.foo.words[(self.addr_reg*2) as usize] },
			(CONFIG_DATA_HIWORD, 1) => unsafe { self.foo.words[(self.addr_reg*2+1) as usize] },
			_ => 0xFFFF
		}
	}
	fn inl(&mut self, port: u16) -> u32 {
		eprintln!("PCI inl(0x{:x})", port);
		match (port, self.addr_dev) {
			(CONFIG_ADDRESS, _) => self.address,
			(CONFIG_DATA, 1) => unsafe { self.foo.lwords[self.addr_reg as usize] },
			_ => 0xFFFFFFFF
		}
	}
	fn outl(&mut self, port: u16, data: u32) {
		match port {
			CONFIG_ADDRESS => {
				self.address = data;
				let enable = data >> 31;
				let bus = (data >> 16) & 0xFF;
				self.addr_dev = (data >> 11) & 0x1F;
				let function = (data >> 8) & 0x7;
				self.addr_reg = (data >> 2) & 0x3F;
				eprintln!("PCI config addr 0x{:08x}: bus={},dev={},func={},reg={}", data, bus, self.addr_dev, function, self.addr_reg);
			},
			CONFIG_DATA => {
				eprintln!("PCI: Writing 0x{:08x} to reg {}", data, self.addr_reg);
				if self.addr_reg == 12 {
					unsafe { self.foo.lwords[self.addr_reg as usize] = data & !0xFFFF }
				}
			}
			_ => eprintln!("PCI outl(0x{:x}, 0x{:x})", port, data)
		};
	}
}
