#[doc = "Register `LM` reader"]
pub struct R(crate::R<LM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LM` writer"]
pub struct W(crate::W<LM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LM_SPEC>;
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
impl From<crate::W<LM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LM` reader - Line Mark value"]
pub type LM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LM` writer - Line Mark value"]
pub type LM_W<'a> = crate::FieldWriter<'a, u32, LM_SPEC, u16, u16, 11, 0>;
impl R {
    #[doc = "Bits 0:10 - Line Mark value"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Line Mark value"]
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W {
        LM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line mark register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lm](index.html) module"]
pub struct LM_SPEC;
impl crate::RegisterSpec for LM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lm::R](R) reader structure"]
impl crate::Readable for LM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lm::W](W) writer structure"]
impl crate::Writable for LM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LM to value 0"]
impl crate::Resettable for LM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
