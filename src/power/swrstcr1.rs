#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SWRSTCR1 {
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
#[doc = "Possible values of the field `AXTALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXTALER {
    #[doc = "Runs the on-chip crystal oscillator/enables the external clock input."]
    RUN,
    #[doc = "Halts the on-chip crystal oscillator/disables the external clock input."]
    HALT,
}
impl AXTALER {
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
            AXTALER::RUN => false,
            AXTALER::HALT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AXTALER {
        match value {
            false => AXTALER::RUN,
            true => AXTALER::HALT,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == AXTALER::RUN
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline]
    pub fn is_halt(&self) -> bool {
        *self == AXTALER::HALT
    }
}
#[doc = r" Value of the field"]
pub struct SRST16R {
    bits: bool,
}
impl SRST16R {
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
pub struct SRST15R {
    bits: bool,
}
impl SRST15R {
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
pub struct SRST14R {
    bits: bool,
}
impl SRST14R {
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
pub struct SRST13R {
    bits: bool,
}
impl SRST13R {
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
pub struct SRST12R {
    bits: bool,
}
impl SRST12R {
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
pub struct SRST11R {
    bits: bool,
}
impl SRST11R {
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
#[doc = "Values that can be written to the field `AXTALE`"]
pub enum AXTALEW {
    #[doc = "Runs the on-chip crystal oscillator/enables the external clock input."]
    RUN,
    #[doc = "Halts the on-chip crystal oscillator/disables the external clock input."]
    HALT,
}
impl AXTALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AXTALEW::RUN => false,
            AXTALEW::HALT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AXTALEW<'a> {
    w: &'a mut W,
}
impl<'a> _AXTALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AXTALEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Runs the on-chip crystal oscillator/enables the external clock input."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(AXTALEW::RUN)
    }
    #[doc = "Halts the on-chip crystal oscillator/disables the external clock input."]
    #[inline]
    pub fn halt(self) -> &'a mut W {
        self.variant(AXTALEW::HALT)
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
pub struct _SRST16W<'a> {
    w: &'a mut W,
}
impl<'a> _SRST16W<'a> {
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
#[doc = r" Proxy"]
pub struct _SRST15W<'a> {
    w: &'a mut W,
}
impl<'a> _SRST15W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRST14W<'a> {
    w: &'a mut W,
}
impl<'a> _SRST14W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRST13W<'a> {
    w: &'a mut W,
}
impl<'a> _SRST13W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRST12W<'a> {
    w: &'a mut W,
}
impl<'a> _SRST12W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRST11W<'a> {
    w: &'a mut W,
}
impl<'a> _SRST11W<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 7 - AUDIO_X1 Clock Control - Controls the function of AUDIO_X1 pin."]
    #[inline]
    pub fn axtale(&self) -> AXTALER {
        AXTALER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Serial Sound Interface Channel 0 Software Reset - Controls the serial sound interface channel 0 reset with software."]
    #[inline]
    pub fn srst16(&self) -> SRST16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        SRST16R { bits }
    }
    #[doc = "Bit 5 - Serial Sound Interface Channel 1 Software Reset - Controls the serial sound interface channel 1 reset with software."]
    #[inline]
    pub fn srst15(&self) -> SRST15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        SRST15R { bits }
    }
    #[doc = "Bit 4 - Serial Sound Interface Channel 2 Software Reset - Controls the serial sound interface channel 2 reset with software."]
    #[inline]
    pub fn srst14(&self) -> SRST14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        SRST14R { bits }
    }
    #[doc = "Bit 3 - Serial Sound Interface Channel 3 Software Reset - Controls the serial sound interface channel 3 reset with software."]
    #[inline]
    pub fn srst13(&self) -> SRST13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        SRST13R { bits }
    }
    #[doc = "Bit 2 - Serial Sound Interface Channel 4 Software Reset - Controls the serial sound interface channel 4 reset with software."]
    #[inline]
    pub fn srst12(&self) -> SRST12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        SRST12R { bits }
    }
    #[doc = "Bit 1 - Serial Sound Interface Channel 5 Software Reset - Controls the serial sound interface channel 5 reset with software."]
    #[inline]
    pub fn srst11(&self) -> SRST11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        SRST11R { bits }
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
    #[doc = "Bit 7 - AUDIO_X1 Clock Control - Controls the function of AUDIO_X1 pin."]
    #[inline]
    pub fn axtale(&mut self) -> _AXTALEW {
        _AXTALEW { w: self }
    }
    #[doc = "Bit 6 - Serial Sound Interface Channel 0 Software Reset - Controls the serial sound interface channel 0 reset with software."]
    #[inline]
    pub fn srst16(&mut self) -> _SRST16W {
        _SRST16W { w: self }
    }
    #[doc = "Bit 5 - Serial Sound Interface Channel 1 Software Reset - Controls the serial sound interface channel 1 reset with software."]
    #[inline]
    pub fn srst15(&mut self) -> _SRST15W {
        _SRST15W { w: self }
    }
    #[doc = "Bit 4 - Serial Sound Interface Channel 2 Software Reset - Controls the serial sound interface channel 2 reset with software."]
    #[inline]
    pub fn srst14(&mut self) -> _SRST14W {
        _SRST14W { w: self }
    }
    #[doc = "Bit 3 - Serial Sound Interface Channel 3 Software Reset - Controls the serial sound interface channel 3 reset with software."]
    #[inline]
    pub fn srst13(&mut self) -> _SRST13W {
        _SRST13W { w: self }
    }
    #[doc = "Bit 2 - Serial Sound Interface Channel 4 Software Reset - Controls the serial sound interface channel 4 reset with software."]
    #[inline]
    pub fn srst12(&mut self) -> _SRST12W {
        _SRST12W { w: self }
    }
    #[doc = "Bit 1 - Serial Sound Interface Channel 5 Software Reset - Controls the serial sound interface channel 5 reset with software."]
    #[inline]
    pub fn srst11(&mut self) -> _SRST11W {
        _SRST11W { w: self }
    }
}
