use std::fs::File;
use std::io::Write;

//mod portio; // should this be part of hw?
use portio::PortIO;

const REG_DATA: u16 = 0;
//const REG_INT_EN: u16 = 1;
const REG_LINESTATUS: u16 = 5;
//const REG_MODEMSTATUS: u16 = 6;
//const REG_SCRATCH: u16 = 7;


#[derive(Debug)]
pub struct SerialPort {
	pub base: u16,
	infile: File,
	outfile: File,
	dlab: bool, // Divisor Latch Access Bit
	scratch: u8,
}

impl SerialPort {
	pub fn new(base: u16, infile: File, outfile: File) -> Self {
		Self {
			base,
			infile,
			outfile,
			dlab: false,
			scratch: 0
		}
	}
}

impl PortIO for SerialPort {
	fn inb(&mut self, port: u16) -> u8 {
		let offset = port - self.base;
		match (offset, self.dlab) {
			(REG_LINESTATUS, _) => 0xFF,
			_ => { eprintln!("serial read +{}", offset); 0 }
		}
	}
	fn outb(&mut self, port: u16, data: u8) {
		let offset = port - self.base;
		match (offset, self.dlab) {
			(REG_DATA, false) => { self.outfile.write(&[data]).unwrap(); },
			(REG_LINESTATUS, _) => {loop{};},
			_ => eprintln!("serial write +{}: 0x{:x}", offset, data)
		};
	}
}
