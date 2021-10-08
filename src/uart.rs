use core::fmt::Write;

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!(crate::uart::UartDriver::new(0x1000_0000), $($args)+);
    });
}

#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

pub struct UartDriver {
    base_address: usize
}

impl Write for UartDriver {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        s.bytes().for_each(|c| self.put(c));

        Ok(())
    }
}

impl UartDriver {

    pub fn new(base_address: usize) -> Self {
        UartDriver { base_address }
    }

    pub fn init(&self) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            let lcr = (1 << 1) | (1 << 0);
            ptr.add(3).write_volatile(lcr);

            ptr.add(2).write_volatile(1 << 0);

            ptr.add(1).write_volatile(1 << 0);

            let divisor: u16 = 592;
            let divisor_least: u8 = (divisor & 0xff) as u8;
            let divisor_most: u8 = (divisor >> 8) as u8;

            ptr.add(3).write_volatile(1 << 7 | lcr);
            ptr.add(0).write_volatile(divisor_least);
            ptr.add(1).write_volatile(divisor_most);

            ptr.add(3).write_volatile(0 << 7 | lcr)
        }
    }

    pub fn put(&self, c: u8) {
        let ptr = self.base_address as *mut u8;
        unsafe { ptr.write_volatile(c) };
    }

    pub fn get(&self) -> Option<u8> {
        let ptr = self.base_address as *mut u8;
        unsafe {
            if ptr.add(5).read_volatile() & 1 == 0 {
                None
            } else {
                Some(ptr.read_volatile())
            }
        }
    }
}
