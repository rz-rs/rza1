#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FRQCR0 {
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
#[doc = "Possible values of the field `CKOEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKOEN2R {
    #[doc = "Unstable clock output"]
    UNSTABLE_CLOCK_OUT,
    #[doc = "Low-level output"]
    LOW_OUT,
}
impl CKOEN2R {
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
            CKOEN2R::UNSTABLE_CLOCK_OUT => false,
            CKOEN2R::LOW_OUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CKOEN2R {
        match value {
            false => CKOEN2R::UNSTABLE_CLOCK_OUT,
            true => CKOEN2R::LOW_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `UNSTABLE_CLOCK_OUT`"]
    #[inline]
    pub fn is_unstable_clock_out(&self) -> bool {
        *self == CKOEN2R::UNSTABLE_CLOCK_OUT
    }
    #[doc = "Checks if the value of the field is `LOW_OUT`"]
    #[inline]
    pub fn is_low_out(&self) -> bool {
        *self == CKOEN2R::LOW_OUT
    }
}
#[doc = "Possible values of the field `CKOEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKOENR {
    #[doc = "Normal operation: output, Software Standy Mode: Hi-Z, Deep Standby Mode: Hi-Z"]
    OUT_HIZ_HIZ,
    #[doc = "Normal operation: output, Software Standy Mode: low-level output, Deep Standby Mode: low-level output"]
    OUT_LOW_LOW,
    #[doc = "Normal operation: output, Software Standy Mode: unstable clock output, Deep Standby Mode: low-level for high-level output"]
    OUT_OUT_OUT,
    #[doc = "Normal operation: Hi-Z, Software Standy Mode: Hi-Z, Deep Standby Mode: Hi-Z"]
    HIZ_HIZ_HIZ,
}
impl CKOENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKOENR::OUT_HIZ_HIZ => 0,
            CKOENR::OUT_LOW_LOW => 0x01,
            CKOENR::OUT_OUT_OUT => 0x02,
            CKOENR::HIZ_HIZ_HIZ => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKOENR {
        match value {
            0 => CKOENR::OUT_HIZ_HIZ,
            1 => CKOENR::OUT_LOW_LOW,
            2 => CKOENR::OUT_OUT_OUT,
            3 => CKOENR::HIZ_HIZ_HIZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUT_HIZ_HIZ`"]
    #[inline]
    pub fn is_out_hiz_hiz(&self) -> bool {
        *self == CKOENR::OUT_HIZ_HIZ
    }
    #[doc = "Checks if the value of the field is `OUT_LOW_LOW`"]
    #[inline]
    pub fn is_out_low_low(&self) -> bool {
        *self == CKOENR::OUT_LOW_LOW
    }
    #[doc = "Checks if the value of the field is `OUT_OUT_OUT`"]
    #[inline]
    pub fn is_out_out_out(&self) -> bool {
        *self == CKOENR::OUT_OUT_OUT
    }
    #[doc = "Checks if the value of the field is `HIZ_HIZ_HIZ`"]
    #[inline]
    pub fn is_hiz_hiz_hiz(&self) -> bool {
        *self == CKOENR::HIZ_HIZ_HIZ
    }
}
#[doc = "Possible values of the field `IFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFCR {
    #[doc = "1/1 time"]
    _1,
    #[doc = "2/3 time"]
    _2_OVER_3,
    #[doc = "1/3 time"]
    _1_OVER_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IFCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IFCR::_1 => 0,
            IFCR::_2_OVER_3 => 0x01,
            IFCR::_1_OVER_3 => 0x03,
            IFCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IFCR {
        match value {
            0 => IFCR::_1,
            1 => IFCR::_2_OVER_3,
            3 => IFCR::_1_OVER_3,
            i => IFCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IFCR::_1
    }
    #[doc = "Checks if the value of the field is `_2_OVER_3`"]
    #[inline]
    pub fn is_2_over_3(&self) -> bool {
        *self == IFCR::_2_OVER_3
    }
    #[doc = "Checks if the value of the field is `_1_OVER_3`"]
    #[inline]
    pub fn is_1_over_3(&self) -> bool {
        *self == IFCR::_1_OVER_3
    }
}
#[doc = "Values that can be written to the field `CKOEN`"]
pub enum CKOENW {
    #[doc = "Normal operation: output, Software Standy Mode: Hi-Z, Deep Standby Mode: Hi-Z"]
    OUT_HIZ_HIZ,
    #[doc = "Normal operation: output, Software Standy Mode: low-level output, Deep Standby Mode: low-level output"]
    OUT_LOW_LOW,
    #[doc = "Normal operation: output, Software Standy Mode: unstable clock output, Deep Standby Mode: low-level for high-level output"]
    OUT_OUT_OUT,
    #[doc = "Normal operation: Hi-Z, Software Standy Mode: Hi-Z, Deep Standby Mode: Hi-Z"]
    HIZ_HIZ_HIZ,
}
impl CKOENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKOENW::OUT_HIZ_HIZ => 0,
            CKOENW::OUT_LOW_LOW => 1,
            CKOENW::OUT_OUT_OUT => 2,
            CKOENW::HIZ_HIZ_HIZ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKOENW<'a> {
    w: &'a mut W,
}
impl<'a> _CKOENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKOENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal operation: output, Software Standy Mode: Hi-Z, Deep Standby Mode: Hi-Z"]
    #[inline]
    pub fn out_hiz_hiz(self) -> &'a mut W {
        self.variant(CKOENW::OUT_HIZ_HIZ)
    }
    #[doc = "Normal operation: output, Software Standy Mode: low-level output, Deep Standby Mode: low-level output"]
    #[inline]
    pub fn out_low_low(self) -> &'a mut W {
        self.variant(CKOENW::OUT_LOW_LOW)
    }
    #[doc = "Normal operation: output, Software Standy Mode: unstable clock output, Deep Standby Mode: low-level for high-level output"]
    #[inline]
    pub fn out_out_out(self) -> &'a mut W {
        self.variant(CKOENW::OUT_OUT_OUT)
    }
    #[doc = "Normal operation: Hi-Z, Software Standy Mode: Hi-Z, Deep Standby Mode: Hi-Z"]
    #[inline]
    pub fn hiz_hiz_hiz(self) -> &'a mut W {
        self.variant(CKOENW::HIZ_HIZ_HIZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IFC`"]
pub enum IFCW {
    #[doc = "1/1 time"]
    _1,
    #[doc = "2/3 time"]
    _2_OVER_3,
    #[doc = "1/3 time"]
    _1_OVER_3,
}
impl IFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IFCW::_1 => 0,
            IFCW::_2_OVER_3 => 1,
            IFCW::_1_OVER_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IFCW<'a> {
    w: &'a mut W,
}
impl<'a> _IFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IFCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1/1 time"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFCW::_1)
    }
    #[doc = "2/3 time"]
    #[inline]
    pub fn _2_over_3(self) -> &'a mut W {
        self.variant(IFCW::_2_OVER_3)
    }
    #[doc = "1/3 time"]
    #[inline]
    pub fn _1_over_3(self) -> &'a mut W {
        self.variant(IFCW::_1_OVER_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 14 - Clock Output Enable 2 - Specifies whether the CKIO pin outputs clock signals or is fixed to the low level when the gain of the crystal oscillator for the XTAL pin is changed. If this bit is set to 1, the CKIO pin is fixed to the low level when the gain of the crystal oscillator for the XTAL pin is changed. Therefore, the malfunction of an external circuit caused by an unstable CKIO clock while changing the gain of the crystal oscillator for the XTAL pin can be prevented."]
    #[inline]
    pub fn ckoen2(&self) -> CKOEN2R {
        CKOEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 12:13 - Clock Output Enable - These bits specify whether the CKIO pin outputs clock signals, or is set to a fixed level or high impedance (Hi-Z) during normal operation mode, deep standby mode, software standby mode, or cancellation of standby mode. If these bits are set to 01, the CKIO pin is fixed at low during deep standby mode, software standby mode, or cancellation of software standby mode. Therefore, the malfunction of an external circuit caused by an unstable CKIO clock during cancellation of software standby mode can be prevented."]
    #[inline]
    pub fn ckoen(&self) -> CKOENR {
        CKOENR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:9 - CPU Clock Frequency Division Ratio - These bits specify the frequency division ratio of the CPU clock with respect to the output frequency of PLL circuit. Note: See section 6.5.1."]
    #[inline]
    pub fn ifc(&self) -> IFCR {
        IFCR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x0335 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:13 - Clock Output Enable - These bits specify whether the CKIO pin outputs clock signals, or is set to a fixed level or high impedance (Hi-Z) during normal operation mode, deep standby mode, software standby mode, or cancellation of standby mode. If these bits are set to 01, the CKIO pin is fixed at low during deep standby mode, software standby mode, or cancellation of software standby mode. Therefore, the malfunction of an external circuit caused by an unstable CKIO clock during cancellation of software standby mode can be prevented."]
    #[inline]
    pub fn ckoen(&mut self) -> _CKOENW {
        _CKOENW { w: self }
    }
    #[doc = "Bits 8:9 - CPU Clock Frequency Division Ratio - These bits specify the frequency division ratio of the CPU clock with respect to the output frequency of PLL circuit. Note: See section 6.5.1."]
    #[inline]
    pub fn ifc(&mut self) -> _IFCW {
        _IFCW { w: self }
    }
}
