#[doc = "Register `L1PPF` reader"]
pub struct R(crate::R<L1PPF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1PPF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1PPF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1PPF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1PPF` writer"]
pub struct W(crate::W<L1PPF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1PPF_SPEC>;
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
impl From<crate::W<L1PPF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1PPF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPF` reader - Packeted Pixel Format"]
pub type PPF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPF` writer - Packeted Pixel Format"]
pub type PPF_W<'a> = crate::FieldWriter<'a, u32, L1PPF_SPEC, u8, u8, 3, 0>;
impl R {
    #[doc = "Bits 0:2 - Packeted Pixel Format"]
    #[inline(always)]
    pub fn ppf(&self) -> PPF_R {
        PPF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Packeted Pixel Format"]
    #[inline(always)]
    pub fn ppf(&mut self) -> PPF_W {
        PPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 1 packeted pixel format register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1ppf](index.html) module"]
pub struct L1PPF_SPEC;
impl crate::RegisterSpec for L1PPF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1ppf::R](R) reader structure"]
impl crate::Readable for L1PPF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1ppf::W](W) writer structure"]
impl crate::Writable for L1PPF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L1PPF to value 0"]
impl crate::Resettable for L1PPF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
