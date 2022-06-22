#[doc = "Register `INTC` reader"]
pub struct R(crate::R<INTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTC` writer"]
pub struct W(crate::W<INTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTC_SPEC>;
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
impl From<crate::W<INTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMNERR` reader - Clear max cycle number error"]
pub type CMNERR_R = crate::BitReader<bool>;
#[doc = "Field `CMNERR` writer - Clear max cycle number error"]
pub type CMNERR_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 1>;
#[doc = "Field `CCTCF` reader - Clear charge-transfer complete flag"]
pub type CCTCF_R = crate::BitReader<bool>;
#[doc = "Field `CCTCF` writer - Clear charge-transfer complete flag"]
pub type CCTCF_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 1 - Clear max cycle number error"]
    #[inline(always)]
    pub fn cmnerr(&self) -> CMNERR_R {
        CMNERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Clear charge-transfer complete flag"]
    #[inline(always)]
    pub fn cctcf(&self) -> CCTCF_R {
        CCTCF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clear max cycle number error"]
    #[inline(always)]
    pub fn cmnerr(&mut self) -> CMNERR_W {
        CMNERR_W::new(self)
    }
    #[doc = "Bit 0 - Clear charge-transfer complete flag"]
    #[inline(always)]
    pub fn cctcf(&mut self) -> CCTCF_W {
        CCTCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intc](index.html) module"]
pub struct INTC_SPEC;
impl crate::RegisterSpec for INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intc::R](R) reader structure"]
impl crate::Readable for INTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intc::W](W) writer structure"]
impl crate::Writable for INTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for INTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
