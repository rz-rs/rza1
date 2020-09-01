#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DSCTR {
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
pub struct EBUSKEEPER {
    bits: bool,
}
impl EBUSKEEPER {
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
#[doc = "Possible values of the field `RAMBOOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMBOOTR {
    #[doc = "Activated according to the boot mode specified for a reset."]
    RESET,
    #[doc = "The program is read from the on-chip data-retention RAM. Instruction fetch from H'20000000"]
    RRAM,
}
impl RAMBOOTR {
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
            RAMBOOTR::RESET => false,
            RAMBOOTR::RRAM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAMBOOTR {
        match value {
            false => RAMBOOTR::RESET,
            true => RAMBOOTR::RRAM,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == RAMBOOTR::RESET
    }
    #[doc = "Checks if the value of the field is `RRAM`"]
    #[inline]
    pub fn is_rram(&self) -> bool {
        *self == RAMBOOTR::RRAM
    }
}
#[doc = r" Proxy"]
pub struct _EBUSKEEPEW<'a> {
    w: &'a mut W,
}
impl<'a> _EBUSKEEPEW<'a> {
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
#[doc = "Values that can be written to the field `RAMBOOT`"]
pub enum RAMBOOTW {
    #[doc = "Activated according to the boot mode specified for a reset."]
    RESET,
    #[doc = "The program is read from the on-chip data-retention RAM. Instruction fetch from H'20000000"]
    RRAM,
}
impl RAMBOOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAMBOOTW::RESET => false,
            RAMBOOTW::RRAM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMBOOTW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMBOOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMBOOTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Activated according to the boot mode specified for a reset."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(RAMBOOTW::RESET)
    }
    #[doc = "The program is read from the on-chip data-retention RAM. Instruction fetch from H'20000000"]
    #[inline]
    pub fn rram(self) -> &'a mut W {
        self.variant(RAMBOOTW::RRAM)
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
    #[doc = "Bit 7 - Retention of External Memory Control Pin State"]
    #[inline]
    pub fn ebuskeepe(&self) -> EBUSKEEPER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        EBUSKEEPER { bits }
    }
    #[doc = "Bit 6 - Selection of Method after Returning from Deep Standby Mode Selects an activation method after returning from deep standby mode."]
    #[inline]
    pub fn ramboot(&self) -> RAMBOOTR {
        RAMBOOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 7 - Retention of External Memory Control Pin State"]
    #[inline]
    pub fn ebuskeepe(&mut self) -> _EBUSKEEPEW {
        _EBUSKEEPEW { w: self }
    }
    #[doc = "Bit 6 - Selection of Method after Returning from Deep Standby Mode Selects an activation method after returning from deep standby mode."]
    #[inline]
    pub fn ramboot(&mut self) -> _RAMBOOTW {
        _RAMBOOTW { w: self }
    }
}
