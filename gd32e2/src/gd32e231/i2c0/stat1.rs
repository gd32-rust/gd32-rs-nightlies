#[doc = "Register `STAT1` reader"]
pub struct R(crate::R<STAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PECV` reader - Packet error checking register"]
pub type PECV_R = crate::FieldReader<u8, u8>;
#[doc = "Dual flag (Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DUMODF` reader - Dual flag (Slave mode)"]
pub type DUMODF_R = crate::BitReader<DUMODF_A>;
impl DUMODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUMODF_A {
        match self.bits {
            false => DUMODF_A::SADDR0,
            true => DUMODF_A::SADDR1,
        }
    }
    #[doc = "Checks if the value of the field is `SADDR0`"]
    #[inline(always)]
    pub fn is_saddr0(&self) -> bool {
        *self == DUMODF_A::SADDR0
    }
    #[doc = "Checks if the value of the field is `SADDR1`"]
    #[inline(always)]
    pub fn is_saddr1(&self) -> bool {
        *self == DUMODF_A::SADDR1
    }
}
#[doc = "SMBus host header (Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSTSMB_A {
    #[doc = "0: No SMBus host header detected"]
    NOHEADER = 0,
    #[doc = "1: SMBus host header detected"]
    HEADER = 1,
}
impl From<HSTSMB_A> for bool {
    #[inline(always)]
    fn from(variant: HSTSMB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSTSMB` reader - SMBus host header (Slave mode)"]
pub type HSTSMB_R = crate::BitReader<HSTSMB_A>;
impl HSTSMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSTSMB_A {
        match self.bits {
            false => HSTSMB_A::NOHEADER,
            true => HSTSMB_A::HEADER,
        }
    }
    #[doc = "Checks if the value of the field is `NOHEADER`"]
    #[inline(always)]
    pub fn is_no_header(&self) -> bool {
        *self == HSTSMB_A::NOHEADER
    }
    #[doc = "Checks if the value of the field is `HEADER`"]
    #[inline(always)]
    pub fn is_header(&self) -> bool {
        *self == HSTSMB_A::HEADER
    }
}
#[doc = "SMBus device default address (Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEFSMB_A {
    #[doc = "0: Default address has not been received"]
    NOTRECEIVED = 0,
    #[doc = "1: Default address has been received"]
    RECEIVED = 1,
}
impl From<DEFSMB_A> for bool {
    #[inline(always)]
    fn from(variant: DEFSMB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEFSMB` reader - SMBus device default address (Slave mode)"]
pub type DEFSMB_R = crate::BitReader<DEFSMB_A>;
impl DEFSMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEFSMB_A {
        match self.bits {
            false => DEFSMB_A::NOTRECEIVED,
            true => DEFSMB_A::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRECEIVED`"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == DEFSMB_A::NOTRECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == DEFSMB_A::RECEIVED
    }
}
#[doc = "General call address (Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXGC_A {
    #[doc = "0: No general call address received"]
    NOTRECEIVED = 0,
    #[doc = "1: General call address received"]
    RECEIVED = 1,
}
impl From<RXGC_A> for bool {
    #[inline(always)]
    fn from(variant: RXGC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXGC` reader - General call address (Slave mode)"]
pub type RXGC_R = crate::BitReader<RXGC_A>;
impl RXGC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXGC_A {
        match self.bits {
            false => RXGC_A::NOTRECEIVED,
            true => RXGC_A::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRECEIVED`"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == RXGC_A::NOTRECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == RXGC_A::RECEIVED
    }
}
#[doc = "Transmitter/receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `TR` reader - Transmitter/receiver"]
pub type TR_R = crate::BitReader<TR_A>;
impl TR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TR_A {
        match self.bits {
            false => TR_A::RECEIVER,
            true => TR_A::TRANSMITTER,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVER`"]
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == TR_A::RECEIVER
    }
    #[doc = "Checks if the value of the field is `TRANSMITTER`"]
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == TR_A::TRANSMITTER
    }
}
#[doc = "Bus busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CBSY_A {
    #[doc = "0: No I2C communication"]
    NOTBUSY = 0,
    #[doc = "1: I2C communication active"]
    BUSY = 1,
}
impl From<I2CBSY_A> for bool {
    #[inline(always)]
    fn from(variant: I2CBSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CBSY` reader - Bus busy"]
pub type I2CBSY_R = crate::BitReader<I2CBSY_A>;
impl I2CBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CBSY_A {
        match self.bits {
            false => I2CBSY_A::NOTBUSY,
            true => I2CBSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == I2CBSY_A::NOTBUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == I2CBSY_A::BUSY
    }
}
#[doc = "Master/slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `MASTER` reader - Master/slave"]
pub type MASTER_R = crate::BitReader<MASTER_A>;
impl MASTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::SLAVE,
            true => MASTER_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MASTER_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MASTER_A::MASTER
    }
}
impl R {
    #[doc = "Bits 8:15 - Packet error checking register"]
    #[inline(always)]
    pub fn pecv(&self) -> PECV_R {
        PECV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - Dual flag (Slave mode)"]
    #[inline(always)]
    pub fn dumodf(&self) -> DUMODF_R {
        DUMODF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus host header (Slave mode)"]
    #[inline(always)]
    pub fn hstsmb(&self) -> HSTSMB_R {
        HSTSMB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus device default address (Slave mode)"]
    #[inline(always)]
    pub fn defsmb(&self) -> DEFSMB_R {
        DEFSMB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - General call address (Slave mode)"]
    #[inline(always)]
    pub fn rxgc(&self) -> RXGC_R {
        RXGC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter/receiver"]
    #[inline(always)]
    pub fn tr(&self) -> TR_R {
        TR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Bus busy"]
    #[inline(always)]
    pub fn i2cbsy(&self) -> I2CBSY_R {
        I2CBSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Master/slave"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Transfer status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat1](index.html) module"]
pub struct STAT1_SPEC;
impl crate::RegisterSpec for STAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat1::R](R) reader structure"]
impl crate::Readable for STAT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT1 to value 0"]
impl crate::Resettable for STAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
