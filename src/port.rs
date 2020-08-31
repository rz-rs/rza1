#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
    pub p0: P,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
    pub p1: P,
    _reserved1: [u8; 2usize],
    #[doc = "0x08 - Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
    pub p2: P,
    _reserved2: [u8; 2usize],
    #[doc = "0x0c - Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
    pub p3: P,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
    pub p4: P,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
    pub p5: P,
    _reserved5: [u8; 2usize],
    #[doc = "0x18 - Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
    pub p6: P,
    _reserved6: [u8; 2usize],
    #[doc = "0x1c - Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
    pub p7: P,
    _reserved7: [u8; 2usize],
    #[doc = "0x20 - Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
    pub p8: P,
    _reserved8: [u8; 2usize],
    #[doc = "0x24 - Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
    pub p9: P,
    _reserved9: [u8; 2usize],
    #[doc = "0x28 - Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
    pub p10: P,
    _reserved10: [u8; 2usize],
    #[doc = "0x2c - Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
    pub p11: P,
    _reserved11: [u8; 210usize],
    #[doc = "0x100 - Port Set and Reset Register (PSRn) - This register provides an alternative method for writing data to the Pn register. The higher 16 bits of the PSRn register specify whether data can be written to the Pn.Pnm bit specified by the lower 16 bits of the PSRn register. When reading, the higher 16 bits are read as 0000H. The lower 16 bits are read as the value of the Pn register."]
    pub psr: [PSR; 12],
    _reserved12: [u8; 208usize],
    #[doc = "0x200 - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub ppr0: PPR,
    _reserved13: [u8; 2usize],
    #[doc = "0x204 - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub ppr1: PPR,
    _reserved14: [u8; 2usize],
    #[doc = "0x208 - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub ppr2: PPR,
    _reserved15: [u8; 2usize],
    #[doc = "0x20c - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub ppr3: PPR,
    _reserved16: [u8; 2usize],
    #[doc = "0x210 - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub ppr4: PPR,
    _reserved17: [u8; 2usize],
    #[doc = "0x214 - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub ppr5: PPR,
    _reserved18: [u8; 2usize],
    #[doc = "0x218 - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub ppr6: PPR,
    _reserved19: [u8; 2usize],
    #[doc = "0x21c - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub ppr7: PPR,
    _reserved20: [u8; 2usize],
    #[doc = "0x220 - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub ppr8: PPR,
    _reserved21: [u8; 2usize],
    #[doc = "0x224 - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub ppr9: PPR,
    _reserved22: [u8; 2usize],
    #[doc = "0x228 - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub ppr10: PPR,
    _reserved23: [u8; 2usize],
    #[doc = "0x22c - Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
    pub ppr11: PPR,
    _reserved24: [u8; 210usize],
    #[doc = "0x300 - Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
    pub pm0: PM,
    _reserved25: [u8; 2usize],
    #[doc = "0x304 - Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
    pub pm1: PM,
    _reserved26: [u8; 2usize],
    #[doc = "0x308 - Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
    pub pm2: PM,
    _reserved27: [u8; 2usize],
    #[doc = "0x30c - Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
    pub pm3: PM,
    _reserved28: [u8; 2usize],
    #[doc = "0x310 - Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
    pub pm4: PM,
    _reserved29: [u8; 2usize],
    #[doc = "0x314 - Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
    pub pm5: PM,
    _reserved30: [u8; 2usize],
    #[doc = "0x318 - Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
    pub pm6: PM,
    _reserved31: [u8; 2usize],
    #[doc = "0x31c - Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
    pub pm7: PM,
    _reserved32: [u8; 2usize],
    #[doc = "0x320 - Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
    pub pm8: PM,
    _reserved33: [u8; 2usize],
    #[doc = "0x324 - Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
    pub pm9: PM,
    _reserved34: [u8; 2usize],
    #[doc = "0x328 - Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
    pub pm10: PM,
    _reserved35: [u8; 2usize],
    #[doc = "0x32c - Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
    pub pm11: PM,
    _reserved36: [u8; 210usize],
    #[doc = "0x400 - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub pmc0: PMC,
    _reserved37: [u8; 2usize],
    #[doc = "0x404 - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub pmc1: PMC,
    _reserved38: [u8; 2usize],
    #[doc = "0x408 - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub pmc2: PMC,
    _reserved39: [u8; 2usize],
    #[doc = "0x40c - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub pmc3: PMC,
    _reserved40: [u8; 2usize],
    #[doc = "0x410 - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub pmc4: PMC,
    _reserved41: [u8; 2usize],
    #[doc = "0x414 - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub pmc5: PMC,
    _reserved42: [u8; 2usize],
    #[doc = "0x418 - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub pmc6: PMC,
    _reserved43: [u8; 2usize],
    #[doc = "0x41c - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub pmc7: PMC,
    _reserved44: [u8; 2usize],
    #[doc = "0x420 - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub pmc8: PMC,
    _reserved45: [u8; 2usize],
    #[doc = "0x424 - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub pmc9: PMC,
    _reserved46: [u8; 2usize],
    #[doc = "0x428 - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub pmc10: PMC,
    _reserved47: [u8; 2usize],
    #[doc = "0x42c - Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
    pub pmc11: PMC,
    _reserved48: [u8; 210usize],
    #[doc = "0x500 - Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfc0: PFC,
    _reserved49: [u8; 2usize],
    #[doc = "0x504 - Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfc1: PFC,
    _reserved50: [u8; 2usize],
    #[doc = "0x508 - Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfc2: PFC,
    _reserved51: [u8; 2usize],
    #[doc = "0x50c - Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfc3: PFC,
    _reserved52: [u8; 2usize],
    #[doc = "0x510 - Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfc4: PFC,
    _reserved53: [u8; 2usize],
    #[doc = "0x514 - Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfc5: PFC,
    _reserved54: [u8; 2usize],
    #[doc = "0x518 - Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfc6: PFC,
    _reserved55: [u8; 2usize],
    #[doc = "0x51c - Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfc7: PFC,
    _reserved56: [u8; 2usize],
    #[doc = "0x520 - Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfc8: PFC,
    _reserved57: [u8; 2usize],
    #[doc = "0x524 - Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfc9: PFC,
    _reserved58: [u8; 2usize],
    #[doc = "0x528 - Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfc10: PFC,
    _reserved59: [u8; 2usize],
    #[doc = "0x52c - Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfc11: PFC,
    _reserved60: [u8; 210usize],
    #[doc = "0x600 - Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfce0: PFCE,
    _reserved61: [u8; 2usize],
    #[doc = "0x604 - Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfce1: PFCE,
    _reserved62: [u8; 2usize],
    #[doc = "0x608 - Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfce2: PFCE,
    _reserved63: [u8; 2usize],
    #[doc = "0x60c - Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfce3: PFCE,
    _reserved64: [u8; 2usize],
    #[doc = "0x610 - Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfce4: PFCE,
    _reserved65: [u8; 2usize],
    #[doc = "0x614 - Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfce5: PFCE,
    _reserved66: [u8; 2usize],
    #[doc = "0x618 - Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfce6: PFCE,
    _reserved67: [u8; 2usize],
    #[doc = "0x61c - Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfce7: PFCE,
    _reserved68: [u8; 2usize],
    #[doc = "0x620 - Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfce8: PFCE,
    _reserved69: [u8; 2usize],
    #[doc = "0x624 - Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfce9: PFCE,
    _reserved70: [u8; 2usize],
    #[doc = "0x628 - Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfce10: PFCE,
    _reserved71: [u8; 2usize],
    #[doc = "0x62c - Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
    pub pfce11: PFCE,
    _reserved72: [u8; 210usize],
    #[doc = "0x700 - Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
    pub pnot0: PNOT,
    _reserved73: [u8; 2usize],
    #[doc = "0x704 - Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
    pub pnot1: PNOT,
    _reserved74: [u8; 2usize],
    #[doc = "0x708 - Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
    pub pnot2: PNOT,
    _reserved75: [u8; 2usize],
    #[doc = "0x70c - Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
    pub pnot3: PNOT,
    _reserved76: [u8; 2usize],
    #[doc = "0x710 - Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
    pub pnot4: PNOT,
    _reserved77: [u8; 2usize],
    #[doc = "0x714 - Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
    pub pnot5: PNOT,
    _reserved78: [u8; 2usize],
    #[doc = "0x718 - Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
    pub pnot6: PNOT,
    _reserved79: [u8; 2usize],
    #[doc = "0x71c - Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
    pub pnot7: PNOT,
    _reserved80: [u8; 2usize],
    #[doc = "0x720 - Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
    pub pnot8: PNOT,
    _reserved81: [u8; 2usize],
    #[doc = "0x724 - Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
    pub pnot9: PNOT,
    _reserved82: [u8; 2usize],
    #[doc = "0x728 - Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
    pub pnot10: PNOT,
    _reserved83: [u8; 2usize],
    #[doc = "0x72c - Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
    pub pnot11: PNOT,
    _reserved84: [u8; 210usize],
    #[doc = "0x800 - Port Mode Set and Reset Register (PMSRn) - This register provides an alternative method for writing data to the PMn register. The higher 16 bits of the PMSRn register specify whether data can be written to the PMn.PMnm bit specified by the lower 16 bits of the PMSRn register. When reading, the higher 16 bits are read as 0000H. The lower 16 bits are read as the value of the PMn register."]
    pub pmsr: [PMSR; 12],
    _reserved85: [u8; 208usize],
    #[doc = "0x900 - Port Mode Control Set and Reset Register (PMCSRn/JPMCSR0) - This register provides an alternative method for writing data to the PMCn register. The higher bits of the PMCSRn register specify whether data can be written to the PMCn.PMCnm bit specified by the lower bits of the PMCSRn register. When reading, the higher 16 bits are read as 0000H. The lower 16 bits are read as the value of the PMCn register."]
    pub pmcsr: [PMCSR; 12],
    _reserved86: [u8; 208usize],
    #[doc = "0xa00 - Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
    pub pfcae0: PFCAE,
    _reserved87: [u8; 2usize],
    #[doc = "0xa04 - Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
    pub pfcae1: PFCAE,
    _reserved88: [u8; 2usize],
    #[doc = "0xa08 - Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
    pub pfcae2: PFCAE,
    _reserved89: [u8; 2usize],
    #[doc = "0xa0c - Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
    pub pfcae3: PFCAE,
    _reserved90: [u8; 2usize],
    #[doc = "0xa10 - Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
    pub pfcae4: PFCAE,
    _reserved91: [u8; 2usize],
    #[doc = "0xa14 - Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
    pub pfcae5: PFCAE,
    _reserved92: [u8; 2usize],
    #[doc = "0xa18 - Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
    pub pfcae6: PFCAE,
    _reserved93: [u8; 2usize],
    #[doc = "0xa1c - Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
    pub pfcae7: PFCAE,
    _reserved94: [u8; 2usize],
    #[doc = "0xa20 - Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
    pub pfcae8: PFCAE,
    _reserved95: [u8; 2usize],
    #[doc = "0xa24 - Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
    pub pfcae9: PFCAE,
    _reserved96: [u8; 2usize],
    #[doc = "0xa28 - Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
    pub pfcae10: PFCAE,
    _reserved97: [u8; 2usize],
    #[doc = "0xa2c - Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
    pub pfcae11: PFCAE,
    _reserved98: [u8; 466usize],
    #[doc = "0xc00 - Serial Sound Interface Noise Canceler Control Register (SNCR) - This register controls the noise canceler in the input route from the LSI pin to a serial sound interface. Each bit can be set only when slave mode is selected for the corresponding channel of the serial sound interface. The bit should be used as it is the initial value (H'00000000) when master mode is selected for the corresponding channel of the serial sound interface."]
    pub sncr: SNCR,
    _reserved99: [u8; 13310usize],
    #[doc = "0x4000 - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub pibc0: PIBC,
    _reserved100: [u8; 2usize],
    #[doc = "0x4004 - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub pibc1: PIBC,
    _reserved101: [u8; 2usize],
    #[doc = "0x4008 - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub pibc2: PIBC,
    _reserved102: [u8; 2usize],
    #[doc = "0x400c - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub pibc3: PIBC,
    _reserved103: [u8; 2usize],
    #[doc = "0x4010 - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub pibc4: PIBC,
    _reserved104: [u8; 2usize],
    #[doc = "0x4014 - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub pibc5: PIBC,
    _reserved105: [u8; 2usize],
    #[doc = "0x4018 - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub pibc6: PIBC,
    _reserved106: [u8; 2usize],
    #[doc = "0x401c - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub pibc7: PIBC,
    _reserved107: [u8; 2usize],
    #[doc = "0x4020 - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub pibc8: PIBC,
    _reserved108: [u8; 2usize],
    #[doc = "0x4024 - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub pibc9: PIBC,
    _reserved109: [u8; 2usize],
    #[doc = "0x4028 - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub pibc10: PIBC,
    _reserved110: [u8; 2usize],
    #[doc = "0x402c - Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
    pub pibc11: PIBC,
    _reserved111: [u8; 210usize],
    #[doc = "0x4100 - Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
    pub pbdc0: PBDC,
    _reserved112: [u8; 2usize],
    #[doc = "0x4104 - Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
    pub pbdc1: PBDC,
    _reserved113: [u8; 2usize],
    #[doc = "0x4108 - Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
    pub pbdc2: PBDC,
    _reserved114: [u8; 2usize],
    #[doc = "0x410c - Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
    pub pbdc3: PBDC,
    _reserved115: [u8; 2usize],
    #[doc = "0x4110 - Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
    pub pbdc4: PBDC,
    _reserved116: [u8; 2usize],
    #[doc = "0x4114 - Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
    pub pbdc5: PBDC,
    _reserved117: [u8; 2usize],
    #[doc = "0x4118 - Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
    pub pbdc6: PBDC,
    _reserved118: [u8; 2usize],
    #[doc = "0x411c - Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
    pub pbdc7: PBDC,
    _reserved119: [u8; 2usize],
    #[doc = "0x4120 - Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
    pub pbdc8: PBDC,
    _reserved120: [u8; 2usize],
    #[doc = "0x4124 - Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
    pub pbdc9: PBDC,
    _reserved121: [u8; 2usize],
    #[doc = "0x4128 - Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
    pub pbdc10: PBDC,
    _reserved122: [u8; 2usize],
    #[doc = "0x412c - Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
    pub pbdc11: PBDC,
    _reserved123: [u8; 210usize],
    #[doc = "0x4200 - Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
    pub pipc0: PIPC,
    _reserved124: [u8; 2usize],
    #[doc = "0x4204 - Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
    pub pipc1: PIPC,
    _reserved125: [u8; 2usize],
    #[doc = "0x4208 - Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
    pub pipc2: PIPC,
    _reserved126: [u8; 2usize],
    #[doc = "0x420c - Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
    pub pipc3: PIPC,
    _reserved127: [u8; 2usize],
    #[doc = "0x4210 - Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
    pub pipc4: PIPC,
    _reserved128: [u8; 2usize],
    #[doc = "0x4214 - Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
    pub pipc5: PIPC,
    _reserved129: [u8; 2usize],
    #[doc = "0x4218 - Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
    pub pipc6: PIPC,
    _reserved130: [u8; 2usize],
    #[doc = "0x421c - Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
    pub pipc7: PIPC,
    _reserved131: [u8; 2usize],
    #[doc = "0x4220 - Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
    pub pipc8: PIPC,
    _reserved132: [u8; 2usize],
    #[doc = "0x4224 - Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
    pub pipc9: PIPC,
    _reserved133: [u8; 2usize],
    #[doc = "0x4228 - Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
    pub pipc10: PIPC,
    _reserved134: [u8; 2usize],
    #[doc = "0x422c - Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
    pub pipc11: PIPC,
}
#[doc = "Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
pub struct P {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port Register (Pn) - In output port mode (PMCn.PMCnm = 0 and PMn.PMnm = 0), this register holds the Pn.Pnm data to be output via the Pn_m pin."]
pub mod p;
#[doc = "Port Set and Reset Register (PSRn) - This register provides an alternative method for writing data to the Pn register. The higher 16 bits of the PSRn register specify whether data can be written to the Pn.Pnm bit specified by the lower 16 bits of the PSRn register. When reading, the higher 16 bits are read as 0000H. The lower 16 bits are read as the value of the Pn register."]
pub struct PSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Set and Reset Register (PSRn) - This register provides an alternative method for writing data to the Pn register. The higher 16 bits of the PSRn register specify whether data can be written to the Pn.Pnm bit specified by the lower 16 bits of the PSRn register. When reading, the higher 16 bits are read as 0000H. The lower 16 bits are read as the value of the Pn register."]
pub mod psr;
#[doc = "Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
pub struct PPR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port Pin Read Register (PPRn/JPPR0) - The PPRn register reflects either the level of the Pn_m pin, the value of the Pn.Pnm bit, or the output level of the alternative function. The value read depends on various register settings as described in Table 54.4, PPRnm Read Values."]
pub mod ppr;
#[doc = "Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
pub struct PM {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port Mode Register (PMn) - The PMn register specifies whether the Pn_m pins are in input mode or in output mode."]
pub mod pm;
#[doc = "Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
pub struct PMC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port Mode Control Register (PMCn/JPMC0) - This register is used to specify whether the Pn_m pins are in port mode or in alternative mode. Caution: The input/output control is not performed just by setting alternative mode (PMCn.PMCnm). Set 1 in the PIPCn.PIPCnm bit too when the I/O control is to be performed by using the alternative function."]
pub mod pmc;
#[doc = "Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
pub struct PFC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port Function Control Register (PFCn) - This register, together with the PFCEn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
pub mod pfc;
#[doc = "Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
pub struct PFCE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port Function Control Expansion Register (PFCEn) This register, together with the PFCn and PFCAEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit."]
pub mod pfce;
#[doc = "Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
pub struct PNOT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port NOT Register (PNOTn) - This register allows a Pn.Pnm bit to be inverted without directly writing to the Pn register."]
pub mod pnot;
#[doc = "Port Mode Set and Reset Register (PMSRn) - This register provides an alternative method for writing data to the PMn register. The higher 16 bits of the PMSRn register specify whether data can be written to the PMn.PMnm bit specified by the lower 16 bits of the PMSRn register. When reading, the higher 16 bits are read as 0000H. The lower 16 bits are read as the value of the PMn register."]
pub struct PMSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Mode Set and Reset Register (PMSRn) - This register provides an alternative method for writing data to the PMn register. The higher 16 bits of the PMSRn register specify whether data can be written to the PMn.PMnm bit specified by the lower 16 bits of the PMSRn register. When reading, the higher 16 bits are read as 0000H. The lower 16 bits are read as the value of the PMn register."]
pub mod pmsr;
#[doc = "Port Mode Control Set and Reset Register (PMCSRn/JPMCSR0) - This register provides an alternative method for writing data to the PMCn register. The higher bits of the PMCSRn register specify whether data can be written to the PMCn.PMCnm bit specified by the lower bits of the PMCSRn register. When reading, the higher 16 bits are read as 0000H. The lower 16 bits are read as the value of the PMCn register."]
pub struct PMCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Mode Control Set and Reset Register (PMCSRn/JPMCSR0) - This register provides an alternative method for writing data to the PMCn register. The higher bits of the PMCSRn register specify whether data can be written to the PMCn.PMCnm bit specified by the lower bits of the PMCSRn register. When reading, the higher 16 bits are read as 0000H. The lower 16 bits are read as the value of the PMCn register."]
pub mod pmcsr;
#[doc = "Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
pub struct PFCAE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port Function Control Additional Expansion Register (PFCAEn) - This register, together with the PFCn and PFCEn register, specifies the alternative function of the pins. The I/O direction of several alternative functions can be controlled directly by using the Pn_m pin. For these alternative functions, the PIPCn.PIPCnm bit must be set to 1. For all other alternative functions, the I/O direction is specified by using the PMn.PMnm bit. After selecting an alternative function by the PFCn.PFCnm, PFCEn.PFCEnm, or PFCAEn.PFCAEnm bit, set the PMCn.PMCnm bit to 1."]
pub mod pfcae;
#[doc = "Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
pub struct PIBC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port Input Buffer Control Register (PIBCn/JPIBC0) In input port mode (PMCn.PMCnm = 0 and PMn.PMnm = 1 or JPMC0.JPMC0 = 0), this register enables or disables the input buffer for the Pn_m pin."]
pub mod pibc;
#[doc = "Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
pub struct PBDC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port Bidirection Control Register (PBDCn) - This register enables or disables the input buffer while the output buffer is enabled. When the input buffer is enabled while the output buffer is enabled, the bidirectional mode is entered, allowing the level of the Pn_m pin to always be read via the PPRn.PPRnm bit."]
pub mod pbdc;
#[doc = "Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
pub struct PIPC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port IP Control Register (PIPCn) - This register is used to specify whether the I/O direction of the Pn_m pin is controlled by the PMn.PMnm bit or by the alternative function in alternative mode (the PMCn.PMCnm bit is 1). When using an alternative function shown in Table 54.7, set the PIPCn.PIPCnm bit to 0. When using an alternative function that is not shown in Table 54.7, set the PIPCn.PIPCnm bit to 1. When the PIPCn.PIPCnm bit is set to 1, the alternative function controls the I/O direction and the PMn.PMnm bit setting becomes invalid."]
pub mod pipc;
#[doc = "Serial Sound Interface Noise Canceler Control Register (SNCR) - This register controls the noise canceler in the input route from the LSI pin to a serial sound interface. Each bit can be set only when slave mode is selected for the corresponding channel of the serial sound interface. The bit should be used as it is the initial value (H'00000000) when master mode is selected for the corresponding channel of the serial sound interface."]
pub struct SNCR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Serial Sound Interface Noise Canceler Control Register (SNCR) - This register controls the noise canceler in the input route from the LSI pin to a serial sound interface. Each bit can be set only when slave mode is selected for the corresponding channel of the serial sound interface. The bit should be used as it is the initial value (H'00000000) when master mode is selected for the corresponding channel of the serial sound interface."]
pub mod sncr;
