#[doc = "Register `STAT1` reader"]
pub type R = crate::R<STAT1_SPEC>;
#[doc = "Field `MASTER` reader - A flag indicating whether I2C block is in master or slave mode"]
pub type MASTER_R = crate::BitReader<MASTER_A>;
#[doc = "A flag indicating whether I2C block is in master or slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER_A {
    #[doc = "0: Slave mode"]
    SLAVE = 0,
    #[doc = "1: Master mode"]
    MASTER = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
impl MASTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::SLAVE,
            true => MASTER_A::MASTER,
        }
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MASTER_A::SLAVE
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MASTER_A::MASTER
    }
}
#[doc = "Field `I2CBSY` reader - Busy flag"]
pub type I2CBSY_R = crate::BitReader<I2CBSY_A>;
#[doc = "Busy flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2CBSY_A {
    #[doc = "0: No I2C communication"]
    NOT_BUSY = 0,
    #[doc = "1: I2C communication active"]
    BUSY = 1,
}
impl From<I2CBSY_A> for bool {
    #[inline(always)]
    fn from(variant: I2CBSY_A) -> Self {
        variant as u8 != 0
    }
}
impl I2CBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CBSY_A {
        match self.bits {
            false => I2CBSY_A::NOT_BUSY,
            true => I2CBSY_A::BUSY,
        }
    }
    #[doc = "No I2C communication"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == I2CBSY_A::NOT_BUSY
    }
    #[doc = "I2C communication active"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == I2CBSY_A::BUSY
    }
}
#[doc = "Field `TR` reader - Whether the I2C is a transmitter or a receiver"]
pub type TR_R = crate::BitReader<TR_A>;
#[doc = "Whether the I2C is a transmitter or a receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR_A {
    #[doc = "0: Receiver"]
    RECEIVER = 0,
    #[doc = "1: Transmitter"]
    TRANSMITTER = 1,
}
impl From<TR_A> for bool {
    #[inline(always)]
    fn from(variant: TR_A) -> Self {
        variant as u8 != 0
    }
}
impl TR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TR_A {
        match self.bits {
            false => TR_A::RECEIVER,
            true => TR_A::TRANSMITTER,
        }
    }
    #[doc = "Receiver"]
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == TR_A::RECEIVER
    }
    #[doc = "Transmitter"]
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == TR_A::TRANSMITTER
    }
}
#[doc = "Field `RXGC` reader - General call address (00h) received"]
pub type RXGC_R = crate::BitReader<RXGC_A>;
#[doc = "General call address (00h) received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXGC_A {
    #[doc = "0: No general call address received"]
    NOT_RECEIVED = 0,
    #[doc = "1: General call address received"]
    RECEIVED = 1,
}
impl From<RXGC_A> for bool {
    #[inline(always)]
    fn from(variant: RXGC_A) -> Self {
        variant as u8 != 0
    }
}
impl RXGC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXGC_A {
        match self.bits {
            false => RXGC_A::NOT_RECEIVED,
            true => RXGC_A::RECEIVED,
        }
    }
    #[doc = "No general call address received"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == RXGC_A::NOT_RECEIVED
    }
    #[doc = "General call address received"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == RXGC_A::RECEIVED
    }
}
#[doc = "Field `DEFSMB` reader - Default address of SMBusDevice"]
pub type DEFSMB_R = crate::BitReader<DEFSMB_A>;
#[doc = "Default address of SMBusDevice\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEFSMB_A {
    #[doc = "0: Default address has not been received"]
    NOT_RECEIVED = 0,
    #[doc = "1: Default address has been received"]
    RECEIVED = 1,
}
impl From<DEFSMB_A> for bool {
    #[inline(always)]
    fn from(variant: DEFSMB_A) -> Self {
        variant as u8 != 0
    }
}
impl DEFSMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEFSMB_A {
        match self.bits {
            false => DEFSMB_A::NOT_RECEIVED,
            true => DEFSMB_A::RECEIVED,
        }
    }
    #[doc = "Default address has not been received"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == DEFSMB_A::NOT_RECEIVED
    }
    #[doc = "Default address has been received"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == DEFSMB_A::RECEIVED
    }
}
#[doc = "Field `HSTSMB` reader - SMBus Host Header detected in slave mode"]
pub type HSTSMB_R = crate::BitReader<HSTSMB_A>;
#[doc = "SMBus Host Header detected in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSTSMB_A {
    #[doc = "0: No SMBus host header detected"]
    NO_HEADER = 0,
    #[doc = "1: SMBus host header detected"]
    HEADER = 1,
}
impl From<HSTSMB_A> for bool {
    #[inline(always)]
    fn from(variant: HSTSMB_A) -> Self {
        variant as u8 != 0
    }
}
impl HSTSMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSTSMB_A {
        match self.bits {
            false => HSTSMB_A::NO_HEADER,
            true => HSTSMB_A::HEADER,
        }
    }
    #[doc = "No SMBus host header detected"]
    #[inline(always)]
    pub fn is_no_header(&self) -> bool {
        *self == HSTSMB_A::NO_HEADER
    }
    #[doc = "SMBus host header detected"]
    #[inline(always)]
    pub fn is_header(&self) -> bool {
        *self == HSTSMB_A::HEADER
    }
}
#[doc = "Field `DUMODF` reader - Dual Flag in slave mode"]
pub type DUMODF_R = crate::BitReader<DUMODF_A>;
#[doc = "Dual Flag in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUMODF_A {
    #[doc = "0: The address matches SADDR0"]
    SADDR0 = 0,
    #[doc = "1: The address matches SADDR1"]
    SADDR1 = 1,
}
impl From<DUMODF_A> for bool {
    #[inline(always)]
    fn from(variant: DUMODF_A) -> Self {
        variant as u8 != 0
    }
}
impl DUMODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUMODF_A {
        match self.bits {
            false => DUMODF_A::SADDR0,
            true => DUMODF_A::SADDR1,
        }
    }
    #[doc = "The address matches SADDR0"]
    #[inline(always)]
    pub fn is_saddr0(&self) -> bool {
        *self == DUMODF_A::SADDR0
    }
    #[doc = "The address matches SADDR1"]
    #[inline(always)]
    pub fn is_saddr1(&self) -> bool {
        *self == DUMODF_A::SADDR1
    }
}
#[doc = "Field `PECV` reader - Packet Error Checking Value that calculated by hardware when PEC is enabled"]
pub type PECV_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - A flag indicating whether I2C block is in master or slave mode"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy flag"]
    #[inline(always)]
    pub fn i2cbsy(&self) -> I2CBSY_R {
        I2CBSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Whether the I2C is a transmitter or a receiver"]
    #[inline(always)]
    pub fn tr(&self) -> TR_R {
        TR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - General call address (00h) received"]
    #[inline(always)]
    pub fn rxgc(&self) -> RXGC_R {
        RXGC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Default address of SMBusDevice"]
    #[inline(always)]
    pub fn defsmb(&self) -> DEFSMB_R {
        DEFSMB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus Host Header detected in slave mode"]
    #[inline(always)]
    pub fn hstsmb(&self) -> HSTSMB_R {
        HSTSMB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dual Flag in slave mode"]
    #[inline(always)]
    pub fn dumodf(&self) -> DUMODF_R {
        DUMODF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Packet Error Checking Value that calculated by hardware when PEC is enabled"]
    #[inline(always)]
    pub fn pecv(&self) -> PECV_R {
        PECV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Transfer status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT1_SPEC;
impl crate::RegisterSpec for STAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat1::R`](R) reader structure"]
impl crate::Readable for STAT1_SPEC {}
#[doc = "`reset()` method sets STAT1 to value 0"]
impl crate::Resettable for STAT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
