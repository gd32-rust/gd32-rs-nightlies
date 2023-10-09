#[doc = "Register `DMA_CTL` reader"]
pub type R = crate::R<DMA_CTL_SPEC>;
#[doc = "Register `DMA_CTL` writer"]
pub type W = crate::W<DMA_CTL_SPEC>;
#[doc = "Field `SRE` reader - Start/stop receive enable"]
pub type SRE_R = crate::BitReader;
#[doc = "Field `SRE` writer - Start/stop receive enable"]
pub type SRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSF` reader - Operate on second frame"]
pub type OSF_R = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on second frame"]
pub type OSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTHC` reader - Receive threshold control"]
pub type RTHC_R = crate::FieldReader;
#[doc = "Field `RTHC` writer - Receive threshold control"]
pub type RTHC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FUF` reader - Forward undersized good frames"]
pub type FUF_R = crate::BitReader;
#[doc = "Field `FUF` writer - Forward undersized good frames"]
pub type FUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FERF` reader - Forward error frames"]
pub type FERF_R = crate::BitReader;
#[doc = "Field `FERF` writer - Forward error frames"]
pub type FERF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STE` reader - Start/stop transmission enable"]
pub type STE_R = crate::BitReader;
#[doc = "Field `STE` writer - Start/stop transmission enable"]
pub type STE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TTHC` reader - Transmit threshold control"]
pub type TTHC_R = crate::FieldReader;
#[doc = "Field `TTHC` writer - Transmit threshold control"]
pub type TTHC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `FTF` reader - Flush transmit FIFO"]
pub type FTF_R = crate::BitReader;
#[doc = "Field `FTF` writer - Flush transmit FIFO"]
pub type FTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSFD` reader - Transmit Store-and-Forward"]
pub type TSFD_R = crate::BitReader;
#[doc = "Field `TSFD` writer - Transmit Store-and-Forward"]
pub type TSFD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAFRF` reader - Disable flushing of received frames"]
pub type DAFRF_R = crate::BitReader;
#[doc = "Field `DAFRF` writer - Disable flushing of received frames"]
pub type DAFRF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSFD` reader - Receive Store-and-Forward"]
pub type RSFD_R = crate::BitReader;
#[doc = "Field `RSFD` writer - Receive Store-and-Forward"]
pub type RSFD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTCERFD` reader - Dropping of TCP/IP checksum error frames disable"]
pub type DTCERFD_R = crate::BitReader;
#[doc = "Field `DTCERFD` writer - Dropping of TCP/IP checksum error frames disable"]
pub type DTCERFD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Start/stop receive enable"]
    #[inline(always)]
    pub fn sre(&self) -> SRE_R {
        SRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rthc(&self) -> RTHC_R {
        RTHC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fuf(&self) -> FUF_R {
        FUF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn ferf(&self) -> FERF_R {
        FERF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Start/stop transmission enable"]
    #[inline(always)]
    pub fn ste(&self) -> STE_R {
        STE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn tthc(&self) -> TTHC_R {
        TTHC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Store-and-Forward"]
    #[inline(always)]
    pub fn tsfd(&self) -> TSFD_R {
        TSFD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dafrf(&self) -> DAFRF_R {
        DAFRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Store-and-Forward"]
    #[inline(always)]
    pub fn rsfd(&self) -> RSFD_R {
        RSFD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Dropping of TCP/IP checksum error frames disable"]
    #[inline(always)]
    pub fn dtcerfd(&self) -> DTCERFD_R {
        DTCERFD_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start/stop receive enable"]
    #[inline(always)]
    #[must_use]
    pub fn sre(&mut self) -> SRE_W<DMA_CTL_SPEC, 1> {
        SRE_W::new(self)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<DMA_CTL_SPEC, 2> {
        OSF_W::new(self)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    #[must_use]
    pub fn rthc(&mut self) -> RTHC_W<DMA_CTL_SPEC, 3> {
        RTHC_W::new(self)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    #[must_use]
    pub fn fuf(&mut self) -> FUF_W<DMA_CTL_SPEC, 6> {
        FUF_W::new(self)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    #[must_use]
    pub fn ferf(&mut self) -> FERF_W<DMA_CTL_SPEC, 7> {
        FERF_W::new(self)
    }
    #[doc = "Bit 13 - Start/stop transmission enable"]
    #[inline(always)]
    #[must_use]
    pub fn ste(&mut self) -> STE_W<DMA_CTL_SPEC, 13> {
        STE_W::new(self)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    #[must_use]
    pub fn tthc(&mut self) -> TTHC_W<DMA_CTL_SPEC, 14> {
        TTHC_W::new(self)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<DMA_CTL_SPEC, 20> {
        FTF_W::new(self)
    }
    #[doc = "Bit 21 - Transmit Store-and-Forward"]
    #[inline(always)]
    #[must_use]
    pub fn tsfd(&mut self) -> TSFD_W<DMA_CTL_SPEC, 21> {
        TSFD_W::new(self)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    #[must_use]
    pub fn dafrf(&mut self) -> DAFRF_W<DMA_CTL_SPEC, 24> {
        DAFRF_W::new(self)
    }
    #[doc = "Bit 25 - Receive Store-and-Forward"]
    #[inline(always)]
    #[must_use]
    pub fn rsfd(&mut self) -> RSFD_W<DMA_CTL_SPEC, 25> {
        RSFD_W::new(self)
    }
    #[doc = "Bit 26 - Dropping of TCP/IP checksum error frames disable"]
    #[inline(always)]
    #[must_use]
    pub fn dtcerfd(&mut self) -> DTCERFD_W<DMA_CTL_SPEC, 26> {
        DTCERFD_W::new(self)
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
#[doc = "Ethernet DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CTL_SPEC;
impl crate::RegisterSpec for DMA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ctl::R`](R) reader structure"]
impl crate::Readable for DMA_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_ctl::W`](W) writer structure"]
impl crate::Writable for DMA_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_CTL to value 0"]
impl crate::Resettable for DMA_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
