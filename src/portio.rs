pub trait PortIO {
	fn inb(&mut self, port: u16) -> u8 {
		eprintln!("STUBBED inb(0x{:x}) => 0", port);
		return 0;
	}
	fn inw(&mut self, port: u16) -> u16 {
		eprintln!("STUBBED inw(0x{:x}) => 0", port);
		return 0;
	}
	fn inl(&mut self, port: u16) -> u32 {
		eprintln!("STUBBED inl(0x{:x}) => 0", port);
		return 0;
	}
	fn outb(&mut self, port: u16, data: u8) {
		eprintln!("STUBBED outb(0x{:x}, 0x{:02x})", port, data);
	}
	fn outw(&mut self, port: u16, data: u16) {
		eprintln!("STUBBED outw(0x{:x}, 0x{:04x})", port, data);
	}
	fn outl(&mut self, port: u16, data: u32) {
		eprintln!("STUBBED outl(0x{:x}, 0x{:08x})", port, data);
	}
}

pub struct PortStub;

impl PortIO for PortStub {}
