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
#[doc = "Field `LCRC` writer - Layer Configuration Reloaded Flag Clear"]
pub type LCRC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 3>;
#[doc = "Field `TEC` writer - Transaction Error Flag Clear"]
pub type TEC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 2>;
#[doc = "Field `FEC` writer - FIFO Error Flag Clear"]
pub type FEC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 1>;
#[doc = "Field `LMC` writer - Line Mark Flag Clear"]
pub type LMC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 0>;
impl W {
    #[doc = "Bit 3 - Layer Configuration Reloaded Flag Clear"]
    #[inline(always)]
    pub fn lcrc(&mut self) -> LCRC_W {
        LCRC_W::new(self)
    }
    #[doc = "Bit 2 - Transaction Error Flag Clear"]
    #[inline(always)]
    pub fn tec(&mut self) -> TEC_W {
        TEC_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Error Flag Clear"]
    #[inline(always)]
    pub fn fec(&mut self) -> FEC_W {
        FEC_W::new(self)
    }
    #[doc = "Bit 0 - Line Mark Flag Clear"]
    #[inline(always)]
    pub fn lmc(&mut self) -> LMC_W {
        LMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intc](index.html) module"]
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
