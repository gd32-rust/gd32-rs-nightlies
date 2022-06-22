#[doc = "Register `DI` reader"]
pub struct R(crate::R<DI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DI` writer"]
pub struct W(crate::W<DI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DI_SPEC>;
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
impl From<crate::W<DI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DI` reader - Message data input"]
pub type DI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DI` writer - Message data input"]
pub type DI_W<'a> = crate::FieldWriter<'a, u32, DI_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Message data input"]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message data input"]
    #[inline(always)]
    pub fn di(&mut self) -> DI_W {
        DI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HAU data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [di](index.html) module"]
pub struct DI_SPEC;
impl crate::RegisterSpec for DI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [di::R](R) reader structure"]
impl crate::Readable for DI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [di::W](W) writer structure"]
impl crate::Writable for DI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DI to value 0"]
impl crate::Resettable for DI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
