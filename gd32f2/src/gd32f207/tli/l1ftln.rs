#[doc = "Register `L1FTLN` reader"]
pub struct R(crate::R<L1FTLN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1FTLN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1FTLN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1FTLN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1FTLN` writer"]
pub struct W(crate::W<L1FTLN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1FTLN_SPEC>;
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
impl From<crate::W<L1FTLN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1FTLN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTLN` reader - Frame Total Line Number"]
pub type FTLN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FTLN` writer - Frame Total Line Number"]
pub type FTLN_W<'a> = crate::FieldWriter<'a, u32, L1FTLN_SPEC, u16, u16, 11, 0>;
impl R {
    #[doc = "Bits 0:10 - Frame Total Line Number"]
    #[inline(always)]
    pub fn ftln(&self) -> FTLN_R {
        FTLN_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame Total Line Number"]
    #[inline(always)]
    pub fn ftln(&mut self) -> FTLN_W {
        FTLN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 1 frame total line number register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1ftln](index.html) module"]
pub struct L1FTLN_SPEC;
impl crate::RegisterSpec for L1FTLN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1ftln::R](R) reader structure"]
impl crate::Readable for L1FTLN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1ftln::W](W) writer structure"]
impl crate::Writable for L1FTLN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L1FTLN to value 0"]
impl crate::Resettable for L1FTLN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
