#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTL {
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
pub struct MD0R {
    bits: bool,
}
impl MD0R {
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
#[doc = "Possible values of the field `MD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MD1R {
    #[doc = "Interval timer mode"]
    INTERVALTIMER,
    #[doc = "Free-running comparison mode"]
    FREERUNNINGCOMPARISON,
}
impl MD1R {
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
            MD1R::INTERVALTIMER => false,
            MD1R::FREERUNNINGCOMPARISON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MD1R {
        match value {
            false => MD1R::INTERVALTIMER,
            true => MD1R::FREERUNNINGCOMPARISON,
        }
    }
    #[doc = "Checks if the value of the field is `INTERVALTIMER`"]
    #[inline]
    pub fn is_interval_timer(&self) -> bool {
        *self == MD1R::INTERVALTIMER
    }
    #[doc = "Checks if the value of the field is `FREERUNNINGCOMPARISON`"]
    #[inline]
    pub fn is_free_running_comparison(&self) -> bool {
        *self == MD1R::FREERUNNINGCOMPARISON
    }
}
#[doc = r" Proxy"]
pub struct _MD0W<'a> {
    w: &'a mut W,
}
impl<'a> _MD0W<'a> {
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
#[doc = "Values that can be written to the field `MD1`"]
pub enum MD1W {
    #[doc = "Interval timer mode"]
    INTERVALTIMER,
    #[doc = "Free-running comparison mode"]
    FREERUNNINGCOMPARISON,
}
impl MD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MD1W::INTERVALTIMER => false,
            MD1W::FREERUNNINGCOMPARISON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MD1W<'a> {
    w: &'a mut W,
}
impl<'a> _MD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MD1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interval timer mode"]
    #[inline]
    pub fn interval_timer(self) -> &'a mut W {
        self.variant(MD1W::INTERVALTIMER)
    }
    #[doc = "Free-running comparison mode"]
    #[inline]
    pub fn free_running_comparison(self) -> &'a mut W {
        self.variant(MD1W::FREERUNNINGCOMPARISON)
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
    #[doc = "Bit 0 - Controls enabling/disabling of OSTMnTINT interrupt requests when counting starts."]
    #[inline]
    pub fn md0(&self) -> MD0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        MD0R { bits }
    }
    #[doc = "Bit 1 - Specifies the operating mode for the counter."]
    #[inline]
    pub fn md1(&self) -> MD1R {
        MD1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Controls enabling/disabling of OSTMnTINT interrupt requests when counting starts."]
    #[inline]
    pub fn md0(&mut self) -> _MD0W {
        _MD0W { w: self }
    }
    #[doc = "Bit 1 - Specifies the operating mode for the counter."]
    #[inline]
    pub fn md1(&mut self) -> _MD1W {
        _MD1W { w: self }
    }
}
