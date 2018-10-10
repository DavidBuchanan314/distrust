// https://wiki.osdev.org/PIT

use portio::PortIO;
use std::time::SystemTime;

pub struct PIT {
	now: SystemTime,
}

impl PIT {
	pub fn new() -> Self {
		Self {
			now: SystemTime::now()
		}
	}
}

impl PortIO for PIT {
	fn inb(&mut self, port: u16) -> u8 {
		if port == 0x40 {
			(self.now.elapsed().unwrap().subsec_nanos()/100000 &0xFF) as u8 // XXX
		} else {
			0
		}
	}
}
