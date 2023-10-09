#[doc = "Register `INTC` writer"]
pub type W = crate::W<INTC_SPEC>;
#[doc = "Field `LMC` writer - Line Mark Flag Clear"]
pub type LMC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FEC` writer - FIFO Error Flag Clear"]
pub type FEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEC` writer - Transaction Error Flag Clear"]
pub type TEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCRC` writer - Layer Configuration Reloaded Flag Clear"]
pub type LCRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Line Mark Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lmc(&mut self) -> LMC_W<INTC_SPEC, 0> {
        LMC_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Error Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fec(&mut self) -> FEC_W<INTC_SPEC, 1> {
        FEC_W::new(self)
    }
    #[doc = "Bit 2 - Transaction Error Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tec(&mut self) -> TEC_W<INTC_SPEC, 2> {
        TEC_W::new(self)
    }
    #[doc = "Bit 3 - Layer Configuration Reloaded Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lcrc(&mut self) -> LCRC_W<INTC_SPEC, 3> {
        LCRC_W::new(self)
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
#[doc = "Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
