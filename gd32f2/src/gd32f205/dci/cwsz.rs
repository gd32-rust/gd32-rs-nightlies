#[doc = "Register `CWSZ` reader"]
pub struct R(crate::R<CWSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWSZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWSZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWSZ` writer"]
pub struct W(crate::W<CWSZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWSZ_SPEC>;
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
impl From<crate::W<CWSZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWSZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WVSZ` reader - Window Vertical Size"]
pub type WVSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WVSZ` writer - Window Vertical Size"]
pub type WVSZ_W<'a> = crate::FieldWriter<'a, u32, CWSZ_SPEC, u16, u16, 14, 16>;
#[doc = "Field `WHSZ` reader - Window horizontal Size"]
pub type WHSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WHSZ` writer - Window horizontal Size"]
pub type WHSZ_W<'a> = crate::FieldWriter<'a, u32, CWSZ_SPEC, u16, u16, 14, 0>;
impl R {
    #[doc = "Bits 16:29 - Window Vertical Size"]
    #[inline(always)]
    pub fn wvsz(&self) -> WVSZ_R {
        WVSZ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 0:13 - Window horizontal Size"]
    #[inline(always)]
    pub fn whsz(&self) -> WHSZ_R {
        WHSZ_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:29 - Window Vertical Size"]
    #[inline(always)]
    pub fn wvsz(&mut self) -> WVSZ_W {
        WVSZ_W::new(self)
    }
    #[doc = "Bits 0:13 - Window horizontal Size"]
    #[inline(always)]
    pub fn whsz(&mut self) -> WHSZ_W {
        WHSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCI Cropping window size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwsz](index.html) module"]
pub struct CWSZ_SPEC;
impl crate::RegisterSpec for CWSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwsz::R](R) reader structure"]
impl crate::Readable for CWSZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwsz::W](W) writer structure"]
impl crate::Writable for CWSZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CWSZ to value 0"]
impl crate::Resettable for CWSZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
