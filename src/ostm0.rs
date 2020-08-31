#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Compare Register - Depending on the mode of operation, this register holds the start value for the down-counter or the value for comparison with that of the counter."]
    pub cmp: CMP,
    #[doc = "0x04 - Counter Register - This register indicates the counter value of the timer."]
    pub cnt: CNT,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Count Enable Status Register - This register indicates whether the counter is enabled or disabled."]
    pub te: TE,
    _reserved1: [u8; 3usize],
    #[doc = "0x14 - Count Start Trigger Register - This register starts the counter."]
    pub ts: TS,
    _reserved2: [u8; 3usize],
    #[doc = "0x18 - Count Stop Trigger Register - This register stops the counter."]
    pub tt: TT,
    _reserved3: [u8; 7usize],
    #[doc = "0x20 - Control Register - This register specifies the operating mode for the counter and controls enabling/disabling of OSTMnTINT interrupt requests when counting starts."]
    pub ctl: CTL,
}
#[doc = "Compare Register - Depending on the mode of operation, this register holds the start value for the down-counter or the value for comparison with that of the counter."]
pub struct CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register - Depending on the mode of operation, this register holds the start value for the down-counter or the value for comparison with that of the counter."]
pub mod cmp;
#[doc = "Counter Register - This register indicates the counter value of the timer."]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Register - This register indicates the counter value of the timer."]
pub mod cnt;
#[doc = "Count Enable Status Register - This register indicates whether the counter is enabled or disabled."]
pub struct TE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Count Enable Status Register - This register indicates whether the counter is enabled or disabled."]
pub mod te;
#[doc = "Count Start Trigger Register - This register starts the counter."]
pub struct TS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Count Start Trigger Register - This register starts the counter."]
pub mod ts;
#[doc = "Count Stop Trigger Register - This register stops the counter."]
pub struct TT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Count Stop Trigger Register - This register stops the counter."]
pub mod tt;
#[doc = "Control Register - This register specifies the operating mode for the counter and controls enabling/disabling of OSTMnTINT interrupt requests when counting starts."]
pub struct CTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control Register - This register specifies the operating mode for the counter and controls enabling/disabling of OSTMnTINT interrupt requests when counting starts."]
pub mod ctl;
