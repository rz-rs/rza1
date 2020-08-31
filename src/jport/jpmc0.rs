#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::JPMC0 {
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
#[doc = "Possible values of the field `0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum _0R {
    #[doc = "undocumented"]
    PORT,
    #[doc = "undocumented"]
    ALT,
}
impl _0R {
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
            _0R::PORT => false,
            _0R::ALT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> _0R {
        match value {
            false => _0R::PORT,
            true => _0R::ALT,
        }
    }
    #[doc = "Checks if the value of the field is `PORT`"]
    #[inline]
    pub fn is_port(&self) -> bool {
        *self == _0R::PORT
    }
    #[doc = "Checks if the value of the field is `ALT`"]
    #[inline]
    pub fn is_alt(&self) -> bool {
        *self == _0R::ALT
    }
}
#[doc = "Possible values of the field `1`"]
pub type _1R = _0R;
#[doc = "Values that can be written to the field `0`"]
pub enum _0W {
    #[doc = "`0`"]
    PORT,
    #[doc = "`1`"]
    ALT,
}
impl _0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            _0W::PORT => false,
            _0W::ALT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct __0W<'a> {
    w: &'a mut W,
}
impl<'a> __0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn port(self) -> &'a mut W {
        self.variant(_0W::PORT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn alt(self) -> &'a mut W {
        self.variant(_0W::ALT)
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
#[doc = "Values that can be written to the field `1`"]
pub type _1W = _0W;
#[doc = r" Proxy"]
pub struct __1W<'a> {
    w: &'a mut W,
}
impl<'a> __1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn port(self) -> &'a mut W {
        self.variant(_0W::PORT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn alt(self) -> &'a mut W {
        self.variant(_0W::ALT)
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
    #[doc = "Bit 0"]
    #[inline]
    pub fn _0(&self) -> _0R {
        _0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn _1(&self) -> _1R {
        _1R::_from({
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
    #[doc = "Bit 0"]
    #[inline]
    pub fn _0(&mut self) -> __0W {
        __0W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn _1(&mut self) -> __1W {
        __1W { w: self }
    }
}
