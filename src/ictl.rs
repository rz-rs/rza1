#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Control Register 0"]
    pub icr0: ICR0,
    #[doc = "0x02 - IRQ Sense Select - These bits select whether interrupt signals corresponding to pins IRQ7 to IRQ0 are detected by a low level, falling edge, rising edge, or both edges. - 00: Interrupt request is detected on low level of IRQn input - 01: Interrupt request is detected on falling edge of IRQn input - 10: Interrupt request is detected on rising edge of IRQn input - 11: Interrupt request is detected on both edges of IRQn input"]
    pub icr1: ICR1,
    #[doc = "0x04 - IRQ Interrupt Request Register (IRQRR) - IRQRR is a 16-bit register that indicates interrupt requests from external input pins IRQ7 to IRQ0. If edge detection is set for the IRQ7 to IRQ0 interrupts, writing 0 to the IRQ7F to IRQ0F bits after reading IRQ7F to IRQ0F = 1 cancels the retained interrupts."]
    pub irqrr: IRQRR,
}
#[doc = "Interrupt Control Register 0"]
pub struct ICR0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Control Register 0"]
pub mod icr0;
#[doc = "IRQ Sense Select - These bits select whether interrupt signals corresponding to pins IRQ7 to IRQ0 are detected by a low level, falling edge, rising edge, or both edges. - 00: Interrupt request is detected on low level of IRQn input - 01: Interrupt request is detected on falling edge of IRQn input - 10: Interrupt request is detected on rising edge of IRQn input - 11: Interrupt request is detected on both edges of IRQn input"]
pub struct ICR1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "IRQ Sense Select - These bits select whether interrupt signals corresponding to pins IRQ7 to IRQ0 are detected by a low level, falling edge, rising edge, or both edges. - 00: Interrupt request is detected on low level of IRQn input - 01: Interrupt request is detected on falling edge of IRQn input - 10: Interrupt request is detected on rising edge of IRQn input - 11: Interrupt request is detected on both edges of IRQn input"]
pub mod icr1;
#[doc = "IRQ Interrupt Request Register (IRQRR) - IRQRR is a 16-bit register that indicates interrupt requests from external input pins IRQ7 to IRQ0. If edge detection is set for the IRQ7 to IRQ0 interrupts, writing 0 to the IRQ7F to IRQ0F bits after reading IRQ7F to IRQ0F = 1 cancels the retained interrupts."]
pub struct IRQRR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "IRQ Interrupt Request Register (IRQRR) - IRQRR is a 16-bit register that indicates interrupt requests from external input pins IRQ7 to IRQ0. If edge detection is set for the IRQ7 to IRQ0 interrupts, writing 0 to the IRQ7F to IRQ0F bits after reading IRQ7F to IRQ0F = 1 cancels the retained interrupts."]
pub mod irqrr;
