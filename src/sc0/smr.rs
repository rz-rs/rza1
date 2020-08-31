#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SMR {
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
#[doc = "Possible values of the field `CA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAR {
    #[doc = "Asynchronous mode"]
    ASYNCHRONOUS,
    #[doc = "Clock synchronous mode"]
    CLOCKSYNCHRONOUS,
}
impl CAR {
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
            CAR::ASYNCHRONOUS => false,
            CAR::CLOCKSYNCHRONOUS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAR {
        match value {
            false => CAR::ASYNCHRONOUS,
            true => CAR::CLOCKSYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline]
    pub fn is_asynchronous(&self) -> bool {
        *self == CAR::ASYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `CLOCKSYNCHRONOUS`"]
    #[inline]
    pub fn is_clock_synchronous(&self) -> bool {
        *self == CAR::CLOCKSYNCHRONOUS
    }
}
#[doc = "Possible values of the field `CHR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHRR {
    #[doc = "8-bit data"]
    _8,
    #[doc = "7-bit data"]
    _7,
}
impl CHRR {
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
            CHRR::_8 => false,
            CHRR::_7 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHRR {
        match value {
            false => CHRR::_8,
            true => CHRR::_7,
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == CHRR::_8
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == CHRR::_7
    }
}
#[doc = r" Value of the field"]
pub struct PER {
    bits: bool,
}
impl PER {
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
#[doc = "Possible values of the field `OE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OER {
    #[doc = "Even parity - If even parity is selected, the parity bit is added to transmit data to make an even number of 1s in the transmitted character and parity bit combined. Receive data is checked to see if it has an even number of 1s in the received character and parity bit combined."]
    EVEN,
    #[doc = "Odd parity - If odd parity is selected, the parity bit is added to transmit data to make an odd number of 1s in the transmitted character and parity bit combined. Receive data is checked to see if it has an odd number of 1s in the received character and parity bit combined."]
    ODD,
}
impl OER {
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
            OER::EVEN => false,
            OER::ODD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OER {
        match value {
            false => OER::EVEN,
            true => OER::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == OER::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == OER::ODD
    }
}
#[doc = "Possible values of the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPR {
    #[doc = "One stop bit - When transmitting, a single 1-bit is added at the end of each transmitted character."]
    _1,
    #[doc = "Two stop bits - When transmitting, two 1 bits are added at the end of each transmitted character."]
    _2,
}
impl STOPR {
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
            STOPR::_1 => false,
            STOPR::_2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPR {
        match value {
            false => STOPR::_1,
            true => STOPR::_2,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STOPR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == STOPR::_2
    }
}
#[doc = "Possible values of the field `CKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSR {
    #[doc = "P1\u{3d5}"]
    DIVIDE_BY_1,
    #[doc = "P1\u{3d5}/4"]
    DIVIDE_BY_4,
    #[doc = "P1\u{3d5}/16"]
    DIVIDE_BY_16,
    #[doc = "P1\u{3d5}/64"]
    DIVIDE_BY_64,
}
impl CKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKSR::DIVIDE_BY_1 => 0,
            CKSR::DIVIDE_BY_4 => 0x01,
            CKSR::DIVIDE_BY_16 => 0x02,
            CKSR::DIVIDE_BY_64 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKSR {
        match value {
            0 => CKSR::DIVIDE_BY_1,
            1 => CKSR::DIVIDE_BY_4,
            2 => CKSR::DIVIDE_BY_16,
            3 => CKSR::DIVIDE_BY_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_1`"]
    #[inline]
    pub fn is_divide_by_1(&self) -> bool {
        *self == CKSR::DIVIDE_BY_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_4`"]
    #[inline]
    pub fn is_divide_by_4(&self) -> bool {
        *self == CKSR::DIVIDE_BY_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_16`"]
    #[inline]
    pub fn is_divide_by_16(&self) -> bool {
        *self == CKSR::DIVIDE_BY_16
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_64`"]
    #[inline]
    pub fn is_divide_by_64(&self) -> bool {
        *self == CKSR::DIVIDE_BY_64
    }
}
#[doc = "Values that can be written to the field `CA`"]
pub enum CAW {
    #[doc = "Asynchronous mode"]
    ASYNCHRONOUS,
    #[doc = "Clock synchronous mode"]
    CLOCKSYNCHRONOUS,
}
impl CAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAW::ASYNCHRONOUS => false,
            CAW::CLOCKSYNCHRONOUS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAW<'a> {
    w: &'a mut W,
}
impl<'a> _CAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Asynchronous mode"]
    #[inline]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(CAW::ASYNCHRONOUS)
    }
    #[doc = "Clock synchronous mode"]
    #[inline]
    pub fn clock_synchronous(self) -> &'a mut W {
        self.variant(CAW::CLOCKSYNCHRONOUS)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHR`"]
pub enum CHRW {
    #[doc = "8-bit data"]
    _8,
    #[doc = "7-bit data"]
    _7,
}
impl CHRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHRW::_8 => false,
            CHRW::_7 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHRW<'a> {
    w: &'a mut W,
}
impl<'a> _CHRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "8-bit data"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(CHRW::_8)
    }
    #[doc = "7-bit data"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(CHRW::_7)
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
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
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
#[doc = "Values that can be written to the field `OE`"]
pub enum OEW {
    #[doc = "Even parity - If even parity is selected, the parity bit is added to transmit data to make an even number of 1s in the transmitted character and parity bit combined. Receive data is checked to see if it has an even number of 1s in the received character and parity bit combined."]
    EVEN,
    #[doc = "Odd parity - If odd parity is selected, the parity bit is added to transmit data to make an odd number of 1s in the transmitted character and parity bit combined. Receive data is checked to see if it has an odd number of 1s in the received character and parity bit combined."]
    ODD,
}
impl OEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OEW::EVEN => false,
            OEW::ODD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OEW<'a> {
    w: &'a mut W,
}
impl<'a> _OEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Even parity - If even parity is selected, the parity bit is added to transmit data to make an even number of 1s in the transmitted character and parity bit combined. Receive data is checked to see if it has an even number of 1s in the received character and parity bit combined."]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(OEW::EVEN)
    }
    #[doc = "Odd parity - If odd parity is selected, the parity bit is added to transmit data to make an odd number of 1s in the transmitted character and parity bit combined. Receive data is checked to see if it has an odd number of 1s in the received character and parity bit combined."]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(OEW::ODD)
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
#[doc = "Values that can be written to the field `STOP`"]
pub enum STOPW {
    #[doc = "One stop bit - When transmitting, a single 1-bit is added at the end of each transmitted character."]
    _1,
    #[doc = "Two stop bits - When transmitting, two 1 bits are added at the end of each transmitted character."]
    _2,
}
impl STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPW::_1 => false,
            STOPW::_2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One stop bit - When transmitting, a single 1-bit is added at the end of each transmitted character."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPW::_1)
    }
    #[doc = "Two stop bits - When transmitting, two 1 bits are added at the end of each transmitted character."]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(STOPW::_2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CKS`"]
pub enum CKSW {
    #[doc = "P1\u{3d5}"]
    DIVIDE_BY_1,
    #[doc = "P1\u{3d5}/4"]
    DIVIDE_BY_4,
    #[doc = "P1\u{3d5}/16"]
    DIVIDE_BY_16,
    #[doc = "P1\u{3d5}/64"]
    DIVIDE_BY_64,
}
impl CKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKSW::DIVIDE_BY_1 => 0,
            CKSW::DIVIDE_BY_4 => 1,
            CKSW::DIVIDE_BY_16 => 2,
            CKSW::DIVIDE_BY_64 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKSW<'a> {
    w: &'a mut W,
}
impl<'a> _CKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "P1\u{3d5}"]
    #[inline]
    pub fn divide_by_1(self) -> &'a mut W {
        self.variant(CKSW::DIVIDE_BY_1)
    }
    #[doc = "P1\u{3d5}/4"]
    #[inline]
    pub fn divide_by_4(self) -> &'a mut W {
        self.variant(CKSW::DIVIDE_BY_4)
    }
    #[doc = "P1\u{3d5}/16"]
    #[inline]
    pub fn divide_by_16(self) -> &'a mut W {
        self.variant(CKSW::DIVIDE_BY_16)
    }
    #[doc = "P1\u{3d5}/64"]
    #[inline]
    pub fn divide_by_64(self) -> &'a mut W {
        self.variant(CKSW::DIVIDE_BY_64)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bit 7 - Communication Mode - Selects operating mode from asynchronous and clock synchronous modes."]
    #[inline]
    pub fn ca(&self) -> CAR {
        CAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Character Length - Selects 7-bit or 8-bit data length in asynchronous mode. In the clock synchronous mode, the data length is always 8 bits, regardless of the CHR setting."]
    #[inline]
    pub fn chr(&self) -> CHRR {
        CHRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Parity Enable - Selects whether to add a parity bit to transmit data and to check the parity of receive data, in asynchronous mode. In clock synchronous mode, a parity bit is neither added nor checked, regardless of the PE setting."]
    #[inline]
    pub fn pe(&self) -> PER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        PER { bits }
    }
    #[doc = "Bit 4 - Parity Mode - Selects even or odd parity when parity bits are added and checked. The O/E setting is used only in asynchronous mode and only when the parity enable bit (PE) is set to 1 to enable parity addition and checking. The O/E setting is ignored in clock synchronous mode or in asynchronous mode when parity addition and checking is disabled."]
    #[inline]
    pub fn oe(&self) -> OER {
        OER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Stop Bit Length - Selects one or two bits as the stop bit length in asynchronous mode. This setting is used only in asynchronous mode. It is ignored in clock synchronous mode because no stop bits are added. When receiving, only the first stop bit is checked, regardless of the STOP bit setting. If the second stop bit is 1, it is treated as a stop bit, but if the second stop bit is 0, it is treated as the start bit of the next incoming character."]
    #[inline]
    pub fn stop(&self) -> STOPR {
        STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 0:1 - Clock Select - Select the internal clock source of the on-chip baud rate generator. For further information on the clock source, bit rate register settings, and baud rate, see section 14.3.8, Bit Rate Register (SCBRR)."]
    #[inline]
    pub fn cks(&self) -> CKSR {
        CKSR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - Communication Mode - Selects operating mode from asynchronous and clock synchronous modes."]
    #[inline]
    pub fn ca(&mut self) -> _CAW {
        _CAW { w: self }
    }
    #[doc = "Bit 6 - Character Length - Selects 7-bit or 8-bit data length in asynchronous mode. In the clock synchronous mode, the data length is always 8 bits, regardless of the CHR setting."]
    #[inline]
    pub fn chr(&mut self) -> _CHRW {
        _CHRW { w: self }
    }
    #[doc = "Bit 5 - Parity Enable - Selects whether to add a parity bit to transmit data and to check the parity of receive data, in asynchronous mode. In clock synchronous mode, a parity bit is neither added nor checked, regardless of the PE setting."]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bit 4 - Parity Mode - Selects even or odd parity when parity bits are added and checked. The O/E setting is used only in asynchronous mode and only when the parity enable bit (PE) is set to 1 to enable parity addition and checking. The O/E setting is ignored in clock synchronous mode or in asynchronous mode when parity addition and checking is disabled."]
    #[inline]
    pub fn oe(&mut self) -> _OEW {
        _OEW { w: self }
    }
    #[doc = "Bit 3 - Stop Bit Length - Selects one or two bits as the stop bit length in asynchronous mode. This setting is used only in asynchronous mode. It is ignored in clock synchronous mode because no stop bits are added. When receiving, only the first stop bit is checked, regardless of the STOP bit setting. If the second stop bit is 1, it is treated as a stop bit, but if the second stop bit is 0, it is treated as the start bit of the next incoming character."]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bits 0:1 - Clock Select - Select the internal clock source of the on-chip baud rate generator. For further information on the clock source, bit rate register settings, and baud rate, see section 14.3.8, Bit Rate Register (SCBRR)."]
    #[inline]
    pub fn cks(&mut self) -> _CKSW {
        _CKSW { w: self }
    }
}
