#[doc = "Register `STATC` reader"]
pub struct R(crate::R<STATC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATC` writer"]
pub struct W(crate::W<STATC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATC_SPEC>;
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
impl From<crate::W<STATC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFC` reader - Start of frame flag clear"]
pub type SOFC_R = crate::BitReader<bool>;
#[doc = "Field `SOFC` writer - Start of frame flag clear"]
pub type SOFC_W<'a> = crate::BitWriter<'a, u32, STATC_SPEC, bool, 1>;
#[doc = "Field `UPDC` reader - SLCD data update done clear bit"]
pub type UPDC_R = crate::BitReader<bool>;
#[doc = "Field `UPDC` writer - SLCD data update done clear bit"]
pub type UPDC_W<'a> = crate::BitWriter<'a, u32, STATC_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 1 - Start of frame flag clear"]
    #[inline(always)]
    pub fn sofc(&self) -> SOFC_R {
        SOFC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SLCD data update done clear bit"]
    #[inline(always)]
    pub fn updc(&self) -> UPDC_R {
        UPDC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start of frame flag clear"]
    #[inline(always)]
    pub fn sofc(&mut self) -> SOFC_W {
        SOFC_W::new(self)
    }
    #[doc = "Bit 3 - SLCD data update done clear bit"]
    #[inline(always)]
    pub fn updc(&mut self) -> UPDC_W {
        UPDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLCD status flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statc](index.html) module"]
pub struct STATC_SPEC;
impl crate::RegisterSpec for STATC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statc::R](R) reader structure"]
impl crate::Readable for STATC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [statc::W](W) writer structure"]
impl crate::Writable for STATC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATC to value 0"]
impl crate::Resettable for STATC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
