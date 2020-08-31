#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 32usize],
    #[doc = "0x20 - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub jppr0: JPPR0,
    _reserved1: [u8; 30usize],
    #[doc = "0x40 - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub jpmc0: JPMC0,
    _reserved2: [u8; 78usize],
    #[doc = "0x90 - Port Mode Control Set and Reset Register (PMCSRn/JPMCSR0) - This register provides an alternative method for writing data to the PMCn register. The higher bits of the PMCSRn register specify whether data can be written to the PMCn.PMCnm bit specified by the lower bits of the PMCSRn register. When reading, the higher 16 bits are read as 0000H. The lower 16 bits are read as the value of the PMCn register."]
    pub jpmcsr0: JPMCSR0,
    _reserved3: [u8; 876usize],
    #[doc = "0x400 - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub jpibc0: JPIBC0,
}
#[doc = "Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
pub struct JPPR0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
pub mod jppr0;
#[doc = "Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
pub struct JPMC0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
pub mod jpmc0;
#[doc = "Port Mode Control Set and Reset Register (PMCSRn/JPMCSR0) - This register provides an alternative method for writing data to the PMCn register. The higher bits of the PMCSRn register specify whether data can be written to the PMCn.PMCnm bit specified by the lower bits of the PMCSRn register. When reading, the higher 16 bits are read as 0000H. The lower 16 bits are read as the value of the PMCn register."]
pub struct JPMCSR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Mode Control Set and Reset Register (PMCSRn/JPMCSR0) - This register provides an alternative method for writing data to the PMCn register. The higher bits of the PMCSRn register specify whether data can be written to the PMCn.PMCnm bit specified by the lower bits of the PMCSRn register. When reading, the higher 16 bits are read as 0000H. The lower 16 bits are read as the value of the PMCn register."]
pub mod jpmcsr0;
#[doc = "Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
pub struct JPIBC0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
pub mod jpibc0;
