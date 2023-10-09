#[doc = "Register `DMA_MFBOCNT` reader"]
pub type R = crate::R<DMA_MFBOCNT_SPEC>;
#[doc = "Field `MSFC` reader - Missed frames by the controller"]
pub type MSFC_R = crate::FieldReader<u16>;
#[doc = "Field `MSFA` reader - Missed frames by the application"]
pub type MSFA_R = crate::FieldReader<u16>;
#[doc = "Field `OBFOC` reader - Overflow bit for FIFO overflow counter bit"]
pub type OBFOC_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Missed frames by the controller"]
    #[inline(always)]
    pub fn msfc(&self) -> MSFC_R {
        MSFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 17:27 - Missed frames by the application"]
    #[inline(always)]
    pub fn msfa(&self) -> MSFA_R {
        MSFA_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflow counter bit"]
    #[inline(always)]
    pub fn obfoc(&self) -> OBFOC_R {
        OBFOC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_mfbocnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_MFBOCNT_SPEC;
impl crate::RegisterSpec for DMA_MFBOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_mfbocnt::R`](R) reader structure"]
impl crate::Readable for DMA_MFBOCNT_SPEC {}
#[doc = "`reset()` method sets DMA_MFBOCNT to value 0"]
impl crate::Resettable for DMA_MFBOCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
