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
#[doc = "Wakeup from Deep-sleep mode clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUC_AW {
    #[doc = "1: Clears the WUF flag in the STAT register"]
    CLEAR = 1,
}
impl From<WUC_AW> for bool {
    #[inline(always)]
    fn from(variant: WUC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUC` writer - Wakeup from Deep-sleep mode clear"]
pub type WUC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, WUC_AW, 20>;
impl<'a> WUC_W<'a> {
    #[doc = "Clears the WUF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUC_AW::CLEAR)
    }
}
#[doc = "ADDR match clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMC_AW {
    #[doc = "1: Clears the AMF flag in the STAT register"]
    CLEAR = 1,
}
impl From<AMC_AW> for bool {
    #[inline(always)]
    fn from(variant: AMC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMC` writer - ADDR match clear"]
pub type AMC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, AMC_AW, 17>;
impl<'a> AMC_W<'a> {
    #[doc = "Clears the AMF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AMC_AW::CLEAR)
    }
}
#[doc = "End of timeout clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBC_AW {
    #[doc = "1: Clears the EBF flag in the STAT register"]
    CLEAR = 1,
}
impl From<EBC_AW> for bool {
    #[inline(always)]
    fn from(variant: EBC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBC` writer - End of timeout clear"]
pub type EBC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, EBC_AW, 12>;
impl<'a> EBC_W<'a> {
    #[doc = "Clears the EBF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EBC_AW::CLEAR)
    }
}
#[doc = "Receiver timeout clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_AW {
    #[doc = "1: Clears the RCF flag in the STAT register"]
    CLEAR = 1,
}
impl From<RTC_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC` writer - Receiver timeout clear flag"]
pub type RTC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, RTC_AW, 11>;
impl<'a> RTC_W<'a> {
    #[doc = "Clears the RCF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RTC_AW::CLEAR)
    }
}
#[doc = "CTS change clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSC_AW {
    #[doc = "1: Clears the CTSF flag in the STAT register"]
    CLEAR = 1,
}
impl From<CTSC_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSC` writer - CTS change clear"]
pub type CTSC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, CTSC_AW, 9>;
impl<'a> CTSC_W<'a> {
    #[doc = "Clears the CTSF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSC_AW::CLEAR)
    }
}
#[doc = "LIN break detected clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDC_AW {
    #[doc = "1: Clears the LBDF flag in the STAT register"]
    CLEAR = 1,
}
impl From<LBDC_AW> for bool {
    #[inline(always)]
    fn from(variant: LBDC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDC` writer - LIN break detected clear"]
pub type LBDC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, LBDC_AW, 8>;
impl<'a> LBDC_W<'a> {
    #[doc = "Clears the LBDF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LBDC_AW::CLEAR)
    }
}
#[doc = "Transmission complete clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCC_AW {
    #[doc = "1: Clears the TC bit in the STAT register"]
    CLEAR = 1,
}
impl From<TCC_AW> for bool {
    #[inline(always)]
    fn from(variant: TCC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCC` writer - Transmission complete clear flag"]
pub type TCC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, TCC_AW, 6>;
impl<'a> TCC_W<'a> {
    #[doc = "Clears the TC bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCC_AW::CLEAR)
    }
}
#[doc = "Idle line detected clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLEC_AW {
    #[doc = "1: Clears the IDLEF flag in the STAT register"]
    CLEAR = 1,
}
impl From<IDLEC_AW> for bool {
    #[inline(always)]
    fn from(variant: IDLEC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEC` writer - Idle line detected clear"]
pub type IDLEC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, IDLEC_AW, 4>;
impl<'a> IDLEC_W<'a> {
    #[doc = "Clears the IDLEF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IDLEC_AW::CLEAR)
    }
}
#[doc = "Overrun error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OREC_AW {
    #[doc = "1: Clears the ORERR bit in the STAT register"]
    CLEAR = 1,
}
impl From<OREC_AW> for bool {
    #[inline(always)]
    fn from(variant: OREC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OREC` writer - Overrun error clear flag"]
pub type OREC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, OREC_AW, 3>;
impl<'a> OREC_W<'a> {
    #[doc = "Clears the ORERR bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OREC_AW::CLEAR)
    }
}
#[doc = "Noise detected clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEC_AW {
    #[doc = "1: Clears the NERR bit in the STAT register"]
    CLEAR = 1,
}
impl From<NEC_AW> for bool {
    #[inline(always)]
    fn from(variant: NEC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEC` writer - Noise detected clear"]
pub type NEC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, NEC_AW, 2>;
impl<'a> NEC_W<'a> {
    #[doc = "Clears the NERR bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NEC_AW::CLEAR)
    }
}
#[doc = "Framing error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEC_AW {
    #[doc = "1: Clears the FERR bit in the STAT register"]
    CLEAR = 1,
}
impl From<FEC_AW> for bool {
    #[inline(always)]
    fn from(variant: FEC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEC` writer - Framing error clear flag"]
pub type FEC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, FEC_AW, 1>;
impl<'a> FEC_W<'a> {
    #[doc = "Clears the FERR bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FEC_AW::CLEAR)
    }
}
#[doc = "Parity error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEC_AW {
    #[doc = "1: Clears the PERR bit in the STAT register"]
    CLEAR = 1,
}
impl From<PEC_AW> for bool {
    #[inline(always)]
    fn from(variant: PEC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEC` writer - Parity error clear flag"]
pub type PEC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, PEC_AW, 0>;
impl<'a> PEC_W<'a> {
    #[doc = "Clears the PERR bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PEC_AW::CLEAR)
    }
}
impl W {
    #[doc = "Bit 20 - Wakeup from Deep-sleep mode clear"]
    #[inline(always)]
    pub fn wuc(&mut self) -> WUC_W {
        WUC_W::new(self)
    }
    #[doc = "Bit 17 - ADDR match clear"]
    #[inline(always)]
    pub fn amc(&mut self) -> AMC_W {
        AMC_W::new(self)
    }
    #[doc = "Bit 12 - End of timeout clear"]
    #[inline(always)]
    pub fn ebc(&mut self) -> EBC_W {
        EBC_W::new(self)
    }
    #[doc = "Bit 11 - Receiver timeout clear flag"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W::new(self)
    }
    #[doc = "Bit 9 - CTS change clear"]
    #[inline(always)]
    pub fn ctsc(&mut self) -> CTSC_W {
        CTSC_W::new(self)
    }
    #[doc = "Bit 8 - LIN break detected clear"]
    #[inline(always)]
    pub fn lbdc(&mut self) -> LBDC_W {
        LBDC_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TCC_W {
        TCC_W::new(self)
    }
    #[doc = "Bit 4 - Idle line detected clear"]
    #[inline(always)]
    pub fn idlec(&mut self) -> IDLEC_W {
        IDLEC_W::new(self)
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline(always)]
    pub fn orec(&mut self) -> OREC_W {
        OREC_W::new(self)
    }
    #[doc = "Bit 2 - Noise detected clear"]
    #[inline(always)]
    pub fn nec(&mut self) -> NEC_W {
        NEC_W::new(self)
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline(always)]
    pub fn fec(&mut self) -> FEC_W {
        FEC_W::new(self)
    }
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline(always)]
    pub fn pec(&mut self) -> PEC_W {
        PEC_W::new(self)
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
