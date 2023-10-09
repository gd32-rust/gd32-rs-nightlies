#[doc = "Register `INTC` writer"]
pub type W = crate::W<INTC_SPEC>;
#[doc = "Field `PEC` writer - Parity error clear"]
pub type PEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FEC` writer - Frame error flag clear"]
pub type FEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NEC` writer - Noise detected clear"]
pub type NEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OREC` writer - Overrun error clear"]
pub type OREC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDLEC` writer - Idle line detected clear"]
pub type IDLEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCC` writer - Transmission complete clear"]
pub type TCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LBDC` writer - LIN break detected clear"]
pub type LBDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTC` writer - Receiver timeout clear"]
pub type RTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EBC` writer - End of block clear"]
pub type EBC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMC` writer - ADDR match clear"]
pub type AMC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUC` writer - Wakeup from Deep-sleep mode clear"]
pub type WUC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Parity error clear"]
    #[inline(always)]
    #[must_use]
    pub fn pec(&mut self) -> PEC_W<INTC_SPEC, 0> {
        PEC_W::new(self)
    }
    #[doc = "Bit 1 - Frame error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn fec(&mut self) -> FEC_W<INTC_SPEC, 1> {
        FEC_W::new(self)
    }
    #[doc = "Bit 2 - Noise detected clear"]
    #[inline(always)]
    #[must_use]
    pub fn nec(&mut self) -> NEC_W<INTC_SPEC, 2> {
        NEC_W::new(self)
    }
    #[doc = "Bit 3 - Overrun error clear"]
    #[inline(always)]
    #[must_use]
    pub fn orec(&mut self) -> OREC_W<INTC_SPEC, 3> {
        OREC_W::new(self)
    }
    #[doc = "Bit 4 - Idle line detected clear"]
    #[inline(always)]
    #[must_use]
    pub fn idlec(&mut self) -> IDLEC_W<INTC_SPEC, 4> {
        IDLEC_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TCC_W<INTC_SPEC, 6> {
        TCC_W::new(self)
    }
    #[doc = "Bit 8 - LIN break detected clear"]
    #[inline(always)]
    #[must_use]
    pub fn lbdc(&mut self) -> LBDC_W<INTC_SPEC, 8> {
        LBDC_W::new(self)
    }
    #[doc = "Bit 11 - Receiver timeout clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<INTC_SPEC, 11> {
        RTC_W::new(self)
    }
    #[doc = "Bit 12 - End of block clear"]
    #[inline(always)]
    #[must_use]
    pub fn ebc(&mut self) -> EBC_W<INTC_SPEC, 12> {
        EBC_W::new(self)
    }
    #[doc = "Bit 17 - ADDR match clear"]
    #[inline(always)]
    #[must_use]
    pub fn amc(&mut self) -> AMC_W<INTC_SPEC, 17> {
        AMC_W::new(self)
    }
    #[doc = "Bit 20 - Wakeup from Deep-sleep mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn wuc(&mut self) -> WUC_W<INTC_SPEC, 20> {
        WUC_W::new(self)
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
#[doc = "Interrupt status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
