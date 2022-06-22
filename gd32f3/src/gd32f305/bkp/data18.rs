#[doc = "Register `DATA18` reader"]
pub struct R(crate::R<DATA18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA18` writer"]
pub struct W(crate::W<DATA18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA18_SPEC>;
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
impl From<crate::W<DATA18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Backup data"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - Backup data"]
pub type DATA_W<'a> = crate::FieldWriter<'a, u32, DATA18_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup data register 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data18](index.html) module"]
pub struct DATA18_SPEC;
impl crate::RegisterSpec for DATA18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data18::R](R) reader structure"]
impl crate::Readable for DATA18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data18::W](W) writer structure"]
impl crate::Writable for DATA18_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA18 to value 0"]
impl crate::Resettable for DATA18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
