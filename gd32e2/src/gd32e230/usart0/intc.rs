#[doc = "Register `INTC` writer"]
pub type W = crate::W<INTC_SPEC>;
#[doc = "Parity error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PEC_AW>;
impl<'a, REG, const O: u8> PEC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the PERR bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PEC_AW::CLEAR)
    }
}
#[doc = "Framing error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type FEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FEC_AW>;
impl<'a, REG, const O: u8> FEC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the FERR bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FEC_AW::CLEAR)
    }
}
#[doc = "Noise detected clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[doc = "Field `NEC` writer - Noise detected clear flag"]
pub type NEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NEC_AW>;
impl<'a, REG, const O: u8> NEC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the NERR bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(NEC_AW::CLEAR)
    }
}
#[doc = "Overrun error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type OREC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OREC_AW>;
impl<'a, REG, const O: u8> OREC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ORERR bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OREC_AW::CLEAR)
    }
}
#[doc = "Idle line detected clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[doc = "Field `IDLEC` writer - Idle line detected clear flag"]
pub type IDLEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IDLEC_AW>;
impl<'a, REG, const O: u8> IDLEC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the IDLEF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEC_AW::CLEAR)
    }
}
#[doc = "Transmission complete clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type TCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TCC_AW>;
impl<'a, REG, const O: u8> TCC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the TC bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TCC_AW::CLEAR)
    }
}
#[doc = "LIN break detection clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[doc = "Field `LBDC` writer - LIN break detection clear flag"]
pub type LBDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LBDC_AW>;
impl<'a, REG, const O: u8> LBDC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the LBDF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LBDC_AW::CLEAR)
    }
}
#[doc = "CTS clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[doc = "Field `CTSC` writer - CTS clear flag"]
pub type CTSC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTSC_AW>;
impl<'a, REG, const O: u8> CTSC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the CTSF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTSC_AW::CLEAR)
    }
}
#[doc = "Receiver timeout clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type RTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTC_AW>;
impl<'a, REG, const O: u8> RTC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the RCF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_AW::CLEAR)
    }
}
#[doc = "End of timeout clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[doc = "Field `EBC` writer - End of timeout clear flag"]
pub type EBC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EBC_AW>;
impl<'a, REG, const O: u8> EBC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the EBF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EBC_AW::CLEAR)
    }
}
#[doc = "Character match clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[doc = "Field `AMC` writer - Character match clear flag"]
pub type AMC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AMC_AW>;
impl<'a, REG, const O: u8> AMC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the AMF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AMC_AW::CLEAR)
    }
}
#[doc = "Wakeup from Stop mode clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[doc = "Field `WUC` writer - Wakeup from Stop mode clear flag"]
pub type WUC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WUC_AW>;
impl<'a, REG, const O: u8> WUC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the WUF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WUC_AW::CLEAR)
    }
}
impl W {
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn pec(&mut self) -> PEC_W<INTC_SPEC, 0> {
        PEC_W::new(self)
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn fec(&mut self) -> FEC_W<INTC_SPEC, 1> {
        FEC_W::new(self)
    }
    #[doc = "Bit 2 - Noise detected clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn nec(&mut self) -> NEC_W<INTC_SPEC, 2> {
        NEC_W::new(self)
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn orec(&mut self) -> OREC_W<INTC_SPEC, 3> {
        OREC_W::new(self)
    }
    #[doc = "Bit 4 - Idle line detected clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn idlec(&mut self) -> IDLEC_W<INTC_SPEC, 4> {
        IDLEC_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TCC_W<INTC_SPEC, 6> {
        TCC_W::new(self)
    }
    #[doc = "Bit 8 - LIN break detection clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn lbdc(&mut self) -> LBDC_W<INTC_SPEC, 8> {
        LBDC_W::new(self)
    }
    #[doc = "Bit 9 - CTS clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctsc(&mut self) -> CTSC_W<INTC_SPEC, 9> {
        CTSC_W::new(self)
    }
    #[doc = "Bit 11 - Receiver timeout clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<INTC_SPEC, 11> {
        RTC_W::new(self)
    }
    #[doc = "Bit 12 - End of timeout clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn ebc(&mut self) -> EBC_W<INTC_SPEC, 12> {
        EBC_W::new(self)
    }
    #[doc = "Bit 17 - Character match clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn amc(&mut self) -> AMC_W<INTC_SPEC, 17> {
        AMC_W::new(self)
    }
    #[doc = "Bit 20 - Wakeup from Stop mode clear flag"]
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
