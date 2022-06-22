#[doc = "Register `INTC` writer"]
pub struct W(crate::W<INTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTC_SPEC>;
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
impl From<crate::W<INTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKOKIC` writer - CKOKIF interrupt clear bit"]
pub type CKOKIC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 0>;
#[doc = "Field `CKWARNIC` writer - CKWARNIF interrupt clear bit"]
pub type CKWARNIC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 1>;
#[doc = "Field `ERRIC` writer - ERRIF interrupt clear bit"]
pub type ERRIC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 2>;
#[doc = "Field `EREFIC` writer - EREFIF interrupt clear bit"]
pub type EREFIC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 3>;
impl W {
    #[doc = "Bit 0 - CKOKIF interrupt clear bit"]
    #[inline(always)]
    pub fn ckokic(&mut self) -> CKOKIC_W {
        CKOKIC_W::new(self)
    }
    #[doc = "Bit 1 - CKWARNIF interrupt clear bit"]
    #[inline(always)]
    pub fn ckwarnic(&mut self) -> CKWARNIC_W {
        CKWARNIC_W::new(self)
    }
    #[doc = "Bit 2 - ERRIF interrupt clear bit"]
    #[inline(always)]
    pub fn erric(&mut self) -> ERRIC_W {
        ERRIC_W::new(self)
    }
    #[doc = "Bit 3 - EREFIF interrupt clear bit"]
    #[inline(always)]
    pub fn erefic(&mut self) -> EREFIC_W {
        EREFIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intc](index.html) module"]
pub struct INTC_SPEC;
impl crate::RegisterSpec for INTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intc::W](W) writer structure"]
impl crate::Writable for INTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for INTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
