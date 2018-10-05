use std::fs::File;
use std::os::unix::io::{RawFd, IntoRawFd};
use nix::sys::mman::{mmap, ProtFlags, MapFlags};

extern crate array_init;
//use array_init::array_init;

//use std::io;
use std::io::prelude::*;

// apparently we need to import the ioctl macros used by the kvm module here???
#[macro_use] extern crate nix;
// The dead code is from unused ioctls and constants from kvm.h
#[allow(dead_code)] mod kvm;

mod portio;// { pub mod portio; }
use portio::*;

mod hw;
use hw::serial::SerialPort;
use hw::cmos::CMOS;
use hw::pit::PIT;

#[derive(Debug)]
struct VM {
	kvmfd: RawFd,
	fd: RawFd,
	mem: usize, // probably shouldn't use a usize for this...
	vcpu_fd: RawFd, // TODO: should probably have a vec of vcpus
	vcpu_kvm_run: usize,
	foo: *mut kvm::kvm_run
}

impl Default for VM {
	fn default() -> VM{ VM {kvmfd:0, fd:0, mem:0, vcpu_fd:0, vcpu_kvm_run:0, foo:0 as *mut kvm::kvm_run} }
}

fn vm_init(vm: &mut VM, mem_size: usize) -> std::io::Result<()> {
	vm.kvmfd = File::open("/dev/kvm")?.into_raw_fd();
	
	let version = unsafe { kvm::get_api_version(vm.kvmfd, 0) };
	if version.is_err() {
		panic!("KVM_GET_API_VERSION failed: {:?}", version);
	}
	let version = version.unwrap();
	if version != kvm::KVM_API_VERSION {
		panic!("Unsupported KVM API version: {} (Expected {})", version, kvm::KVM_API_VERSION);
	}
	
	let vmfd = unsafe { kvm::create_vm(vm.kvmfd, 0) };
	if vmfd.is_err() {
		panic!("KVM_CREATE_VM failed: {:?}", vmfd);
	}
	vm.fd = vmfd.unwrap();
	
	let err = unsafe { kvm::set_tss_addr(vm.fd, 0xfffbd000u32 as i32) };
	if err.is_err() {
		panic!("KVM_SET_TSS_ADDR failed: {:?}", err);
	}
	
	let guestmapping = unsafe { mmap(std::ptr::null_mut(), mem_size,
		ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
		MapFlags::MAP_PRIVATE | MapFlags::MAP_ANONYMOUS,
		-1, 0) };
	
	if guestmapping.is_err() {
		panic!("mmap: {:?}", guestmapping);
	}
	
	vm.mem = guestmapping.unwrap() as usize;
	
	let guestregion = kvm::userspace_memory_region {
		slot: 0,
		flags: 0,
		guest_phys_addr: 0,
		memory_size: mem_size as u64,
		userspace_addr: vm.mem as u64
	};
	
	let err = unsafe { kvm::set_user_memory_region(vm.fd, &guestregion) };
	if err.is_err() {
		panic!("KVM_SET_USER_MEMORY_REGION failed: {:?}", err);
	}
	
	Ok(())
}

fn vcpu_init(vm: &mut VM) -> std::io::Result<()> {
	let vcpu_fd = unsafe { kvm::create_vcpu(vm.fd, 0) };
	if vcpu_fd.is_err() {
		panic!("KVM_CREATE_VCPU failed: {:?}", vcpu_fd);
	}
	vm.vcpu_fd = vcpu_fd.unwrap();
	
	let mmap_size = unsafe { kvm::get_vcpu_mmap_size(vm.kvmfd, 0) };
	if mmap_size.is_err() {
		panic!("KVM_GET_VCPU_MMAP_SIZE failed: {:?}", mmap_size);
	}
	
	let mapping = unsafe { mmap(std::ptr::null_mut(), mmap_size.unwrap() as usize,
		ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
		MapFlags::MAP_SHARED, vm.vcpu_fd, 0) };
	
	if mapping.is_err() {
		panic!("mmap: {:?}", mapping);
	}
	
	vm.vcpu_kvm_run = mapping.unwrap() as usize;
	vm.foo = vm.vcpu_kvm_run as *mut kvm::kvm_run;
	
	Ok(())
}

