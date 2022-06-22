#[doc = "Register `DATALEN` reader"]
pub struct R(crate::R<DATALEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATALEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATALEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATALEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATALEN` writer"]
pub struct W(crate::W<DATALEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATALEN_SPEC>;
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
impl From<crate::W<DATALEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATALEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATALEN` reader - Data transfer length"]
pub type DATALEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATALEN` writer - Data transfer length"]
pub type DATALEN_W<'a> = crate::FieldWriter<'a, u32, DATALEN_SPEC, u32, u32, 25, 0>;
impl R {
    #[doc = "Bits 0:24 - Data transfer length"]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - Data transfer length"]
    #[inline(always)]
    pub fn datalen(&mut self) -> DATALEN_W {
        DATALEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datalen](index.html) module"]
pub struct DATALEN_SPEC;
impl crate::RegisterSpec for DATALEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datalen::R](R) reader structure"]
impl crate::Readable for DATALEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datalen::W](W) writer structure"]
impl crate::Writable for DATALEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATALEN to value 0"]
impl crate::Resettable for DATALEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
