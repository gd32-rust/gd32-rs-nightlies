#[doc = "Register `INTC` writer"]
pub type W = crate::W<INTC_SPEC>;
#[doc = "Field `EFFC` writer - Clear End of Frame Flag"]
pub type EFFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVRFC` writer - Clear FIFO Overrun Flag"]
pub type OVRFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESEFC` writer - Clear embedded synchronous Error Flag"]
pub type ESEFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VSFC` writer - Vsync flag clear"]
pub type VSFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ELFC` writer - End of Line Flag Clear"]
pub type ELFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Clear End of Frame Flag"]
    #[inline(always)]
    #[must_use]
    pub fn effc(&mut self) -> EFFC_W<INTC_SPEC, 0> {
        EFFC_W::new(self)
    }
    #[doc = "Bit 1 - Clear FIFO Overrun Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovrfc(&mut self) -> OVRFC_W<INTC_SPEC, 1> {
        OVRFC_W::new(self)
    }
    #[doc = "Bit 2 - Clear embedded synchronous Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn esefc(&mut self) -> ESEFC_W<INTC_SPEC, 2> {
        ESEFC_W::new(self)
    }
    #[doc = "Bit 3 - Vsync flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn vsfc(&mut self) -> VSFC_W<INTC_SPEC, 3> {
        VSFC_W::new(self)
    }
    #[doc = "Bit 4 - End of Line Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn elfc(&mut self) -> ELFC_W<INTC_SPEC, 4> {
        ELFC_W::new(self)
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
#[doc = "DCI Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTC_SPEC;
impl crate::RegisterSpec for INTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for INTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
