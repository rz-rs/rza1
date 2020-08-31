#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SPTR {
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
pub struct RTSIOR {
    bits: bool,
}
impl RTSIOR {
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
#[doc = "Possible values of the field `RTSDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSDTR {
    #[doc = "Input/output data is low level"]
    LOW,
    #[doc = "Input/output data is high level"]
    HIGH,
}
impl RTSDTR {
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
            RTSDTR::LOW => false,
            RTSDTR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTSDTR {
        match value {
            false => RTSDTR::LOW,
            true => RTSDTR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == RTSDTR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == RTSDTR::HIGH
    }
}
#[doc = r" Value of the field"]
pub struct CTSIOR {
    bits: bool,
}
impl CTSIOR {
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
#[doc = "Possible values of the field `CTSDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSDTR {
    #[doc = "Input/output data is low level"]
    LOW,
    #[doc = "Input/output data is high level"]
    HIGH,
}
impl CTSDTR {
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
            CTSDTR::LOW => false,
            CTSDTR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSDTR {
        match value {
            false => CTSDTR::LOW,
            true => CTSDTR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CTSDTR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CTSDTR::HIGH
    }
}
#[doc = r" Value of the field"]
pub struct SCKIOR {
    bits: bool,
}
impl SCKIOR {
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
#[doc = "Possible values of the field `SCKDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKDTR {
    #[doc = "Input/output data is low level"]
    LOW,
    #[doc = "Input/output data is high level"]
    HIGH,
}
impl SCKDTR {
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
            SCKDTR::LOW => false,
            SCKDTR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCKDTR {
        match value {
            false => SCKDTR::LOW,
            true => SCKDTR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SCKDTR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SCKDTR::HIGH
    }
}
#[doc = r" Value of the field"]
pub struct SPB2IOR {
    bits: bool,
}
impl SPB2IOR {
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
#[doc = "Possible values of the field `SPB2DT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPB2DTR {
    #[doc = "Input/output data is low level"]
    LOW,
    #[doc = "Input/output data is high level"]
    HIGH,
}
impl SPB2DTR {
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
            SPB2DTR::LOW => false,
            SPB2DTR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPB2DTR {
        match value {
            false => SPB2DTR::LOW,
            true => SPB2DTR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SPB2DTR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SPB2DTR::HIGH
    }
}
#[doc = r" Proxy"]
pub struct _RTSIOW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSIOW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTSDT`"]
pub enum RTSDTW {
    #[doc = "Input/output data is low level"]
    LOW,
    #[doc = "Input/output data is high level"]
    HIGH,
}
impl RTSDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTSDTW::LOW => false,
            RTSDTW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTSDTW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTSDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input/output data is low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(RTSDTW::LOW)
    }
    #[doc = "Input/output data is high level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(RTSDTW::HIGH)
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
pub struct _CTSIOW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSIOW<'a> {
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
#[doc = "Values that can be written to the field `CTSDT`"]
pub enum CTSDTW {
    #[doc = "Input/output data is low level"]
    LOW,
    #[doc = "Input/output data is high level"]
    HIGH,
}
impl CTSDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSDTW::LOW => false,
            CTSDTW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSDTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input/output data is low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CTSDTW::LOW)
    }
    #[doc = "Input/output data is high level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CTSDTW::HIGH)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCKIOW<'a> {
    w: &'a mut W,
}
impl<'a> _SCKIOW<'a> {
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
#[doc = "Values that can be written to the field `SCKDT`"]
pub enum SCKDTW {
    #[doc = "Input/output data is low level"]
    LOW,
    #[doc = "Input/output data is high level"]
    HIGH,
}
impl SCKDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCKDTW::LOW => false,
            SCKDTW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCKDTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCKDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCKDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input/output data is low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SCKDTW::LOW)
    }
    #[doc = "Input/output data is high level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SCKDTW::HIGH)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPB2IOW<'a> {
    w: &'a mut W,
}
impl<'a> _SPB2IOW<'a> {
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
#[doc = "Values that can be written to the field `SPB2DT`"]
pub enum SPB2DTW {
    #[doc = "Input/output data is low level"]
    LOW,
    #[doc = "Input/output data is high level"]
    HIGH,
}
impl SPB2DTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPB2DTW::LOW => false,
            SPB2DTW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPB2DTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPB2DTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPB2DTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input/output data is low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SPB2DTW::LOW)
    }
    #[doc = "Input/output data is high level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SPB2DTW::HIGH)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 7 - RTS Port Input/Output - Specifies input or output for the serial port RTS pin. When the RTS pin is actually used as a port outputting the RTSDT bit value, the MCE bit in SCFCR should be cleared to 0."]
    #[inline]
    pub fn rtsio(&self) -> RTSIOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RTSIOR { bits }
    }
    #[doc = "Bit 6 - RTS Port Data - Specifies the input/output data of the serial port RTS pin. Input/output is specified by the RTSIO bit. For output, the RTSDT bit value is output to the RTS pin. The RTS pin status is read from the RTSDT bit regardless of the RTSIO bit setting. However, RTS input/output must be set in the general purpose I/O ports."]
    #[inline]
    pub fn rtsdt(&self) -> RTSDTR {
        RTSDTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - CTS Port Input/Output - Specifies input or output for the serial port CTS pin. When the CTS pin is actually used as a port outputting the CTSDT bit value, the MCE bit in SCFCR should be cleared to 0."]
    #[inline]
    pub fn ctsio(&self) -> CTSIOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CTSIOR { bits }
    }
    #[doc = "Bit 4 - CTS Port Data - Specifies the input/output data of the serial port CTS pin. Input/output is specified by the CTSIO bit. For output, the CTSDT bit value is output to the CTS pin. The CTS pin status is read from the CTSDT bit regardless of the CTSIO bit setting. However, CTS input/output must be set in the general purpose I/O ports."]
    #[inline]
    pub fn ctsdt(&self) -> CTSDTR {
        CTSDTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - SCK Port Input/Output - Specifies input or output for the serial port SCK pin. When the SCK pin is actually used as a port outputting the SCKDT bit value, the CKE\\[1:0\\] bits in SCSCR should be cleared to 0."]
    #[inline]
    pub fn sckio(&self) -> SCKIOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        SCKIOR { bits }
    }
    #[doc = "Bit 2 - SCK Port Data - Specifies the input/output data of the serial port SCK pin. Input/output is specified by the SCKIO bit. For output, the SCKDT bit value is output to the SCK pin. The SCK pin status is read from the SCKDT bit regardless of the SCKIO bit setting. However, SCK input/output must be set in the general purpose I/O ports."]
    #[inline]
    pub fn sckdt(&self) -> SCKDTR {
        SCKDTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Serial Port Break Input/Output - Specifies input or output for the serial port TxD pin. When the TxD pin is actually used as a port outputting the SPB2DT bit value, the TE bit in SCSCR should be cleared to 0."]
    #[inline]
    pub fn spb2io(&self) -> SPB2IOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        SPB2IOR { bits }
    }
    #[doc = "Bit 0 - Serial Port Break Data - Specifies the input data of the RxD pin and the output data of the TxD pin used as serial ports. Input/output is specified by the SPB2IO bit. When the TxD pin is set to output, the SPB2DT bit value is output to the TxD pin. The RxD pin status is read from the SPB2DT bit regardless of the SPB2IO bit setting. However, RxD input and TxD output must be set in the general purpose I/O ports."]
    #[inline]
    pub fn spb2dt(&self) -> SPB2DTR {
        SPB2DTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 7 - RTS Port Input/Output - Specifies input or output for the serial port RTS pin. When the RTS pin is actually used as a port outputting the RTSDT bit value, the MCE bit in SCFCR should be cleared to 0."]
    #[inline]
    pub fn rtsio(&mut self) -> _RTSIOW {
        _RTSIOW { w: self }
    }
    #[doc = "Bit 6 - RTS Port Data - Specifies the input/output data of the serial port RTS pin. Input/output is specified by the RTSIO bit. For output, the RTSDT bit value is output to the RTS pin. The RTS pin status is read from the RTSDT bit regardless of the RTSIO bit setting. However, RTS input/output must be set in the general purpose I/O ports."]
    #[inline]
    pub fn rtsdt(&mut self) -> _RTSDTW {
        _RTSDTW { w: self }
    }
    #[doc = "Bit 5 - CTS Port Input/Output - Specifies input or output for the serial port CTS pin. When the CTS pin is actually used as a port outputting the CTSDT bit value, the MCE bit in SCFCR should be cleared to 0."]
    #[inline]
    pub fn ctsio(&mut self) -> _CTSIOW {
        _CTSIOW { w: self }
    }
    #[doc = "Bit 4 - CTS Port Data - Specifies the input/output data of the serial port CTS pin. Input/output is specified by the CTSIO bit. For output, the CTSDT bit value is output to the CTS pin. The CTS pin status is read from the CTSDT bit regardless of the CTSIO bit setting. However, CTS input/output must be set in the general purpose I/O ports."]
    #[inline]
    pub fn ctsdt(&mut self) -> _CTSDTW {
        _CTSDTW { w: self }
    }
    #[doc = "Bit 3 - SCK Port Input/Output - Specifies input or output for the serial port SCK pin. When the SCK pin is actually used as a port outputting the SCKDT bit value, the CKE\\[1:0\\] bits in SCSCR should be cleared to 0."]
    #[inline]
    pub fn sckio(&mut self) -> _SCKIOW {
        _SCKIOW { w: self }
    }
    #[doc = "Bit 2 - SCK Port Data - Specifies the input/output data of the serial port SCK pin. Input/output is specified by the SCKIO bit. For output, the SCKDT bit value is output to the SCK pin. The SCK pin status is read from the SCKDT bit regardless of the SCKIO bit setting. However, SCK input/output must be set in the general purpose I/O ports."]
    #[inline]
    pub fn sckdt(&mut self) -> _SCKDTW {
        _SCKDTW { w: self }
    }
    #[doc = "Bit 1 - Serial Port Break Input/Output - Specifies input or output for the serial port TxD pin. When the TxD pin is actually used as a port outputting the SPB2DT bit value, the TE bit in SCSCR should be cleared to 0."]
    #[inline]
    pub fn spb2io(&mut self) -> _SPB2IOW {
        _SPB2IOW { w: self }
    }
    #[doc = "Bit 0 - Serial Port Break Data - Specifies the input data of the RxD pin and the output data of the TxD pin used as serial ports. Input/output is specified by the SPB2IO bit. When the TxD pin is set to output, the SPB2DT bit value is output to the TxD pin. The RxD pin status is read from the SPB2DT bit regardless of the SPB2IO bit setting. However, RxD input and TxD output must be set in the general purpose I/O ports."]
    #[inline]
    pub fn spb2dt(&mut self) -> _SPB2DTW {
        _SPB2DTW { w: self }
    }
}
