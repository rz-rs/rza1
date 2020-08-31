#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SCEMR {
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
#[doc = "Possible values of the field `BGDM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGDMR {
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "Double-speed mode"]
    DOUBLE,
}
impl BGDMR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BGDMR::NORMAL => false,
            BGDMR::DOUBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGDMR {
        match value {
            false => BGDMR::NORMAL,
            true => BGDMR::DOUBLE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == BGDMR::NORMAL
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline]
    pub fn is_double(&self) -> bool {
        *self == BGDMR::DOUBLE
    }
}
#[doc = "Possible values of the field `ABCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABCSR {
    #[doc = "Base clock frequency is 16 times the bit rate"]
    _16,
    #[doc = "Base clock frequency is 8 times the bit rate"]
    _8,
}
impl ABCSR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ABCSR::_16 => false,
            ABCSR::_8 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABCSR {
        match value {
            false => ABCSR::_16,
            true => ABCSR::_8,
        }
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == ABCSR::_16
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == ABCSR::_8
    }
}
#[doc = "Values that can be written to the field `BGDM`"]
pub enum BGDMW {
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "Double-speed mode"]
    DOUBLE,
}
impl BGDMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGDMW::NORMAL => false,
            BGDMW::DOUBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGDMW<'a> {
    w: &'a mut W,
}
impl<'a> _BGDMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGDMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(BGDMW::NORMAL)
    }
    #[doc = "Double-speed mode"]
    #[inline]
    pub fn double(self) -> &'a mut W {
        self.variant(BGDMW::DOUBLE)
    }
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
#[doc = "Values that can be written to the field `ABCS`"]
pub enum ABCSW {
    #[doc = "Base clock frequency is 16 times the bit rate"]
    _16,
    #[doc = "Base clock frequency is 8 times the bit rate"]
    _8,
}
impl ABCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABCSW::_16 => false,
            ABCSW::_8 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABCSW<'a> {
    w: &'a mut W,
}
impl<'a> _ABCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Base clock frequency is 16 times the bit rate"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(ABCSW::_16)
    }
    #[doc = "Base clock frequency is 8 times the bit rate"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(ABCSW::_8)
    }
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
    #[doc = "Bit 7 - Baud Rate Generator Double-Speed Mode - When the BGDM bit is set to 1, the baud rate generator in this module operates in double-speed mode. This bit is valid only when asynchronous mode is selected by setting the C/A bit in SCSMR to 0 and an internal clock is selected as a clock source and the SCK pin is set as an input pin by setting the CKE\\[1:0\\] bits in SCSCR to 00. In other settings, use normal mode."]
    #[inline]
    pub fn bgdm(&self) -> BGDMR {
        BGDMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 0 - Base Clock Select in Asynchronous Mode - This bit selects the base clock frequency within a bit period in asynchronous mode. This bit is valid only in asynchronous mode (when the C/A bit in SCSMR is 0)."]
    #[inline]
    pub fn abcs(&self) -> ABCSR {
        ABCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    #[doc = "Bit 7 - Baud Rate Generator Double-Speed Mode - When the BGDM bit is set to 1, the baud rate generator in this module operates in double-speed mode. This bit is valid only when asynchronous mode is selected by setting the C/A bit in SCSMR to 0 and an internal clock is selected as a clock source and the SCK pin is set as an input pin by setting the CKE\\[1:0\\] bits in SCSCR to 00. In other settings, use normal mode."]
    #[inline]
    pub fn bgdm(&mut self) -> _BGDMW {
        _BGDMW { w: self }
    }
    #[doc = "Bit 0 - Base Clock Select in Asynchronous Mode - This bit selects the base clock frequency within a bit period in asynchronous mode. This bit is valid only in asynchronous mode (when the C/A bit in SCSMR is 0)."]
    #[inline]
    pub fn abcs(&mut self) -> _ABCSW {
        _ABCSW { w: self }
    }
}
