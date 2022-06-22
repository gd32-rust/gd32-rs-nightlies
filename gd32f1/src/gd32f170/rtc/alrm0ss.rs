#[doc = "Register `ALRM0SS` reader"]
pub struct R(crate::R<ALRM0SS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRM0SS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRM0SS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRM0SS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALRM0SS` writer"]
pub struct W(crate::W<ALRM0SS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRM0SS_SPEC>;
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
impl From<crate::W<ALRM0SS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRM0SS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSKSSC` reader - Mask control bit of SSC"]
pub type MSKSSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSKSSC` writer - Mask control bit of SSC"]
pub type MSKSSC_W<'a> = crate::FieldWriter<'a, u32, ALRM0SS_SPEC, u8, u8, 4, 24>;
#[doc = "Field `SSC` reader - Alarm sub second value"]
pub type SSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SSC` writer - Alarm sub second value"]
pub type SSC_W<'a> = crate::FieldWriter<'a, u32, ALRM0SS_SPEC, u16, u16, 15, 0>;
impl R {
    #[doc = "Bits 24:27 - Mask control bit of SSC"]
    #[inline(always)]
    pub fn mskssc(&self) -> MSKSSC_R {
        MSKSSC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:14 - Alarm sub second value"]
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:27 - Mask control bit of SSC"]
    #[inline(always)]
    pub fn mskssc(&mut self) -> MSKSSC_W {
        MSKSSC_W::new(self)
    }
    #[doc = "Bits 0:14 - Alarm sub second value"]
    #[inline(always)]
    pub fn ssc(&mut self) -> SSC_W {
        SSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm 0 sub second register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrm0ss](index.html) module"]
pub struct ALRM0SS_SPEC;
impl crate::RegisterSpec for ALRM0SS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alrm0ss::R](R) reader structure"]
impl crate::Readable for ALRM0SS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alrm0ss::W](W) writer structure"]
impl crate::Writable for ALRM0SS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALRM0SS to value 0"]
impl crate::Resettable for ALRM0SS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
