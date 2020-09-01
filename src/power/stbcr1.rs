#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::STBCR1 {
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
#[doc = "Possible values of the field `STBY_DEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBY_DEEPR {
    #[doc = "Executing WFI or WFE (*1) instruction puts chip into sleep mode."]
    SLEEP,
    #[doc = "Executing WFI instruction puts chip into software standby mode. (*2)"]
    SOFTWARE_STANDBY,
    #[doc = "Executing WFI instruction puts chip into deep standby mode. (*2)"]
    DEEP_STANDBY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STBY_DEEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STBY_DEEPR::SLEEP => 0,
            STBY_DEEPR::SOFTWARE_STANDBY => 0x02,
            STBY_DEEPR::DEEP_STANDBY => 0x03,
            STBY_DEEPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STBY_DEEPR {
        match value {
            0 => STBY_DEEPR::SLEEP,
            2 => STBY_DEEPR::SOFTWARE_STANDBY,
            3 => STBY_DEEPR::DEEP_STANDBY,
            i => STBY_DEEPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline]
    pub fn is_sleep(&self) -> bool {
        *self == STBY_DEEPR::SLEEP
    }
    #[doc = "Checks if the value of the field is `SOFTWARE_STANDBY`"]
    #[inline]
    pub fn is_software_standby(&self) -> bool {
        *self == STBY_DEEPR::SOFTWARE_STANDBY
    }
    #[doc = "Checks if the value of the field is `DEEP_STANDBY`"]
    #[inline]
    pub fn is_deep_standby(&self) -> bool {
        *self == STBY_DEEPR::DEEP_STANDBY
    }
}
#[doc = "Values that can be written to the field `STBY_DEEP`"]
pub enum STBY_DEEPW {
    #[doc = "Executing WFI or WFE (*1) instruction puts chip into sleep mode."]
    SLEEP,
    #[doc = "Executing WFI instruction puts chip into software standby mode. (*2)"]
    SOFTWARE_STANDBY,
    #[doc = "Executing WFI instruction puts chip into deep standby mode. (*2)"]
    DEEP_STANDBY,
}
impl STBY_DEEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STBY_DEEPW::SLEEP => 0,
            STBY_DEEPW::SOFTWARE_STANDBY => 2,
            STBY_DEEPW::DEEP_STANDBY => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STBY_DEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _STBY_DEEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STBY_DEEPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Executing WFI or WFE (*1) instruction puts chip into sleep mode."]
    #[inline]
    pub fn sleep(self) -> &'a mut W {
        self.variant(STBY_DEEPW::SLEEP)
    }
    #[doc = "Executing WFI instruction puts chip into software standby mode. (*2)"]
    #[inline]
    pub fn software_standby(self) -> &'a mut W {
        self.variant(STBY_DEEPW::SOFTWARE_STANDBY)
    }
    #[doc = "Executing WFI instruction puts chip into deep standby mode. (*2)"]
    #[inline]
    pub fn deep_standby(self) -> &'a mut W {
        self.variant(STBY_DEEPW::DEEP_STANDBY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
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
    #[doc = "Bits 6:7 - Software Standby, Deep Standby - Specifies transition to software standby mode or deep standby mode. (*1) When using the WFE instruction, execute the following three instructions in the given sequence: SEV, WFE, and WFE. The instruction to be executed at the time of a transition to the sleep mode depends on a condition. For details, see 55.3.1 (1) Transition to Sleep Mode. (*2) Do not execute the WFE instruction while the STBY bit in STBCR1 is 1."]
    #[inline]
    pub fn stby_deep(&self) -> STBY_DEEPR {
        STBY_DEEPR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    #[doc = "Bits 6:7 - Software Standby, Deep Standby - Specifies transition to software standby mode or deep standby mode. (*1) When using the WFE instruction, execute the following three instructions in the given sequence: SEV, WFE, and WFE. The instruction to be executed at the time of a transition to the sleep mode depends on a condition. For details, see 55.3.1 (1) Transition to Sleep Mode. (*2) Do not execute the WFE instruction while the STBY bit in STBCR1 is 1."]
    #[inline]
    pub fn stby_deep(&mut self) -> _STBY_DEEPW {
        _STBY_DEEPW { w: self }
    }
}
