// <linux/kvm.h>

pub const KVM_API_VERSION: i32 = 12;

const KVMIO: u8 = 0xAE;

// ioctls for /dev/kvm fds:
const KVM_GET_API_VERSION:            u8 = 0x00;
const KVM_CREATE_VM:                  u8 = 0x01;
const KVM_GET_MSR_INDEX_LIST:         u8 = 0x02;
const KVM_CHECK_EXTENSION:            u8 = 0x03;
const KVM_GET_VCPU_MMAP_SIZE:         u8 = 0x04;
const KVM_GET_SUPPORTED_CPUID:        u8 = 0x05;
const KVM_GET_EMULATED_CPUID:         u8 = 0x09;
const KVM_GET_MSR_FEATURE_INDEX_LIST: u8 = 0x0a;

// ioctls for VM fds:
const KVM_SET_MEMORY_REGION:      u8 = 0x40;
const KVM_CREATE_VCPU:            u8 = 0x41;
const KVM_GET_DIRTY_LOG :         u8 = 0x42;
const KVM_SET_MEMORY_ALIAS:       u8 = 0x43;
const KVM_SET_NR_MMU_PAGES:       u8 = 0x44;
const KVM_GET_NR_MMU_PAGES:       u8 = 0x45;
const KVM_SET_USER_MEMORY_REGION: u8 = 0x46;
const KVM_SET_TSS_ADDR:           u8 = 0x47;
const KVM_SET_IDENTITY_MAP_ADDR:  u8 = 0x48;
// ...

// ioctls for vcpu fds
const KVM_RUN:             u8 = 0x80;
const KVM_GET_REGS:        u8 = 0x81;
const KVM_SET_REGS:        u8 = 0x82;
const KVM_GET_SREGS:       u8 = 0x83;
const KVM_SET_SREGS:       u8 = 0x84;
const KVM_TRANSLATE:       u8 = 0x85;
const KVM_INTERRUPT:       u8 = 0x86;
const KVM_GET_MSRS:        u8 = 0x88;
const KVM_SET_MSRS:        u8 = 0x89;
const KVM_SET_CPUID:       u8 = 0x8a;
const KVM_SET_SIGNAL_MASK: u8 = 0x8b;
const KVM_GET_FPU:         u8 = 0x8c;
const KVM_SET_FPU:         u8 = 0x8d;
const KVM_GET_LAPIC:       u8 = 0x8e;
const KVM_SET_LAPIC:       u8 = 0x8f;
const KVM_SET_CPUID2:      u8 = 0x90;
const KVM_GET_CPUID2:      u8 = 0x91;

const KVM_SET_GUEST_DEBUG: u8 = 0x9b;

pub const KVM_CAP_SET_GUEST_DEBUG: u64 = 23;

pub const KVM_GUESTDBG_ENABLE: u32 = 1;
pub const KVM_GUESTDBG_SINGLESTEP: u32 = 2;

// exit reasons
pub const KVM_EXIT_UNKNOWN:         u32 = 0;
pub const KVM_EXIT_EXCEPTION:       u32 = 1;
pub const KVM_EXIT_IO:              u32 = 2;
pub const KVM_EXIT_HYPERCALL:       u32 = 3;
pub const KVM_EXIT_DEBUG:           u32 = 4;
pub const KVM_EXIT_HLT:             u32 = 5;
pub const KVM_EXIT_MMIO:            u32 = 6;
pub const KVM_EXIT_IRQ_WINDOW_OPEN: u32 = 7;
pub const KVM_EXIT_SHUTDOWN:        u32 = 8;
pub const KVM_EXIT_FAIL_ENTRY:      u32 = 9;
pub const KVM_EXIT_INTR:            u32 = 10;
pub const KVM_EXIT_SET_TPR:         u32 = 11;
pub const KVM_EXIT_TPR_ACCESS:      u32 = 12;
pub const KVM_EXIT_S390_SIEIC:      u32 = 13;
pub const KVM_EXIT_S390_RESET:      u32 = 14;
pub const KVM_EXIT_DCR:             u32 = 15; /* deprecated */
pub const KVM_EXIT_NMI:             u32 = 16;
pub const KVM_EXIT_INTERNAL_ERROR:  u32 = 17;
pub const KVM_EXIT_OSI:             u32 = 18;
pub const KVM_EXIT_PAPR_HCALL:      u32 = 19;
pub const KVM_EXIT_S390_UCONTROL:   u32 = 20;
pub const KVM_EXIT_WATCHDOG:        u32 = 21;
pub const KVM_EXIT_S390_TSCH:       u32 = 22;
pub const KVM_EXIT_EPR:             u32 = 23;
pub const KVM_EXIT_SYSTEM_EVENT:    u32 = 24;
pub const KVM_EXIT_S390_STSI:       u32 = 25;
pub const KVM_EXIT_IOAPIC_EOI:      u32 = 26;
pub const KVM_EXIT_HYPERV:          u32 = 27;

