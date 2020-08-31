#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SCR {
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
pub struct TIER {
    bits: bool,
}
impl TIER {
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
pub struct RIER {
    bits: bool,
}
impl RIER {
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
pub struct TER {
    bits: bool,
}
impl TER {
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
pub struct RER {
    bits: bool,
}
impl RER {
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
pub struct REIER {
    bits: bool,
}
impl REIER {
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
#[doc = "Possible values of the field `CKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKER {
    #[doc = "Asynchronous mode: Internal clock, SCK pin used for input (input signal is ignored). Clock synchronous mode: Internal clock, SCK pin used for synchronous clock output."]
    INTERNAL_SCK_IN,
    #[doc = "Internal clock, SCK pin used for synchronous clock output"]
    INTERNAL_SCK_OUT,
    #[doc = "External clock, SCK pin used for synchronous clock input"]
    EXTERNAL_SCK_IN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CKER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKER::INTERNAL_SCK_IN => 0,
            CKER::INTERNAL_SCK_OUT => 0x01,
            CKER::EXTERNAL_SCK_IN => 0x02,
            CKER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKER {
        match value {
            0 => CKER::INTERNAL_SCK_IN,
            1 => CKER::INTERNAL_SCK_OUT,
            2 => CKER::EXTERNAL_SCK_IN,
            i => CKER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL_SCK_IN`"]
    #[inline]
    pub fn is_internal_sck_in(&self) -> bool {
        *self == CKER::INTERNAL_SCK_IN
    }
    #[doc = "Checks if the value of the field is `INTERNAL_SCK_OUT`"]
    #[inline]
    pub fn is_internal_sck_out(&self) -> bool {
        *self == CKER::INTERNAL_SCK_OUT
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_SCK_IN`"]
    #[inline]
    pub fn is_external_sck_in(&self) -> bool {
        *self == CKER::EXTERNAL_SCK_IN
    }
}
#[doc = r" Proxy"]
pub struct _TIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIEW<'a> {
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
pub struct _RIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RIEW<'a> {
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
pub struct _TEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEW<'a> {
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
pub struct _REW<'a> {
    w: &'a mut W,
}
impl<'a> _REW<'a> {
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
pub struct _REIEW<'a> {
    w: &'a mut W,
}
impl<'a> _REIEW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CKE`"]
pub enum CKEW {
    #[doc = "Asynchronous mode: Internal clock, SCK pin used for input (input signal is ignored). Clock synchronous mode: Internal clock, SCK pin used for synchronous clock output."]
    INTERNAL_SCK_IN,
    #[doc = "Internal clock, SCK pin used for synchronous clock output"]
    INTERNAL_SCK_OUT,
    #[doc = "External clock, SCK pin used for synchronous clock input"]
    EXTERNAL_SCK_IN,
}
impl CKEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKEW::INTERNAL_SCK_IN => 0,
            CKEW::INTERNAL_SCK_OUT => 1,
            CKEW::EXTERNAL_SCK_IN => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKEW<'a> {
    w: &'a mut W,
}
impl<'a> _CKEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Asynchronous mode: Internal clock, SCK pin used for input (input signal is ignored). Clock synchronous mode: Internal clock, SCK pin used for synchronous clock output."]
    #[inline]
    pub fn internal_sck_in(self) -> &'a mut W {
        self.variant(CKEW::INTERNAL_SCK_IN)
    }
    #[doc = "Internal clock, SCK pin used for synchronous clock output"]
    #[inline]
    pub fn internal_sck_out(self) -> &'a mut W {
        self.variant(CKEW::INTERNAL_SCK_OUT)
    }
    #[doc = "External clock, SCK pin used for synchronous clock input"]
    #[inline]
    pub fn external_sck_in(self) -> &'a mut W {
        self.variant(CKEW::EXTERNAL_SCK_IN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
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
    #[doc = "Bit 7 - Transmit Interrupt Enable - Enables or disables the transmit-FIFO-data-empty interrupt (TXI) requested when the serial transmit data is transferred from the transmit FIFO data register (SCFTDR) to the transmit shift register (SCTSR), the quantity of data in the transmit FIFO register becomes less than the specified number of transmission triggers, and then the TDFE flag in the serial status register (SCFSR) is set to 1."]
    #[inline]
    pub fn tie(&self) -> TIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TIER { bits }
    }
    #[doc = "Bit 6 - Receive Interrupt Enable - Enables or disables the receive FIFO data full (RXI) interrupts requested when the RDF flag or DR flag in serial status register (SCFSR) is set to 1, receive-error (ERI) interrupts requested when the ER flag in SCFSR is set to 1, and break (BRI) interrupts requested when the BRK flag in SCFSR or the ORER flag in line status register (SCLSR) is set to 1."]
    #[inline]
    pub fn rie(&self) -> RIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RIER { bits }
    }
    #[doc = "Bit 5 - Transmit Enable - Enables or disables serial transmission."]
    #[inline]
    pub fn te(&self) -> TER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TER { bits }
    }
    #[doc = "Bit 4 - Receive Enable - Enables or disables serial reception."]
    #[inline]
    pub fn re(&self) -> RER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RER { bits }
    }
    #[doc = "Bit 3 - Receive Error Interrupt Enable - Enables or disables the receive-error (ERI) interrupts and break (BRI) interrupts. The setting of REIE bit is valid only when RIE bit is set to 0."]
    #[inline]
    pub fn reie(&self) -> REIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        REIER { bits }
    }
    #[doc = "Bits 0:1 - Clock Enable - Select the clock source and enable or disable clock output from the SCK pin. Depending on CKE\\[1:0\\], the SCK pin can be used for serial clock output or serial clock input. If synchronous clock output is set in clock synchronous mode, set the C/A bit in SCSMR to 1, and then set CKE\\[1:0\\]."]
    #[inline]
    pub fn cke(&self) -> CKER {
        CKER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
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
    #[doc = "Bit 7 - Transmit Interrupt Enable - Enables or disables the transmit-FIFO-data-empty interrupt (TXI) requested when the serial transmit data is transferred from the transmit FIFO data register (SCFTDR) to the transmit shift register (SCTSR), the quantity of data in the transmit FIFO register becomes less than the specified number of transmission triggers, and then the TDFE flag in the serial status register (SCFSR) is set to 1."]
    #[inline]
    pub fn tie(&mut self) -> _TIEW {
        _TIEW { w: self }
    }
    #[doc = "Bit 6 - Receive Interrupt Enable - Enables or disables the receive FIFO data full (RXI) interrupts requested when the RDF flag or DR flag in serial status register (SCFSR) is set to 1, receive-error (ERI) interrupts requested when the ER flag in SCFSR is set to 1, and break (BRI) interrupts requested when the BRK flag in SCFSR or the ORER flag in line status register (SCLSR) is set to 1."]
    #[inline]
    pub fn rie(&mut self) -> _RIEW {
        _RIEW { w: self }
    }
    #[doc = "Bit 5 - Transmit Enable - Enables or disables serial transmission."]
    #[inline]
    pub fn te(&mut self) -> _TEW {
        _TEW { w: self }
    }
    #[doc = "Bit 4 - Receive Enable - Enables or disables serial reception."]
    #[inline]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
    #[doc = "Bit 3 - Receive Error Interrupt Enable - Enables or disables the receive-error (ERI) interrupts and break (BRI) interrupts. The setting of REIE bit is valid only when RIE bit is set to 0."]
    #[inline]
    pub fn reie(&mut self) -> _REIEW {
        _REIEW { w: self }
    }
    #[doc = "Bits 0:1 - Clock Enable - Select the clock source and enable or disable clock output from the SCK pin. Depending on CKE\\[1:0\\], the SCK pin can be used for serial clock output or serial clock input. If synchronous clock output is set in clock synchronous mode, set the C/A bit in SCSMR to 1, and then set CKE\\[1:0\\]."]
    #[inline]
    pub fn cke(&mut self) -> _CKEW {
        _CKEW { w: self }
    }
}
