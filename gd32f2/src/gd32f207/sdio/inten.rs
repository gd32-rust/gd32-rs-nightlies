#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCRCERRIE` reader - Command response CRC fail interrupt enable"]
pub type CCRCERRIE_R = crate::BitReader<bool>;
#[doc = "Field `CCRCERRIE` writer - Command response CRC fail interrupt enable"]
pub type CCRCERRIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 0>;
#[doc = "Field `DTCRCERRIE` reader - Data CRC fail interrupt enable"]
pub type DTCRCERRIE_R = crate::BitReader<bool>;
#[doc = "Field `DTCRCERRIE` writer - Data CRC fail interrupt enable"]
pub type DTCRCERRIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 1>;
#[doc = "Field `CMDTMOUTIE` reader - Command response timeout interrupt enable"]
pub type CMDTMOUTIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDTMOUTIE` writer - Command response timeout interrupt enable"]
pub type CMDTMOUTIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 2>;
#[doc = "Field `DTTMOUTIE` reader - Data timeout interrupt enable"]
pub type DTTMOUTIE_R = crate::BitReader<bool>;
#[doc = "Field `DTTMOUTIE` writer - Data timeout interrupt enable"]
pub type DTTMOUTIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 3>;
#[doc = "Field `TXUREIE` reader - Transmit FIFO underrun error interrupt enable"]
pub type TXUREIE_R = crate::BitReader<bool>;
#[doc = "Field `TXUREIE` writer - Transmit FIFO underrun error interrupt enable"]
pub type TXUREIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 4>;
#[doc = "Field `RXOREIE` reader - Received FIFO overrun error interrupt enable"]
pub type RXOREIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOREIE` writer - Received FIFO overrun error interrupt enable"]
pub type RXOREIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 5>;
#[doc = "Field `CMDRECVIE` reader - Command response received interrupt enable"]
pub type CMDRECVIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDRECVIE` writer - Command response received interrupt enable"]
pub type CMDRECVIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 6>;
#[doc = "Field `CMDSENDIE` reader - Command sent interrupt enable"]
pub type CMDSENDIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDSENDIE` writer - Command sent interrupt enable"]
pub type CMDSENDIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 7>;
#[doc = "Field `DTENDIE` reader - Data end interrupt enable"]
pub type DTENDIE_R = crate::BitReader<bool>;
#[doc = "Field `DTENDIE` writer - Data end interrupt enable"]
pub type DTENDIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 8>;
#[doc = "Field `STBITEIE` reader - Start bit error interrupt enable"]
pub type STBITEIE_R = crate::BitReader<bool>;
#[doc = "Field `STBITEIE` writer - Start bit error interrupt enable"]
pub type STBITEIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 9>;
#[doc = "Field `DTBLKENDIE` reader - Data block end interrupt enable"]
pub type DTBLKENDIE_R = crate::BitReader<bool>;
#[doc = "Field `DTBLKENDIE` writer - Data block end interrupt enable"]
pub type DTBLKENDIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 10>;
#[doc = "Field `CMDRUNIE` reader - Command transmission interrupt enable"]
pub type CMDRUNIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDRUNIE` writer - Command transmission interrupt enable"]
pub type CMDRUNIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 11>;
#[doc = "Field `TXRUNIE` reader - Data transmission interrupt enable"]
pub type TXRUNIE_R = crate::BitReader<bool>;
#[doc = "Field `TXRUNIE` writer - Data transmission interrupt enable"]
pub type TXRUNIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 12>;
#[doc = "Field `RXRUNIE` reader - Data reception interrupt enable"]
pub type RXRUNIE_R = crate::BitReader<bool>;
#[doc = "Field `RXRUNIE` writer - Data reception interrupt enable"]
pub type RXRUNIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 13>;
#[doc = "Field `TFHIE` reader - Transmit FIFO half empty interrupt enable"]
pub type TFHIE_R = crate::BitReader<bool>;
#[doc = "Field `TFHIE` writer - Transmit FIFO half empty interrupt enable"]
pub type TFHIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 14>;
#[doc = "Field `RFHIE` reader - Receive FIFO half full interrupt enable"]
pub type RFHIE_R = crate::BitReader<bool>;
#[doc = "Field `RFHIE` writer - Receive FIFO half full interrupt enable"]
pub type RFHIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 15>;
#[doc = "Field `TFFIE` reader - Transmit FIFO full interrupt enable"]
pub type TFFIE_R = crate::BitReader<bool>;
#[doc = "Field `TFFIE` writer - Transmit FIFO full interrupt enable"]
pub type TFFIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 16>;
#[doc = "Field `RFFIE` reader - Receive FIFO full interrupt enable"]
pub type RFFIE_R = crate::BitReader<bool>;
#[doc = "Field `RFFIE` writer - Receive FIFO full interrupt enable"]
pub type RFFIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 17>;
#[doc = "Field `TFEIE` reader - Transmit FIFO empty interrupt enable"]
pub type TFEIE_R = crate::BitReader<bool>;
#[doc = "Field `TFEIE` writer - Transmit FIFO empty interrupt enable"]
pub type TFEIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 18>;
#[doc = "Field `RFEIE` reader - Receive FIFO empty interrupt enable"]
pub type RFEIE_R = crate::BitReader<bool>;
#[doc = "Field `RFEIE` writer - Receive FIFO empty interrupt enable"]
pub type RFEIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 19>;
#[doc = "Field `TXDTVALIE` reader - Data valid in transmit FIFO interrupt enable"]
pub type TXDTVALIE_R = crate::BitReader<bool>;
#[doc = "Field `TXDTVALIE` writer - Data valid in transmit FIFO interrupt enable"]
pub type TXDTVALIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 20>;
#[doc = "Field `RXDTVALIE` reader - Data valid in receive FIFO interrupt enable"]
pub type RXDTVALIE_R = crate::BitReader<bool>;
#[doc = "Field `RXDTVALIE` writer - Data valid in receive FIFO interrupt enable"]
pub type RXDTVALIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 21>;
#[doc = "Field `SDIOINTIE` reader - SD I/O interrupt received interrupt enable"]
pub type SDIOINTIE_R = crate::BitReader<bool>;
#[doc = "Field `SDIOINTIE` writer - SD I/O interrupt received interrupt enable"]
pub type SDIOINTIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 22>;
#[doc = "Field `ATAENDIE` reader - CE-ATA command completion signal received interrupt enable"]
pub type ATAENDIE_R = crate::BitReader<bool>;
#[doc = "Field `ATAENDIE` writer - CE-ATA command completion signal received interrupt enable"]
pub type ATAENDIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 0 - Command response CRC fail interrupt enable"]
    #[inline(always)]
    pub fn ccrcerrie(&self) -> CCRCERRIE_R {
        CCRCERRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    pub fn dtcrcerrie(&self) -> DTCRCERRIE_R {
        DTCRCERRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command response timeout interrupt enable"]
    #[inline(always)]
    pub fn cmdtmoutie(&self) -> CMDTMOUTIE_R {
        CMDTMOUTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dttmoutie(&self) -> DTTMOUTIE_R {
        DTTMOUTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error interrupt enable"]
    #[inline(always)]
    pub fn txureie(&self) -> TXUREIE_R {
        TXUREIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxoreie(&self) -> RXOREIE_R {
        RXOREIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    pub fn cmdrecvie(&self) -> CMDRECVIE_R {
        CMDRECVIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    pub fn cmdsendie(&self) -> CMDSENDIE_R {
        CMDSENDIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    pub fn dtendie(&self) -> DTENDIE_R {
        DTENDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn stbiteie(&self) -> STBITEIE_R {
        STBITEIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    pub fn dtblkendie(&self) -> DTBLKENDIE_R {
        DTBLKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command transmission interrupt enable"]
    #[inline(always)]
    pub fn cmdrunie(&self) -> CMDRUNIE_R {
        CMDRUNIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmission interrupt enable"]
    #[inline(always)]
    pub fn txrunie(&self) -> TXRUNIE_R {
        TXRUNIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data reception interrupt enable"]
    #[inline(always)]
    pub fn rxrunie(&self) -> RXRUNIE_R {
        RXRUNIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty interrupt enable"]
    #[inline(always)]
    pub fn tfhie(&self) -> TFHIE_R {
        TFHIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full interrupt enable"]
    #[inline(always)]
    pub fn rfhie(&self) -> RFHIE_R {
        RFHIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO full interrupt enable"]
    #[inline(always)]
    pub fn tffie(&self) -> TFFIE_R {
        TFFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rffie(&self) -> RFFIE_R {
        RFFIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn tfeie(&self) -> TFEIE_R {
        TFEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn rfeie(&self) -> RFEIE_R {
        RFEIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data valid in transmit FIFO interrupt enable"]
    #[inline(always)]
    pub fn txdtvalie(&self) -> TXDTVALIE_R {
        TXDTVALIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data valid in receive FIFO interrupt enable"]
    #[inline(always)]
    pub fn rxdtvalie(&self) -> RXDTVALIE_R {
        RXDTVALIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SD I/O interrupt received interrupt enable"]
    #[inline(always)]
    pub fn sdiointie(&self) -> SDIOINTIE_R {
        SDIOINTIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received interrupt enable"]
    #[inline(always)]
    pub fn ataendie(&self) -> ATAENDIE_R {
        ATAENDIE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command response CRC fail interrupt enable"]
    #[inline(always)]
    pub fn ccrcerrie(&mut self) -> CCRCERRIE_W {
        CCRCERRIE_W::new(self)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    pub fn dtcrcerrie(&mut self) -> DTCRCERRIE_W {
        DTCRCERRIE_W::new(self)
    }
    #[doc = "Bit 2 - Command response timeout interrupt enable"]
    #[inline(always)]
    pub fn cmdtmoutie(&mut self) -> CMDTMOUTIE_W {
        CMDTMOUTIE_W::new(self)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dttmoutie(&mut self) -> DTTMOUTIE_W {
        DTTMOUTIE_W::new(self)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error interrupt enable"]
    #[inline(always)]
    pub fn txureie(&mut self) -> TXUREIE_W {
        TXUREIE_W::new(self)
    }
    #[doc = "Bit 5 - Received FIFO overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxoreie(&mut self) -> RXOREIE_W {
        RXOREIE_W::new(self)
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    pub fn cmdrecvie(&mut self) -> CMDRECVIE_W {
        CMDRECVIE_W::new(self)
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    pub fn cmdsendie(&mut self) -> CMDSENDIE_W {
        CMDSENDIE_W::new(self)
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    pub fn dtendie(&mut self) -> DTENDIE_W {
        DTENDIE_W::new(self)
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn stbiteie(&mut self) -> STBITEIE_W {
        STBITEIE_W::new(self)
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    pub fn dtblkendie(&mut self) -> DTBLKENDIE_W {
        DTBLKENDIE_W::new(self)
    }
    #[doc = "Bit 11 - Command transmission interrupt enable"]
    #[inline(always)]
    pub fn cmdrunie(&mut self) -> CMDRUNIE_W {
        CMDRUNIE_W::new(self)
    }
    #[doc = "Bit 12 - Data transmission interrupt enable"]
    #[inline(always)]
    pub fn txrunie(&mut self) -> TXRUNIE_W {
        TXRUNIE_W::new(self)
    }
    #[doc = "Bit 13 - Data reception interrupt enable"]
    #[inline(always)]
    pub fn rxrunie(&mut self) -> RXRUNIE_W {
        RXRUNIE_W::new(self)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty interrupt enable"]
    #[inline(always)]
    pub fn tfhie(&mut self) -> TFHIE_W {
        TFHIE_W::new(self)
    }
    #[doc = "Bit 15 - Receive FIFO half full interrupt enable"]
    #[inline(always)]
    pub fn rfhie(&mut self) -> RFHIE_W {
        RFHIE_W::new(self)
    }
    #[doc = "Bit 16 - Transmit FIFO full interrupt enable"]
    #[inline(always)]
    pub fn tffie(&mut self) -> TFFIE_W {
        TFFIE_W::new(self)
    }
    #[doc = "Bit 17 - Receive FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rffie(&mut self) -> RFFIE_W {
        RFFIE_W::new(self)
    }
    #[doc = "Bit 18 - Transmit FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn tfeie(&mut self) -> TFEIE_W {
        TFEIE_W::new(self)
    }
    #[doc = "Bit 19 - Receive FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn rfeie(&mut self) -> RFEIE_W {
        RFEIE_W::new(self)
    }
    #[doc = "Bit 20 - Data valid in transmit FIFO interrupt enable"]
    #[inline(always)]
    pub fn txdtvalie(&mut self) -> TXDTVALIE_W {
        TXDTVALIE_W::new(self)
    }
    #[doc = "Bit 21 - Data valid in receive FIFO interrupt enable"]
    #[inline(always)]
    pub fn rxdtvalie(&mut self) -> RXDTVALIE_W {
        RXDTVALIE_W::new(self)
    }
    #[doc = "Bit 22 - SD I/O interrupt received interrupt enable"]
    #[inline(always)]
    pub fn sdiointie(&mut self) -> SDIOINTIE_W {
        SDIOINTIE_W::new(self)
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received interrupt enable"]
    #[inline(always)]
    pub fn ataendie(&mut self) -> ATAENDIE_W {
        ATAENDIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
