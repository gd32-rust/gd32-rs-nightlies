#[doc = "Register `PLLTINT` reader"]
pub struct R(crate::R<PLLTINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLTINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLTINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLTINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLTINT` writer"]
pub struct W(crate::W<PLLTINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLTINT_SPEC>;
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
impl From<crate::W<PLLTINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLTINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLTSTBIF` reader - PLLT stabilization interrupt flag"]
pub type PLLTSTBIF_R = crate::BitReader<bool>;
#[doc = "Field `PLLTSTBIE` reader - PLLT stabilization Interrupt Enable"]
pub type PLLTSTBIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLTSTBIE` writer - PLLT stabilization Interrupt Enable"]
pub type PLLTSTBIE_W<'a> = crate::BitWriter<'a, u32, PLLTINT_SPEC, bool, 14>;
#[doc = "Field `PLLTSTBIC` reader - PLLT stabilization Interrupt clear"]
pub type PLLTSTBIC_R = crate::BitReader<bool>;
#[doc = "Field `PLLTSTBIC` writer - PLLT stabilization Interrupt clear"]
pub type PLLTSTBIC_W<'a> = crate::BitWriter<'a, u32, PLLTINT_SPEC, bool, 22>;
impl R {
    #[doc = "Bit 6 - PLLT stabilization interrupt flag"]
    #[inline(always)]
    pub fn plltstbif(&self) -> PLLTSTBIF_R {
        PLLTSTBIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - PLLT stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn plltstbie(&self) -> PLLTSTBIE_R {
        PLLTSTBIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 22 - PLLT stabilization Interrupt clear"]
    #[inline(always)]
    pub fn plltstbic(&self) -> PLLTSTBIC_R {
        PLLTSTBIC_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - PLLT stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn plltstbie(&mut self) -> PLLTSTBIE_W {
        PLLTSTBIE_W::new(self)
    }
    #[doc = "Bit 22 - PLLT stabilization Interrupt clear"]
    #[inline(always)]
    pub fn plltstbic(&mut self) -> PLLTSTBIC_W {
        PLLTSTBIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLT interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plltint](index.html) module"]
pub struct PLLTINT_SPEC;
impl crate::RegisterSpec for PLLTINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plltint::R](R) reader structure"]
impl crate::Readable for PLLTINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plltint::W](W) writer structure"]
impl crate::Writable for PLLTINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLTINT to value 0"]
impl crate::Resettable for PLLTINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
