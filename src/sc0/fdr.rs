#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::FDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TR {
    bits: u8,
}
impl TR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RR {
    bits: u8,
}
impl RR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 8:12 - T4 to T0 bits indicate the quantity of non-transmitted data stored in SCFTDR. H'00 means no transmit data, and H'10 means that all transmit data is stored in SCFTDR."]
    #[inline]
    pub fn t(&self) -> TR {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        TR { bits }
    }
    #[doc = "Bits 0:4 - R4 to R0 bits indicate the quantity of receive data stored in SCFRDR. H'00 means no receive data, and H'10 means that all receive data is stored in SCFRDR."]
    #[inline]
    pub fn r(&self) -> RR {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        RR { bits }
    }
}
