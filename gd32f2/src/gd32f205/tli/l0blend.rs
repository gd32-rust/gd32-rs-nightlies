#[doc = "Register `L0BLEND` reader"]
pub struct R(crate::R<L0BLEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L0BLEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L0BLEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L0BLEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L0BLEND` writer"]
pub struct W(crate::W<L0BLEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L0BLEND_SPEC>;
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
impl From<crate::W<L0BLEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L0BLEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACF1` reader - Alpha Calculation Factor 1 of Blending Method"]
pub type ACF1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACF1` writer - Alpha Calculation Factor 1 of Blending Method"]
pub type ACF1_W<'a> = crate::FieldWriter<'a, u32, L0BLEND_SPEC, u8, u8, 3, 8>;
#[doc = "Field `ACF2` reader - Alpha Calculation Factor 2 of Blending Method"]
pub type ACF2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACF2` writer - Alpha Calculation Factor 2 of Blending Method"]
pub type ACF2_W<'a> = crate::FieldWriter<'a, u32, L0BLEND_SPEC, u8, u8, 3, 0>;
impl R {
    #[doc = "Bits 8:10 - Alpha Calculation Factor 1 of Blending Method"]
    #[inline(always)]
    pub fn acf1(&self) -> ACF1_R {
        ACF1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 0:2 - Alpha Calculation Factor 2 of Blending Method"]
    #[inline(always)]
    pub fn acf2(&self) -> ACF2_R {
        ACF2_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Alpha Calculation Factor 1 of Blending Method"]
    #[inline(always)]
    pub fn acf1(&mut self) -> ACF1_W {
        ACF1_W::new(self)
    }
    #[doc = "Bits 0:2 - Alpha Calculation Factor 2 of Blending Method"]
    #[inline(always)]
    pub fn acf2(&mut self) -> ACF2_W {
        ACF2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 0 blending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l0blend](index.html) module"]
pub struct L0BLEND_SPEC;
impl crate::RegisterSpec for L0BLEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l0blend::R](R) reader structure"]
impl crate::Readable for L0BLEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l0blend::W](W) writer structure"]
impl crate::Writable for L0BLEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L0BLEND to value 0x0607"]
impl crate::Resettable for L0BLEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0607
    }
}
