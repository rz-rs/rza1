#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::STBCR3 {
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
pub struct MSTP37R {
    bits: bool,
}
impl MSTP37R {
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
pub struct MSTP36R {
    bits: bool,
}
impl MSTP36R {
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
pub struct MSTP35R {
    bits: bool,
}
impl MSTP35R {
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
pub struct MSTP34R {
    bits: bool,
}
impl MSTP34R {
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
pub struct MSTP33R {
    bits: bool,
}
impl MSTP33R {
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
pub struct MSTP32R {
    bits: bool,
}
impl MSTP32R {
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
pub struct MSTP31R {
    bits: bool,
}
impl MSTP31R {
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
pub struct MSTP30R {
    bits: bool,
}
impl MSTP30R {
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
pub struct _MSTP37W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP37W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSTP36W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP36W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSTP35W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP35W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSTP34W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP34W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSTP33W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP33W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSTP32W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP32W<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSTP31W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP31W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSTP30W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP30W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 7 - Module Stop 37 - When the MSTP37 bit is set to 1, the clock supply to the IEBusTM controller is halted."]
    #[inline]
    pub fn mstp37(&self) -> MSTP37R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP37R { bits }
    }
    #[doc = "Bit 6 - Module Stop 36 - When the MSTP36 bit is set to 1, the clock supply to serial communication interface (IrDA) is halted."]
    #[inline]
    pub fn mstp36(&self) -> MSTP36R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP36R { bits }
    }
    #[doc = "Bit 5 - Module Stop 35 - When the MSTP35 bit is set to 1, the clock supply to the LIN interface channel 0 is halted."]
    #[inline]
    pub fn mstp35(&self) -> MSTP35R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP35R { bits }
    }
    #[doc = "Bit 4 - Module Stop 34 - When the MSTP34 bit is set to 1, the clock supply to the LIN interface channel 1 is halted."]
    #[inline]
    pub fn mstp34(&self) -> MSTP34R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP34R { bits }
    }
    #[doc = "Bit 3 - Module Stop 33 - When the MSTP33 bit is set to 1, the clock supply to the multi-function timer pulse unit 2 is halted."]
    #[inline]
    pub fn mstp33(&self) -> MSTP33R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP33R { bits }
    }
    #[doc = "Bit 2 - Module Stop 32 - When the MSTP32 bit is set to 1, the clock supply to the CAN interface is halted."]
    #[inline]
    pub fn mstp32(&self) -> MSTP32R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP32R { bits }
    }
    #[doc = "Bit 1 - Module Stop 31 - When the MSTP31 bit is set to 1, the clock supply to the A/D converter is halted and the analog power supply to the A/D converter is shut off."]
    #[inline]
    pub fn mstp31(&self) -> MSTP31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP31R { bits }
    }
    #[doc = "Bit 0 - Module Stop 30 - When the MSTP30 bit is set to 1, the clock supply to the motor control PWM timer is halted."]
    #[inline]
    pub fn mstp30(&self) -> MSTP30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP30R { bits }
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - Module Stop 37 - When the MSTP37 bit is set to 1, the clock supply to the IEBusTM controller is halted."]
    #[inline]
    pub fn mstp37(&mut self) -> _MSTP37W {
        _MSTP37W { w: self }
    }
    #[doc = "Bit 6 - Module Stop 36 - When the MSTP36 bit is set to 1, the clock supply to serial communication interface (IrDA) is halted."]
    #[inline]
    pub fn mstp36(&mut self) -> _MSTP36W {
        _MSTP36W { w: self }
    }
    #[doc = "Bit 5 - Module Stop 35 - When the MSTP35 bit is set to 1, the clock supply to the LIN interface channel 0 is halted."]
    #[inline]
    pub fn mstp35(&mut self) -> _MSTP35W {
        _MSTP35W { w: self }
    }
    #[doc = "Bit 4 - Module Stop 34 - When the MSTP34 bit is set to 1, the clock supply to the LIN interface channel 1 is halted."]
    #[inline]
    pub fn mstp34(&mut self) -> _MSTP34W {
        _MSTP34W { w: self }
    }
    #[doc = "Bit 3 - Module Stop 33 - When the MSTP33 bit is set to 1, the clock supply to the multi-function timer pulse unit 2 is halted."]
    #[inline]
    pub fn mstp33(&mut self) -> _MSTP33W {
        _MSTP33W { w: self }
    }
    #[doc = "Bit 2 - Module Stop 32 - When the MSTP32 bit is set to 1, the clock supply to the CAN interface is halted."]
    #[inline]
    pub fn mstp32(&mut self) -> _MSTP32W {
        _MSTP32W { w: self }
    }
    #[doc = "Bit 1 - Module Stop 31 - When the MSTP31 bit is set to 1, the clock supply to the A/D converter is halted and the analog power supply to the A/D converter is shut off."]
    #[inline]
    pub fn mstp31(&mut self) -> _MSTP31W {
        _MSTP31W { w: self }
    }
    #[doc = "Bit 0 - Module Stop 30 - When the MSTP30 bit is set to 1, the clock supply to the motor control PWM timer is halted."]
    #[inline]
    pub fn mstp30(&mut self) -> _MSTP30W {
        _MSTP30W { w: self }
    }
}
