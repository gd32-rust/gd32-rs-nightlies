#[doc = "Register `L0FLLEN` reader"]
pub struct R(crate::R<L0FLLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L0FLLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L0FLLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L0FLLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L0FLLEN` writer"]
pub struct W(crate::W<L0FLLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L0FLLEN_SPEC>;
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
impl From<crate::W<L0FLLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L0FLLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STDOFF` reader - Frame Buffer Stride Offset"]
pub type STDOFF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STDOFF` writer - Frame Buffer Stride Offset"]
pub type STDOFF_W<'a> = crate::FieldWriter<'a, u32, L0FLLEN_SPEC, u16, u16, 14, 16>;
#[doc = "Field `FLL` reader - Frame Line Length"]
pub type FLL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLL` writer - Frame Line Length"]
pub type FLL_W<'a> = crate::FieldWriter<'a, u32, L0FLLEN_SPEC, u16, u16, 14, 0>;
impl R {
    #[doc = "Bits 16:29 - Frame Buffer Stride Offset"]
    #[inline(always)]
    pub fn stdoff(&self) -> STDOFF_R {
        STDOFF_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 0:13 - Frame Line Length"]
    #[inline(always)]
    pub fn fll(&self) -> FLL_R {
        FLL_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:29 - Frame Buffer Stride Offset"]
    #[inline(always)]
    pub fn stdoff(&mut self) -> STDOFF_W {
        STDOFF_W::new(self)
    }
    #[doc = "Bits 0:13 - Frame Line Length"]
    #[inline(always)]
    pub fn fll(&mut self) -> FLL_W {
        FLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 0 frame line length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l0fllen](index.html) module"]
pub struct L0FLLEN_SPEC;
impl crate::RegisterSpec for L0FLLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l0fllen::R](R) reader structure"]
impl crate::Readable for L0FLLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l0fllen::W](W) writer structure"]
impl crate::Writable for L0FLLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L0FLLEN to value 0"]
impl crate::Resettable for L0FLLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
