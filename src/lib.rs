#![doc = "Peripheral access API for RZ/A1H microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = "Clock Pulse Generator"]
pub struct CPG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPG {}
impl CPG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cpg::RegisterBlock {
        0xfcfe_0010 as *const _
    }
}
impl Deref for CPG {
    type Target = cpg::RegisterBlock;
    fn deref(&self) -> &cpg::RegisterBlock {
        unsafe { &*CPG::ptr() }
    }
}
#[doc = "Clock Pulse Generator"]
pub mod cpg;
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
#[doc = "Serial Communication Interface with FIFO 0"]
pub struct SC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SC0 {}
impl SC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sc0::RegisterBlock {
        0xe800_7000 as *const _
    }
}
impl Deref for SC0 {
    type Target = sc0::RegisterBlock;
    fn deref(&self) -> &sc0::RegisterBlock {
        unsafe { &*SC0::ptr() }
    }
}
#[doc = "Serial Communication Interface with FIFO 0"]
pub mod sc0;
#[doc = "Serial Communication Interface with FIFO 1"]
pub struct SC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SC1 {}
impl SC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sc0::RegisterBlock {
        0xe800_7800 as *const _
    }
}
impl Deref for SC1 {
    type Target = sc0::RegisterBlock;
    fn deref(&self) -> &sc0::RegisterBlock {
        unsafe { &*SC1::ptr() }
    }
}
#[doc = "Serial Communication Interface with FIFO 2"]
pub struct SC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SC2 {}
impl SC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sc0::RegisterBlock {
        0xe800_8000 as *const _
    }
}
impl Deref for SC2 {
    type Target = sc0::RegisterBlock;
    fn deref(&self) -> &sc0::RegisterBlock {
        unsafe { &*SC2::ptr() }
    }
}
#[doc = "Serial Communication Interface with FIFO 3"]
pub struct SC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SC3 {}
impl SC3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sc0::RegisterBlock {
        0xe800_8800 as *const _
    }
}
impl Deref for SC3 {
    type Target = sc0::RegisterBlock;
    fn deref(&self) -> &sc0::RegisterBlock {
        unsafe { &*SC3::ptr() }
    }
}
#[doc = "Serial Communication Interface with FIFO 4"]
pub struct SC4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SC4 {}
impl SC4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sc0::RegisterBlock {
        0xe800_9000 as *const _
    }
}
impl Deref for SC4 {
    type Target = sc0::RegisterBlock;
    fn deref(&self) -> &sc0::RegisterBlock {
        unsafe { &*SC4::ptr() }
    }
}
#[doc = "Serial Communication Interface with FIFO 5"]
pub struct SC5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SC5 {}
impl SC5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sc0::RegisterBlock {
        0xe800_9800 as *const _
    }
}
impl Deref for SC5 {
    type Target = sc0::RegisterBlock;
    fn deref(&self) -> &sc0::RegisterBlock {
        unsafe { &*SC5::ptr() }
    }
}
#[doc = "Serial Communication Interface with FIFO 6"]
pub struct SC6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SC6 {}
impl SC6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sc0::RegisterBlock {
        0xe800_a000 as *const _
    }
}
impl Deref for SC6 {
    type Target = sc0::RegisterBlock;
    fn deref(&self) -> &sc0::RegisterBlock {
        unsafe { &*SC6::ptr() }
    }
}
#[doc = "Serial Communication Interface with FIFO 7"]
pub struct SC7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SC7 {}
impl SC7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sc0::RegisterBlock {
        0xe800_a800 as *const _
    }
}
impl Deref for SC7 {
    type Target = sc0::RegisterBlock;
    fn deref(&self) -> &sc0::RegisterBlock {
        unsafe { &*SC7::ptr() }
    }
}
#[doc = "General ports"]
pub struct PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT {}
impl PORT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port::RegisterBlock {
        0xfcfe_3000 as *const _
    }
}
impl Deref for PORT {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &port::RegisterBlock {
        unsafe { &*PORT::ptr() }
    }
}
#[doc = "General ports"]
pub mod port;
#[doc = "JTAG ports"]
pub struct JPORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for JPORT {}
impl JPORT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const jport::RegisterBlock {
        0xfcfe_7b00 as *const _
    }
}
impl Deref for JPORT {
    type Target = jport::RegisterBlock;
    fn deref(&self) -> &jport::RegisterBlock {
        unsafe { &*JPORT::ptr() }
    }
}
#[doc = "JTAG ports"]
pub mod jport;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "CPG"]
    pub CPG: CPG,
    #[doc = "ICTL"]
    pub ICTL: ICTL,
    #[doc = "OSTM0"]
    pub OSTM0: OSTM0,
    #[doc = "OSTM1"]
    pub OSTM1: OSTM1,
    #[doc = "SC0"]
    pub SC0: SC0,
    #[doc = "SC1"]
    pub SC1: SC1,
    #[doc = "SC2"]
    pub SC2: SC2,
    #[doc = "SC3"]
    pub SC3: SC3,
    #[doc = "SC4"]
    pub SC4: SC4,
    #[doc = "SC5"]
    pub SC5: SC5,
    #[doc = "SC6"]
    pub SC6: SC6,
    #[doc = "SC7"]
    pub SC7: SC7,
    #[doc = "PORT"]
    pub PORT: PORT,
    #[doc = "JPORT"]
    pub JPORT: JPORT,
}
impl Peripherals {
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            CPG: CPG {
                _marker: PhantomData,
            },
            ICTL: ICTL {
                _marker: PhantomData,
            },
            OSTM0: OSTM0 {
                _marker: PhantomData,
            },
            OSTM1: OSTM1 {
                _marker: PhantomData,
            },
            SC0: SC0 {
                _marker: PhantomData,
            },
            SC1: SC1 {
                _marker: PhantomData,
            },
            SC2: SC2 {
                _marker: PhantomData,
            },
            SC3: SC3 {
                _marker: PhantomData,
            },
            SC4: SC4 {
                _marker: PhantomData,
            },
            SC5: SC5 {
                _marker: PhantomData,
            },
            SC6: SC6 {
                _marker: PhantomData,
            },
            SC7: SC7 {
                _marker: PhantomData,
            },
            PORT: PORT {
                _marker: PhantomData,
            },
            JPORT: JPORT {
                _marker: PhantomData,
            },
        }
    }
}
