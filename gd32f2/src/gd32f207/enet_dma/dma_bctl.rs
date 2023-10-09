#[doc = "Register `DMA_BCTL` reader"]
pub type R = crate::R<DMA_BCTL_SPEC>;
#[doc = "Register `DMA_BCTL` writer"]
pub type W = crate::W<DMA_BCTL_SPEC>;
#[doc = "Field `SWR` reader - Software reset"]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - Software reset"]
pub type SWR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAB` reader - DMA Arbitration"]
pub type DAB_R = crate::BitReader;
#[doc = "Field `DAB` writer - DMA Arbitration"]
pub type DAB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPSL` reader - Descriptor skip length"]
pub type DPSL_R = crate::FieldReader;
#[doc = "Field `DPSL` writer - Descriptor skip length"]
pub type DPSL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PGBL` reader - Programmable burst length"]
pub type PGBL_R = crate::FieldReader;
#[doc = "Field `PGBL` writer - Programmable burst length"]
pub type PGBL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `RTPR` reader - RxDMA and TxDMA transfer priority ratio"]
pub type RTPR_R = crate::FieldReader;
#[doc = "Field `RTPR` writer - RxDMA and TxDMA transfer priority ratio"]
pub type RTPR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FB` reader - Fixed burst"]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed burst"]
pub type FB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXDP` reader - Rx DMA PGBL"]
pub type RXDP_R = crate::FieldReader;
#[doc = "Field `RXDP` writer - Rx DMA PGBL"]
pub type RXDP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `UIP` reader - Use independent PGBL"]
pub type UIP_R = crate::BitReader;
#[doc = "Field `UIP` writer - Use independent PGBL"]
pub type UIP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPBL` reader - Four times PGBL mode"]
pub type FPBL_R = crate::BitReader;
#[doc = "Field `FPBL` writer - Four times PGBL mode"]
pub type FPBL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AA` reader - Address-aligned"]
pub type AA_R = crate::BitReader;
#[doc = "Field `AA` writer - Address-aligned"]
pub type AA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<DMA_BCTL_SPEC, 0> {
        SWR_W::new(self)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn dab(&mut self) -> DAB_W<DMA_BCTL_SPEC, 1> {
        DAB_W::new(self)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    #[must_use]
    pub fn dpsl(&mut self) -> DPSL_W<DMA_BCTL_SPEC, 2> {
        DPSL_W::new(self)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    #[must_use]
    pub fn pgbl(&mut self) -> PGBL_W<DMA_BCTL_SPEC, 8> {
        PGBL_W::new(self)
    }
    #[doc = "Bits 14:15 - RxDMA and TxDMA transfer priority ratio"]
    #[inline(always)]
    #[must_use]
    pub fn rtpr(&mut self) -> RTPR_W<DMA_BCTL_SPEC, 14> {
        RTPR_W::new(self)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<DMA_BCTL_SPEC, 16> {
        FB_W::new(self)
    }
    #[doc = "Bits 17:22 - Rx DMA PGBL"]
    #[inline(always)]
    #[must_use]
    pub fn rxdp(&mut self) -> RXDP_W<DMA_BCTL_SPEC, 17> {
        RXDP_W::new(self)
    }
    #[doc = "Bit 23 - Use independent PGBL"]
    #[inline(always)]
    #[must_use]
    pub fn uip(&mut self) -> UIP_W<DMA_BCTL_SPEC, 23> {
        UIP_W::new(self)
    }
    #[doc = "Bit 24 - Four times PGBL mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpbl(&mut self) -> FPBL_W<DMA_BCTL_SPEC, 24> {
        FPBL_W::new(self)
    }
    #[doc = "Bit 25 - Address-aligned"]
    #[inline(always)]
    #[must_use]
    pub fn aa(&mut self) -> AA_W<DMA_BCTL_SPEC, 25> {
        AA_W::new(self)
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
#[doc = "Ethernet DMA bus control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_bctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_bctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_BCTL_SPEC;
impl crate::RegisterSpec for DMA_BCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_bctl::R`](R) reader structure"]
impl crate::Readable for DMA_BCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_bctl::W`](W) writer structure"]
impl crate::Writable for DMA_BCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_BCTL to value 0x2101"]
impl crate::Resettable for DMA_BCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2101;
}
