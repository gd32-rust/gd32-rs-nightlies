#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `WDCHSEL` reader - Analog watchdog channel select"]
pub type WDCHSEL_R = crate::FieldReader;
#[doc = "Field `WDCHSEL` writer - Analog watchdog channel select"]
pub type WDCHSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `EOCIE` reader - Interrupt enable for EOC"]
pub type EOCIE_R = crate::BitReader<EOCIE_A>;
#[doc = "Interrupt enable for EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE_A {
    #[doc = "0: EOC interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set"]
    ENABLED = 1,
}
impl From<EOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCIE_A {
        match self.bits {
            false => EOCIE_A::DISABLED,
            true => EOCIE_A::ENABLED,
        }
    }
    #[doc = "EOC interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE_A::DISABLED
    }
    #[doc = "EOC interrupt enabled. An interrupt is generated when the EOC bit is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIE_A::ENABLED
    }
}
#[doc = "Field `EOCIE` writer - Interrupt enable for EOC"]
pub type EOCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EOCIE_A>;
impl<'a, REG, const O: u8> EOCIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE_A::DISABLED)
    }
    #[doc = "EOC interrupt enabled. An interrupt is generated when the EOC bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE_A::ENABLED)
    }
}
#[doc = "Field `WDEIE` reader - Interrupt enable for WDE"]
pub type WDEIE_R = crate::BitReader<WDEIE_A>;
#[doc = "Interrupt enable for WDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDEIE_A {
    #[doc = "0: WDE interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: WDE interrupt enabled. An interrupt is generated when the WDE bit is set"]
    ENABLED = 1,
}
impl From<WDEIE_A> for bool {
    #[inline(always)]
    fn from(variant: WDEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WDEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDEIE_A {
        match self.bits {
            false => WDEIE_A::DISABLED,
            true => WDEIE_A::ENABLED,
        }
    }
    #[doc = "WDE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDEIE_A::DISABLED
    }
    #[doc = "WDE interrupt enabled. An interrupt is generated when the WDE bit is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDEIE_A::ENABLED
    }
}
#[doc = "Field `WDEIE` writer - Interrupt enable for WDE"]
pub type WDEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDEIE_A>;
impl<'a, REG, const O: u8> WDEIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDEIE_A::DISABLED)
    }
    #[doc = "WDE interrupt enabled. An interrupt is generated when the WDE bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDEIE_A::ENABLED)
    }
}
#[doc = "Field `EOICIE` reader - Interrupt enable for EOIC"]
pub type EOICIE_R = crate::BitReader<EOICIE_A>;
#[doc = "Interrupt enable for EOIC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOICIE_A {
    #[doc = "0: EOIC interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: EOIC interrupt enabled. An interrupt is generated when the EOIC bit is set"]
    ENABLED = 1,
}
impl From<EOICIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOICIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOICIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOICIE_A {
        match self.bits {
            false => EOICIE_A::DISABLED,
            true => EOICIE_A::ENABLED,
        }
    }
    #[doc = "EOIC interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOICIE_A::DISABLED
    }
    #[doc = "EOIC interrupt enabled. An interrupt is generated when the EOIC bit is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOICIE_A::ENABLED
    }
}
#[doc = "Field `EOICIE` writer - Interrupt enable for EOIC"]
pub type EOICIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EOICIE_A>;
impl<'a, REG, const O: u8> EOICIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOIC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOICIE_A::DISABLED)
    }
    #[doc = "EOIC interrupt enabled. An interrupt is generated when the EOIC bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOICIE_A::ENABLED)
    }
}
#[doc = "Field `SM` reader - Scan mode"]
pub type SM_R = crate::BitReader<SM_A>;
#[doc = "Scan mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM_A {
    #[doc = "0: Scan mode disabled"]
    DISABLED = 0,
    #[doc = "1: Scan mode enabled"]
    ENABLED = 1,
}
impl From<SM_A> for bool {
    #[inline(always)]
    fn from(variant: SM_A) -> Self {
        variant as u8 != 0
    }
}
impl SM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM_A {
        match self.bits {
            false => SM_A::DISABLED,
            true => SM_A::ENABLED,
        }
    }
    #[doc = "Scan mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SM_A::DISABLED
    }
    #[doc = "Scan mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SM_A::ENABLED
    }
}
#[doc = "Field `SM` writer - Scan mode"]
pub type SM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SM_A>;
impl<'a, REG, const O: u8> SM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scan mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::DISABLED)
    }
    #[doc = "Scan mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::ENABLED)
    }
}
#[doc = "Field `WDSC` reader - When in scan mode, analog watchdog is effective on a single channel"]
pub type WDSC_R = crate::BitReader<WDSC_A>;
#[doc = "When in scan mode, analog watchdog is effective on a single channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDSC_A {
    #[doc = "0: Analog watchdog enabled on all channels"]
    ALL = 0,
    #[doc = "1: Analog watchdog enabled on a single channel"]
    SINGLE = 1,
}
impl From<WDSC_A> for bool {
    #[inline(always)]
    fn from(variant: WDSC_A) -> Self {
        variant as u8 != 0
    }
}
impl WDSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDSC_A {
        match self.bits {
            false => WDSC_A::ALL,
            true => WDSC_A::SINGLE,
        }
    }
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == WDSC_A::ALL
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == WDSC_A::SINGLE
    }
}
#[doc = "Field `WDSC` writer - When in scan mode, analog watchdog is effective on a single channel"]
pub type WDSC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDSC_A>;
impl<'a, REG, const O: u8> WDSC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(WDSC_A::ALL)
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(WDSC_A::SINGLE)
    }
}
#[doc = "Field `ICA` reader - Inserted channel group convert automatically"]
pub type ICA_R = crate::BitReader<ICA_A>;
#[doc = "Inserted channel group convert automatically\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICA_A {
    #[doc = "0: Automatic inserted group conversion disabled"]
    DISABLED = 0,
    #[doc = "1: Automatic inserted group conversion enabled"]
    ENABLED = 1,
}
impl From<ICA_A> for bool {
    #[inline(always)]
    fn from(variant: ICA_A) -> Self {
        variant as u8 != 0
    }
}
impl ICA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICA_A {
        match self.bits {
            false => ICA_A::DISABLED,
            true => ICA_A::ENABLED,
        }
    }
    #[doc = "Automatic inserted group conversion disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICA_A::DISABLED
    }
    #[doc = "Automatic inserted group conversion enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICA_A::ENABLED
    }
}
#[doc = "Field `ICA` writer - Inserted channel group convert automatically"]
pub type ICA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ICA_A>;
impl<'a, REG, const O: u8> ICA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic inserted group conversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICA_A::DISABLED)
    }
    #[doc = "Automatic inserted group conversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICA_A::ENABLED)
    }
}
#[doc = "Field `DISRC` reader - Discontinuous mode on regular channels"]
pub type DISRC_R = crate::BitReader<DISRC_A>;
#[doc = "Discontinuous mode on regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISRC_A {
    #[doc = "0: Discontinuous mode on regular channels disabled"]
    DISABLED = 0,
    #[doc = "1: Discontinuous mode on regular channels enabled"]
    ENABLED = 1,
}
impl From<DISRC_A> for bool {
    #[inline(always)]
    fn from(variant: DISRC_A) -> Self {
        variant as u8 != 0
    }
}
impl DISRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISRC_A {
        match self.bits {
            false => DISRC_A::DISABLED,
            true => DISRC_A::ENABLED,
        }
    }
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISRC_A::DISABLED
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISRC_A::ENABLED
    }
}
#[doc = "Field `DISRC` writer - Discontinuous mode on regular channels"]
pub type DISRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DISRC_A>;
impl<'a, REG, const O: u8> DISRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISRC_A::DISABLED)
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISRC_A::ENABLED)
    }
}
#[doc = "Field `DISIC` reader - Discontinuous mode on inserted channels"]
pub type DISIC_R = crate::BitReader<DISIC_A>;
#[doc = "Discontinuous mode on inserted channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISIC_A {
    #[doc = "0: Discontinuous mode on inserted channels disabled"]
    DISABLED = 0,
    #[doc = "1: Discontinuous mode on inserted channels enabled"]
    ENABLED = 1,
}
impl From<DISIC_A> for bool {
    #[inline(always)]
    fn from(variant: DISIC_A) -> Self {
        variant as u8 != 0
    }
}
impl DISIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISIC_A {
        match self.bits {
            false => DISIC_A::DISABLED,
            true => DISIC_A::ENABLED,
        }
    }
    #[doc = "Discontinuous mode on inserted channels disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISIC_A::DISABLED
    }
    #[doc = "Discontinuous mode on inserted channels enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISIC_A::ENABLED
    }
}
#[doc = "Field `DISIC` writer - Discontinuous mode on inserted channels"]
pub type DISIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DISIC_A>;
impl<'a, REG, const O: u8> DISIC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discontinuous mode on inserted channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISIC_A::DISABLED)
    }
    #[doc = "Discontinuous mode on inserted channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISIC_A::ENABLED)
    }
}
#[doc = "Field `DISNUM` reader - Number of conversions in discontinuous mode"]
pub type DISNUM_R = crate::FieldReader;
#[doc = "Field `DISNUM` writer - Number of conversions in discontinuous mode"]
pub type DISNUM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O>;
#[doc = "Field `SYNCM` reader - sync mode selection"]
pub type SYNCM_R = crate::FieldReader;
#[doc = "Field `SYNCM` writer - sync mode selection"]
pub type SYNCM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `IWDEN` reader - Inserted channel analog watchdog enable"]
pub type IWDEN_R = crate::BitReader<IWDEN_A>;
#[doc = "Inserted channel analog watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDEN_A {
    #[doc = "0: Analog watchdog disabled on inserted channels"]
    DISABLED = 0,
    #[doc = "1: Analog watchdog enabled on inserted channels"]
    ENABLED = 1,
}
impl From<IWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: IWDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDEN_A {
        match self.bits {
            false => IWDEN_A::DISABLED,
            true => IWDEN_A::ENABLED,
        }
    }
    #[doc = "Analog watchdog disabled on inserted channels"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IWDEN_A::DISABLED
    }
    #[doc = "Analog watchdog enabled on inserted channels"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IWDEN_A::ENABLED
    }
}
#[doc = "Field `IWDEN` writer - Inserted channel analog watchdog enable"]
pub type IWDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IWDEN_A>;
impl<'a, REG, const O: u8> IWDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog disabled on inserted channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IWDEN_A::DISABLED)
    }
    #[doc = "Analog watchdog enabled on inserted channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IWDEN_A::ENABLED)
    }
}
#[doc = "Field `RWDEN` reader - Regular channel analog watchdog enable"]
pub type RWDEN_R = crate::BitReader<RWDEN_A>;
#[doc = "Regular channel analog watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWDEN_A {
    #[doc = "0: Analog watchdog disabled on regular channels"]
    DISABLED = 0,
    #[doc = "1: Analog watchdog enabled on regular channels"]
    ENABLED = 1,
}
impl From<RWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: RWDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RWDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWDEN_A {
        match self.bits {
            false => RWDEN_A::DISABLED,
            true => RWDEN_A::ENABLED,
        }
    }
    #[doc = "Analog watchdog disabled on regular channels"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RWDEN_A::DISABLED
    }
    #[doc = "Analog watchdog enabled on regular channels"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RWDEN_A::ENABLED
    }
}
#[doc = "Field `RWDEN` writer - Regular channel analog watchdog enable"]
pub type RWDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RWDEN_A>;
impl<'a, REG, const O: u8> RWDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog disabled on regular channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RWDEN_A::DISABLED)
    }
    #[doc = "Analog watchdog enabled on regular channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RWDEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog watchdog channel select"]
    #[inline(always)]
    pub fn wdchsel(&self) -> WDCHSEL_R {
        WDCHSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable for WDE"]
    #[inline(always)]
    pub fn wdeie(&self) -> WDEIE_R {
        WDEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    pub fn eoicie(&self) -> EOICIE_R {
        EOICIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog is effective on a single channel"]
    #[inline(always)]
    pub fn wdsc(&self) -> WDSC_R {
        WDSC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Inserted channel group convert automatically"]
    #[inline(always)]
    pub fn ica(&self) -> ICA_R {
        ICA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn disrc(&self) -> DISRC_R {
        DISRC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Discontinuous mode on inserted channels"]
    #[inline(always)]
    pub fn disic(&self) -> DISIC_R {
        DISIC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    pub fn disnum(&self) -> DISNUM_R {
        DISNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - sync mode selection"]
    #[inline(always)]
    pub fn syncm(&self) -> SYNCM_R {
        SYNCM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog enable"]
    #[inline(always)]
    pub fn iwden(&self) -> IWDEN_R {
        IWDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Regular channel analog watchdog enable"]
    #[inline(always)]
    pub fn rwden(&self) -> RWDEN_R {
        RWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog watchdog channel select"]
    #[inline(always)]
    #[must_use]
    pub fn wdchsel(&mut self) -> WDCHSEL_W<CTL0_SPEC, 0> {
        WDCHSEL_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<CTL0_SPEC, 5> {
        EOCIE_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt enable for WDE"]
    #[inline(always)]
    #[must_use]
    pub fn wdeie(&mut self) -> WDEIE_W<CTL0_SPEC, 6> {
        WDEIE_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    #[must_use]
    pub fn eoicie(&mut self) -> EOICIE_W<CTL0_SPEC, 7> {
        EOICIE_W::new(self)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<CTL0_SPEC, 8> {
        SM_W::new(self)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog is effective on a single channel"]
    #[inline(always)]
    #[must_use]
    pub fn wdsc(&mut self) -> WDSC_W<CTL0_SPEC, 9> {
        WDSC_W::new(self)
    }
    #[doc = "Bit 10 - Inserted channel group convert automatically"]
    #[inline(always)]
    #[must_use]
    pub fn ica(&mut self) -> ICA_W<CTL0_SPEC, 10> {
        ICA_W::new(self)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn disrc(&mut self) -> DISRC_W<CTL0_SPEC, 11> {
        DISRC_W::new(self)
    }
    #[doc = "Bit 12 - Discontinuous mode on inserted channels"]
    #[inline(always)]
    #[must_use]
    pub fn disic(&mut self) -> DISIC_W<CTL0_SPEC, 12> {
        DISIC_W::new(self)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn disnum(&mut self) -> DISNUM_W<CTL0_SPEC, 13> {
        DISNUM_W::new(self)
    }
    #[doc = "Bits 16:19 - sync mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn syncm(&mut self) -> SYNCM_W<CTL0_SPEC, 16> {
        SYNCM_W::new(self)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn iwden(&mut self) -> IWDEN_W<CTL0_SPEC, 22> {
        IWDEN_W::new(self)
    }
    #[doc = "Bit 23 - Regular channel analog watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwden(&mut self) -> RWDEN_W<CTL0_SPEC, 23> {
        RWDEN_W::new(self)
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
#[doc = "control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
