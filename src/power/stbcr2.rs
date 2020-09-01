#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::STBCR2 {
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
#[doc = "Possible values of the field `HIZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIZR {
    #[doc = "The pin state is retained in software standby mode or deep standby mode."]
    RETAIN,
    #[doc = "The pin is set to high-impedance in software standby mode or deep standby mode."]
    HIZ,
}
impl HIZR {
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
            HIZR::RETAIN => false,
            HIZR::HIZ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIZR {
        match value {
            false => HIZR::RETAIN,
            true => HIZR::HIZ,
        }
    }
    #[doc = "Checks if the value of the field is `RETAIN`"]
    #[inline]
    pub fn is_retain(&self) -> bool {
        *self == HIZR::RETAIN
    }
    #[doc = "Checks if the value of the field is `HIZ`"]
    #[inline]
    pub fn is_hiz(&self) -> bool {
        *self == HIZR::HIZ
    }
}
#[doc = r" Value of the field"]
pub struct MSTP20R {
    bits: bool,
}
impl MSTP20R {
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
#[doc = "Values that can be written to the field `HIZ`"]
pub enum HIZW {
    #[doc = "The pin state is retained in software standby mode or deep standby mode."]
    RETAIN,
    #[doc = "The pin is set to high-impedance in software standby mode or deep standby mode."]
    HIZ,
}
impl HIZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIZW::RETAIN => false,
            HIZW::HIZ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIZW<'a> {
    w: &'a mut W,
}
impl<'a> _HIZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The pin state is retained in software standby mode or deep standby mode."]
    #[inline]
    pub fn retain(self) -> &'a mut W {
        self.variant(HIZW::RETAIN)
    }
    #[doc = "The pin is set to high-impedance in software standby mode or deep standby mode."]
    #[inline]
    pub fn hiz(self) -> &'a mut W {
        self.variant(HIZW::HIZ)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSTP20W<'a> {
    w: &'a mut W,
}
impl<'a> _MSTP20W<'a> {
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
    #[doc = "Bit 7 - Port High Impedance - Selects whether the state of specific output pin is retained or high impedance in software standby mode or deep standby mode. As to which pins are controlled, see section 60.1, Pin States in section 60, States and Handling of Pins."]
    #[inline]
    pub fn hiz(&self) -> HIZR {
        HIZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 0 - Module Stop 20 \\[not for disclosure\\] - When the MSTP20 bit is set to 1, the clock supply to CoreSight is halted."]
    #[inline]
    pub fn mstp20(&self) -> MSTP20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MSTP20R { bits }
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
    #[doc = "Bit 7 - Port High Impedance - Selects whether the state of specific output pin is retained or high impedance in software standby mode or deep standby mode. As to which pins are controlled, see section 60.1, Pin States in section 60, States and Handling of Pins."]
    #[inline]
    pub fn hiz(&mut self) -> _HIZW {
        _HIZW { w: self }
    }
    #[doc = "Bit 0 - Module Stop 20 \\[not for disclosure\\] - When the MSTP20 bit is set to 1, the clock supply to CoreSight is halted."]
    #[inline]
    pub fn mstp20(&mut self) -> _MSTP20W {
        _MSTP20W { w: self }
    }
}
