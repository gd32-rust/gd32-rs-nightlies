#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Regular channel analog watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RWDEN` reader - Regular channel analog watchdog enable"]
pub type RWDEN_R = crate::BitReader<RWDEN_A>;
impl RWDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWDEN_A {
        match self.bits {
            false => RWDEN_A::DISABLED,
            true => RWDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RWDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RWDEN_A::ENABLED
    }
}
#[doc = "Field `RWDEN` writer - Regular channel analog watchdog enable"]
pub type RWDEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, RWDEN_A, 23>;
impl<'a> RWDEN_W<'a> {
    #[doc = "Analog watchdog disabled on regular channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RWDEN_A::DISABLED)
    }
    #[doc = "Analog watchdog enabled on regular channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RWDEN_A::ENABLED)
    }
}
#[doc = "Inserted channel analog watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `IWDEN` reader - Inserted channel analog watchdog enable"]
pub type IWDEN_R = crate::BitReader<IWDEN_A>;
impl IWDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDEN_A {
        match self.bits {
            false => IWDEN_A::DISABLED,
            true => IWDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IWDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IWDEN_A::ENABLED
    }
}
#[doc = "Field `IWDEN` writer - Inserted channel analog watchdog enable"]
pub type IWDEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, IWDEN_A, 22>;
impl<'a> IWDEN_W<'a> {
    #[doc = "Analog watchdog disabled on inserted channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IWDEN_A::DISABLED)
    }
    #[doc = "Analog watchdog enabled on inserted channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IWDEN_A::ENABLED)
    }
}
#[doc = "Field `SYNCM` reader - sync mode selection"]
pub type SYNCM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCM` writer - sync mode selection"]
pub type SYNCM_W<'a> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 4, 16>;
#[doc = "Field `DISNUM` reader - Number of conversions in discontinuous mode"]
pub type DISNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISNUM` writer - Number of conversions in discontinuous mode"]
pub type DISNUM_W<'a> = crate::FieldWriterSafe<'a, u32, CTL0_SPEC, u8, u8, 3, 13>;
#[doc = "Discontinuous mode on inserted channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DISIC` reader - Discontinuous mode on inserted channels"]
pub type DISIC_R = crate::BitReader<DISIC_A>;
impl DISIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISIC_A {
        match self.bits {
            false => DISIC_A::DISABLED,
            true => DISIC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISIC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISIC_A::ENABLED
    }
}
#[doc = "Field `DISIC` writer - Discontinuous mode on inserted channels"]
pub type DISIC_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, DISIC_A, 12>;
impl<'a> DISIC_W<'a> {
    #[doc = "Discontinuous mode on inserted channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISIC_A::DISABLED)
    }
    #[doc = "Discontinuous mode on inserted channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISIC_A::ENABLED)
    }
}
#[doc = "Discontinuous mode on regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DISRC` reader - Discontinuous mode on regular channels"]
pub type DISRC_R = crate::BitReader<DISRC_A>;
impl DISRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISRC_A {
        match self.bits {
            false => DISRC_A::DISABLED,
            true => DISRC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISRC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISRC_A::ENABLED
    }
}
#[doc = "Field `DISRC` writer - Discontinuous mode on regular channels"]
pub type DISRC_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, DISRC_A, 11>;
impl<'a> DISRC_W<'a> {
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISRC_A::DISABLED)
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISRC_A::ENABLED)
    }
}
#[doc = "Inserted channel group convert automatically\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ICA` reader - Inserted channel group convert automatically"]
pub type ICA_R = crate::BitReader<ICA_A>;
impl ICA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICA_A {
        match self.bits {
            false => ICA_A::DISABLED,
            true => ICA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICA_A::ENABLED
    }
}
#[doc = "Field `ICA` writer - Inserted channel group convert automatically"]
pub type ICA_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, ICA_A, 10>;
impl<'a> ICA_W<'a> {
    #[doc = "Automatic inserted group conversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ICA_A::DISABLED)
    }
    #[doc = "Automatic inserted group conversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ICA_A::ENABLED)
    }
}
#[doc = "When in scan mode, analog watchdog is effective on a single channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `WDSC` reader - When in scan mode, analog watchdog is effective on a single channel"]
pub type WDSC_R = crate::BitReader<WDSC_A>;
impl WDSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDSC_A {
        match self.bits {
            false => WDSC_A::ALL,
            true => WDSC_A::SINGLE,
        }
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == WDSC_A::ALL
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == WDSC_A::SINGLE
    }
}
#[doc = "Field `WDSC` writer - When in scan mode, analog watchdog is effective on a single channel"]
pub type WDSC_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, WDSC_A, 9>;
impl<'a> WDSC_W<'a> {
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(WDSC_A::ALL)
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(WDSC_A::SINGLE)
    }
}
#[doc = "Scan mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SM` reader - Scan mode"]
pub type SM_R = crate::BitReader<SM_A>;
impl SM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM_A {
        match self.bits {
            false => SM_A::DISABLED,
            true => SM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SM_A::ENABLED
    }
}
#[doc = "Field `SM` writer - Scan mode"]
pub type SM_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, SM_A, 8>;
impl<'a> SM_W<'a> {
    #[doc = "Scan mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SM_A::DISABLED)
    }
    #[doc = "Scan mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SM_A::ENABLED)
    }
}
#[doc = "Interrupt enable for EOIC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EOICIE` reader - Interrupt enable for EOIC"]
pub type EOICIE_R = crate::BitReader<EOICIE_A>;
impl EOICIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOICIE_A {
        match self.bits {
            false => EOICIE_A::DISABLED,
            true => EOICIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOICIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOICIE_A::ENABLED
    }
}
#[doc = "Field `EOICIE` writer - Interrupt enable for EOIC"]
pub type EOICIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, EOICIE_A, 7>;
impl<'a> EOICIE_W<'a> {
    #[doc = "EOIC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOICIE_A::DISABLED)
    }
    #[doc = "EOIC interrupt enabled. An interrupt is generated when the EOIC bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOICIE_A::ENABLED)
    }
}
#[doc = "Interrupt enable for WDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `WDEIE` reader - Interrupt enable for WDE"]
pub type WDEIE_R = crate::BitReader<WDEIE_A>;
impl WDEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDEIE_A {
        match self.bits {
            false => WDEIE_A::DISABLED,
            true => WDEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDEIE_A::ENABLED
    }
}
#[doc = "Field `WDEIE` writer - Interrupt enable for WDE"]
pub type WDEIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, WDEIE_A, 6>;
impl<'a> WDEIE_W<'a> {
    #[doc = "WDE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDEIE_A::DISABLED)
    }
    #[doc = "WDE interrupt enabled. An interrupt is generated when the WDE bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDEIE_A::ENABLED)
    }
}
#[doc = "Interrupt enable for EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EOCIE` reader - Interrupt enable for EOC"]
pub type EOCIE_R = crate::BitReader<EOCIE_A>;
impl EOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCIE_A {
        match self.bits {
            false => EOCIE_A::DISABLED,
            true => EOCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIE_A::ENABLED
    }
}
#[doc = "Field `EOCIE` writer - Interrupt enable for EOC"]
pub type EOCIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, EOCIE_A, 5>;
impl<'a> EOCIE_W<'a> {
    #[doc = "EOC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOCIE_A::DISABLED)
    }
    #[doc = "EOC interrupt enabled. An interrupt is generated when the EOC bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOCIE_A::ENABLED)
    }
}
#[doc = "Field `WDCHSEL` reader - Analog watchdog channel select"]
pub type WDCHSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDCHSEL` writer - Analog watchdog channel select"]
pub type WDCHSEL_W<'a> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bit 23 - Regular channel analog watchdog enable"]
    #[inline(always)]
    pub fn rwden(&self) -> RWDEN_R {
        RWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog enable"]
    #[inline(always)]
    pub fn iwden(&self) -> IWDEN_R {
        IWDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 16:19 - sync mode selection"]
    #[inline(always)]
    pub fn syncm(&self) -> SYNCM_R {
        SYNCM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    pub fn disnum(&self) -> DISNUM_R {
        DISNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 12 - Discontinuous mode on inserted channels"]
    #[inline(always)]
    pub fn disic(&self) -> DISIC_R {
        DISIC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn disrc(&self) -> DISRC_R {
        DISRC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Inserted channel group convert automatically"]
    #[inline(always)]
    pub fn ica(&self) -> ICA_R {
        ICA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog is effective on a single channel"]
    #[inline(always)]
    pub fn wdsc(&self) -> WDSC_R {
        WDSC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    pub fn eoicie(&self) -> EOICIE_R {
        EOICIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable for WDE"]
    #[inline(always)]
    pub fn wdeie(&self) -> WDEIE_R {
        WDEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 0:4 - Analog watchdog channel select"]
    #[inline(always)]
    pub fn wdchsel(&self) -> WDCHSEL_R {
        WDCHSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 23 - Regular channel analog watchdog enable"]
    #[inline(always)]
    pub fn rwden(&mut self) -> RWDEN_W {
        RWDEN_W::new(self)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog enable"]
    #[inline(always)]
    pub fn iwden(&mut self) -> IWDEN_W {
        IWDEN_W::new(self)
    }
    #[doc = "Bits 16:19 - sync mode selection"]
    #[inline(always)]
    pub fn syncm(&mut self) -> SYNCM_W {
        SYNCM_W::new(self)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    pub fn disnum(&mut self) -> DISNUM_W {
        DISNUM_W::new(self)
    }
    #[doc = "Bit 12 - Discontinuous mode on inserted channels"]
    #[inline(always)]
    pub fn disic(&mut self) -> DISIC_W {
        DISIC_W::new(self)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn disrc(&mut self) -> DISRC_W {
        DISRC_W::new(self)
    }
    #[doc = "Bit 10 - Inserted channel group convert automatically"]
    #[inline(always)]
    pub fn ica(&mut self) -> ICA_W {
        ICA_W::new(self)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog is effective on a single channel"]
    #[inline(always)]
    pub fn wdsc(&mut self) -> WDSC_W {
        WDSC_W::new(self)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn sm(&mut self) -> SM_W {
        SM_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    pub fn eoicie(&mut self) -> EOICIE_W {
        EOICIE_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt enable for WDE"]
    #[inline(always)]
    pub fn wdeie(&mut self) -> WDEIE_W {
        WDEIE_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W {
        EOCIE_W::new(self)
    }
    #[doc = "Bits 0:4 - Analog watchdog channel select"]
    #[inline(always)]
    pub fn wdchsel(&mut self) -> WDCHSEL_W {
        WDCHSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
