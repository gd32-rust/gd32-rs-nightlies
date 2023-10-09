#[doc = "Register `L0BLEND` reader"]
pub type R = crate::R<L0BLEND_SPEC>;
#[doc = "Register `L0BLEND` writer"]
pub type W = crate::W<L0BLEND_SPEC>;
#[doc = "Field `ACF2` reader - Alpha Calculation Factor 2 of Blending Method"]
pub type ACF2_R = crate::FieldReader;
#[doc = "Field `ACF2` writer - Alpha Calculation Factor 2 of Blending Method"]
pub type ACF2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ACF1` reader - Alpha Calculation Factor 1 of Blending Method"]
pub type ACF1_R = crate::FieldReader;
#[doc = "Field `ACF1` writer - Alpha Calculation Factor 1 of Blending Method"]
pub type ACF1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Alpha Calculation Factor 2 of Blending Method"]
    #[inline(always)]
    pub fn acf2(&self) -> ACF2_R {
        ACF2_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Alpha Calculation Factor 1 of Blending Method"]
    #[inline(always)]
    pub fn acf1(&self) -> ACF1_R {
        ACF1_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alpha Calculation Factor 2 of Blending Method"]
    #[inline(always)]
    #[must_use]
    pub fn acf2(&mut self) -> ACF2_W<L0BLEND_SPEC, 0> {
        ACF2_W::new(self)
    }
    #[doc = "Bits 8:10 - Alpha Calculation Factor 1 of Blending Method"]
    #[inline(always)]
    #[must_use]
    pub fn acf1(&mut self) -> ACF1_W<L0BLEND_SPEC, 8> {
        ACF1_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Layer 0 blending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0blend::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0blend::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0BLEND_SPEC;
impl crate::RegisterSpec for L0BLEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0blend::R`](R) reader structure"]
impl crate::Readable for L0BLEND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l0blend::W`](W) writer structure"]
impl crate::Writable for L0BLEND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L0BLEND to value 0x0607"]
impl crate::Resettable for L0BLEND_SPEC {
    const RESET_VALUE: Self::Ux = 0x0607;
}
