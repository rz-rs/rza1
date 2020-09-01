#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::STBCR4 {
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
pub struct MSTP47R {
    bits: bool,
}
impl MSTP47R {
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
pub struct MSTP46R {
    bits: bool,
}
impl MSTP46R {
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
pub struct MSTP45R {
    bits: bool,
}
impl MSTP45R {
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
pub struct MSTP44R {
    bits: bool,
}
impl MSTP44R {
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
pub struct MSTP43R {
    bits: bool,
}
impl MSTP43R {
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
pub struct MSTP42R {
    bits: bool,
}
impl MSTP42R {
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
pub struct MSTP41R {
    bits: bool,
}
impl MSTP41R {
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
pub struct MSTP40R {
    bits: bool,
}
impl MSTP40R {
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
pub struct _MSTP47W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP47W<'a> {
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
pub struct _MSTP46W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP46W<'a> {
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
pub struct _MSTP45W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP45W<'a> {
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
pub struct _MSTP44W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP44W<'a> {
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
pub struct _MSTP43W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP43W<'a> {
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
pub struct _MSTP42W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP42W<'a> {
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
pub struct _MSTP41W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP41W<'a> {
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
pub struct _MSTP40W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP40W<'a> {
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
    #[doc = "Bit 7 - Module Stop 47 - When the MSTP47 bit is set to 1, the clock supply to channel 0 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp47(&self) -> MSTP47R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP47R { bits }
    }
    #[doc = "Bit 6 - Module Stop 46 - When the MSTP46 bit is set to 1, the clock supply to channel 1 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp46(&self) -> MSTP46R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP46R { bits }
    }
    #[doc = "Bit 5 - Module Stop 45 - When the MSTP45 bit is set to 1, the clock supply to channel 2 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp45(&self) -> MSTP45R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP45R { bits }
    }
    #[doc = "Bit 4 - Module Stop 44 - When the MSTP44 bit is set to 1, the clock supply to channel 3 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp44(&self) -> MSTP44R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP44R { bits }
    }
    #[doc = "Bit 3 - Module Stop 43 - When the MSTP43 bit is set to 1, the clock supply to channel 4 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp43(&self) -> MSTP43R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP43R { bits }
    }
    #[doc = "Bit 2 - Module Stop 42 - When the MSTP42 bit is set to 1, the clock supply to channel 5 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp42(&self) -> MSTP42R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP42R { bits }
    }
    #[doc = "Bit 1 - Module Stop 41 - When the MSTP41 bit is set to 1, the clock supply to channel 6 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp41(&self) -> MSTP41R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP41R { bits }
    }
    #[doc = "Bit 0 - Module Stop 40 - When the MSTP40 bit is set to 1, the clock supply to channel 7 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp40(&self) -> MSTP40R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP40R { bits }
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
    #[doc = "Bit 7 - Module Stop 47 - When the MSTP47 bit is set to 1, the clock supply to channel 0 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp47(&mut self) -> _MSTP47W {
        _MSTP47W { w: self }
    }
    #[doc = "Bit 6 - Module Stop 46 - When the MSTP46 bit is set to 1, the clock supply to channel 1 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp46(&mut self) -> _MSTP46W {
        _MSTP46W { w: self }
    }
    #[doc = "Bit 5 - Module Stop 45 - When the MSTP45 bit is set to 1, the clock supply to channel 2 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp45(&mut self) -> _MSTP45W {
        _MSTP45W { w: self }
    }
    #[doc = "Bit 4 - Module Stop 44 - When the MSTP44 bit is set to 1, the clock supply to channel 3 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp44(&mut self) -> _MSTP44W {
        _MSTP44W { w: self }
    }
    #[doc = "Bit 3 - Module Stop 43 - When the MSTP43 bit is set to 1, the clock supply to channel 4 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp43(&mut self) -> _MSTP43W {
        _MSTP43W { w: self }
    }
    #[doc = "Bit 2 - Module Stop 42 - When the MSTP42 bit is set to 1, the clock supply to channel 5 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp42(&mut self) -> _MSTP42W {
        _MSTP42W { w: self }
    }
    #[doc = "Bit 1 - Module Stop 41 - When the MSTP41 bit is set to 1, the clock supply to channel 6 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp41(&mut self) -> _MSTP41W {
        _MSTP41W { w: self }
    }
    #[doc = "Bit 0 - Module Stop 40 - When the MSTP40 bit is set to 1, the clock supply to channel 7 of the serial communication interface with FIFO is halted."]
    #[inline]
    pub fn mstp40(&mut self) -> _MSTP40W {
        _MSTP40W { w: self }
    }
}
