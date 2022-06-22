#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmitting On-going Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRANS_A {
    #[doc = "0: SPI or I2S is idle"]
    IDLE = 0,
    #[doc = "1: SPI or I2S is currently transmitting or receiving"]
    BUSY = 1,
}
impl From<TRANS_A> for bool {
    #[inline(always)]
    fn from(variant: TRANS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRANS` reader - Transmitting On-going Bit"]
pub type TRANS_R = crate::BitReader<TRANS_A>;
impl TRANS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRANS_A {
        match self.bits {
            false => TRANS_A::IDLE,
            true => TRANS_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TRANS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TRANS_A::BUSY
    }
}
#[doc = "Reception Overrun Error Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXORERR_A {
    #[doc = "0: No overrun occurred"]
    NOOVERRUN = 0,
    #[doc = "1: Overrun occurred"]
    OVERRUN = 1,
}
impl From<RXORERR_A> for bool {
    #[inline(always)]
    fn from(variant: RXORERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXORERR` reader - Reception Overrun Error Bit"]
pub type RXORERR_R = crate::BitReader<RXORERR_A>;
impl RXORERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXORERR_A {
        match self.bits {
            false => RXORERR_A::NOOVERRUN,
            true => RXORERR_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == RXORERR_A::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == RXORERR_A::OVERRUN
    }
}
#[doc = "SPI Configuration error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONFERR_A {
    #[doc = "0: No configuration fault occurred"]
    NOFAULT = 0,
    #[doc = "1: Configuration fault occurred"]
    FAULT = 1,
}
impl From<CONFERR_A> for bool {
    #[inline(always)]
    fn from(variant: CONFERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONFERR` reader - SPI Configuration error"]
pub type CONFERR_R = crate::BitReader<CONFERR_A>;
impl CONFERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONFERR_A {
        match self.bits {
            false => CONFERR_A::NOFAULT,
            true => CONFERR_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == CONFERR_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == CONFERR_A::FAULT
    }
}
#[doc = "SPI CRC Error Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERR_A {
    #[doc = "0: CRC value received matches the SPIx_RXCRCR value"]
    MATCH = 0,
    #[doc = "1: CRC value received does not match the SPIx_RXCRCR value"]
    NOMATCH = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR` reader - SPI CRC Error Bit"]
pub type CRCERR_R = crate::BitReader<CRCERR_A>;
impl CRCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::MATCH,
            true => CRCERR_A::NOMATCH,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CRCERR_A::MATCH
    }
    #[doc = "Checks if the value of the field is `NOMATCH`"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CRCERR_A::NOMATCH
    }
}
#[doc = "SPI CRC Error Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERR_AW {
    #[doc = "0: Clear CRC error"]
    CLEAR = 0,
}
impl From<CRCERR_AW> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR` writer - SPI CRC Error Bit"]
pub type CRCERR_W<'a> = crate::BitWriter<'a, u16, STAT_SPEC, CRCERR_AW, 4>;
impl<'a> CRCERR_W<'a> {
    #[doc = "Clear CRC error"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CRCERR_AW::CLEAR)
    }
}
#[doc = "Transmission underrun error bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXURERR_A {
    #[doc = "0: No underrun occurred"]
    NOUNDERRUN = 0,
    #[doc = "1: Underrun occurred"]
    UNDERRUN = 1,
}
impl From<TXURERR_A> for bool {
    #[inline(always)]
    fn from(variant: TXURERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXURERR` reader - Transmission underrun error bit"]
pub type TXURERR_R = crate::BitReader<TXURERR_A>;
impl TXURERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXURERR_A {
        match self.bits {
            false => TXURERR_A::NOUNDERRUN,
            true => TXURERR_A::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOUNDERRUN`"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == TXURERR_A::NOUNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == TXURERR_A::UNDERRUN
    }
}
#[doc = "I2S channel side\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SCH_A {
    #[doc = "0: Channel left has to be transmitted or has been received"]
    LEFT = 0,
    #[doc = "1: Channel right has to be transmitted or has been received"]
    RIGHT = 1,
}
impl From<I2SCH_A> for bool {
    #[inline(always)]
    fn from(variant: I2SCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SCH` reader - I2S channel side"]
pub type I2SCH_R = crate::BitReader<I2SCH_A>;
impl I2SCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SCH_A {
        match self.bits {
            false => I2SCH_A::LEFT,
            true => I2SCH_A::RIGHT,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == I2SCH_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == I2SCH_A::RIGHT
    }
}
#[doc = "Transmit Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBE_A {
    #[doc = "0: Tx buffer not empty"]
    NOTEMPTY = 0,
    #[doc = "1: Tx buffer empty"]
    EMPTY = 1,
}
impl From<TBE_A> for bool {
    #[inline(always)]
    fn from(variant: TBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBE` reader - Transmit Buffer Empty"]
pub type TBE_R = crate::BitReader<TBE_A>;
impl TBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBE_A {
        match self.bits {
            false => TBE_A::NOTEMPTY,
            true => TBE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TBE_A::NOTEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TBE_A::EMPTY
    }
}
#[doc = "Receive Buffer Not Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBNE_A {
    #[doc = "0: Rx buffer empty"]
    EMPTY = 0,
    #[doc = "1: Rx buffer not empty"]
    NOTEMPTY = 1,
}
impl From<RBNE_A> for bool {
    #[inline(always)]
    fn from(variant: RBNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBNE` reader - Receive Buffer Not Empty"]
pub type RBNE_R = crate::BitReader<RBNE_A>;
impl RBNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBNE_A {
        match self.bits {
            false => RBNE_A::EMPTY,
            true => RBNE_A::NOTEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RBNE_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RBNE_A::NOTEMPTY
    }
}
impl R {
    #[doc = "Bit 7 - Transmitting On-going Bit"]
    #[inline(always)]
    pub fn trans(&self) -> TRANS_R {
        TRANS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Reception Overrun Error Bit"]
    #[inline(always)]
    pub fn rxorerr(&self) -> RXORERR_R {
        RXORERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Configuration error"]
    #[inline(always)]
    pub fn conferr(&self) -> CONFERR_R {
        CONFERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI CRC Error Bit"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission underrun error bit"]
    #[inline(always)]
    pub fn txurerr(&self) -> TXURERR_R {
        TXURERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - I2S channel side"]
    #[inline(always)]
    pub fn i2sch(&self) -> I2SCH_R {
        I2SCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Buffer Empty"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Receive Buffer Not Empty"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - SPI CRC Error Bit"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W {
        CRCERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0x02"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
