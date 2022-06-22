#[doc = "Register `DMA_BCTL` reader"]
pub struct R(crate::R<DMA_BCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_BCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_BCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_BCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_BCTL` writer"]
pub struct W(crate::W<DMA_BCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_BCTL_SPEC>;
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
impl From<crate::W<DMA_BCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_BCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWR` reader - Software reset"]
pub type SWR_R = crate::BitReader<bool>;
#[doc = "Field `SWR` writer - Software reset"]
pub type SWR_W<'a> = crate::BitWriter<'a, u32, DMA_BCTL_SPEC, bool, 0>;
#[doc = "Field `DAB` reader - DMA Arbitration"]
pub type DAB_R = crate::BitReader<bool>;
#[doc = "Field `DAB` writer - DMA Arbitration"]
pub type DAB_W<'a> = crate::BitWriter<'a, u32, DMA_BCTL_SPEC, bool, 1>;
#[doc = "Field `DPSL` reader - Descriptor skip length"]
pub type DPSL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPSL` writer - Descriptor skip length"]
pub type DPSL_W<'a> = crate::FieldWriter<'a, u32, DMA_BCTL_SPEC, u8, u8, 5, 2>;
#[doc = "Field `PGBL` reader - Programmable burst length"]
pub type PGBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PGBL` writer - Programmable burst length"]
pub type PGBL_W<'a> = crate::FieldWriter<'a, u32, DMA_BCTL_SPEC, u8, u8, 6, 8>;
#[doc = "Field `RTPR` reader - RxDMA and TxDMA transfer priority ratio"]
pub type RTPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTPR` writer - RxDMA and TxDMA transfer priority ratio"]
pub type RTPR_W<'a> = crate::FieldWriter<'a, u32, DMA_BCTL_SPEC, u8, u8, 2, 14>;
#[doc = "Field `FB` reader - Fixed burst"]
pub type FB_R = crate::BitReader<bool>;
#[doc = "Field `FB` writer - Fixed burst"]
pub type FB_W<'a> = crate::BitWriter<'a, u32, DMA_BCTL_SPEC, bool, 16>;
#[doc = "Field `RXDP` reader - Rx DMA PGBL"]
pub type RXDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXDP` writer - Rx DMA PGBL"]
pub type RXDP_W<'a> = crate::FieldWriter<'a, u32, DMA_BCTL_SPEC, u8, u8, 6, 17>;
#[doc = "Field `UIP` reader - Use independent PGBL"]
pub type UIP_R = crate::BitReader<bool>;
#[doc = "Field `UIP` writer - Use independent PGBL"]
pub type UIP_W<'a> = crate::BitWriter<'a, u32, DMA_BCTL_SPEC, bool, 23>;
#[doc = "Field `FPBL` reader - Four times PGBL mode"]
pub type FPBL_R = crate::BitReader<bool>;
#[doc = "Field `FPBL` writer - Four times PGBL mode"]
pub type FPBL_W<'a> = crate::BitWriter<'a, u32, DMA_BCTL_SPEC, bool, 24>;
#[doc = "Field `AA` reader - Address-aligned"]
pub type AA_R = crate::BitReader<bool>;
#[doc = "Field `AA` writer - Address-aligned"]
pub type AA_W<'a> = crate::BitWriter<'a, u32, DMA_BCTL_SPEC, bool, 25>;
impl R {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn dab(&self) -> DAB_R {
        DAB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dpsl(&self) -> DPSL_R {
        DPSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pgbl(&self) -> PGBL_R {
        PGBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - RxDMA and TxDMA transfer priority ratio"]
    #[inline(always)]
    pub fn rtpr(&self) -> RTPR_R {
        RTPR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PGBL"]
    #[inline(always)]
    pub fn rxdp(&self) -> RXDP_R {
        RXDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use independent PGBL"]
    #[inline(always)]
    pub fn uip(&self) -> UIP_R {
        UIP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Four times PGBL mode"]
    #[inline(always)]
    pub fn fpbl(&self) -> FPBL_R {
        FPBL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address-aligned"]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W::new(self)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn dab(&mut self) -> DAB_W {
        DAB_W::new(self)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dpsl(&mut self) -> DPSL_W {
        DPSL_W::new(self)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pgbl(&mut self) -> PGBL_W {
        PGBL_W::new(self)
    }
    #[doc = "Bits 14:15 - RxDMA and TxDMA transfer priority ratio"]
    #[inline(always)]
    pub fn rtpr(&mut self) -> RTPR_W {
        RTPR_W::new(self)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W {
        FB_W::new(self)
    }
    #[doc = "Bits 17:22 - Rx DMA PGBL"]
    #[inline(always)]
    pub fn rxdp(&mut self) -> RXDP_W {
        RXDP_W::new(self)
    }
    #[doc = "Bit 23 - Use independent PGBL"]
    #[inline(always)]
    pub fn uip(&mut self) -> UIP_W {
        UIP_W::new(self)
    }
    #[doc = "Bit 24 - Four times PGBL mode"]
    #[inline(always)]
    pub fn fpbl(&mut self) -> FPBL_W {
        FPBL_W::new(self)
    }
    #[doc = "Bit 25 - Address-aligned"]
    #[inline(always)]
    pub fn aa(&mut self) -> AA_W {
        AA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA bus control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_bctl](index.html) module"]
pub struct DMA_BCTL_SPEC;
impl crate::RegisterSpec for DMA_BCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_bctl::R](R) reader structure"]
impl crate::Readable for DMA_BCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_bctl::W](W) writer structure"]
impl crate::Writable for DMA_BCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_BCTL to value 0x2101"]
impl crate::Resettable for DMA_BCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2101
    }
}
