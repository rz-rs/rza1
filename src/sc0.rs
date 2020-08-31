#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Serial Mode Register (SCSMR) - SCSMR specifies the serial communication format and selects the clock source for the baud rate generator."]
    pub smr: SMR,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - Bit Rate Register (SCBRR) - SCBRR is an 8-bit register that is used with the CKS1 and CKS0 bits in the serial mode register (SCSMR) and the BGDM and ABCS bits in the serial extension mode register (SCEMR) to determine the serial transmit/receive bit rate."]
    pub brr: BRR,
    _reserved1: [u8; 3usize],
    #[doc = "0x08 - Serial Control Register (SCSCR) - SCSCR enables/disables the transmitter/receiver operation and interrupt requests, and selects the transmit/receive clock source. The CPU can always read and write to SCSCR."]
    pub scr: SCR,
    _reserved2: [u8; 2usize],
    #[doc = "0x0c - Transmit FIFO Data Register (SCFTDR) - SCFTDR is a 16-stage FIFO register that stores data for serial transmission. When the transmit shift register (SCTSR) empty is detected, transmit data written in the SCFTDR is moved to SCTSR and serial transmission is started. Continuous serial transmission is performed until there is no transmit data left in SCFTDR. The CPU can write to SCFTDR at all times."]
    pub ftdr: FTDR,
    _reserved3: [u8; 3usize],
    #[doc = "0x10 - Serial Status Register (SCFSR) - SCFSR is a 16-bit register. The upper 8 bits indicate the number of receive errors in the receive FIFO data register, and the lower 8 bits indicate the status flag indicating operating state. The CPU can always read from and write to SCFSR, but cannot write 1 to the status flags (ER, TEND, TDFE, BRK, RDF, and DR). These flags can be cleared to 0 only if they have first been read (after being set to 1). The PER flag (bits 15 to 12 and bit 2) and the FER flag (bits 11 to 8 and bit 3) are read-only bits that cannot be written."]
    pub fsr: FSR,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - Receive FIFO Data Register (SCFRDR) - SCFRDR is a 16-stage FIFO register that stores serial receive data. The reception of one byte of serial data is complete when the received data is moved from the receive shift register (SCRSR) to SCFRDR for storage. Continuous reception is possible until 16 bytes are stored. The CPU can read but not write to SCFRDR. If data is read when there is no receive data in the SCFRDR, the value is undefined."]
    pub frdr: FRDR,
    _reserved5: [u8; 3usize],
    #[doc = "0x18 - FIFO Control Register (SCFCR) - SCFCR resets the quantity of data in the transmit and receive FIFO data registers, sets the trigger data quantity, and contains an enable bit for loop-back testing. SCFCR can always be read and written to by the CPU."]
    pub fcr: FCR,
    _reserved6: [u8; 2usize],
    #[doc = "0x1c - FIFO Data Count Set Register (SCFDR) - SCFDR is a 16-bit register which indicates the quantity of data stored in the transmit FIFO data register (SCFTDR) and the receive FIFO data register (SCFRDR). It indicates the quantity of transmit data in SCFTDR with the upper 8 bits, and the quantity of receive data in SCFRDR with the lower 8 bits. SCFDR can always be read by the CPU."]
    pub fdr: FDR,
    _reserved7: [u8; 2usize],
    #[doc = "0x20 - Serial Port Register (SCSPTR) - SCSPTR controls input/output and data of pins multiplexed to the functions of this module. Bits 7 and 6 can control input/output data of RTS pin. Bits 5 and 4 can control input/output data of CTS pin. Bits 3 and 2 can control input/output data of SCK pin. Bits 1 and 0 can input data from RxD pin and output data to TxD pin, so they control break of serial transmitting/receiving."]
    pub sptr: SPTR,
    _reserved8: [u8; 2usize],
    #[doc = "0x24 - Line Status Register (SCLSR) - The CPU can always read or write to SCLSR, but cannot write 1 to the ORER flag. This flag can be cleared to 0 only if it has first been read (after being set to 1)."]
    pub lsr: LSR,
    _reserved9: [u8; 2usize],
    #[doc = "0x28 - Serial Extension Mode Register (SCEMR) - The CPU can always read from or write to SCEMR. Setting the BGDM bit in this register to 1 allows the baud rate generator in this module to operate in double-speed mode when asynchronous mode is selected (by setting the C/A bit in SCSMR to 0) and an internal clock is selected as a clock source and the SCK pin is set as an input pin (by setting the CKE\\[1:0\\] bits in SCSCR to 00)."]
    pub scemr: SCEMR,
}
#[doc = "Serial Mode Register (SCSMR) - SCSMR specifies the serial communication format and selects the clock source for the baud rate generator."]
pub struct SMR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Serial Mode Register (SCSMR) - SCSMR specifies the serial communication format and selects the clock source for the baud rate generator."]
pub mod smr;
#[doc = "Bit Rate Register (SCBRR) - SCBRR is an 8-bit register that is used with the CKS1 and CKS0 bits in the serial mode register (SCSMR) and the BGDM and ABCS bits in the serial extension mode register (SCEMR) to determine the serial transmit/receive bit rate."]
pub struct BRR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Bit Rate Register (SCBRR) - SCBRR is an 8-bit register that is used with the CKS1 and CKS0 bits in the serial mode register (SCSMR) and the BGDM and ABCS bits in the serial extension mode register (SCEMR) to determine the serial transmit/receive bit rate."]
pub mod brr;
#[doc = "Serial Control Register (SCSCR) - SCSCR enables/disables the transmitter/receiver operation and interrupt requests, and selects the transmit/receive clock source. The CPU can always read and write to SCSCR."]
pub struct SCR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Serial Control Register (SCSCR) - SCSCR enables/disables the transmitter/receiver operation and interrupt requests, and selects the transmit/receive clock source. The CPU can always read and write to SCSCR."]
pub mod scr;
#[doc = "Transmit FIFO Data Register (SCFTDR) - SCFTDR is a 16-stage FIFO register that stores data for serial transmission. When the transmit shift register (SCTSR) empty is detected, transmit data written in the SCFTDR is moved to SCTSR and serial transmission is started. Continuous serial transmission is performed until there is no transmit data left in SCFTDR. The CPU can write to SCFTDR at all times."]
pub struct FTDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Transmit FIFO Data Register (SCFTDR) - SCFTDR is a 16-stage FIFO register that stores data for serial transmission. When the transmit shift register (SCTSR) empty is detected, transmit data written in the SCFTDR is moved to SCTSR and serial transmission is started. Continuous serial transmission is performed until there is no transmit data left in SCFTDR. The CPU can write to SCFTDR at all times."]
pub mod ftdr;
#[doc = "Serial Status Register (SCFSR) - SCFSR is a 16-bit register. The upper 8 bits indicate the number of receive errors in the receive FIFO data register, and the lower 8 bits indicate the status flag indicating operating state. The CPU can always read from and write to SCFSR, but cannot write 1 to the status flags (ER, TEND, TDFE, BRK, RDF, and DR). These flags can be cleared to 0 only if they have first been read (after being set to 1). The PER flag (bits 15 to 12 and bit 2) and the FER flag (bits 11 to 8 and bit 3) are read-only bits that cannot be written."]
pub struct FSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Serial Status Register (SCFSR) - SCFSR is a 16-bit register. The upper 8 bits indicate the number of receive errors in the receive FIFO data register, and the lower 8 bits indicate the status flag indicating operating state. The CPU can always read from and write to SCFSR, but cannot write 1 to the status flags (ER, TEND, TDFE, BRK, RDF, and DR). These flags can be cleared to 0 only if they have first been read (after being set to 1). The PER flag (bits 15 to 12 and bit 2) and the FER flag (bits 11 to 8 and bit 3) are read-only bits that cannot be written."]
pub mod fsr;
#[doc = "Receive FIFO Data Register (SCFRDR) - SCFRDR is a 16-stage FIFO register that stores serial receive data. The reception of one byte of serial data is complete when the received data is moved from the receive shift register (SCRSR) to SCFRDR for storage. Continuous reception is possible until 16 bytes are stored. The CPU can read but not write to SCFRDR. If data is read when there is no receive data in the SCFRDR, the value is undefined."]
pub struct FRDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Receive FIFO Data Register (SCFRDR) - SCFRDR is a 16-stage FIFO register that stores serial receive data. The reception of one byte of serial data is complete when the received data is moved from the receive shift register (SCRSR) to SCFRDR for storage. Continuous reception is possible until 16 bytes are stored. The CPU can read but not write to SCFRDR. If data is read when there is no receive data in the SCFRDR, the value is undefined."]
pub mod frdr;
#[doc = "FIFO Control Register (SCFCR) - SCFCR resets the quantity of data in the transmit and receive FIFO data registers, sets the trigger data quantity, and contains an enable bit for loop-back testing. SCFCR can always be read and written to by the CPU."]
pub struct FCR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "FIFO Control Register (SCFCR) - SCFCR resets the quantity of data in the transmit and receive FIFO data registers, sets the trigger data quantity, and contains an enable bit for loop-back testing. SCFCR can always be read and written to by the CPU."]
pub mod fcr;
#[doc = "FIFO Data Count Set Register (SCFDR) - SCFDR is a 16-bit register which indicates the quantity of data stored in the transmit FIFO data register (SCFTDR) and the receive FIFO data register (SCFRDR). It indicates the quantity of transmit data in SCFTDR with the upper 8 bits, and the quantity of receive data in SCFRDR with the lower 8 bits. SCFDR can always be read by the CPU."]
pub struct FDR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "FIFO Data Count Set Register (SCFDR) - SCFDR is a 16-bit register which indicates the quantity of data stored in the transmit FIFO data register (SCFTDR) and the receive FIFO data register (SCFRDR). It indicates the quantity of transmit data in SCFTDR with the upper 8 bits, and the quantity of receive data in SCFRDR with the lower 8 bits. SCFDR can always be read by the CPU."]
pub mod fdr;
#[doc = "Serial Port Register (SCSPTR) - SCSPTR controls input/output and data of pins multiplexed to the functions of this module. Bits 7 and 6 can control input/output data of RTS pin. Bits 5 and 4 can control input/output data of CTS pin. Bits 3 and 2 can control input/output data of SCK pin. Bits 1 and 0 can input data from RxD pin and output data to TxD pin, so they control break of serial transmitting/receiving."]
pub struct SPTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Serial Port Register (SCSPTR) - SCSPTR controls input/output and data of pins multiplexed to the functions of this module. Bits 7 and 6 can control input/output data of RTS pin. Bits 5 and 4 can control input/output data of CTS pin. Bits 3 and 2 can control input/output data of SCK pin. Bits 1 and 0 can input data from RxD pin and output data to TxD pin, so they control break of serial transmitting/receiving."]
pub mod sptr;
#[doc = "Line Status Register (SCLSR) - The CPU can always read or write to SCLSR, but cannot write 1 to the ORER flag. This flag can be cleared to 0 only if it has first been read (after being set to 1)."]
pub struct LSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Line Status Register (SCLSR) - The CPU can always read or write to SCLSR, but cannot write 1 to the ORER flag. This flag can be cleared to 0 only if it has first been read (after being set to 1)."]
pub mod lsr;
#[doc = "Serial Extension Mode Register (SCEMR) - The CPU can always read from or write to SCEMR. Setting the BGDM bit in this register to 1 allows the baud rate generator in this module to operate in double-speed mode when asynchronous mode is selected (by setting the C/A bit in SCSMR to 0) and an internal clock is selected as a clock source and the SCK pin is set as an input pin (by setting the CKE\\[1:0\\] bits in SCSCR to 00)."]
pub struct SCEMR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Serial Extension Mode Register (SCEMR) - The CPU can always read from or write to SCEMR. Setting the BGDM bit in this register to 1 allows the baud rate generator in this module to operate in double-speed mode when asynchronous mode is selected (by setting the C/A bit in SCSMR to 0) and an internal clock is selected as a clock source and the SCK pin is set as an input pin (by setting the CKE\\[1:0\\] bits in SCSCR to 00)."]
pub mod scemr;