pub const KVM_EXIT_IO_IN:  u8 = 0;
pub const KVM_EXIT_IO_OUT: u8 = 1;


#[repr(C)]
pub struct userspace_memory_region {
	pub slot: u32,
	pub flags: u32,
	pub guest_phys_addr: u64,
	pub memory_size: u64,
	pub userspace_addr: u64,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct kvm_run {
	/* in */
	pub request_interrupt_window: u8,
	pub padding1: [u8; 7],
	
	/* out */
	pub exit_reason: u32,
	pub ready_for_interrupt_injection: u8,
	pub if_flag: u8,
	pub padding2: [u8; 2],
	
	/* in (pre_kvm_run), out (post_kvm_run) */
	pub cr8: u64,
	pub apic_base: u64,
	
	
	pub direction: u8,
	pub size: u8,
	pub port: u16,
	pub count: u32,
	pub data_offset: u64,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct guest_debug {
	pub control: u32,
	pub pad: u32,
	pub arch: [u64; 8]
}

// x86 arch specific stuff

pub const KVM_NR_INTERRUPTS: usize = 256;

#[repr(C)]
#[derive(Debug, Default)]
pub struct kvm_segment {
	pub base: u64,
	pub limit: u32,
	pub selector: u16,
	pub type_: u8,
	pub present: u8,
	pub dpl: u8,
	pub db: u8,
	pub s: u8,
	pub l: u8,
	pub g: u8,
	pub avl: u8,
	pub unusable: u8,
	pub padding: u8,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct kvm_dtable {
	pub base: u64,
	pub limit: u16,
	pub padding: [u16; 3]
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct sregs {
	pub cs: kvm_segment,
	pub ds: kvm_segment,
	pub es: kvm_segment,
	pub fs: kvm_segment,
	pub gs: kvm_segment,
	pub ss: kvm_segment,
	pub tr: kvm_segment,
	pub ldt: kvm_segment,
	pub gdt: kvm_dtable,
	pub idt: kvm_dtable,
	pub cr0: u64,
	pub cr2: u64,
	pub cr3: u64,
	pub cr4: u64,
	pub cr8: u64,
	pub efer: u64,
	pub apic_base: u64,
	pub interrupt_bitmap: [u64; (KVM_NR_INTERRUPTS+63)/64]
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct regs {
	pub rax: u64,
	pub rbx: u64,
	pub rcx: u64,
	pub rdx: u64,
	pub rsi: u64,
	pub rdi: u64,
	pub rsp: u64,
	pub rbp: u64,
	pub r8: u64,
	pub r9: u64,
	pub r10: u64,
	pub r11: u64,
	pub r12: u64,
	pub r13: u64,
	pub r14: u64,
	pub r15: u64,
	pub rip: u64,
	pub rflags: u64
}

ioctl_write_int_bad!(get_api_version, request_code_none!(KVMIO, KVM_GET_API_VERSION));
ioctl_write_int_bad!(create_vm, request_code_none!(KVMIO, KVM_CREATE_VM));

ioctl_write_int_bad!(set_tss_addr, request_code_none!(KVMIO, KVM_SET_TSS_ADDR));
ioctl_write_ptr!(set_user_memory_region, KVMIO, KVM_SET_USER_MEMORY_REGION, userspace_memory_region);
ioctl_write_int_bad!(create_vcpu, request_code_none!(KVMIO, KVM_CREATE_VCPU));

ioctl_write_int_bad!(get_vcpu_mmap_size, request_code_none!(KVMIO, KVM_GET_VCPU_MMAP_SIZE));

ioctl_write_int_bad!(run, request_code_none!(KVMIO, KVM_RUN));
ioctl_read!(get_sregs, KVMIO, KVM_GET_SREGS, sregs);
ioctl_write_ptr!(set_sregs, KVMIO, KVM_SET_SREGS, sregs);
ioctl_read!(get_regs, KVMIO, KVM_GET_REGS, regs);
ioctl_write_ptr!(set_regs, KVMIO, KVM_SET_REGS, regs);

ioctl_write_ptr!(set_guest_debug, KVMIO, KVM_SET_GUEST_DEBUG, guest_debug);
