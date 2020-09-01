#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 24usize],
    #[doc = "0x18 - CPU Status Register (CPUSTS) - CPUSTS is an 8-bit readable register that indicates the status of the CPU."]
    pub cpusts: CPUSTS,
    _reserved1: [u8; 7usize],
    #[doc = "0x20 - Standby Control Register 1 (STBCR1) - STBCR1 is an 8-bit readable/writable register that specifies the state of the power-down mode."]
    pub stbcr1: STBCR1,
    _reserved2: [u8; 3usize],
    #[doc = "0x24 - Standby Control Register 2 (STBCR2) - STBCR2 is an 8-bit readable/writable register that controls the operation of each module."]
    pub stbcr2: STBCR2,
    _reserved3: [u8; 11usize],
    #[doc = "0x30 - Standby Request Register 1 (STBREQ1) - This register is used to request notification of whether a CPU or peripheral module is ready for standby. The CPU or peripheral module returns a standby acknowledgement on receipt of a request for notification if it is ready for standby."]
    pub stbreq1: STBREQ1,
    _reserved4: [u8; 3usize],
    #[doc = "0x34 - Standby Request Register 2 (STBREQ2) - This register is used to request notification of whether a peripheral module is ready for standby. The peripheral module returns a standby acknowledgement on receipt of a request for notification if it is ready for standby."]
    pub stbreq2: STBREQ2,
    _reserved5: [u8; 11usize],
    #[doc = "0x40 - Standby Acknowledge Register 1 (STBACK1) - This register is used to provide notification that a CPU or peripheral module is in the standby ready state. The CPU or peripheral module returns a standby acknowledgement on reception of a standby notification request if it is ready for standby. This register is a read-only register."]
    pub stback1: STBACK1,
    _reserved6: [u8; 3usize],
    #[doc = "0x44 - Standby Acknowledge Register 2 (STBACK2) - This register is used to provide notification that a peripheral module is in the standby ready state. The peripheral module returns a standby acknowledgement on reception of a standby notification request if it is ready for standby. This register is a read-only register."]
    pub stback2: STBACK2,
    _reserved7: [u8; 955usize],
    #[doc = "0x400 - System Control Register 1 (SYSCR1) - SYSCR1 is an 8-bit readable/writable register that enables or disables access (read and write) to a specified page in the large-capacity on-chip RAM. When a VRAMEn (n = 0 to 4) bit in SYSCR1 is set to 1, access to page n is enabled. When a VRAMEn bit is cleared to 0, page n cannot be accessed. In this case, an undefined value is returned when reading data or fetching an instruction from page n, and writing to page n is ignored. The initial value of a VRAMEn bit is 1. SYSCR1 should be set with a program located in an area other than the large-capacity on-chip RAM. Furthermore, an instruction to read SYSCR1 should be located immediately after the instruction to write to SYSCR1. If not, normal access is not guaranteed."]
    pub syscr1: SYSCR1,
    _reserved8: [u8; 3usize],
    #[doc = "0x404 - System Control Register 2 (SYSCR2) - SYSCR2 is an 8-bit readable/writable register that enables or disables writing to a specified page in the large-capacity on-chip RAM. When a VRAMWEn (n = 0 to 4) bit in SYSCR2 is set to 1, writing to page n is enabled. When a VRAMWEn bit is cleared to 0, writing to page n is ignored. The initial value of a VRAMWEn bit is 1. SYSCR2 should be set with a program located in an area other than the large-capacity on-chip RAM. Furthermore, an instruction to read SYSCR2 should be located immediately after the instruction to write to SYSCR2. If not, normal access is not guaranteed."]
    pub syscr2: SYSCR2,
    _reserved9: [u8; 3usize],
    #[doc = "0x408 - System Control Register 3 (SYSCR3) - SYSCR3 is an 8-bit readable/writable register that enables or disables writing to a specified page in the on-chip data- retention RAM. When a RRAMWEn (n = 0 to 3) bit in SYSCR3 is set to 1, writing to page n is enabled. When a RRAMWEn bit is cleared to 0, writing to page n is ignored. The initial value of a RRAMWEn bit is 0. SYSCR3 should be set with a program located in an area other than the on-chip data-retention RAM."]
    pub syscr3: SYSCR3,
    _reserved10: [u8; 23usize],
    #[doc = "0x420 - Standby Control Register 3 (STBCR3) - STBCR3 is an 8-bit readable/writable register that controls the operation of each module."]
    pub stbcr3: STBCR3,
    _reserved11: [u8; 3usize],
    #[doc = "0x424 - Standby Control Register 4 (STBCR4) - STBCR4 is an 8-bit readable/writable register that controls the operation of each module."]
    pub stbcr4: STBCR4,
    _reserved12: [u8; 3usize],
    #[doc = "0x428 - Standby Control Register 5 (STBCR5) - STBCR5 is an 8-bit readable/writable register that controls the operation of each module."]
    pub stbcr5: STBCR5,
    _reserved13: [u8; 3usize],
    #[doc = "0x42c - Standby Control Register 6 (STBCR6) - STBCR6 is an 8-bit readable/writable register that controls the operation of each module."]
    pub stbcr6: STBCR6,
    _reserved14: [u8; 3usize],
    #[doc = "0x430 - Standby Control Register 7 (STBCR7) - STBCR7 is an 8-bit readable/writable register that controls the operation of each module."]
    pub stbcr7: STBCR7,
    _reserved15: [u8; 3usize],
    #[doc = "0x434 - Standby Control Register 8 (STBCR8) - STBCR8 is an 8-bit readable/writable register that controls the operation of each module."]
    pub stbcr8: STBCR8,
    _reserved16: [u8; 3usize],
    #[doc = "0x438 - Standby Control Register 9 (STBCR9) - STBCR9 is an 8-bit readable/writable register that controls the operation of each module."]
    pub stbcr9: STBCR9,
    _reserved17: [u8; 3usize],
    #[doc = "0x43c - Standby Control Register 10 (STBCR10) - STBCR10 is an 8-bit readable/writable register that controls the operation of each module."]
    pub stbcr10: STBCR10,
    _reserved18: [u8; 3usize],
    #[doc = "0x440 - Standby Control Register 10 (STBCR11) - STBCR11 is an 8-bit readable/writable register that controls the operation of each module."]
    pub stbcr11: STBCR11,
    _reserved19: [u8; 3usize],
    #[doc = "0x444 - Standby Control Register 12 (STBCR12) - STBCR12 is an 8-bit readable/writable register that controls the operation of each module."]
    pub stbcr12: STBCR12,
    _reserved20: [u8; 27usize],
    #[doc = "0x460 - Software Reset Control Register 1 (SWRSTCR1) - SWRSTCR1 is an 8-bit readable/writable register that controls a software reset for the serial sound interface and the operation of the crystal resonator for audio."]
    pub swrstcr1: SWRSTCR1,
    _reserved21: [u8; 3usize],
    #[doc = "0x464 - Software Reset Control Register 2 (SWRSTCR2) - SWRSTCR2 is an 8-bit readable/writable register that controls a software reset for each module."]
    pub swrstcr2: SWRSTCR2,
    _reserved22: [u8; 3usize],
    #[doc = "0x468 - Software Reset Control Register 3 (SWRSTCR3) - SWRSTCR3 is an 8-bit readable/writable register that controls a software reset for each module."]
    pub swrstcr3: SWRSTCR3,
    _reserved23: [u8; 7usize],
    #[doc = "0x470 - Standby Control Register 13 (STBCR13) - STBCR13 is an 8-bit readable/writable register that controls the operation of each module."]
    pub stbcr13: STBCR13,
    _reserved24: [u8; 70543usize],
    #[doc = "0x11800 - On-Chip Data-Retention RAM Area Setting Register (RRAMKP) - RRAMKP is an 8-bit readable/writable register that selects whether the contents of the corresponding area of the on-chip data-retention RAM are retained or not in deep standby mode. When the RRAMKP3 to RRAMKP0 bits are set to 1, the contents of the corresponding area of the on-chip data-retention RAM are retained in deep standby mode. When these bits are cleared to 0, the contents of the corresponding area of the on-chip data-retention RAM are not retained in deep standby mode."]
    pub rramkp: RRAMKP,
    _reserved25: [u8; 1usize],
    #[doc = "0x11802 - Deep Standby Control Register (DSCTR) - DSCTR is an 8-bit readable/writable register that selects whether the states of the external memory control pins are retained or not when returning from deep standby mode and specifies the method to start the LSI. The following setting is prohibited: (EBUSKEEPE, RAMBOOT) = (1, 0)."]
    pub dsctr: DSCTR,
    _reserved26: [u8; 1usize],
    #[doc = "0x11804 - Deep Standby Cancel Source Select Register (DSSSR) - DSSSR is a 16-bit readable/writable register that consists of the bits for selecting a source to cancel deep standby mode. The realtime clock alarm interrupt or change on the pins for canceling (P2_12, P2_15, P3_1, P3_3, P3_9, P5_9, P6_2, P6_4, P7_8, P8_2, P8_7, and P9_1) can be selected as a cancel source. The pins for canceling can be used for canceling deep standby, regardless of pin function settings in the general I/O port."]
    pub dsssr: DSSSR,
    #[doc = "0x11806 - Deep Standby Cancel Edge Select Register (DSESR) - DSESR is a 16-bit readable/writable register that consists of the bits for selecting an edge to be detected for the pin specified as a deep standby cancel source with DSSSR. This register setting is always valid for canceling deep standby, regardless of the interrupt controller setting."]
    pub dsesr: DSESR,
    #[doc = "0x11808 - Deep Standby Cancel Source Flag Register (DSFR) - DSFR is a 16-bit readable/writable register composed of two types of bits. One is the flag that confirms which source canceled deep standby mode. The other is the bit that releases the state of pins after canceling deep standby mode. When deep standby mode is canceled by an interrupt (NMI or realtime clock alarm interrupt) and changes on the pins for canceling, this register retains the previous data although power-on reset exception handling is executed. When deep standby mode is canceled by a power-on reset, this register is initialized to H'0000. All flags must be cleared immediately before transition to deep standby mode."]
    pub dsfr: DSFR,
    _reserved27: [u8; 6usize],
    #[doc = "0x11810 - XTAL Crystal Oscillator Gain Control Register (XTALCTR) - XTALCTR is an 8-bit readable/writable register that controls the gain of the crystal oscillator for XTAL and the realtime clock. If the realtime clock uses the EXTAL input, the GAIN0 bit retains the previous value when software standby mode or deep standby mode is canceled by a source other than a power-on reset. If the realtime clock does not use the EXTAL input, this bit is initialized to 0 when software standby or deep standby mode is entered. If the realtime clock uses the RTC_X1 or RTC_X3 input, the GAIN1 bit retains the previous value when software standby mode or deep standby mode is canceled by a source other than a power-on reset. If the realtime clock does not use the RTC_X1 or RTC_X3 input, this bit is initialized to 0 when software standby or deep standby mode is entered. This bit is initialized to H'00 when software standby or deep standby mode is canceled by a power-on reset."]
    pub xtalctr: XTALCTR,
}
#[doc = "Standby Control Register 1 (STBCR1) - STBCR1 is an 8-bit readable/writable register that specifies the state of the power-down mode."]
pub struct STBCR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 1 (STBCR1) - STBCR1 is an 8-bit readable/writable register that specifies the state of the power-down mode."]
pub mod stbcr1;
#[doc = "Standby Control Register 2 (STBCR2) - STBCR2 is an 8-bit readable/writable register that controls the operation of each module."]
pub struct STBCR2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 2 (STBCR2) - STBCR2 is an 8-bit readable/writable register that controls the operation of each module."]
pub mod stbcr2;
#[doc = "Standby Control Register 3 (STBCR3) - STBCR3 is an 8-bit readable/writable register that controls the operation of each module."]
pub struct STBCR3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 3 (STBCR3) - STBCR3 is an 8-bit readable/writable register that controls the operation of each module."]
pub mod stbcr3;
#[doc = "Standby Control Register 4 (STBCR4) - STBCR4 is an 8-bit readable/writable register that controls the operation of each module."]
pub struct STBCR4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 4 (STBCR4) - STBCR4 is an 8-bit readable/writable register that controls the operation of each module."]
pub mod stbcr4;
#[doc = "Standby Control Register 5 (STBCR5) - STBCR5 is an 8-bit readable/writable register that controls the operation of each module."]
pub struct STBCR5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 5 (STBCR5) - STBCR5 is an 8-bit readable/writable register that controls the operation of each module."]
pub mod stbcr5;
#[doc = "Standby Control Register 6 (STBCR6) - STBCR6 is an 8-bit readable/writable register that controls the operation of each module."]
pub struct STBCR6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 6 (STBCR6) - STBCR6 is an 8-bit readable/writable register that controls the operation of each module."]
pub mod stbcr6;
#[doc = "Standby Control Register 7 (STBCR7) - STBCR7 is an 8-bit readable/writable register that controls the operation of each module."]
pub struct STBCR7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 7 (STBCR7) - STBCR7 is an 8-bit readable/writable register that controls the operation of each module."]
pub mod stbcr7;
#[doc = "Standby Control Register 8 (STBCR8) - STBCR8 is an 8-bit readable/writable register that controls the operation of each module."]
pub struct STBCR8 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 8 (STBCR8) - STBCR8 is an 8-bit readable/writable register that controls the operation of each module."]
pub mod stbcr8;
#[doc = "Standby Control Register 9 (STBCR9) - STBCR9 is an 8-bit readable/writable register that controls the operation of each module."]
pub struct STBCR9 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 9 (STBCR9) - STBCR9 is an 8-bit readable/writable register that controls the operation of each module."]
pub mod stbcr9;
#[doc = "Standby Control Register 10 (STBCR10) - STBCR10 is an 8-bit readable/writable register that controls the operation of each module."]
pub struct STBCR10 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 10 (STBCR10) - STBCR10 is an 8-bit readable/writable register that controls the operation of each module."]
pub mod stbcr10;
#[doc = "Standby Control Register 10 (STBCR11) - STBCR11 is an 8-bit readable/writable register that controls the operation of each module."]
pub struct STBCR11 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 10 (STBCR11) - STBCR11 is an 8-bit readable/writable register that controls the operation of each module."]
pub mod stbcr11;
#[doc = "Standby Control Register 12 (STBCR12) - STBCR12 is an 8-bit readable/writable register that controls the operation of each module."]
pub struct STBCR12 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 12 (STBCR12) - STBCR12 is an 8-bit readable/writable register that controls the operation of each module."]
pub mod stbcr12;
#[doc = "Standby Control Register 13 (STBCR13) - STBCR13 is an 8-bit readable/writable register that controls the operation of each module."]
pub struct STBCR13 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Control Register 13 (STBCR13) - STBCR13 is an 8-bit readable/writable register that controls the operation of each module."]
pub mod stbcr13;
#[doc = "Software Reset Control Register 1 (SWRSTCR1) - SWRSTCR1 is an 8-bit readable/writable register that controls a software reset for the serial sound interface and the operation of the crystal resonator for audio."]
pub struct SWRSTCR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Software Reset Control Register 1 (SWRSTCR1) - SWRSTCR1 is an 8-bit readable/writable register that controls a software reset for the serial sound interface and the operation of the crystal resonator for audio."]
pub mod swrstcr1;
#[doc = "Software Reset Control Register 2 (SWRSTCR2) - SWRSTCR2 is an 8-bit readable/writable register that controls a software reset for each module."]
pub struct SWRSTCR2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Software Reset Control Register 2 (SWRSTCR2) - SWRSTCR2 is an 8-bit readable/writable register that controls a software reset for each module."]
pub mod swrstcr2;
#[doc = "Software Reset Control Register 3 (SWRSTCR3) - SWRSTCR3 is an 8-bit readable/writable register that controls a software reset for each module."]
pub struct SWRSTCR3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Software Reset Control Register 3 (SWRSTCR3) - SWRSTCR3 is an 8-bit readable/writable register that controls a software reset for each module."]
pub mod swrstcr3;
#[doc = "System Control Register 1 (SYSCR1) - SYSCR1 is an 8-bit readable/writable register that enables or disables access (read and write) to a specified page in the large-capacity on-chip RAM. When a VRAMEn (n = 0 to 4) bit in SYSCR1 is set to 1, access to page n is enabled. When a VRAMEn bit is cleared to 0, page n cannot be accessed. In this case, an undefined value is returned when reading data or fetching an instruction from page n, and writing to page n is ignored. The initial value of a VRAMEn bit is 1. SYSCR1 should be set with a program located in an area other than the large-capacity on-chip RAM. Furthermore, an instruction to read SYSCR1 should be located immediately after the instruction to write to SYSCR1. If not, normal access is not guaranteed."]
pub struct SYSCR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "System Control Register 1 (SYSCR1) - SYSCR1 is an 8-bit readable/writable register that enables or disables access (read and write) to a specified page in the large-capacity on-chip RAM. When a VRAMEn (n = 0 to 4) bit in SYSCR1 is set to 1, access to page n is enabled. When a VRAMEn bit is cleared to 0, page n cannot be accessed. In this case, an undefined value is returned when reading data or fetching an instruction from page n, and writing to page n is ignored. The initial value of a VRAMEn bit is 1. SYSCR1 should be set with a program located in an area other than the large-capacity on-chip RAM. Furthermore, an instruction to read SYSCR1 should be located immediately after the instruction to write to SYSCR1. If not, normal access is not guaranteed."]
pub mod syscr1;
#[doc = "System Control Register 2 (SYSCR2) - SYSCR2 is an 8-bit readable/writable register that enables or disables writing to a specified page in the large-capacity on-chip RAM. When a VRAMWEn (n = 0 to 4) bit in SYSCR2 is set to 1, writing to page n is enabled. When a VRAMWEn bit is cleared to 0, writing to page n is ignored. The initial value of a VRAMWEn bit is 1. SYSCR2 should be set with a program located in an area other than the large-capacity on-chip RAM. Furthermore, an instruction to read SYSCR2 should be located immediately after the instruction to write to SYSCR2. If not, normal access is not guaranteed."]
pub struct SYSCR2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "System Control Register 2 (SYSCR2) - SYSCR2 is an 8-bit readable/writable register that enables or disables writing to a specified page in the large-capacity on-chip RAM. When a VRAMWEn (n = 0 to 4) bit in SYSCR2 is set to 1, writing to page n is enabled. When a VRAMWEn bit is cleared to 0, writing to page n is ignored. The initial value of a VRAMWEn bit is 1. SYSCR2 should be set with a program located in an area other than the large-capacity on-chip RAM. Furthermore, an instruction to read SYSCR2 should be located immediately after the instruction to write to SYSCR2. If not, normal access is not guaranteed."]
pub mod syscr2;
#[doc = "System Control Register 3 (SYSCR3) - SYSCR3 is an 8-bit readable/writable register that enables or disables writing to a specified page in the on-chip data- retention RAM. When a RRAMWEn (n = 0 to 3) bit in SYSCR3 is set to 1, writing to page n is enabled. When a RRAMWEn bit is cleared to 0, writing to page n is ignored. The initial value of a RRAMWEn bit is 0. SYSCR3 should be set with a program located in an area other than the on-chip data-retention RAM."]
pub struct SYSCR3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "System Control Register 3 (SYSCR3) - SYSCR3 is an 8-bit readable/writable register that enables or disables writing to a specified page in the on-chip data- retention RAM. When a RRAMWEn (n = 0 to 3) bit in SYSCR3 is set to 1, writing to page n is enabled. When a RRAMWEn bit is cleared to 0, writing to page n is ignored. The initial value of a RRAMWEn bit is 0. SYSCR3 should be set with a program located in an area other than the on-chip data-retention RAM."]
pub mod syscr3;
#[doc = "CPU Status Register (CPUSTS) - CPUSTS is an 8-bit readable register that indicates the status of the CPU."]
pub struct CPUSTS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CPU Status Register (CPUSTS) - CPUSTS is an 8-bit readable register that indicates the status of the CPU."]
pub mod cpusts;
#[doc = "Standby Request Register 1 (STBREQ1) - This register is used to request notification of whether a CPU or peripheral module is ready for standby. The CPU or peripheral module returns a standby acknowledgement on receipt of a request for notification if it is ready for standby."]
pub struct STBREQ1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Request Register 1 (STBREQ1) - This register is used to request notification of whether a CPU or peripheral module is ready for standby. The CPU or peripheral module returns a standby acknowledgement on receipt of a request for notification if it is ready for standby."]
pub mod stbreq1;
#[doc = "Standby Request Register 2 (STBREQ2) - This register is used to request notification of whether a peripheral module is ready for standby. The peripheral module returns a standby acknowledgement on receipt of a request for notification if it is ready for standby."]
pub struct STBREQ2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Request Register 2 (STBREQ2) - This register is used to request notification of whether a peripheral module is ready for standby. The peripheral module returns a standby acknowledgement on receipt of a request for notification if it is ready for standby."]
pub mod stbreq2;
#[doc = "Standby Acknowledge Register 1 (STBACK1) - This register is used to provide notification that a CPU or peripheral module is in the standby ready state. The CPU or peripheral module returns a standby acknowledgement on reception of a standby notification request if it is ready for standby. This register is a read-only register."]
pub struct STBACK1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Acknowledge Register 1 (STBACK1) - This register is used to provide notification that a CPU or peripheral module is in the standby ready state. The CPU or peripheral module returns a standby acknowledgement on reception of a standby notification request if it is ready for standby. This register is a read-only register."]
pub mod stback1;
#[doc = "Standby Acknowledge Register 2 (STBACK2) - This register is used to provide notification that a peripheral module is in the standby ready state. The peripheral module returns a standby acknowledgement on reception of a standby notification request if it is ready for standby. This register is a read-only register."]
pub struct STBACK2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Acknowledge Register 2 (STBACK2) - This register is used to provide notification that a peripheral module is in the standby ready state. The peripheral module returns a standby acknowledgement on reception of a standby notification request if it is ready for standby. This register is a read-only register."]
pub mod stback2;
#[doc = "On-Chip Data-Retention RAM Area Setting Register (RRAMKP) - RRAMKP is an 8-bit readable/writable register that selects whether the contents of the corresponding area of the on-chip data-retention RAM are retained or not in deep standby mode. When the RRAMKP3 to RRAMKP0 bits are set to 1, the contents of the corresponding area of the on-chip data-retention RAM are retained in deep standby mode. When these bits are cleared to 0, the contents of the corresponding area of the on-chip data-retention RAM are not retained in deep standby mode."]
pub struct RRAMKP {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "On-Chip Data-Retention RAM Area Setting Register (RRAMKP) - RRAMKP is an 8-bit readable/writable register that selects whether the contents of the corresponding area of the on-chip data-retention RAM are retained or not in deep standby mode. When the RRAMKP3 to RRAMKP0 bits are set to 1, the contents of the corresponding area of the on-chip data-retention RAM are retained in deep standby mode. When these bits are cleared to 0, the contents of the corresponding area of the on-chip data-retention RAM are not retained in deep standby mode."]
pub mod rramkp;
#[doc = "Deep Standby Control Register (DSCTR) - DSCTR is an 8-bit readable/writable register that selects whether the states of the external memory control pins are retained or not when returning from deep standby mode and specifies the method to start the LSI. The following setting is prohibited: (EBUSKEEPE, RAMBOOT) = (1, 0)."]
pub struct DSCTR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Deep Standby Control Register (DSCTR) - DSCTR is an 8-bit readable/writable register that selects whether the states of the external memory control pins are retained or not when returning from deep standby mode and specifies the method to start the LSI. The following setting is prohibited: (EBUSKEEPE, RAMBOOT) = (1, 0)."]
pub mod dsctr;
#[doc = "Deep Standby Cancel Source Select Register (DSSSR) - DSSSR is a 16-bit readable/writable register that consists of the bits for selecting a source to cancel deep standby mode. The realtime clock alarm interrupt or change on the pins for canceling (P2_12, P2_15, P3_1, P3_3, P3_9, P5_9, P6_2, P6_4, P7_8, P8_2, P8_7, and P9_1) can be selected as a cancel source. The pins for canceling can be used for canceling deep standby, regardless of pin function settings in the general I/O port."]
pub struct DSSSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Deep Standby Cancel Source Select Register (DSSSR) - DSSSR is a 16-bit readable/writable register that consists of the bits for selecting a source to cancel deep standby mode. The realtime clock alarm interrupt or change on the pins for canceling (P2_12, P2_15, P3_1, P3_3, P3_9, P5_9, P6_2, P6_4, P7_8, P8_2, P8_7, and P9_1) can be selected as a cancel source. The pins for canceling can be used for canceling deep standby, regardless of pin function settings in the general I/O port."]
pub mod dsssr;
#[doc = "Deep Standby Cancel Edge Select Register (DSESR) - DSESR is a 16-bit readable/writable register that consists of the bits for selecting an edge to be detected for the pin specified as a deep standby cancel source with DSSSR. This register setting is always valid for canceling deep standby, regardless of the interrupt controller setting."]
pub struct DSESR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Deep Standby Cancel Edge Select Register (DSESR) - DSESR is a 16-bit readable/writable register that consists of the bits for selecting an edge to be detected for the pin specified as a deep standby cancel source with DSSSR. This register setting is always valid for canceling deep standby, regardless of the interrupt controller setting."]
pub mod dsesr;
#[doc = "Deep Standby Cancel Source Flag Register (DSFR) - DSFR is a 16-bit readable/writable register composed of two types of bits. One is the flag that confirms which source canceled deep standby mode. The other is the bit that releases the state of pins after canceling deep standby mode. When deep standby mode is canceled by an interrupt (NMI or realtime clock alarm interrupt) and changes on the pins for canceling, this register retains the previous data although power-on reset exception handling is executed. When deep standby mode is canceled by a power-on reset, this register is initialized to H'0000. All flags must be cleared immediately before transition to deep standby mode."]
pub struct DSFR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Deep Standby Cancel Source Flag Register (DSFR) - DSFR is a 16-bit readable/writable register composed of two types of bits. One is the flag that confirms which source canceled deep standby mode. The other is the bit that releases the state of pins after canceling deep standby mode. When deep standby mode is canceled by an interrupt (NMI or realtime clock alarm interrupt) and changes on the pins for canceling, this register retains the previous data although power-on reset exception handling is executed. When deep standby mode is canceled by a power-on reset, this register is initialized to H'0000. All flags must be cleared immediately before transition to deep standby mode."]
pub mod dsfr;
#[doc = "XTAL Crystal Oscillator Gain Control Register (XTALCTR) - XTALCTR is an 8-bit readable/writable register that controls the gain of the crystal oscillator for XTAL and the realtime clock. If the realtime clock uses the EXTAL input, the GAIN0 bit retains the previous value when software standby mode or deep standby mode is canceled by a source other than a power-on reset. If the realtime clock does not use the EXTAL input, this bit is initialized to 0 when software standby or deep standby mode is entered. If the realtime clock uses the RTC_X1 or RTC_X3 input, the GAIN1 bit retains the previous value when software standby mode or deep standby mode is canceled by a source other than a power-on reset. If the realtime clock does not use the RTC_X1 or RTC_X3 input, this bit is initialized to 0 when software standby or deep standby mode is entered. This bit is initialized to H'00 when software standby or deep standby mode is canceled by a power-on reset."]
pub struct XTALCTR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "XTAL Crystal Oscillator Gain Control Register (XTALCTR) - XTALCTR is an 8-bit readable/writable register that controls the gain of the crystal oscillator for XTAL and the realtime clock. If the realtime clock uses the EXTAL input, the GAIN0 bit retains the previous value when software standby mode or deep standby mode is canceled by a source other than a power-on reset. If the realtime clock does not use the EXTAL input, this bit is initialized to 0 when software standby or deep standby mode is entered. If the realtime clock uses the RTC_X1 or RTC_X3 input, the GAIN1 bit retains the previous value when software standby mode or deep standby mode is canceled by a source other than a power-on reset. If the realtime clock does not use the RTC_X1 or RTC_X3 input, this bit is initialized to 0 when software standby or deep standby mode is entered. This bit is initialized to H'00 when software standby or deep standby mode is canceled by a power-on reset."]
pub mod xtalctr;
