#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::XTALCTR {
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
#[doc = "Possible values of the field `GAIN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN1R {
    #[doc = "Large gain"]
    LARGE,
    #[doc = "Small gain"]
    SMALL,
}
impl GAIN1R {
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
            GAIN1R::LARGE => false,
            GAIN1R::SMALL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GAIN1R {
        match value {
            false => GAIN1R::LARGE,
            true => GAIN1R::SMALL,
        }
    }
    #[doc = "Checks if the value of the field is `LARGE`"]
    #[inline]
    pub fn is_large(&self) -> bool {
        *self == GAIN1R::LARGE
    }
    #[doc = "Checks if the value of the field is `SMALL`"]
    #[inline]
    pub fn is_small(&self) -> bool {
        *self == GAIN1R::SMALL
    }
}
#[doc = "Possible values of the field `GAIN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN0R {
    #[doc = "Large gain"]
    LARGE,
    #[doc = "Small gain"]
    SMALL,
}
impl GAIN0R {
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
            GAIN0R::LARGE => false,
            GAIN0R::SMALL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GAIN0R {
        match value {
            false => GAIN0R::LARGE,
            true => GAIN0R::SMALL,
        }
    }
    #[doc = "Checks if the value of the field is `LARGE`"]
    #[inline]
    pub fn is_large(&self) -> bool {
        *self == GAIN0R::LARGE
    }
    #[doc = "Checks if the value of the field is `SMALL`"]
    #[inline]
    pub fn is_small(&self) -> bool {
        *self == GAIN0R::SMALL
    }
}
#[doc = "Values that can be written to the field `GAIN1`"]
pub enum GAIN1W {
    #[doc = "Large gain"]
    LARGE,
    #[doc = "Small gain"]
    SMALL,
}
impl GAIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GAIN1W::LARGE => false,
            GAIN1W::SMALL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAIN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Large gain"]
    #[inline]
    pub fn large(self) -> &'a mut W {
        self.variant(GAIN1W::LARGE)
    }
    #[doc = "Small gain"]
    #[inline]
    pub fn small(self) -> &'a mut W {
        self.variant(GAIN1W::SMALL)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GAIN0`"]
pub enum GAIN0W {
    #[doc = "Large gain"]
    LARGE,
    #[doc = "Small gain"]
    SMALL,
}
impl GAIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GAIN0W::LARGE => false,
            GAIN0W::SMALL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAIN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Large gain"]
    #[inline]
    pub fn large(self) -> &'a mut W {
        self.variant(GAIN0W::LARGE)
    }
    #[doc = "Small gain"]
    #[inline]
    pub fn small(self) -> &'a mut W {
        self.variant(GAIN0W::SMALL)
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
    #[doc = "Bit 1 - Realtime Clock Crystal Oscillator (RTC_X3 or RTC_X4 Pin) Gain Select"]
    #[inline]
    pub fn gain1(&self) -> GAIN1R {
        GAIN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 0 - XTAL Crystal Oscillator (EXTAL or XTAL Pin) Gain Select"]
    #[inline]
    pub fn gain0(&self) -> GAIN0R {
        GAIN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Realtime Clock Crystal Oscillator (RTC_X3 or RTC_X4 Pin) Gain Select"]
    #[inline]
    pub fn gain1(&mut self) -> _GAIN1W {
        _GAIN1W { w: self }
    }
    #[doc = "Bit 0 - XTAL Crystal Oscillator (EXTAL or XTAL Pin) Gain Select"]
    #[inline]
    pub fn gain0(&mut self) -> _GAIN0W {
        _GAIN0W { w: self }
    }
}
