#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FSR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct PERNUMR {
    bits: u8,
}
impl PERNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FERNUMR {
    bits: u8,
}
impl FERNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ERR {
    bits: bool,
}
impl ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TENDR {
    bits: bool,
}
impl TENDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TDFER {
    bits: bool,
}
impl TDFER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct BRKR {
    bits: bool,
}
impl BRKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FERR {
    bits: bool,
}
impl FERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PERR {
    bits: bool,
}
impl PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RDFR {
    bits: bool,
}
impl RDFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DRR {
    bits: bool,
}
impl DRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _PERNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _PERNUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FERNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _FERNUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ERW<'a> {
    w: &'a mut W,
}
impl<'a> _ERW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TENDW<'a> {
    w: &'a mut W,
}
impl<'a> _TENDW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TDFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDFEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BRKW<'a> {
    w: &'a mut W,
}
impl<'a> _BRKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RDFW<'a> {
    w: &'a mut W,
}
impl<'a> _RDFW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DRW<'a> {
    w: &'a mut W,
}
impl<'a> _DRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 12:15 - Number of Parity Errors - Indicate the quantity of data including a parity error in the receive data stored in the receive FIFO data register (SCFRDR). The value indicated by bits 15 to 12 after the ER bit in SCFSR is set, represents the number of parity errors in SCFRDR. When parity errors have occurred in all 16-byte receive data in SCFRDR, PER\\[3:0\\] shows 0000."]
    #[inline]
    pub fn pernum(&self) -> PERNUMR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        PERNUMR { bits }
    }
    #[doc = "Bits 8:11 - Number of Framing Errors - Indicate the quantity of data including a framing error in the receive data stored in SCFRDR. The value indicated by bits 11 to 8 after the ER bit in SCFSR is set, represents the number of framing errors in SCFRDR. When framing errors have occurred in all 16-byte receive data in SCFRDR, FER\\[3:0\\] shows 0000."]
    #[inline]
    pub fn fernum(&self) -> FERNUMR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        FERNUMR { bits }
    }
    #[doc = "Bit 7 - Receive Error - currence of a framing error, or of a parity error when receiving data that includes parity.(*1) 0: Receiving is in progress or has ended normally \\[Clearing conditions\\] - ER is cleared to 0 by a power-on reset - ER is cleared to 0 when 0 is written to after 1 is read from ER 1: A framing error or parity error has occurred. \\[Setting conditions\\] - ER is set to 1 when the stop bit is 0 after checking whether or not the last stop bit of the received data is 1 at the end of one data receive operation (*2) - ER is set to 1 when the total number of 1s in the receive data plus parity bit does not match the even/odd parity specified by the O/E bit in SCSMR (*1) Clearing the RE bit to 0 in SCSCR does not affect the ER bit, which retains its previous value. Even if a receive error occurs, the receive data is transferred to SCFRDR and the receive operation is continued. Whether or not the data read from SCFRDR includes a receive error can be detected by the FER and PER bits in SCFSR. (*2) In two stop bits mode, only the first stop bit is checked; the second stop bit is not checked."]
    #[inline]
    pub fn er(&self) -> ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        ERR { bits }
    }
    #[doc = "Bit 6 - Transmit End - Indicates that when the last bit of a serial character was transmitted, SCFTDR did not contain valid data, so transmission has ended. 0: Transmission is in progress \\[Clearing condition\\] - TEND is cleared to 0 when 0 is written after 1 is read from TEND after transmit data is written in SCFTDR (*1) 1: End of transmission \\[Setting conditions\\] - TEND is set to 1 by a power-on reset - TEND is set to 1 when TE is cleared to 0 in the serial control register (SCSCR) - TEND is set to 1 when SCFTDR does not contain transmit data when the last bit of a one-byte serial character is transmitted (*1) Do not use this bit as a transmit end flag when the direct memory access controller writes data to SCFTDR due to a TXI interrupt request."]
    #[inline]
    pub fn tend(&self) -> TENDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TENDR { bits }
    }
    #[doc = "Bit 5 - Transmit FIFO Data Empty - Indicates that data has been transferred from the transmit FIFO data register (SCFTDR) to the transmit shift register (SCTSR), the quantity of data in SCFTDR has become less than the transmission trigger number specified by the TTRG\\[1:0\\] bits in the FIFO control register (SCFCR), and writing of transmit data to SCFTDR is enabled. 0: The quantity of transmit data written to SCFTDR is greater than the specified transmission trigger number \\[Clearing conditions\\] - TDFE is cleared to 0 when data exceeding the specified transmission trigger number is written to SCFTDR after 1 is read from TDFE and then 0 is written to TDFE - TDFE is cleared to 0 when direct memory access controller is activated by transmit FIFO data empty interrupt (TXI) and data exceeding the specified transmission trigger number is written to SCFTDR 1: The quantity of transmit data in SCFTDR is less than or equal to the specified transmission trigger number (*1) \\[Setting conditions\\] - TDFE is set to 1 by a power-on reset - TDFE is set to 1 when the quantity of transmit data in SCFTDR becomes less than or equal to the specified transmission trigger number as a result of transmission (*1) Since SCFTDR is a 16-byte FIFO register, the maximum quantity of data that can be written when TDFE is 1 is \"16 minus the specified transmission trigger number\". If an attempt is made to write additional data, the data is ignored. The quantity of data in SCFTDR is indicated by the upper 8 bits of SCFDR."]
    #[inline]
    pub fn tdfe(&self) -> TDFER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TDFER { bits }
    }
    #[doc = "Bit 4 - Break Detection - Indicates that a break signal has been detected in receive data. 0: No break signal received \\[Clearing conditions\\] - BRK is cleared to 0 by a power-on reset - BRK is cleared to 0 when software reads BRK after it has been set to 1, then writes 0 to BRK 1: Break signal received (*1) \\[Setting condition\\] - BRK is set to 1 when data including a framing error is received, followed by at least one frame at the space 0 level (low level) (*1) When a break is detected, transfer of the receive data (H'00) to SCFRDR stops after detection. When the break ends and the receive signal becomes mark 1, the transfer of receive data resumes."]
    #[inline]
    pub fn brk(&self) -> BRKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        BRKR { bits }
    }
    #[doc = "Bit 3 - Framing Error Indication -Indicates a framing error in the data read from the receive FIFO data register (SCFRDR) in asynchronous mode. 0: No receive framing error occurred in the next data read from SCFRDR \\[Clearing conditions\\] - FER is cleared to 0 by a power-on reset - FER is cleared to 0 when no framing error is present in the next data read from SCFRDR 1: A receive framing error occurred in the next data read from SCFRDR \\[Setting condition\\] - FER is set to 1 when a framing error is present in the next data read from SCFRDR"]
    #[inline]
    pub fn fer(&self) -> FERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        FERR { bits }
    }
    #[doc = "Bit 2 - Parity Error Indication - Indicates a parity error in the data read from the receive FIFO data register (SCFRDR) in asynchronous mode. 0: No receive parity error occurred in the next data read from SCFRDR \\[Clearing conditions\\] - PER is cleared to 0 by a power-on reset - PER is cleared to 0 when no parity error is present in the next data read from SCFRDR 1: A receive parity error occurred in the next data read from SCFRDR \\[Setting condition\\] - PER is set to 1 when a parity error is present in the next data read from SCFRDR"]
    #[inline]
    pub fn per(&self) -> PERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        PERR { bits }
    }
    #[doc = "Bit 1 - Receive FIFO Data Full - Indicates that receive data has been transferred to the receive FIFO data register (SCFRDR), and the quantity of data in SCFRDR has become more than the receive trigger number specified by the RTRG\\[1:0\\] bits in the FIFO control register (SCFCR). 0: The quantity of transmit data written to SCFRDR is less than the specified receive trigger number \\[Clearing conditions\\] - RDF is cleared to 0 by a power-on reset - RDF is cleared to 0 when the SCFRDR is read until the quantity of receive data in SCFRDR becomes less than the specified receive trigger number after 1 is read from RDF, and then 0 is written - RDF is cleared to 0 when the direct memory access controller is activated by receive FIFO data full interrupt (RXI) and SCFRDR is read until the quantity of receive data in it becomes less than the specified receive trigger number 1: The quantity of receive data in SCFRDR is more than the specified receive trigger number \\[Setting condition\\] - RDF is set to 1 when a quantity of receive data more than the specified receive trigger number is stored in SCFRDR (*1) (*1) As SCFRDR is a 16-byte FIFO register, the maximum quantity of data that can be read when RDF is 1 is the specified receive trigger number. If an attempt is made to read after all the data in SCFRDR has been read, the data is undefined. The quantity of receive data in SCFRDR is indicated by the lower 8 bits of SCFDR."]
    #[inline]
    pub fn rdf(&self) -> RDFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RDFR { bits }
    }
    #[doc = "Bit 0 - Receive Data Ready -Indicates that the quantity of data in the receive FIFO data register (SCFRDR) is less than the specified receive trigger number, and that the next data has not yet been received after the elapse of 15 ETU from the last stop bit in asynchronous mode. In clock synchronous mode, this bit is not set to 1. 0: Receiving is in progress, or no receive data remains in SCFRDR after receiving ended normally \\[Clearing conditions\\] - DR is cleared to 0 by a power-on reset - DR is cleared to 0 when all receive data are read from SCFRDR after 1 is read from DR, and then 0 is written. - DR is cleared to 0 when all receive data are read from SCFRDR after the direct memory access controller is activated by receive FIFO data full interrupt (RXI). 1: Next receive data has not been received \\[Setting condition\\] - DR is set to 1 when SCFRDR contains less data than the specified receive trigger number, and the next data has not yet been received after the elapse of 15 ETU from the last stop bit. (*1) (*1) This is equivalent to 1.5 frames with the 8-bit, 1-stop-bit format. (ETU: elementary time unit)"]
    #[inline]
    pub fn dr(&self) -> DRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DRR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:15 - Number of Parity Errors - Indicate the quantity of data including a parity error in the receive data stored in the receive FIFO data register (SCFRDR). The value indicated by bits 15 to 12 after the ER bit in SCFSR is set, represents the number of parity errors in SCFRDR. When parity errors have occurred in all 16-byte receive data in SCFRDR, PER\\[3:0\\] shows 0000."]
    #[inline]
    pub fn pernum(&mut self) -> _PERNUMW {
        _PERNUMW { w: self }
    }
    #[doc = "Bits 8:11 - Number of Framing Errors - Indicate the quantity of data including a framing error in the receive data stored in SCFRDR. The value indicated by bits 11 to 8 after the ER bit in SCFSR is set, represents the number of framing errors in SCFRDR. When framing errors have occurred in all 16-byte receive data in SCFRDR, FER\\[3:0\\] shows 0000."]
    #[inline]
    pub fn fernum(&mut self) -> _FERNUMW {
        _FERNUMW { w: self }
    }
    #[doc = "Bit 7 - Receive Error - currence of a framing error, or of a parity error when receiving data that includes parity.(*1) 0: Receiving is in progress or has ended normally \\[Clearing conditions\\] - ER is cleared to 0 by a power-on reset - ER is cleared to 0 when 0 is written to after 1 is read from ER 1: A framing error or parity error has occurred. \\[Setting conditions\\] - ER is set to 1 when the stop bit is 0 after checking whether or not the last stop bit of the received data is 1 at the end of one data receive operation (*2) - ER is set to 1 when the total number of 1s in the receive data plus parity bit does not match the even/odd parity specified by the O/E bit in SCSMR (*1) Clearing the RE bit to 0 in SCSCR does not affect the ER bit, which retains its previous value. Even if a receive error occurs, the receive data is transferred to SCFRDR and the receive operation is continued. Whether or not the data read from SCFRDR includes a receive error can be detected by the FER and PER bits in SCFSR. (*2) In two stop bits mode, only the first stop bit is checked; the second stop bit is not checked."]
    #[inline]
    pub fn er(&mut self) -> _ERW {
        _ERW { w: self }
    }
    #[doc = "Bit 6 - Transmit End - Indicates that when the last bit of a serial character was transmitted, SCFTDR did not contain valid data, so transmission has ended. 0: Transmission is in progress \\[Clearing condition\\] - TEND is cleared to 0 when 0 is written after 1 is read from TEND after transmit data is written in SCFTDR (*1) 1: End of transmission \\[Setting conditions\\] - TEND is set to 1 by a power-on reset - TEND is set to 1 when TE is cleared to 0 in the serial control register (SCSCR) - TEND is set to 1 when SCFTDR does not contain transmit data when the last bit of a one-byte serial character is transmitted (*1) Do not use this bit as a transmit end flag when the direct memory access controller writes data to SCFTDR due to a TXI interrupt request."]
    #[inline]
    pub fn tend(&mut self) -> _TENDW {
        _TENDW { w: self }
    }
    #[doc = "Bit 5 - Transmit FIFO Data Empty - Indicates that data has been transferred from the transmit FIFO data register (SCFTDR) to the transmit shift register (SCTSR), the quantity of data in SCFTDR has become less than the transmission trigger number specified by the TTRG\\[1:0\\] bits in the FIFO control register (SCFCR), and writing of transmit data to SCFTDR is enabled. 0: The quantity of transmit data written to SCFTDR is greater than the specified transmission trigger number \\[Clearing conditions\\] - TDFE is cleared to 0 when data exceeding the specified transmission trigger number is written to SCFTDR after 1 is read from TDFE and then 0 is written to TDFE - TDFE is cleared to 0 when direct memory access controller is activated by transmit FIFO data empty interrupt (TXI) and data exceeding the specified transmission trigger number is written to SCFTDR 1: The quantity of transmit data in SCFTDR is less than or equal to the specified transmission trigger number (*1) \\[Setting conditions\\] - TDFE is set to 1 by a power-on reset - TDFE is set to 1 when the quantity of transmit data in SCFTDR becomes less than or equal to the specified transmission trigger number as a result of transmission (*1) Since SCFTDR is a 16-byte FIFO register, the maximum quantity of data that can be written when TDFE is 1 is \"16 minus the specified transmission trigger number\". If an attempt is made to write additional data, the data is ignored. The quantity of data in SCFTDR is indicated by the upper 8 bits of SCFDR."]
    #[inline]
    pub fn tdfe(&mut self) -> _TDFEW {
        _TDFEW { w: self }
    }
    #[doc = "Bit 4 - Break Detection - Indicates that a break signal has been detected in receive data. 0: No break signal received \\[Clearing conditions\\] - BRK is cleared to 0 by a power-on reset - BRK is cleared to 0 when software reads BRK after it has been set to 1, then writes 0 to BRK 1: Break signal received (*1) \\[Setting condition\\] - BRK is set to 1 when data including a framing error is received, followed by at least one frame at the space 0 level (low level) (*1) When a break is detected, transfer of the receive data (H'00) to SCFRDR stops after detection. When the break ends and the receive signal becomes mark 1, the transfer of receive data resumes."]
    #[inline]
    pub fn brk(&mut self) -> _BRKW {
        _BRKW { w: self }
    }
    #[doc = "Bit 1 - Receive FIFO Data Full - Indicates that receive data has been transferred to the receive FIFO data register (SCFRDR), and the quantity of data in SCFRDR has become more than the receive trigger number specified by the RTRG\\[1:0\\] bits in the FIFO control register (SCFCR). 0: The quantity of transmit data written to SCFRDR is less than the specified receive trigger number \\[Clearing conditions\\] - RDF is cleared to 0 by a power-on reset - RDF is cleared to 0 when the SCFRDR is read until the quantity of receive data in SCFRDR becomes less than the specified receive trigger number after 1 is read from RDF, and then 0 is written - RDF is cleared to 0 when the direct memory access controller is activated by receive FIFO data full interrupt (RXI) and SCFRDR is read until the quantity of receive data in it becomes less than the specified receive trigger number 1: The quantity of receive data in SCFRDR is more than the specified receive trigger number \\[Setting condition\\] - RDF is set to 1 when a quantity of receive data more than the specified receive trigger number is stored in SCFRDR (*1) (*1) As SCFRDR is a 16-byte FIFO register, the maximum quantity of data that can be read when RDF is 1 is the specified receive trigger number. If an attempt is made to read after all the data in SCFRDR has been read, the data is undefined. The quantity of receive data in SCFRDR is indicated by the lower 8 bits of SCFDR."]
    #[inline]
    pub fn rdf(&mut self) -> _RDFW {
        _RDFW { w: self }
    }
    #[doc = "Bit 0 - Receive Data Ready -Indicates that the quantity of data in the receive FIFO data register (SCFRDR) is less than the specified receive trigger number, and that the next data has not yet been received after the elapse of 15 ETU from the last stop bit in asynchronous mode. In clock synchronous mode, this bit is not set to 1. 0: Receiving is in progress, or no receive data remains in SCFRDR after receiving ended normally \\[Clearing conditions\\] - DR is cleared to 0 by a power-on reset - DR is cleared to 0 when all receive data are read from SCFRDR after 1 is read from DR, and then 0 is written. - DR is cleared to 0 when all receive data are read from SCFRDR after the direct memory access controller is activated by receive FIFO data full interrupt (RXI). 1: Next receive data has not been received \\[Setting condition\\] - DR is set to 1 when SCFRDR contains less data than the specified receive trigger number, and the next data has not yet been received after the elapse of 15 ETU from the last stop bit. (*1) (*1) This is equivalent to 1.5 frames with the 8-bit, 1-stop-bit format. (ETU: elementary time unit)"]
    #[inline]
    pub fn dr(&mut self) -> _DRW {
        _DRW { w: self }
    }
}
