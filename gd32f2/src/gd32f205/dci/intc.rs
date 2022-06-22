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
#[doc = "Field `ELFC` writer - End of Line Flag Clear"]
pub type ELFC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 4>;
#[doc = "Field `VSFC` writer - Vsync flag clear"]
pub type VSFC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 3>;
#[doc = "Field `ESEFC` writer - Clear embedded synchronous Error Flag"]
pub type ESEFC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 2>;
#[doc = "Field `OVRFC` writer - Clear FIFO Overrun Flag"]
pub type OVRFC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 1>;
#[doc = "Field `EFFC` writer - Clear End of Frame Flag"]
pub type EFFC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 0>;
impl W {
    #[doc = "Bit 4 - End of Line Flag Clear"]
    #[inline(always)]
    pub fn elfc(&mut self) -> ELFC_W {
        ELFC_W::new(self)
    }
    #[doc = "Bit 3 - Vsync flag clear"]
    #[inline(always)]
    pub fn vsfc(&mut self) -> VSFC_W {
        VSFC_W::new(self)
    }
    #[doc = "Bit 2 - Clear embedded synchronous Error Flag"]
    #[inline(always)]
    pub fn esefc(&mut self) -> ESEFC_W {
        ESEFC_W::new(self)
    }
    #[doc = "Bit 1 - Clear FIFO Overrun Flag"]
    #[inline(always)]
    pub fn ovrfc(&mut self) -> OVRFC_W {
        OVRFC_W::new(self)
    }
    #[doc = "Bit 0 - Clear End of Frame Flag"]
    #[inline(always)]
    pub fn effc(&mut self) -> EFFC_W {
        EFFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCI Interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intc](index.html) module"]
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
