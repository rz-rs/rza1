#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SNCR {
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
#[doc = "Possible values of the field `ETSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETSELR {
    #[doc = "The Ethernet controller is enabled and the EthernetAVB is disabled."]
    ETHERNET,
    #[doc = "The EthernetAVB is enabled and the Ethernet controller is disabled."]
    EAVB,
}
impl ETSELR {
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
            ETSELR::ETHERNET => false,
            ETSELR::EAVB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETSELR {
        match value {
            false => ETSELR::ETHERNET,
            true => ETSELR::EAVB,
        }
    }
    #[doc = "Checks if the value of the field is `ETHERNET`"]
    #[inline]
    pub fn is_ethernet(&self) -> bool {
        *self == ETSELR::ETHERNET
    }
    #[doc = "Checks if the value of the field is `EAVB`"]
    #[inline]
    pub fn is_eavb(&self) -> bool {
        *self == ETSELR::EAVB
    }
}
#[doc = r" Value of the field"]
pub struct SSI5NCER {
    bits: bool,
}
impl SSI5NCER {
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
pub struct SSI4NCER {
    bits: bool,
}
impl SSI4NCER {
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
pub struct SSI3NCER {
    bits: bool,
}
impl SSI3NCER {
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
pub struct SSI2NCER {
    bits: bool,
}
impl SSI2NCER {
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
pub struct SSI1NCER {
    bits: bool,
}
impl SSI1NCER {
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
pub struct SSI0NCER {
    bits: bool,
}
impl SSI0NCER {
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
#[doc = "Values that can be written to the field `ETSEL`"]
pub enum ETSELW {
    #[doc = "The Ethernet controller is enabled and the EthernetAVB is disabled."]
    ETHERNET,
    #[doc = "The EthernetAVB is enabled and the Ethernet controller is disabled."]
    EAVB,
}
impl ETSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETSELW::ETHERNET => false,
            ETSELW::EAVB => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ETSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Ethernet controller is enabled and the EthernetAVB is disabled."]
    #[inline]
    pub fn ethernet(self) -> &'a mut W {
        self.variant(ETSELW::ETHERNET)
    }
    #[doc = "The EthernetAVB is enabled and the Ethernet controller is disabled."]
    #[inline]
    pub fn eavb(self) -> &'a mut W {
        self.variant(ETSELW::EAVB)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SSI5NCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI5NCEW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SSI4NCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI4NCEW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SSI3NCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI3NCEW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SSI2NCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI2NCEW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SSI1NCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI1NCEW<'a> {
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
#[doc = r" Proxy"]
pub struct _SSI0NCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI0NCEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 6 - Enables or disables the Ethernet controller and EthernetAVB with respect to the pins for MII supported in the Ethernet."]
    #[inline]
    pub fn etsel(&self) -> ETSELR {
        ETSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Serial Sound Interface Channel 5 Noise Canceler Enable - Enables or disables the noise canceler of SSISCK5, SSIWS5, and SSIRxD5."]
    #[inline]
    pub fn ssi5nce(&self) -> SSI5NCER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        SSI5NCER { bits }
    }
    #[doc = "Bit 4 - Serial Sound Interface Channel 4 Noise Canceler Enable - Enables or disables the noise canceler of SSISCK4, SSIWS4, and SSIRxD4."]
    #[inline]
    pub fn ssi4nce(&self) -> SSI4NCER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        SSI4NCER { bits }
    }
    #[doc = "Bit 3 - Serial Sound Interface Channel 3 Noise Canceler Enable - Enables or disables the noise canceler of SSISCK3, SSIWS3, and SSIRxD3."]
    #[inline]
    pub fn ssi3nce(&self) -> SSI3NCER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        SSI3NCER { bits }
    }
    #[doc = "Bit 2 - Serial Sound Interface Channel 2 Noise Canceler Enable - Enables or disables the noise canceler of SSISCK2, SSIWS2, and SSIRxD2."]
    #[inline]
    pub fn ssi2nce(&self) -> SSI2NCER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        SSI2NCER { bits }
    }
    #[doc = "Bit 1 - Serial Sound Interface Channel 1 Noise Canceler Enable - Enables or disables the noise canceler of SSISCK1, SSIWS1, and SSIRxD1."]
    #[inline]
    pub fn ssi1nce(&self) -> SSI1NCER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        SSI1NCER { bits }
    }
    #[doc = "Bit 0 - Serial Sound Interface Channel 0 Noise Canceler Enable - Enables or disables the noise canceler of SSISCK0, SSIWS0, and SSIRxD0."]
    #[inline]
    pub fn ssi0nce(&self) -> SSI0NCER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        SSI0NCER { bits }
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
    #[doc = "Bit 6 - Enables or disables the Ethernet controller and EthernetAVB with respect to the pins for MII supported in the Ethernet."]
    #[inline]
    pub fn etsel(&mut self) -> _ETSELW {
        _ETSELW { w: self }
    }
    #[doc = "Bit 5 - Serial Sound Interface Channel 5 Noise Canceler Enable - Enables or disables the noise canceler of SSISCK5, SSIWS5, and SSIRxD5."]
    #[inline]
    pub fn ssi5nce(&mut self) -> _SSI5NCEW {
        _SSI5NCEW { w: self }
    }
    #[doc = "Bit 4 - Serial Sound Interface Channel 4 Noise Canceler Enable - Enables or disables the noise canceler of SSISCK4, SSIWS4, and SSIRxD4."]
    #[inline]
    pub fn ssi4nce(&mut self) -> _SSI4NCEW {
        _SSI4NCEW { w: self }
    }
    #[doc = "Bit 3 - Serial Sound Interface Channel 3 Noise Canceler Enable - Enables or disables the noise canceler of SSISCK3, SSIWS3, and SSIRxD3."]
    #[inline]
    pub fn ssi3nce(&mut self) -> _SSI3NCEW {
        _SSI3NCEW { w: self }
    }
    #[doc = "Bit 2 - Serial Sound Interface Channel 2 Noise Canceler Enable - Enables or disables the noise canceler of SSISCK2, SSIWS2, and SSIRxD2."]
    #[inline]
    pub fn ssi2nce(&mut self) -> _SSI2NCEW {
        _SSI2NCEW { w: self }
    }
    #[doc = "Bit 1 - Serial Sound Interface Channel 1 Noise Canceler Enable - Enables or disables the noise canceler of SSISCK1, SSIWS1, and SSIRxD1."]
    #[inline]
    pub fn ssi1nce(&mut self) -> _SSI1NCEW {
        _SSI1NCEW { w: self }
    }
    #[doc = "Bit 0 - Serial Sound Interface Channel 0 Noise Canceler Enable - Enables or disables the noise canceler of SSISCK0, SSIWS0, and SSIRxD0."]
    #[inline]
    pub fn ssi0nce(&mut self) -> _SSI0NCEW {
        _SSI0NCEW { w: self }
    }
}
