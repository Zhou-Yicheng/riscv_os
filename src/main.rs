#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(asm)]
#![feature(global_asm)]

global_asm!(include_str!("asm/boot.S"));
global_asm!(include_str!("asm/trap.S"));

mod uart;

#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	print!("Aborting: ");
	if let Some(p) = info.location() {
		println!("line {}, file {}: {}",
				 p.line(),
				 p.file(),
				 info.message().unwrap()
		);
	}
	else {
		println!("no information available.");
	}
	abort();
}

#[no_mangle]
extern "C" fn abort() -> ! {
	loop {
		unsafe {
			asm!("wfi");
		}
	}
}

#[no_mangle]
extern "C" fn kmain() {
	let uart = uart::UartDriver::new(0x1000_0000);
	uart.init();

	println!("hello world");

	loop {
		match uart.get() {
			Some(8) | Some(127) => {
				print!("{}{}{}", 8 as char, ' ', 8 as char);
			},
			Some(10) | Some(13) => {
				println!();
			},
			Some(c) => {
				print!("{}", c as char);
			}
			_ => {}
		}
	}
}
