#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Frequency Control Register (FRQCR) - FRQCR is a 16-bit readable/writable register used to specify whether a clock is output from the CKIO pin during normal operation mode, change of gain of crystal oscillator for the XTAL pin, software standby mode, deep standby mode, and standby mode cancellation. The register specifies the frequency division ratio for the CPU clock (I\u{3c6}). FRQCR can be accessed in 16-bit units."]
    pub frqcr0: FRQCR0,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - Frequency Control Register 2 (FRQCR2) - FRQCR2 is a 16-bit readable/writable register used to specify the frequency division ratio for the image processing clock (G\u{3c6}). FRQCR2 can be accessed in 16-bit units."]
    pub frqcr2: FRQCR2,
}
#[doc = "Frequency Control Register (FRQCR) - FRQCR is a 16-bit readable/writable register used to specify whether a clock is output from the CKIO pin during normal operation mode, change of gain of crystal oscillator for the XTAL pin, software standby mode, deep standby mode, and standby mode cancellation. The register specifies the frequency division ratio for the CPU clock (I\u{3c6}). FRQCR can be accessed in 16-bit units."]
pub struct FRQCR0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Frequency Control Register (FRQCR) - FRQCR is a 16-bit readable/writable register used to specify whether a clock is output from the CKIO pin during normal operation mode, change of gain of crystal oscillator for the XTAL pin, software standby mode, deep standby mode, and standby mode cancellation. The register specifies the frequency division ratio for the CPU clock (I\u{3c6}). FRQCR can be accessed in 16-bit units."]
pub mod frqcr0;
#[doc = "Frequency Control Register 2 (FRQCR2) - FRQCR2 is a 16-bit readable/writable register used to specify the frequency division ratio for the image processing clock (G\u{3c6}). FRQCR2 can be accessed in 16-bit units."]
pub struct FRQCR2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Frequency Control Register 2 (FRQCR2) - FRQCR2 is a 16-bit readable/writable register used to specify the frequency division ratio for the image processing clock (G\u{3c6}). FRQCR2 can be accessed in 16-bit units."]
pub mod frqcr2;
