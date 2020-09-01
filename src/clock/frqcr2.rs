#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FRQCR2 {
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
#[doc = "Possible values of the field `GFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFCR {
    #[doc = "2/3 time"]
    _2_OVER_3,
    #[doc = "1/3 time"]
    _1_OVER_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GFCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GFCR::_2_OVER_3 => 0x01,
            GFCR::_1_OVER_3 => 0x03,
            GFCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GFCR {
        match value {
            1 => GFCR::_2_OVER_3,
            3 => GFCR::_1_OVER_3,
            i => GFCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2_OVER_3`"]
    #[inline]
    pub fn is_2_over_3(&self) -> bool {
        *self == GFCR::_2_OVER_3
    }
    #[doc = "Checks if the value of the field is `_1_OVER_3`"]
    #[inline]
    pub fn is_1_over_3(&self) -> bool {
        *self == GFCR::_1_OVER_3
    }
}
#[doc = "Values that can be written to the field `GFC`"]
pub enum GFCW {
    #[doc = "2/3 time"]
    _2_OVER_3,
    #[doc = "1/3 time"]
    _1_OVER_3,
}
impl GFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GFCW::_2_OVER_3 => 1,
            GFCW::_1_OVER_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GFCW<'a> {
    w: &'a mut W,
}
impl<'a> _GFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GFCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "2/3 time"]
    #[inline]
    pub fn _2_over_3(self) -> &'a mut W {
        self.variant(GFCW::_2_OVER_3)
    }
    #[doc = "1/3 time"]
    #[inline]
    pub fn _1_over_3(self) -> &'a mut W {
        self.variant(GFCW::_1_OVER_3)
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
    #[doc = "Bits 0:1 - Image Processing Clock Frequency Division Ratio - These bits specify the frequency division ratio of the image processing clock with respect to the output frequency of PLL circuit. Note: See section 6.5.1."]
    #[inline]
    pub fn gfc(&self) -> GFCR {
        GFCR::_from({
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
        W { bits: 0x03 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Image Processing Clock Frequency Division Ratio - These bits specify the frequency division ratio of the image processing clock with respect to the output frequency of PLL circuit. Note: See section 6.5.1."]
    #[inline]
    pub fn gfc(&mut self) -> _GFCW {
        _GFCW { w: self }
    }
}
