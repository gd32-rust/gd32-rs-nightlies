#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `CCRCERRIE` reader - Command response CRC fail interrupt enable"]
pub type CCRCERRIE_R = crate::BitReader;
#[doc = "Field `CCRCERRIE` writer - Command response CRC fail interrupt enable"]
pub type CCRCERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTCRCERRIE` reader - Data CRC fail interrupt enable"]
pub type DTCRCERRIE_R = crate::BitReader;
#[doc = "Field `DTCRCERRIE` writer - Data CRC fail interrupt enable"]
pub type DTCRCERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDTMOUTIE` reader - Command response timeout interrupt enable"]
pub type CMDTMOUTIE_R = crate::BitReader;
#[doc = "Field `CMDTMOUTIE` writer - Command response timeout interrupt enable"]
pub type CMDTMOUTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTTMOUTIE` reader - Data timeout interrupt enable"]
pub type DTTMOUTIE_R = crate::BitReader;
#[doc = "Field `DTTMOUTIE` writer - Data timeout interrupt enable"]
pub type DTTMOUTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUREIE` reader - Transmit FIFO underrun error interrupt enable"]
pub type TXUREIE_R = crate::BitReader;
#[doc = "Field `TXUREIE` writer - Transmit FIFO underrun error interrupt enable"]
pub type TXUREIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOREIE` reader - Received FIFO overrun error interrupt enable"]
pub type RXOREIE_R = crate::BitReader;
#[doc = "Field `RXOREIE` writer - Received FIFO overrun error interrupt enable"]
pub type RXOREIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDRECVIE` reader - Command response received interrupt enable"]
pub type CMDRECVIE_R = crate::BitReader;
#[doc = "Field `CMDRECVIE` writer - Command response received interrupt enable"]
pub type CMDRECVIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDSENDIE` reader - Command sent interrupt enable"]
pub type CMDSENDIE_R = crate::BitReader;
#[doc = "Field `CMDSENDIE` writer - Command sent interrupt enable"]
pub type CMDSENDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTENDIE` reader - Data end interrupt enable"]
pub type DTENDIE_R = crate::BitReader;
#[doc = "Field `DTENDIE` writer - Data end interrupt enable"]
pub type DTENDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STBITEIE` reader - Start bit error interrupt enable"]
pub type STBITEIE_R = crate::BitReader;
#[doc = "Field `STBITEIE` writer - Start bit error interrupt enable"]
pub type STBITEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTBLKENDIE` reader - Data block end interrupt enable"]
pub type DTBLKENDIE_R = crate::BitReader;
#[doc = "Field `DTBLKENDIE` writer - Data block end interrupt enable"]
pub type DTBLKENDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDRUNIE` reader - Command transmission interrupt enable"]
pub type CMDRUNIE_R = crate::BitReader;
#[doc = "Field `CMDRUNIE` writer - Command transmission interrupt enable"]
pub type CMDRUNIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXRUNIE` reader - Data transmission interrupt enable"]
pub type TXRUNIE_R = crate::BitReader;
#[doc = "Field `TXRUNIE` writer - Data transmission interrupt enable"]
pub type TXRUNIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXRUNIE` reader - Data reception interrupt enable"]
pub type RXRUNIE_R = crate::BitReader;
#[doc = "Field `RXRUNIE` writer - Data reception interrupt enable"]
pub type RXRUNIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFHIE` reader - Transmit FIFO half empty interrupt enable"]
pub type TFHIE_R = crate::BitReader;
#[doc = "Field `TFHIE` writer - Transmit FIFO half empty interrupt enable"]
pub type TFHIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFHIE` reader - Receive FIFO half full interrupt enable"]
pub type RFHIE_R = crate::BitReader;
#[doc = "Field `RFHIE` writer - Receive FIFO half full interrupt enable"]
pub type RFHIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFFIE` reader - Transmit FIFO full interrupt enable"]
pub type TFFIE_R = crate::BitReader;
#[doc = "Field `TFFIE` writer - Transmit FIFO full interrupt enable"]
pub type TFFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFFIE` reader - Receive FIFO full interrupt enable"]
pub type RFFIE_R = crate::BitReader;
#[doc = "Field `RFFIE` writer - Receive FIFO full interrupt enable"]
pub type RFFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFEIE` reader - Transmit FIFO empty interrupt enable"]
pub type TFEIE_R = crate::BitReader;
#[doc = "Field `TFEIE` writer - Transmit FIFO empty interrupt enable"]
pub type TFEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFEIE` reader - Receive FIFO empty interrupt enable"]
pub type RFEIE_R = crate::BitReader;
#[doc = "Field `RFEIE` writer - Receive FIFO empty interrupt enable"]
pub type RFEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDTVALIE` reader - Data valid in transmit FIFO interrupt enable"]
pub type TXDTVALIE_R = crate::BitReader;
#[doc = "Field `TXDTVALIE` writer - Data valid in transmit FIFO interrupt enable"]
pub type TXDTVALIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXDTVALIE` reader - Data valid in receive FIFO interrupt enable"]
pub type RXDTVALIE_R = crate::BitReader;
#[doc = "Field `RXDTVALIE` writer - Data valid in receive FIFO interrupt enable"]
pub type RXDTVALIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIOINTIE` reader - SD I/O interrupt received interrupt enable"]
pub type SDIOINTIE_R = crate::BitReader;
#[doc = "Field `SDIOINTIE` writer - SD I/O interrupt received interrupt enable"]
pub type SDIOINTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ATAENDIE` reader - CE-ATA command completion signal received interrupt enable"]
pub type ATAENDIE_R = crate::BitReader;
#[doc = "Field `ATAENDIE` writer - CE-ATA command completion signal received interrupt enable"]
pub type ATAENDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[must_use]
    pub fn ccrcerrie(&mut self) -> CCRCERRIE_W<INTEN_SPEC, 0> {
        CCRCERRIE_W::new(self)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtcrcerrie(&mut self) -> DTCRCERRIE_W<INTEN_SPEC, 1> {
        DTCRCERRIE_W::new(self)
    }
    #[doc = "Bit 2 - Command response timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtmoutie(&mut self) -> CMDTMOUTIE_W<INTEN_SPEC, 2> {
        CMDTMOUTIE_W::new(self)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dttmoutie(&mut self) -> DTTMOUTIE_W<INTEN_SPEC, 3> {
        DTTMOUTIE_W::new(self)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txureie(&mut self) -> TXUREIE_W<INTEN_SPEC, 4> {
        TXUREIE_W::new(self)
    }
    #[doc = "Bit 5 - Received FIFO overrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoreie(&mut self) -> RXOREIE_W<INTEN_SPEC, 5> {
        RXOREIE_W::new(self)
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrecvie(&mut self) -> CMDRECVIE_W<INTEN_SPEC, 6> {
        CMDRECVIE_W::new(self)
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsendie(&mut self) -> CMDSENDIE_W<INTEN_SPEC, 7> {
        CMDSENDIE_W::new(self)
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtendie(&mut self) -> DTENDIE_W<INTEN_SPEC, 8> {
        DTENDIE_W::new(self)
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stbiteie(&mut self) -> STBITEIE_W<INTEN_SPEC, 9> {
        STBITEIE_W::new(self)
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtblkendie(&mut self) -> DTBLKENDIE_W<INTEN_SPEC, 10> {
        DTBLKENDIE_W::new(self)
    }
    #[doc = "Bit 11 - Command transmission interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrunie(&mut self) -> CMDRUNIE_W<INTEN_SPEC, 11> {
        CMDRUNIE_W::new(self)
    }
    #[doc = "Bit 12 - Data transmission interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrunie(&mut self) -> TXRUNIE_W<INTEN_SPEC, 12> {
        TXRUNIE_W::new(self)
    }
    #[doc = "Bit 13 - Data reception interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrunie(&mut self) -> RXRUNIE_W<INTEN_SPEC, 13> {
        RXRUNIE_W::new(self)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfhie(&mut self) -> TFHIE_W<INTEN_SPEC, 14> {
        TFHIE_W::new(self)
    }
    #[doc = "Bit 15 - Receive FIFO half full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfhie(&mut self) -> RFHIE_W<INTEN_SPEC, 15> {
        RFHIE_W::new(self)
    }
    #[doc = "Bit 16 - Transmit FIFO full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tffie(&mut self) -> TFFIE_W<INTEN_SPEC, 16> {
        TFFIE_W::new(self)
    }
    #[doc = "Bit 17 - Receive FIFO full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rffie(&mut self) -> RFFIE_W<INTEN_SPEC, 17> {
        RFFIE_W::new(self)
    }
    #[doc = "Bit 18 - Transmit FIFO empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfeie(&mut self) -> TFEIE_W<INTEN_SPEC, 18> {
        TFEIE_W::new(self)
    }
    #[doc = "Bit 19 - Receive FIFO empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfeie(&mut self) -> RFEIE_W<INTEN_SPEC, 19> {
        RFEIE_W::new(self)
    }
    #[doc = "Bit 20 - Data valid in transmit FIFO interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdtvalie(&mut self) -> TXDTVALIE_W<INTEN_SPEC, 20> {
        TXDTVALIE_W::new(self)
    }
    #[doc = "Bit 21 - Data valid in receive FIFO interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdtvalie(&mut self) -> RXDTVALIE_W<INTEN_SPEC, 21> {
        RXDTVALIE_W::new(self)
    }
    #[doc = "Bit 22 - SD I/O interrupt received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdiointie(&mut self) -> SDIOINTIE_W<INTEN_SPEC, 22> {
        SDIOINTIE_W::new(self)
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ataendie(&mut self) -> ATAENDIE_W<INTEN_SPEC, 23> {
        ATAENDIE_W::new(self)
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
#[doc = "Interrupt Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
