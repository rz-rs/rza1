#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMSR {
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
    OUTPUT,
    #[doc = "undocumented"]
    INPUT,
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
            _0R::OUTPUT => false,
            _0R::INPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> _0R {
        match value {
            false => _0R::OUTPUT,
            true => _0R::INPUT,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == _0R::OUTPUT
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == _0R::INPUT
    }
}
#[doc = "Possible values of the field `1`"]
pub type _1R = _0R;
#[doc = "Possible values of the field `2`"]
pub type _2R = _0R;
#[doc = "Possible values of the field `3`"]
pub type _3R = _0R;
#[doc = "Possible values of the field `4`"]
pub type _4R = _0R;
#[doc = "Possible values of the field `5`"]
pub type _5R = _0R;
#[doc = "Possible values of the field `6`"]
pub type _6R = _0R;
#[doc = "Possible values of the field `7`"]
pub type _7R = _0R;
#[doc = "Possible values of the field `8`"]
pub type _8R = _0R;
#[doc = "Possible values of the field `9`"]
pub type _9R = _0R;
#[doc = "Possible values of the field `10`"]
pub type _10R = _0R;
#[doc = "Possible values of the field `11`"]
pub type _11R = _0R;
#[doc = "Possible values of the field `12`"]
pub type _12R = _0R;
#[doc = "Possible values of the field `13`"]
pub type _13R = _0R;
#[doc = "Possible values of the field `14`"]
pub type _14R = _0R;
#[doc = "Possible values of the field `15`"]
pub type _15R = _0R;
#[doc = r" Value of the field"]
pub struct ENABLE_0R {
    bits: bool,
}
impl ENABLE_0R {
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
pub struct ENABLE_1R {
    bits: bool,
}
impl ENABLE_1R {
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
pub struct ENABLE_2R {
    bits: bool,
}
impl ENABLE_2R {
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
pub struct ENABLE_3R {
    bits: bool,
}
impl ENABLE_3R {
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
pub struct ENABLE_4R {
    bits: bool,
}
impl ENABLE_4R {
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
pub struct ENABLE_5R {
    bits: bool,
}
impl ENABLE_5R {
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
pub struct ENABLE_6R {
    bits: bool,
}
impl ENABLE_6R {
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
pub struct ENABLE_7R {
    bits: bool,
}
impl ENABLE_7R {
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
pub struct ENABLE_8R {
    bits: bool,
}
impl ENABLE_8R {
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
pub struct ENABLE_9R {
    bits: bool,
}
impl ENABLE_9R {
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
pub struct ENABLE_10R {
    bits: bool,
}
impl ENABLE_10R {
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
pub struct ENABLE_11R {
    bits: bool,
}
impl ENABLE_11R {
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
pub struct ENABLE_12R {
    bits: bool,
}
impl ENABLE_12R {
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
pub struct ENABLE_13R {
    bits: bool,
}
impl ENABLE_13R {
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
pub struct ENABLE_14R {
    bits: bool,
}
impl ENABLE_14R {
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
pub struct ENABLE_15R {
    bits: bool,
}
impl ENABLE_15R {
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
#[doc = "Values that can be written to the field `0`"]
pub enum _0W {
    #[doc = "`0`"]
    OUTPUT,
    #[doc = "`1`"]
    INPUT,
}
impl _0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            _0W::OUTPUT => false,
            _0W::INPUT => true,
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
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
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
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `2`"]
pub type _2W = _0W;
#[doc = r" Proxy"]
pub struct __2W<'a> {
    w: &'a mut W,
}
impl<'a> __2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `3`"]
pub type _3W = _0W;
#[doc = r" Proxy"]
pub struct __3W<'a> {
    w: &'a mut W,
}
impl<'a> __3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `4`"]
pub type _4W = _0W;
#[doc = r" Proxy"]
pub struct __4W<'a> {
    w: &'a mut W,
}
impl<'a> __4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `5`"]
pub type _5W = _0W;
#[doc = r" Proxy"]
pub struct __5W<'a> {
    w: &'a mut W,
}
impl<'a> __5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `6`"]
pub type _6W = _0W;
#[doc = r" Proxy"]
pub struct __6W<'a> {
    w: &'a mut W,
}
impl<'a> __6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `7`"]
pub type _7W = _0W;
#[doc = r" Proxy"]
pub struct __7W<'a> {
    w: &'a mut W,
}
impl<'a> __7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `8`"]
pub type _8W = _0W;
#[doc = r" Proxy"]
pub struct __8W<'a> {
    w: &'a mut W,
}
impl<'a> __8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `9`"]
pub type _9W = _0W;
#[doc = r" Proxy"]
pub struct __9W<'a> {
    w: &'a mut W,
}
impl<'a> __9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `10`"]
pub type _10W = _0W;
#[doc = r" Proxy"]
pub struct __10W<'a> {
    w: &'a mut W,
}
impl<'a> __10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `11`"]
pub type _11W = _0W;
#[doc = r" Proxy"]
pub struct __11W<'a> {
    w: &'a mut W,
}
impl<'a> __11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `12`"]
pub type _12W = _0W;
#[doc = r" Proxy"]
pub struct __12W<'a> {
    w: &'a mut W,
}
impl<'a> __12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `13`"]
pub type _13W = _0W;
#[doc = r" Proxy"]
pub struct __13W<'a> {
    w: &'a mut W,
}
impl<'a> __13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `14`"]
pub type _14W = _0W;
#[doc = r" Proxy"]
pub struct __14W<'a> {
    w: &'a mut W,
}
impl<'a> __14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `15`"]
pub type _15W = _0W;
#[doc = r" Proxy"]
pub struct __15W<'a> {
    w: &'a mut W,
}
impl<'a> __15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: _15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(_0W::OUTPUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(_0W::INPUT)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_0W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_0W<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_1W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_1W<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_2W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_2W<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_3W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_3W<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_4W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_4W<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_5W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_5W<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_6W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_6W<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_7W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_7W<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_8W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_8W<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_9W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_9W<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_10W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_10W<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_11W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_11W<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_12W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_12W<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_13W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_13W<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_14W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_14W<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_15W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_15W<'a> {
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn _0(&self) -> _0R {
        _0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn _1(&self) -> _1R {
        _1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn _2(&self) -> _2R {
        _2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn _3(&self) -> _3R {
        _3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn _4(&self) -> _4R {
        _4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn _5(&self) -> _5R {
        _5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6"]
    #[inline]
    pub fn _6(&self) -> _6R {
        _6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn _7(&self) -> _7R {
        _7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn _8(&self) -> _8R {
        _8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn _9(&self) -> _9R {
        _9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10"]
    #[inline]
    pub fn _10(&self) -> _10R {
        _10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn _11(&self) -> _11R {
        _11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn _12(&self) -> _12R {
        _12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13"]
    #[inline]
    pub fn _13(&self) -> _13R {
        _13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14"]
    #[inline]
    pub fn _14(&self) -> _14R {
        _14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15"]
    #[inline]
    pub fn _15(&self) -> _15R {
        _15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn enable_0(&self) -> ENABLE_0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_0R { bits }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn enable_1(&self) -> ENABLE_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_1R { bits }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn enable_2(&self) -> ENABLE_2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_2R { bits }
    }
    #[doc = "Bit 19"]
    #[inline]
    pub fn enable_3(&self) -> ENABLE_3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_3R { bits }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn enable_4(&self) -> ENABLE_4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_4R { bits }
    }
    #[doc = "Bit 21"]
    #[inline]
    pub fn enable_5(&self) -> ENABLE_5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_5R { bits }
    }
    #[doc = "Bit 22"]
    #[inline]
    pub fn enable_6(&self) -> ENABLE_6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_6R { bits }
    }
    #[doc = "Bit 23"]
    #[inline]
    pub fn enable_7(&self) -> ENABLE_7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_7R { bits }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn enable_8(&self) -> ENABLE_8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_8R { bits }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn enable_9(&self) -> ENABLE_9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_9R { bits }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn enable_10(&self) -> ENABLE_10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_10R { bits }
    }
    #[doc = "Bit 27"]
    #[inline]
    pub fn enable_11(&self) -> ENABLE_11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_11R { bits }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn enable_12(&self) -> ENABLE_12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_12R { bits }
    }
    #[doc = "Bit 29"]
    #[inline]
    pub fn enable_13(&self) -> ENABLE_13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_13R { bits }
    }
    #[doc = "Bit 30"]
    #[inline]
    pub fn enable_14(&self) -> ENABLE_14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_14R { bits }
    }
    #[doc = "Bit 31"]
    #[inline]
    pub fn enable_15(&self) -> ENABLE_15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_15R { bits }
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
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
    #[doc = "Bit 2"]
    #[inline]
    pub fn _2(&mut self) -> __2W {
        __2W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn _3(&mut self) -> __3W {
        __3W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn _4(&mut self) -> __4W {
        __4W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn _5(&mut self) -> __5W {
        __5W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline]
    pub fn _6(&mut self) -> __6W {
        __6W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn _7(&mut self) -> __7W {
        __7W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn _8(&mut self) -> __8W {
        __8W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn _9(&mut self) -> __9W {
        __9W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline]
    pub fn _10(&mut self) -> __10W {
        __10W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn _11(&mut self) -> __11W {
        __11W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn _12(&mut self) -> __12W {
        __12W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline]
    pub fn _13(&mut self) -> __13W {
        __13W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline]
    pub fn _14(&mut self) -> __14W {
        __14W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline]
    pub fn _15(&mut self) -> __15W {
        __15W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn enable_0(&mut self) -> _ENABLE_0W {
        _ENABLE_0W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn enable_1(&mut self) -> _ENABLE_1W {
        _ENABLE_1W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn enable_2(&mut self) -> _ENABLE_2W {
        _ENABLE_2W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline]
    pub fn enable_3(&mut self) -> _ENABLE_3W {
        _ENABLE_3W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn enable_4(&mut self) -> _ENABLE_4W {
        _ENABLE_4W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline]
    pub fn enable_5(&mut self) -> _ENABLE_5W {
        _ENABLE_5W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline]
    pub fn enable_6(&mut self) -> _ENABLE_6W {
        _ENABLE_6W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline]
    pub fn enable_7(&mut self) -> _ENABLE_7W {
        _ENABLE_7W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn enable_8(&mut self) -> _ENABLE_8W {
        _ENABLE_8W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn enable_9(&mut self) -> _ENABLE_9W {
        _ENABLE_9W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn enable_10(&mut self) -> _ENABLE_10W {
        _ENABLE_10W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline]
    pub fn enable_11(&mut self) -> _ENABLE_11W {
        _ENABLE_11W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn enable_12(&mut self) -> _ENABLE_12W {
        _ENABLE_12W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline]
    pub fn enable_13(&mut self) -> _ENABLE_13W {
        _ENABLE_13W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline]
    pub fn enable_14(&mut self) -> _ENABLE_14W {
        _ENABLE_14W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline]
    pub fn enable_15(&mut self) -> _ENABLE_15W {
        _ENABLE_15W { w: self }
    }
}
