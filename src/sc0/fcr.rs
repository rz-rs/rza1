#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FCR {
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
#[doc = "Possible values of the field `RSTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTRGR {
    #[doc = "undocumented"]
    _15,
    #[doc = "undocumented"]
    _1,
    #[doc = "undocumented"]
    _4,
    #[doc = "undocumented"]
    _6,
    #[doc = "undocumented"]
    _8,
    #[doc = "undocumented"]
    _10,
    #[doc = "undocumented"]
    _12,
    #[doc = "undocumented"]
    _14,
}
impl RSTRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSTRGR::_15 => 0,
            RSTRGR::_1 => 0x01,
            RSTRGR::_4 => 0x02,
            RSTRGR::_6 => 0x03,
            RSTRGR::_8 => 0x04,
            RSTRGR::_10 => 0x05,
            RSTRGR::_12 => 0x06,
            RSTRGR::_14 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSTRGR {
        match value {
            0 => RSTRGR::_15,
            1 => RSTRGR::_1,
            2 => RSTRGR::_4,
            3 => RSTRGR::_6,
            4 => RSTRGR::_8,
            5 => RSTRGR::_10,
            6 => RSTRGR::_12,
            7 => RSTRGR::_14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline]
    pub fn is_15(&self) -> bool {
        *self == RSTRGR::_15
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSTRGR::_1
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == RSTRGR::_4
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == RSTRGR::_6
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == RSTRGR::_8
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RSTRGR::_10
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline]
    pub fn is_12(&self) -> bool {
        *self == RSTRGR::_12
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline]
    pub fn is_14(&self) -> bool {
        *self == RSTRGR::_14
    }
}
#[doc = "Possible values of the field `RTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTRGR {
    #[doc = "undocumented"]
    _1,
    #[doc = "\n                    Asynchronous mode: 4, Clock synchronous mode: 2\n                  "]
    _4OR2,
    #[doc = "undocumented"]
    _8,
    #[doc = "undocumented"]
    _14,
}
impl RTRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTRGR::_1 => 0,
            RTRGR::_4OR2 => 0x01,
            RTRGR::_8 => 0x02,
            RTRGR::_14 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTRGR {
        match value {
            0 => RTRGR::_1,
            1 => RTRGR::_4OR2,
            2 => RTRGR::_8,
            3 => RTRGR::_14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RTRGR::_1
    }
    #[doc = "Checks if the value of the field is `_4OR2`"]
    #[inline]
    pub fn is_4or2(&self) -> bool {
        *self == RTRGR::_4OR2
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == RTRGR::_8
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline]
    pub fn is_14(&self) -> bool {
        *self == RTRGR::_14
    }
}
#[doc = "Possible values of the field `TTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TTRGR {
    #[doc = "(8)"]
    _8,
    #[doc = "(12)"]
    _4,
    #[doc = "(14)"]
    _2,
    #[doc = "(16)"]
    _0,
}
impl TTRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TTRGR::_8 => 0,
            TTRGR::_4 => 0x01,
            TTRGR::_2 => 0x02,
            TTRGR::_0 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TTRGR {
        match value {
            0 => TTRGR::_8,
            1 => TTRGR::_4,
            2 => TTRGR::_2,
            3 => TTRGR::_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == TTRGR::_8
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == TTRGR::_4
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == TTRGR::_2
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TTRGR::_0
    }
}
#[doc = r" Value of the field"]
pub struct MCRR {
    bits: bool,
}
impl MCRR {
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
pub struct TFRSTR {
    bits: bool,
}
impl TFRSTR {
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
pub struct RFRSTR {
    bits: bool,
}
impl RFRSTR {
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
pub struct LOOPR {
    bits: bool,
}
impl LOOPR {
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
#[doc = "Values that can be written to the field `RSTRG`"]
pub enum RSTRGW {
    #[doc = "`0`"]
    _15,
    #[doc = "`1`"]
    _1,
    #[doc = "`10`"]
    _4,
    #[doc = "`11`"]
    _6,
    #[doc = "`100`"]
    _8,
    #[doc = "`101`"]
    _10,
    #[doc = "`110`"]
    _12,
    #[doc = "`111`"]
    _14,
}
impl RSTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSTRGW::_15 => 0,
            RSTRGW::_1 => 1,
            RSTRGW::_4 => 2,
            RSTRGW::_6 => 3,
            RSTRGW::_8 => 4,
            RSTRGW::_10 => 5,
            RSTRGW::_12 => 6,
            RSTRGW::_14 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn _15(self) -> &'a mut W {
        self.variant(RSTRGW::_15)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTRGW::_1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(RSTRGW::_4)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(RSTRGW::_6)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(RSTRGW::_8)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSTRGW::_10)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn _12(self) -> &'a mut W {
        self.variant(RSTRGW::_12)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn _14(self) -> &'a mut W {
        self.variant(RSTRGW::_14)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTRG`"]
pub enum RTRGW {
    #[doc = "`0`"]
    _1,
    #[doc = "\n                    Asynchronous mode: 4, Clock synchronous mode: 2\n                  "]
    _4OR2,
    #[doc = "`10`"]
    _8,
    #[doc = "`11`"]
    _14,
}
impl RTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTRGW::_1 => 0,
            RTRGW::_4OR2 => 1,
            RTRGW::_8 => 2,
            RTRGW::_14 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _RTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTRGW::_1)
    }
    #[doc = "Asynchronous mode: 4, Clock synchronous mode: 2"]
    #[inline]
    pub fn _4or2(self) -> &'a mut W {
        self.variant(RTRGW::_4OR2)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(RTRGW::_8)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn _14(self) -> &'a mut W {
        self.variant(RTRGW::_14)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TTRG`"]
pub enum TTRGW {
    #[doc = "(8)"]
    _8,
    #[doc = "(12)"]
    _4,
    #[doc = "(14)"]
    _2,
    #[doc = "(16)"]
    _0,
}
impl TTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TTRGW::_8 => 0,
            TTRGW::_4 => 1,
            TTRGW::_2 => 2,
            TTRGW::_0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _TTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TTRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "(8)"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(TTRGW::_8)
    }
    #[doc = "(12)"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(TTRGW::_4)
    }
    #[doc = "(14)"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(TTRGW::_2)
    }
    #[doc = "(16)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TTRGW::_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCRW<'a> {
    w: &'a mut W,
}
impl<'a> _MCRW<'a> {
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
pub struct _TFRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TFRSTW<'a> {
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
pub struct _RFRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RFRSTW<'a> {
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
pub struct _LOOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPW<'a> {
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
    #[doc = "Bits 8:10 - RTS Output Active Trigger - When the quantity of receive data in receive FIFO data register (SCFRDR) becomes more than the number shown below, RTS signal is set to high."]
    #[inline]
    pub fn rstrg(&self) -> RSTRGR {
        RSTRGR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 6:7 - Receive FIFO Data Trigger - Set the quantity of receive data which sets the receive data full (RDF) flag in the serial status register (SCFSR). The RDF flag is set to 1 when the quantity of receive data stored in the receive FIFO data register (SCFRDR) is increased more than the set trigger number shown below. Note: In clock synchronous mode, to transfer the receive data using the direct memory access controller, set the receive trigger number to 1. If set to other than 1, CPU must read the receive data left in SCFRDR."]
    #[inline]
    pub fn rtrg(&self) -> RTRGR {
        RTRGR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:5 - Transmit FIFO Data Trigger - Set the quantity of remaining transmit data which sets the transmit FIFO data register empty (TDFE) flag in the serial status register (SCFSR). The TDFE flag is set to 1 when the quantity of transmit data in the transmit FIFO data register (SCFTDR) becomes less than the set trigger number shown below. Note: Values in parentheses mean the number of empty bytes in SCFTDR when the TDFE flag is set to 1."]
    #[inline]
    pub fn ttrg(&self) -> TTRGR {
        TTRGR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 3 - Modem Control Enable - Enables modem control signals CTS and RTS. For channels 0, 2 to 4, and 6 in clock synchronous mode, MCE bit should always be 0."]
    #[inline]
    pub fn mcr(&self) -> MCRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        MCRR { bits }
    }
    #[doc = "Bit 2 - Transmit FIFO Data Register Reset - Disables the transmit data in the transmit FIFO data register and resets the register to the empty state."]
    #[inline]
    pub fn tfrst(&self) -> TFRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TFRSTR { bits }
    }
    #[doc = "Bit 1 - Receive FIFO Data Register Reset - Disables the receive data in the receive FIFO data register and resets the register to the empty state."]
    #[inline]
    pub fn rfrst(&self) -> RFRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RFRSTR { bits }
    }
    #[doc = "Bit 0 - Loop-Back Test - Internally connects the transmit output pin (TxD) and receive input pin (RxD) and internally connects the RTS pin and CTS pin and enables loop-back testing."]
    #[inline]
    pub fn loop_(&self) -> LOOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        LOOPR { bits }
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
    #[doc = "Bits 8:10 - RTS Output Active Trigger - When the quantity of receive data in receive FIFO data register (SCFRDR) becomes more than the number shown below, RTS signal is set to high."]
    #[inline]
    pub fn rstrg(&mut self) -> _RSTRGW {
        _RSTRGW { w: self }
    }
    #[doc = "Bits 6:7 - Receive FIFO Data Trigger - Set the quantity of receive data which sets the receive data full (RDF) flag in the serial status register (SCFSR). The RDF flag is set to 1 when the quantity of receive data stored in the receive FIFO data register (SCFRDR) is increased more than the set trigger number shown below. Note: In clock synchronous mode, to transfer the receive data using the direct memory access controller, set the receive trigger number to 1. If set to other than 1, CPU must read the receive data left in SCFRDR."]
    #[inline]
    pub fn rtrg(&mut self) -> _RTRGW {
        _RTRGW { w: self }
    }
    #[doc = "Bits 4:5 - Transmit FIFO Data Trigger - Set the quantity of remaining transmit data which sets the transmit FIFO data register empty (TDFE) flag in the serial status register (SCFSR). The TDFE flag is set to 1 when the quantity of transmit data in the transmit FIFO data register (SCFTDR) becomes less than the set trigger number shown below. Note: Values in parentheses mean the number of empty bytes in SCFTDR when the TDFE flag is set to 1."]
    #[inline]
    pub fn ttrg(&mut self) -> _TTRGW {
        _TTRGW { w: self }
    }
    #[doc = "Bit 3 - Modem Control Enable - Enables modem control signals CTS and RTS. For channels 0, 2 to 4, and 6 in clock synchronous mode, MCE bit should always be 0."]
    #[inline]
    pub fn mcr(&mut self) -> _MCRW {
        _MCRW { w: self }
    }
    #[doc = "Bit 2 - Transmit FIFO Data Register Reset - Disables the transmit data in the transmit FIFO data register and resets the register to the empty state."]
    #[inline]
    pub fn tfrst(&mut self) -> _TFRSTW {
        _TFRSTW { w: self }
    }
    #[doc = "Bit 1 - Receive FIFO Data Register Reset - Disables the receive data in the receive FIFO data register and resets the register to the empty state."]
    #[inline]
    pub fn rfrst(&mut self) -> _RFRSTW {
        _RFRSTW { w: self }
    }
    #[doc = "Bit 0 - Loop-Back Test - Internally connects the transmit output pin (TxD) and receive input pin (RxD) and internally connects the RTS pin and CTS pin and enables loop-back testing."]
    #[inline]
    pub fn loop_(&mut self) -> _LOOPW {
        _LOOPW { w: self }
    }
}
