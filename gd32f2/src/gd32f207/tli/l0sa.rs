#[doc = "Register `L0SA` reader"]
pub struct R(crate::R<L0SA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L0SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L0SA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L0SA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L0SA` writer"]
pub struct W(crate::W<L0SA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L0SA_SPEC>;
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
impl From<crate::W<L0SA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L0SA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SA` reader - Specified alpha"]
pub type SA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SA` writer - Specified alpha"]
pub type SA_W<'a> = crate::FieldWriter<'a, u32, L0SA_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Specified alpha"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specified alpha"]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 0 specified alpha register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l0sa](index.html) module"]
pub struct L0SA_SPEC;
impl crate::RegisterSpec for L0SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l0sa::R](R) reader structure"]
impl crate::Readable for L0SA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l0sa::W](W) writer structure"]
impl crate::Writable for L0SA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L0SA to value 0xff"]
impl crate::Resettable for L0SA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
