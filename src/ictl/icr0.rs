#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::ICR0 {
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
#[doc = "Possible values of the field `NMIL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMILR {
    #[doc = "Low level is input to NMI pin"]
    LOW,
    #[doc = "High level is input to NMI pin"]
    HIGH,
}
impl NMILR {
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
            NMILR::LOW => false,
            NMILR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NMILR {
        match value {
            false => NMILR::LOW,
            true => NMILR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == NMILR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == NMILR::HIGH
    }
}
#[doc = "Possible values of the field `NMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIER {
    #[doc = "Interrupt request is detected on falling edge of NMI input"]
    FALLINGEDGE,
    #[doc = "Interrupt request is detected on rising edge of NMI input"]
    RISINGEDGE,
}
impl NMIER {
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
            NMIER::FALLINGEDGE => false,
            NMIER::RISINGEDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NMIER {
        match value {
            false => NMIER::FALLINGEDGE,
            true => NMIER::RISINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == NMIER::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == NMIER::RISINGEDGE
    }
}
#[doc = r" Value of the field"]
pub struct NMIFR {
    bits: bool,
}
impl NMIFR {
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
#[doc = "Values that can be written to the field `NMIE`"]
pub enum NMIEW {
    #[doc = "Interrupt request is detected on falling edge of NMI input"]
    FALLINGEDGE,
    #[doc = "Interrupt request is detected on rising edge of NMI input"]
    RISINGEDGE,
}
impl NMIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NMIEW::FALLINGEDGE => false,
            NMIEW::RISINGEDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _NMIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NMIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request is detected on falling edge of NMI input"]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(NMIEW::FALLINGEDGE)
    }
    #[doc = "Interrupt request is detected on rising edge of NMI input"]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(NMIEW::RISINGEDGE)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NMIFW<'a> {
    w: &'a mut W,
}
impl<'a> _NMIFW<'a> {
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
    #[doc = "Bit 15 - NMI Input Level - Sets the level of the signal input at the NMI pin. The NMI pin level can be obtained by reading this bit. This bit cannot be modified."]
    #[inline]
    pub fn nmil(&self) -> NMILR {
        NMILR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - NMI Edge Select - Selects whether the falling or rising edge of the interrupt request signal on the NMI pin is detected."]
    #[inline]
    pub fn nmie(&self) -> NMIER {
        NMIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - NMI Interrupt Request - This bit indicates the status of the NMI interrupt request. This bit cannot be modified. 0: NMI interrupt request has not occurred \\[Clearing conditions\\] - Cleared by changing NMIE of ICR0 - Cleared by reading NMIF while NMIF = 1, then writing 0 to NMIF 1: NMI interrupt request is detected \\[Setting condition\\] - Edge corresponding to NMIE of ICR0 has occurred at NMI pin"]
    #[inline]
    pub fn nmif(&self) -> NMIFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        NMIFR { bits }
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
    #[doc = "Bit 8 - NMI Edge Select - Selects whether the falling or rising edge of the interrupt request signal on the NMI pin is detected."]
    #[inline]
    pub fn nmie(&mut self) -> _NMIEW {
        _NMIEW { w: self }
    }
    #[doc = "Bit 1 - NMI Interrupt Request - This bit indicates the status of the NMI interrupt request. This bit cannot be modified. 0: NMI interrupt request has not occurred \\[Clearing conditions\\] - Cleared by changing NMIE of ICR0 - Cleared by reading NMIF while NMIF = 1, then writing 0 to NMIF 1: NMI interrupt request is detected \\[Setting condition\\] - Edge corresponding to NMIE of ICR0 has occurred at NMI pin"]
    #[inline]
    pub fn nmif(&mut self) -> _NMIFW {
        _NMIFW { w: self }
    }
}
