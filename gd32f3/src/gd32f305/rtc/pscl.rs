#[doc = "Register `PSCL` writer"]
pub struct W(crate::W<PSCL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCL_SPEC>;
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
impl From<crate::W<PSCL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSC` writer - RTC prescaler value low"]
pub type PSC_W<'a> = crate::FieldWriter<'a, u32, PSCL_SPEC, u16, u16, 16, 0>;
impl W {
    #[doc = "Bits 0:15 - RTC prescaler value low"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC prescaler low register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscl](index.html) module"]
pub struct PSCL_SPEC;
impl crate::RegisterSpec for PSCL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pscl::W](W) writer structure"]
impl crate::Writable for PSCL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSCL to value 0x8000"]
impl crate::Resettable for PSCL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
