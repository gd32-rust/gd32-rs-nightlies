#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `RBNE` reader - Receive Buffer Not Empty"]
pub type RBNE_R = crate::BitReader<RBNE_A>;
#[doc = "Receive Buffer Not Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBNE_A {
    #[doc = "0: Rx buffer empty"]
    EMPTY = 0,
    #[doc = "1: Rx buffer not empty"]
    NOT_EMPTY = 1,
}
impl From<RBNE_A> for bool {
    #[inline(always)]
    fn from(variant: RBNE_A) -> Self {
        variant as u8 != 0
    }
}
impl RBNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBNE_A {
        match self.bits {
            false => RBNE_A::EMPTY,
            true => RBNE_A::NOT_EMPTY,
        }
    }
    #[doc = "Rx buffer empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RBNE_A::EMPTY
    }
    #[doc = "Rx buffer not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RBNE_A::NOT_EMPTY
    }
}
#[doc = "Field `TBE` reader - Transmit Buffer Empty"]
pub type TBE_R = crate::BitReader<TBE_A>;
#[doc = "Transmit Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBE_A {
    #[doc = "0: Tx buffer not empty"]
    NOT_EMPTY = 0,
    #[doc = "1: Tx buffer empty"]
    EMPTY = 1,
}
impl From<TBE_A> for bool {
    #[inline(always)]
    fn from(variant: TBE_A) -> Self {
        variant as u8 != 0
    }
}
impl TBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBE_A {
        match self.bits {
            false => TBE_A::NOT_EMPTY,
            true => TBE_A::EMPTY,
        }
    }
    #[doc = "Tx buffer not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TBE_A::NOT_EMPTY
    }
    #[doc = "Tx buffer empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TBE_A::EMPTY
    }
}
#[doc = "Field `I2SCH` reader - I2S channel side"]
pub type I2SCH_R = crate::BitReader<I2SCH_A>;
#[doc = "I2S channel side\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl I2SCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SCH_A {
        match self.bits {
            false => I2SCH_A::LEFT,
            true => I2SCH_A::RIGHT,
        }
    }
    #[doc = "Channel left has to be transmitted or has been received"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == I2SCH_A::LEFT
    }
    #[doc = "Channel right has to be transmitted or has been received"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == I2SCH_A::RIGHT
    }
}
#[doc = "Field `TXURERR` reader - Transmission underrun error bit"]
pub type TXURERR_R = crate::BitReader<TXURERR_A>;
#[doc = "Transmission underrun error bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXURERR_A {
    #[doc = "0: No underrun occurred"]
    NO_UNDERRUN = 0,
    #[doc = "1: Underrun occurred"]
    UNDERRUN = 1,
}
impl From<TXURERR_A> for bool {
    #[inline(always)]
    fn from(variant: TXURERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TXURERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXURERR_A {
        match self.bits {
            false => TXURERR_A::NO_UNDERRUN,
            true => TXURERR_A::UNDERRUN,
        }
    }
    #[doc = "No underrun occurred"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == TXURERR_A::NO_UNDERRUN
    }
    #[doc = "Underrun occurred"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == TXURERR_A::UNDERRUN
    }
}
#[doc = "Field `CRCERR` reader - SPI CRC Error Bit"]
pub type CRCERR_R = crate::BitReader<CRCERRR_A>;
#[doc = "SPI CRC Error Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERRR_A {
    #[doc = "0: CRC value received matches the SPIx_RXCRCR value"]
    MATCH = 0,
    #[doc = "1: CRC value received does not match the SPIx_RXCRCR value"]
    NO_MATCH = 1,
}
impl From<CRCERRR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERRR_A {
        match self.bits {
            false => CRCERRR_A::MATCH,
            true => CRCERRR_A::NO_MATCH,
        }
    }
    #[doc = "CRC value received matches the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CRCERRR_A::MATCH
    }
    #[doc = "CRC value received does not match the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CRCERRR_A::NO_MATCH
    }
}
#[doc = "SPI CRC Error Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERRW_AW {
    #[doc = "0: Clear CRC error"]
    CLEAR = 0,
}
impl From<CRCERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: CRCERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR` writer - SPI CRC Error Bit"]
pub type CRCERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CRCERRW_AW>;
impl<'a, REG, const O: u8> CRCERR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear CRC error"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CRCERRW_AW::CLEAR)
    }
}
#[doc = "Field `CONFERR` reader - SPI Configuration error"]
pub type CONFERR_R = crate::BitReader<CONFERR_A>;
#[doc = "SPI Configuration error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONFERR_A {
    #[doc = "0: No configuration fault occurred"]
    NO_FAULT = 0,
    #[doc = "1: Configuration fault occurred"]
    FAULT = 1,
}
impl From<CONFERR_A> for bool {
    #[inline(always)]
    fn from(variant: CONFERR_A) -> Self {
        variant as u8 != 0
    }
}
impl CONFERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONFERR_A {
        match self.bits {
            false => CONFERR_A::NO_FAULT,
            true => CONFERR_A::FAULT,
        }
    }
    #[doc = "No configuration fault occurred"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == CONFERR_A::NO_FAULT
    }
    #[doc = "Configuration fault occurred"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == CONFERR_A::FAULT
    }
}
#[doc = "Field `RXORERR` reader - Reception Overrun Error Bit"]
pub type RXORERR_R = crate::BitReader<RXORERR_A>;
#[doc = "Reception Overrun Error Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXORERR_A {
    #[doc = "0: No overrun occurred"]
    NO_OVERRUN = 0,
    #[doc = "1: Overrun occurred"]
    OVERRUN = 1,
}
impl From<RXORERR_A> for bool {
    #[inline(always)]
    fn from(variant: RXORERR_A) -> Self {
        variant as u8 != 0
    }
}
impl RXORERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXORERR_A {
        match self.bits {
            false => RXORERR_A::NO_OVERRUN,
            true => RXORERR_A::OVERRUN,
        }
    }
    #[doc = "No overrun occurred"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == RXORERR_A::NO_OVERRUN
    }
    #[doc = "Overrun occurred"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == RXORERR_A::OVERRUN
    }
}
#[doc = "Field `TRANS` reader - Transmitting On-going Bit"]
pub type TRANS_R = crate::BitReader<TRANS_A>;
#[doc = "Transmitting On-going Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TRANS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRANS_A {
        match self.bits {
            false => TRANS_A::IDLE,
            true => TRANS_A::BUSY,
        }
    }
    #[doc = "SPI or I2S is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TRANS_A::IDLE
    }
    #[doc = "SPI or I2S is currently transmitting or receiving"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TRANS_A::BUSY
    }
}
impl R {
    #[doc = "Bit 0 - Receive Buffer Not Empty"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Buffer Empty"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2S channel side"]
    #[inline(always)]
    pub fn i2sch(&self) -> I2SCH_R {
        I2SCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission underrun error bit"]
    #[inline(always)]
    pub fn txurerr(&self) -> TXURERR_R {
        TXURERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI CRC Error Bit"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Configuration error"]
    #[inline(always)]
    pub fn conferr(&self) -> CONFERR_R {
        CONFERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reception Overrun Error Bit"]
    #[inline(always)]
    pub fn rxorerr(&self) -> RXORERR_R {
        RXORERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitting On-going Bit"]
    #[inline(always)]
    pub fn trans(&self) -> TRANS_R {
        TRANS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - SPI CRC Error Bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CRCERR_W<STAT_SPEC, 4> {
        CRCERR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0x02"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
