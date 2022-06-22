#[doc = "Register `DMAEN` reader"]
pub struct R(crate::R<DMAEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAEN` writer"]
pub struct W(crate::W<DMAEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAEN_SPEC>;
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
impl From<crate::W<DMAEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAOEN` reader - Out FIFO DMA enable"]
pub type DMAOEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAOEN` writer - Out FIFO DMA enable"]
pub type DMAOEN_W<'a> = crate::BitWriter<'a, u32, DMAEN_SPEC, bool, 1>;
#[doc = "Field `DMAIEN` reader - In FIFO DMA enable"]
pub type DMAIEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAIEN` writer - In FIFO DMA enable"]
pub type DMAIEN_W<'a> = crate::BitWriter<'a, u32, DMAEN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 1 - Out FIFO DMA enable"]
    #[inline(always)]
    pub fn dmaoen(&self) -> DMAOEN_R {
        DMAOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - In FIFO DMA enable"]
    #[inline(always)]
    pub fn dmaien(&self) -> DMAIEN_R {
        DMAIEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Out FIFO DMA enable"]
    #[inline(always)]
    pub fn dmaoen(&mut self) -> DMAOEN_W {
        DMAOEN_W::new(self)
    }
    #[doc = "Bit 0 - In FIFO DMA enable"]
    #[inline(always)]
    pub fn dmaien(&mut self) -> DMAIEN_W {
        DMAIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAU DMA enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaen](index.html) module"]
pub struct DMAEN_SPEC;
impl crate::RegisterSpec for DMAEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaen::R](R) reader structure"]
impl crate::Readable for DMAEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaen::W](W) writer structure"]
impl crate::Writable for DMAEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAEN to value 0"]
impl crate::Resettable for DMAEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
