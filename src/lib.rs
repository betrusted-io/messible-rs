#![no_std]
use core::fmt;
use core::fmt::Write;
use core::marker::PhantomData;
use core::ops::Deref;

#[macro_use]
mod macros;

mod generic;
extern crate vcell;
use generic::*;

mod messible;

static mut MESSIBLE_ADDRESS: usize = 0xe000_7800;
pub struct MESSIBLE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MESSIBLE {}
impl MESSIBLE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub unsafe fn ptr() -> *const messible::RegisterBlock {
        MESSIBLE_ADDRESS as *const _
    }
}
impl Deref for MESSIBLE {
    type Target = messible::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MESSIBLE::ptr() }
    }
}

struct Port;

/// Implements the `fmt::Write` trait.  This allows us to
/// have formatted output.
impl fmt::Write for Port {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_all(s.as_bytes());
        Ok(())
    }
}

/// Writes `fmt::Arguments` to the ITM `port`
pub fn write_fmt(args: fmt::Arguments) {
    Port.write_fmt(args).ok();
}

/// Writes a string to the Messible
pub fn write_str(string: &str) {
    write_all(string.as_bytes());
}

/// Core function -- actually writes data out the Messible
fn write_all(buffer: &[u8]) {
    unsafe {
        let m = MESSIBLE::ptr();
        for b in buffer {
            while (*m).status.read().full().bit() {}
            (*m).in_.write(|x| x.bits(*b as u32));
        }
    }
}

pub unsafe fn set_address(addr: usize) {
    MESSIBLE_ADDRESS = addr;
}
