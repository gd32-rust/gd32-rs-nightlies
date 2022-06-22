#[doc = "Register `ADDINT` reader"]
pub struct R(crate::R<ADDINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDINT` writer"]
pub struct W(crate::W<ADDINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDINT_SPEC>;
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
impl From<crate::W<ADDINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRC48MSTBIF` reader - IRC48M stabilization interrupt flag"]
pub type IRC48MSTBIF_R = crate::BitReader<bool>;
#[doc = "Field `IRC48MSTBIE` reader - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
pub type IRC48MSTBIE_R = crate::BitReader<bool>;
#[doc = "Field `IRC48MSTBIE` writer - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
pub type IRC48MSTBIE_W<'a> = crate::BitWriter<'a, u32, ADDINT_SPEC, bool, 14>;
#[doc = "Field `IRC48MSTBIC` writer - Internal 48 MHz RC oscillator Stabilization Interrupt Clear"]
pub type IRC48MSTBIC_W<'a> = crate::BitWriter<'a, u32, ADDINT_SPEC, bool, 22>;
impl R {
    #[doc = "Bit 6 - IRC48M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc48mstbif(&self) -> IRC48MSTBIF_R {
        IRC48MSTBIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc48mstbie(&self) -> IRC48MSTBIE_R {
        IRC48MSTBIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc48mstbie(&mut self) -> IRC48MSTBIE_W {
        IRC48MSTBIE_W::new(self)
    }
    #[doc = "Bit 22 - Internal 48 MHz RC oscillator Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc48mstbic(&mut self) -> IRC48MSTBIC_W {
        IRC48MSTBIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Additional clock interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addint](index.html) module"]
pub struct ADDINT_SPEC;
impl crate::RegisterSpec for ADDINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addint::R](R) reader structure"]
impl crate::Readable for ADDINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addint::W](W) writer structure"]
impl crate::Writable for ADDINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDINT to value 0"]
impl crate::Resettable for ADDINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
