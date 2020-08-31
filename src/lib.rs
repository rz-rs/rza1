#![doc = "Peripheral access API for RZ/A1H microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = "Interrupt controller (chip-specific registers)"]
pub struct ICTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICTL {}
impl ICTL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ictl::RegisterBlock {
        0xfcfe_f800 as *const _
    }
}
impl Deref for ICTL {
    type Target = ictl::RegisterBlock;
    fn deref(&self) -> &ictl::RegisterBlock {
        unsafe { &*ICTL::ptr() }
    }
}
#[doc = "Interrupt controller (chip-specific registers)"]
pub mod ictl;
#[doc = "OS Timer 0"]
pub struct OSTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSTM0 {}
impl OSTM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ostm0::RegisterBlock {
        0xfcfe_c000 as *const _
    }
}
impl Deref for OSTM0 {
    type Target = ostm0::RegisterBlock;
    fn deref(&self) -> &ostm0::RegisterBlock {
        unsafe { &*OSTM0::ptr() }
    }
}
#[doc = "OS Timer 0"]
pub mod ostm0;
#[doc = "OS Timer 1"]
pub struct OSTM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSTM1 {}
impl OSTM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ostm0::RegisterBlock {
        0xfcfe_c400 as *const _
    }
}
impl Deref for OSTM1 {
    type Target = ostm0::RegisterBlock;
    fn deref(&self) -> &ostm0::RegisterBlock {
        unsafe { &*OSTM1::ptr() }
    }
}
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ICTL"]
    pub ICTL: ICTL,
    #[doc = "OSTM0"]
    pub OSTM0: OSTM0,
    #[doc = "OSTM1"]
    pub OSTM1: OSTM1,
}
impl Peripherals {
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ICTL: ICTL {
                _marker: PhantomData,
            },
            OSTM0: OSTM0 {
                _marker: PhantomData,
            },
            OSTM1: OSTM1 {
                _marker: PhantomData,
            },
        }
    }
}