fn main() -> std::io::Result<()> {
	let mut vm = VM::default();
	vm_init(&mut vm, 0x100000000)?;
	vcpu_init(&mut vm)?;
	
	let mut sregs = kvm::sregs::default();
	let err = unsafe{ kvm::get_sregs(vm.vcpu_fd, &mut sregs) };
	if err.is_err() {
		panic!("KVM_GET_SREGS failed: {:?}", err);
	}
	
	//println!("SREGS: {:?}", sregs);
	
	// these seem to be the default values, but it's good to be explicit
	sregs.cs.base = 0xF0000;//0xFFFF0000;
	sregs.cs.selector = 0xF000;
	
	let err = unsafe{ kvm::set_sregs(vm.vcpu_fd, &mut sregs) };
	if err.is_err() {
		panic!("KVM_SET_SREGS failed: {:?}", err);
	}
	
	let mut regs = kvm::regs::default();
	regs.rflags = 2;
	regs.rip = 0xFFF0;
	
	let err = unsafe{ kvm::set_regs(vm.vcpu_fd, &mut regs) };
	if err.is_err() {
		panic!("KVM_SET_REGS failed: {:?}", err);
	}
	
	// TODO: load code
	let mut program = Vec::new();
	let mut f = File::open("seabios/out/bios.bin")?;
	f.read_to_end(&mut program)?;
	
	unsafe {
		let mem = vm.mem as *mut [u8; 0x100000];
		let start = 0x100000 - program.len();
		for i in start..0x100000 {
			(*mem)[i] = program[i-start];
		}
	}
	
	/*
	let mut debug = kvm::guest_debug::default();
	debug.control = kvm::KVM_GUESTDBG_ENABLE | kvm::KVM_GUESTDBG_SINGLESTEP;
	let err = unsafe { kvm::set_guest_debug(vm.vcpu_fd, &debug) };
	if err.is_err() {
		panic!("KVM_GUESTDBG_ENABLE failed: {:?}", err);
	}
	// */
	
	/* SETUP IO PORTS */
	
	let mut stub = PortStub;
	let mut pit = PIT::new();
	let mut cmos = CMOS::new();
	let mut com1 = SerialPort::new(0x3f8,
	                               File::open("/dev/stdin")?,
	                               File::create("/dev/stdout")?);
	
	let mut devices: Vec<&mut PortIO> = Vec::new();
	devices.push(&mut stub);
	
	let mut ports = [0usize; 0x10000];
	
	devices.push(&mut pit);
	for i in 0x40..0x40+8 {
		ports[i as usize] = devices.len()-1;
	}
	
	devices.push(&mut cmos);
	for i in 0x70..=0x71 {
		ports[i as usize] = devices.len()-1;
	}
	
	devices.push(&mut com1);
	for i in 0x3f8..0x3f8+8 {
		ports[i as usize] = devices.len()-1;
	}
	
	loop {
	
		let err = unsafe{ kvm::run(vm.vcpu_fd, 0) };
		if err.is_err() {
			panic!("KVM_RUN failed: {:?}", err);
		}
		
		let reason = unsafe{ (*vm.foo).exit_reason };
		
		match reason {
			kvm::KVM_EXIT_IO => {
				let direction = unsafe{ (*vm.foo).direction };
				let size = unsafe{ (*vm.foo).size };
				let port = unsafe{ (*vm.foo).port };
				let count = unsafe{ (*vm.foo).count };
				let data = vm.vcpu_kvm_run + unsafe{ (*vm.foo).data_offset } as usize;
				
				if count != 1 { panic!("IO operations with count != 1 unimplemented...") };
				
				match (direction, size) {
					(kvm::KVM_EXIT_IO_IN, 1) => unsafe{
						*(data as *mut u8) = devices[ports[port as usize]].inb(port)
					},
					(kvm::KVM_EXIT_IO_IN, 2) => unsafe{
						*(data as *mut u16) = devices[ports[port as usize]].inw(port)
					},
					(kvm::KVM_EXIT_IO_IN, 4) => unsafe{
						*(data as *mut u32) = devices[ports[port as usize]].inl(port)
					},
					(kvm::KVM_EXIT_IO_OUT, 1) =>
						devices[ports[port as usize]].outb(port, unsafe{ *(data as *const u8) }),
					(kvm::KVM_EXIT_IO_OUT, 2) =>
						devices[ports[port as usize]].outw(port, unsafe{ *(data as *const u16) }),
					(kvm::KVM_EXIT_IO_OUT, 4) =>
						devices[ports[port as usize]].outl(port, unsafe{ *(data as *const u32) }),
					_ => panic!("Invalid IO operation: dir={}, size={}", direction, size)
				};
			},
			
			kvm::KVM_EXIT_DEBUG => {
				let mut regs = kvm::regs::default();
				let err = unsafe{ kvm::get_regs(vm.vcpu_fd, &mut regs) };
				if err.is_err() {
					panic!("KVM_GET_REGS failed: {:?}", err);
				}
				
				let mut sregs = kvm::sregs::default();
				let err = unsafe{ kvm::get_sregs(vm.vcpu_fd, &mut sregs) };
				if err.is_err() {
					panic!("KVM_GET_SREGS failed: {:?}", err);
				}
				
				eprintln!("rip: 0x{:x}:0x{:x}", sregs.cs.selector, regs.rip);
			},
			kvm::KVM_EXIT_HLT => { eprintln!("KVM_EXIT_HLT"); break },
			kvm::KVM_EXIT_SHUTDOWN => { eprintln!("KVM_EXIT_SHUTDOWN"); break },
			_ => panic!("Unhandled VM_RUN exit reason: {}", reason)
		}
	}
	
	Ok(())
}
