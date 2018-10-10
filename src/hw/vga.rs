use portio::PortIO;

pub struct VGA {

}

impl VGA {
	pub fn new() -> Self {
		Self {

		}
	}
}

impl PortIO for VGA {
	fn inb(&mut self, port: u16) -> u8 {
		eprintln!("VGA inb(0x{:x})", port);
		0
	}
	fn outb(&mut self, port: u16, data: u8) {
		eprintln!("VGA outb(0x{:x}, 0x{:x})", port, data);
	}
}
