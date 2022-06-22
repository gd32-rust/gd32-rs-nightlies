#[doc = "Register `DMA_CTL` reader"]
pub struct R(crate::R<DMA_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CTL` writer"]
pub struct W(crate::W<DMA_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CTL_SPEC>;
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
impl From<crate::W<DMA_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRE` reader - Start/stop receive enable"]
pub type SRE_R = crate::BitReader<bool>;
#[doc = "Field `SRE` writer - Start/stop receive enable"]
pub type SRE_W<'a> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, bool, 1>;
#[doc = "Field `OSF` reader - Operate on second frame"]
pub type OSF_R = crate::BitReader<bool>;
#[doc = "Field `OSF` writer - Operate on second frame"]
pub type OSF_W<'a> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, bool, 2>;
#[doc = "Field `RTHC` reader - Receive threshold control"]
pub type RTHC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTHC` writer - Receive threshold control"]
pub type RTHC_W<'a> = crate::FieldWriter<'a, u32, DMA_CTL_SPEC, u8, u8, 2, 3>;
#[doc = "Field `FUF` reader - Forward undersized good frames"]
pub type FUF_R = crate::BitReader<bool>;
#[doc = "Field `FUF` writer - Forward undersized good frames"]
pub type FUF_W<'a> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, bool, 6>;
#[doc = "Field `FERF` reader - Forward error frames"]
pub type FERF_R = crate::BitReader<bool>;
#[doc = "Field `FERF` writer - Forward error frames"]
pub type FERF_W<'a> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, bool, 7>;
#[doc = "Field `STE` reader - Start/stop transmission enable"]
pub type STE_R = crate::BitReader<bool>;
#[doc = "Field `STE` writer - Start/stop transmission enable"]
pub type STE_W<'a> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, bool, 13>;
#[doc = "Field `TTHC` reader - Transmit threshold control"]
pub type TTHC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTHC` writer - Transmit threshold control"]
pub type TTHC_W<'a> = crate::FieldWriter<'a, u32, DMA_CTL_SPEC, u8, u8, 3, 14>;
#[doc = "Field `FTF` reader - Flush transmit FIFO"]
pub type FTF_R = crate::BitReader<bool>;
#[doc = "Field `FTF` writer - Flush transmit FIFO"]
pub type FTF_W<'a> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, bool, 20>;
#[doc = "Field `TSFD` reader - Transmit Store-and-Forward"]
pub type TSFD_R = crate::BitReader<bool>;
#[doc = "Field `TSFD` writer - Transmit Store-and-Forward"]
pub type TSFD_W<'a> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, bool, 21>;
#[doc = "Field `DAFRF` reader - Disable flushing of received frames"]
pub type DAFRF_R = crate::BitReader<bool>;
#[doc = "Field `DAFRF` writer - Disable flushing of received frames"]
pub type DAFRF_W<'a> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, bool, 24>;
#[doc = "Field `RSFD` reader - Receive Store-and-Forward"]
pub type RSFD_R = crate::BitReader<bool>;
#[doc = "Field `RSFD` writer - Receive Store-and-Forward"]
pub type RSFD_W<'a> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, bool, 25>;
#[doc = "Field `DTCERFD` reader - Dropping of TCP/IP checksum error frames disable"]
pub type DTCERFD_R = crate::BitReader<bool>;
#[doc = "Field `DTCERFD` writer - Dropping of TCP/IP checksum error frames disable"]
pub type DTCERFD_W<'a> = crate::BitWriter<'a, u32, DMA_CTL_SPEC, bool, 26>;
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
    pub fn sre(&mut self) -> SRE_W {
        SRE_W::new(self)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W {
        OSF_W::new(self)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rthc(&mut self) -> RTHC_W {
        RTHC_W::new(self)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fuf(&mut self) -> FUF_W {
        FUF_W::new(self)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn ferf(&mut self) -> FERF_W {
        FERF_W::new(self)
    }
    #[doc = "Bit 13 - Start/stop transmission enable"]
    #[inline(always)]
    pub fn ste(&mut self) -> STE_W {
        STE_W::new(self)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn tthc(&mut self) -> TTHC_W {
        TTHC_W::new(self)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W {
        FTF_W::new(self)
    }
    #[doc = "Bit 21 - Transmit Store-and-Forward"]
    #[inline(always)]
    pub fn tsfd(&mut self) -> TSFD_W {
        TSFD_W::new(self)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dafrf(&mut self) -> DAFRF_W {
        DAFRF_W::new(self)
    }
    #[doc = "Bit 25 - Receive Store-and-Forward"]
    #[inline(always)]
    pub fn rsfd(&mut self) -> RSFD_W {
        RSFD_W::new(self)
    }
    #[doc = "Bit 26 - Dropping of TCP/IP checksum error frames disable"]
    #[inline(always)]
    pub fn dtcerfd(&mut self) -> DTCERFD_W {
        DTCERFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ctl](index.html) module"]
pub struct DMA_CTL_SPEC;
impl crate::RegisterSpec for DMA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_ctl::R](R) reader structure"]
impl crate::Readable for DMA_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_ctl::W](W) writer structure"]
impl crate::Writable for DMA_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CTL to value 0"]
impl crate::Resettable for DMA_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
