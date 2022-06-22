#[doc = "Register `L1FBADDR` reader"]
pub struct R(crate::R<L1FBADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1FBADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1FBADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1FBADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1FBADDR` writer"]
pub struct W(crate::W<L1FBADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1FBADDR_SPEC>;
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
impl From<crate::W<L1FBADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1FBADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBADD` reader - Frame Buffer base Address"]
pub type FBADD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FBADD` writer - Frame Buffer base Address"]
pub type FBADD_W<'a> = crate::FieldWriter<'a, u32, L1FBADDR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Frame Buffer base Address"]
    #[inline(always)]
    pub fn fbadd(&self) -> FBADD_R {
        FBADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Frame Buffer base Address"]
    #[inline(always)]
    pub fn fbadd(&mut self) -> FBADD_W {
        FBADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 1 frame base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1fbaddr](index.html) module"]
pub struct L1FBADDR_SPEC;
impl crate::RegisterSpec for L1FBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1fbaddr::R](R) reader structure"]
impl crate::Readable for L1FBADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1fbaddr::W](W) writer structure"]
impl crate::Writable for L1FBADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L1FBADDR to value 0"]
impl crate::Resettable for L1FBADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
