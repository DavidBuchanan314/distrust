use portio::PortIO;
use std::time::SystemTime;

pub const REG_IDX: u16 = 0x70;
pub const REG_DATA: u16 = 0x71;

pub struct CMOS {
	nmi: bool,
	idx: usize,
	regs: [u8; 128],
	now: SystemTime,
}

impl CMOS {
	pub fn new() -> Self {
		let mut regs = [0; 128];
		//regs[0x34] = 0x80;
		regs[0x35] = 0x07;
		Self {
			nmi: false,
			idx: 0xd,
			regs,
			now: SystemTime::now()
		}
	}
}

impl PortIO for CMOS {
	fn inb(&mut self, port: u16) -> u8 {
		if port == REG_DATA {
			let res;
			if self.idx == 0 {
				res = (self.now.elapsed().unwrap().as_secs() &0xFF) as u8;
			} else {
				res = self.regs[self.idx];
			}
			eprintln!("CMOS[0x{:x}] => 0x{:x}", self.idx, res);
			self.idx = 0xd;
			res
		} else {
			0
		}
	}
	fn outb(&mut self, port: u16, data: u8) {
		if port == REG_IDX {
			self.nmi = (data & 0x80) > 0;
			self.idx = (data & 0x7F) as usize;
		}
	}
}
