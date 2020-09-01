#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::CPUSTS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ISBUSYR {
    bits: bool,
}
impl ISBUSYR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 4 - State during Changing of the Frequency of CPU and Return from Software Standby - Indicates that the frequency of CPU is being changed or that return from software standby is in progress. Do not execute a WFI instruction while this bit is 1."]
    #[inline]
    pub fn isbusy(&self) -> ISBUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        ISBUSYR { bits }
    }
}
