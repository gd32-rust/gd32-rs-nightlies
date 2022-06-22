#[doc = "Register `SMCFG` reader"]
pub struct R(crate::R<SMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCFG` writer"]
pub struct W(crate::W<SMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCFG_SPEC>;
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
impl From<crate::W<SMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External trigger polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETP_A {
    #[doc = "0: ETI is noninverted, active at high level or rising edge"]
    NOTINVERTED = 0,
    #[doc = "1: ETI is inverted, active at low level or falling edge"]
    INVERTED = 1,
}
impl From<ETP_A> for bool {
    #[inline(always)]
    fn from(variant: ETP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETP` reader - External trigger polarity"]
pub type ETP_R = crate::BitReader<ETP_A>;
impl ETP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETP_A {
        match self.bits {
            false => ETP_A::NOTINVERTED,
            true => ETP_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == ETP_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == ETP_A::INVERTED
    }
}
#[doc = "Field `ETP` writer - External trigger polarity"]
pub type ETP_W<'a> = crate::BitWriter<'a, u32, SMCFG_SPEC, ETP_A, 15>;
impl<'a> ETP_W<'a> {
    #[doc = "ETI is noninverted, active at high level or rising edge"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(ETP_A::NOTINVERTED)
    }
    #[doc = "ETI is inverted, active at low level or falling edge"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(ETP_A::INVERTED)
    }
}
#[doc = "Part of SMC for enable External clock mode1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMC1_A {
    #[doc = "0: External clock mode 1 disabled"]
    DISABLED = 0,
    #[doc = "1: External clock mode 1 enabled. The counter is clocked by any active edge on the ETIF signal."]
    ENABLED = 1,
}
impl From<SMC1_A> for bool {
    #[inline(always)]
    fn from(variant: SMC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMC1` reader - Part of SMC for enable External clock mode1"]
pub type SMC1_R = crate::BitReader<SMC1_A>;
impl SMC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMC1_A {
        match self.bits {
            false => SMC1_A::DISABLED,
            true => SMC1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMC1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMC1_A::ENABLED
    }
}
#[doc = "Field `SMC1` writer - Part of SMC for enable External clock mode1"]
pub type SMC1_W<'a> = crate::BitWriter<'a, u32, SMCFG_SPEC, SMC1_A, 14>;
impl<'a> SMC1_W<'a> {
    #[doc = "External clock mode 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMC1_A::DISABLED)
    }
    #[doc = "External clock mode 1 enabled. The counter is clocked by any active edge on the ETIF signal."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMC1_A::ENABLED)
    }
}
#[doc = "External trigger prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETPSC_A {
    #[doc = "0: External trigger prescaler disabled"]
    DIV1 = 0,
    #[doc = "1: ETI frequency divided by 2"]
    DIV2 = 1,
    #[doc = "2: ETI frequency divided by 4"]
    DIV4 = 2,
    #[doc = "3: ETI frequency divided by 8"]
    DIV8 = 3,
}
impl From<ETPSC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETPSC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ETPSC` reader - External trigger prescaler"]
pub type ETPSC_R = crate::FieldReader<u8, ETPSC_A>;
impl ETPSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETPSC_A {
        match self.bits {
            0 => ETPSC_A::DIV1,
            1 => ETPSC_A::DIV2,
            2 => ETPSC_A::DIV4,
            3 => ETPSC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == ETPSC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ETPSC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ETPSC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ETPSC_A::DIV8
    }
}
#[doc = "Field `ETPSC` writer - External trigger prescaler"]
pub type ETPSC_W<'a> = crate::FieldWriterSafe<'a, u32, SMCFG_SPEC, u8, ETPSC_A, 2, 12>;
impl<'a> ETPSC_W<'a> {
    #[doc = "External trigger prescaler disabled"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(ETPSC_A::DIV1)
    }
    #[doc = "ETI frequency divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ETPSC_A::DIV2)
    }
    #[doc = "ETI frequency divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ETPSC_A::DIV4)
    }
    #[doc = "ETI frequency divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ETPSC_A::DIV8)
    }
}
#[doc = "External trigger filter control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETFC_A {
    #[doc = "0: Filter disabled. fSAMP=fDTS, N=1"]
    NOFILTER = 0,
    #[doc = "1: fSAMP=fTIMER_CK, N=2"]
    TIMERCK_N2 = 1,
    #[doc = "2: fSAMP=fTIMER_CK, N=4"]
    TIMERCK_N4 = 2,
    #[doc = "3: fSAMP=fTIMER_CK, N=8"]
    TIMERCK_N8 = 3,
    #[doc = "4: fSAMP=fDTS/2, N=6"]
    FDTS_DIV2_N6 = 4,
    #[doc = "5: fSAMP=fDTS/2, N=8"]
    FDTS_DIV2_N8 = 5,
    #[doc = "6: fSAMP=fDTS/4, N=6"]
    FDTS_DIV4_N6 = 6,
    #[doc = "7: fSAMP=fDTS/4, N=8"]
    FDTS_DIV4_N8 = 7,
    #[doc = "8: fSAMP=fDTS/8, N=6"]
    FDTS_DIV8_N6 = 8,
    #[doc = "9: fSAMP=fDTS/8, N=8"]
    FDTS_DIV8_N8 = 9,
    #[doc = "10: fSAMP=fDTS/16, N=5"]
    FDTS_DIV16_N5 = 10,
    #[doc = "11: fSAMP=fDTS/16, N=6"]
    FDTS_DIV16_N6 = 11,
    #[doc = "12: fSAMP=fDTS/16, N=8"]
    FDTS_DIV16_N8 = 12,
    #[doc = "13: fSAMP=fDTS/32, N=5"]
    FDTS_DIV32_N5 = 13,
    #[doc = "14: fSAMP=fDTS/32, N=6"]
    FDTS_DIV32_N6 = 14,
    #[doc = "15: fSAMP=fDTS/32, N=8"]
    FDTS_DIV32_N8 = 15,
}
impl From<ETFC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETFC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ETFC` reader - External trigger filter control"]
pub type ETFC_R = crate::FieldReader<u8, ETFC_A>;
impl ETFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETFC_A {
        match self.bits {
            0 => ETFC_A::NOFILTER,
            1 => ETFC_A::TIMERCK_N2,
            2 => ETFC_A::TIMERCK_N4,
            3 => ETFC_A::TIMERCK_N8,
            4 => ETFC_A::FDTS_DIV2_N6,
            5 => ETFC_A::FDTS_DIV2_N8,
            6 => ETFC_A::FDTS_DIV4_N6,
            7 => ETFC_A::FDTS_DIV4_N8,
            8 => ETFC_A::FDTS_DIV8_N6,
            9 => ETFC_A::FDTS_DIV8_N8,
            10 => ETFC_A::FDTS_DIV16_N5,
            11 => ETFC_A::FDTS_DIV16_N6,
            12 => ETFC_A::FDTS_DIV16_N8,
            13 => ETFC_A::FDTS_DIV32_N5,
            14 => ETFC_A::FDTS_DIV32_N6,
            15 => ETFC_A::FDTS_DIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOFILTER`"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == ETFC_A::NOFILTER
    }
    #[doc = "Checks if the value of the field is `TIMERCK_N2`"]
    #[inline(always)]
    pub fn is_timer_ck_n2(&self) -> bool {
        *self == ETFC_A::TIMERCK_N2
    }
    #[doc = "Checks if the value of the field is `TIMERCK_N4`"]
    #[inline(always)]
    pub fn is_timer_ck_n4(&self) -> bool {
        *self == ETFC_A::TIMERCK_N4
    }
    #[doc = "Checks if the value of the field is `TIMERCK_N8`"]
    #[inline(always)]
    pub fn is_timer_ck_n8(&self) -> bool {
        *self == ETFC_A::TIMERCK_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV2_N6`"]
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == ETFC_A::FDTS_DIV2_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV2_N8`"]
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == ETFC_A::FDTS_DIV2_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV4_N6`"]
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == ETFC_A::FDTS_DIV4_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV4_N8`"]
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == ETFC_A::FDTS_DIV4_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV8_N6`"]
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == ETFC_A::FDTS_DIV8_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV8_N8`"]
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == ETFC_A::FDTS_DIV8_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N5`"]
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == ETFC_A::FDTS_DIV16_N5
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N6`"]
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == ETFC_A::FDTS_DIV16_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N8`"]
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == ETFC_A::FDTS_DIV16_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N5`"]
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == ETFC_A::FDTS_DIV32_N5
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N6`"]
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == ETFC_A::FDTS_DIV32_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N8`"]
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == ETFC_A::FDTS_DIV32_N8
    }
}
#[doc = "Field `ETFC` writer - External trigger filter control"]
pub type ETFC_W<'a> = crate::FieldWriterSafe<'a, u32, SMCFG_SPEC, u8, ETFC_A, 4, 8>;
impl<'a> ETFC_W<'a> {
    #[doc = "Filter disabled. fSAMP=fDTS, N=1"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(ETFC_A::NOFILTER)
    }
    #[doc = "fSAMP=fTIMER_CK, N=2"]
    #[inline(always)]
    pub fn timer_ck_n2(self) -> &'a mut W {
        self.variant(ETFC_A::TIMERCK_N2)
    }
    #[doc = "fSAMP=fTIMER_CK, N=4"]
    #[inline(always)]
    pub fn timer_ck_n4(self) -> &'a mut W {
        self.variant(ETFC_A::TIMERCK_N4)
    }
    #[doc = "fSAMP=fTIMER_CK, N=8"]
    #[inline(always)]
    pub fn timer_ck_n8(self) -> &'a mut W {
        self.variant(ETFC_A::TIMERCK_N8)
    }
    #[doc = "fSAMP=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(ETFC_A::FDTS_DIV2_N6)
    }
    #[doc = "fSAMP=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(ETFC_A::FDTS_DIV2_N8)
    }
    #[doc = "fSAMP=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(ETFC_A::FDTS_DIV4_N6)
    }
    #[doc = "fSAMP=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(ETFC_A::FDTS_DIV4_N8)
    }
    #[doc = "fSAMP=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(ETFC_A::FDTS_DIV8_N6)
    }
    #[doc = "fSAMP=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(ETFC_A::FDTS_DIV8_N8)
    }
    #[doc = "fSAMP=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(ETFC_A::FDTS_DIV16_N5)
    }
    #[doc = "fSAMP=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(ETFC_A::FDTS_DIV16_N6)
    }
    #[doc = "fSAMP=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(ETFC_A::FDTS_DIV16_N8)
    }
    #[doc = "fSAMP=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(ETFC_A::FDTS_DIV32_N5)
    }
    #[doc = "fSAMP=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(ETFC_A::FDTS_DIV32_N6)
    }
    #[doc = "fSAMP=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(ETFC_A::FDTS_DIV32_N8)
    }
}
#[doc = "Master-slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSM_A {
    #[doc = "0: No action"]
    NOSYNC = 0,
    #[doc = "1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    SYNC = 1,
}
impl From<MSM_A> for bool {
    #[inline(always)]
    fn from(variant: MSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSM` reader - Master-slave mode"]
pub type MSM_R = crate::BitReader<MSM_A>;
impl MSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSM_A {
        match self.bits {
            false => MSM_A::NOSYNC,
            true => MSM_A::SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        *self == MSM_A::NOSYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == MSM_A::SYNC
    }
}
#[doc = "Field `MSM` writer - Master-slave mode"]
pub type MSM_W<'a> = crate::BitWriter<'a, u32, SMCFG_SPEC, MSM_A, 7>;
impl<'a> MSM_W<'a> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_sync(self) -> &'a mut W {
        self.variant(MSM_A::NOSYNC)
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(MSM_A::SYNC)
    }
}
#[doc = "Trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGS_A {
    #[doc = "0: Internal Trigger 0 (ITI0)"]
    ITI0 = 0,
    #[doc = "1: Internal Trigger 1 (ITI1)"]
    ITI1 = 1,
    #[doc = "2: Internal Trigger 2 (ITI2)"]
    ITI2 = 2,
    #[doc = "4: CI0 Edge Detector (CI0F_ED)"]
    CI0F_ED = 4,
    #[doc = "5: Filtered Timer Input 0 (CI0FE0)"]
    CI0FE0 = 5,
    #[doc = "6: Filtered Timer Input 1 (CI1FE1)"]
    CI1FE1 = 6,
    #[doc = "7: External Trigger input (ETIFP)"]
    ETIFP = 7,
}
impl From<TRGS_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGS` reader - Trigger selection"]
pub type TRGS_R = crate::FieldReader<u8, TRGS_A>;
impl TRGS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRGS_A> {
        match self.bits {
            0 => Some(TRGS_A::ITI0),
            1 => Some(TRGS_A::ITI1),
            2 => Some(TRGS_A::ITI2),
            4 => Some(TRGS_A::CI0F_ED),
            5 => Some(TRGS_A::CI0FE0),
            6 => Some(TRGS_A::CI1FE1),
            7 => Some(TRGS_A::ETIFP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ITI0`"]
    #[inline(always)]
    pub fn is_iti0(&self) -> bool {
        *self == TRGS_A::ITI0
    }
    #[doc = "Checks if the value of the field is `ITI1`"]
    #[inline(always)]
    pub fn is_iti1(&self) -> bool {
        *self == TRGS_A::ITI1
    }
    #[doc = "Checks if the value of the field is `ITI2`"]
    #[inline(always)]
    pub fn is_iti2(&self) -> bool {
        *self == TRGS_A::ITI2
    }
    #[doc = "Checks if the value of the field is `CI0F_ED`"]
    #[inline(always)]
    pub fn is_ci0f_ed(&self) -> bool {
        *self == TRGS_A::CI0F_ED
    }
    #[doc = "Checks if the value of the field is `CI0FE0`"]
    #[inline(always)]
    pub fn is_ci0fe0(&self) -> bool {
        *self == TRGS_A::CI0FE0
    }
    #[doc = "Checks if the value of the field is `CI1FE1`"]
    #[inline(always)]
    pub fn is_ci1fe1(&self) -> bool {
        *self == TRGS_A::CI1FE1
    }
    #[doc = "Checks if the value of the field is `ETIFP`"]
    #[inline(always)]
    pub fn is_etifp(&self) -> bool {
        *self == TRGS_A::ETIFP
    }
}
#[doc = "Field `TRGS` writer - Trigger selection"]
pub type TRGS_W<'a> = crate::FieldWriter<'a, u32, SMCFG_SPEC, u8, TRGS_A, 3, 4>;
impl<'a> TRGS_W<'a> {
    #[doc = "Internal Trigger 0 (ITI0)"]
    #[inline(always)]
    pub fn iti0(self) -> &'a mut W {
        self.variant(TRGS_A::ITI0)
    }
    #[doc = "Internal Trigger 1 (ITI1)"]
    #[inline(always)]
    pub fn iti1(self) -> &'a mut W {
        self.variant(TRGS_A::ITI1)
    }
    #[doc = "Internal Trigger 2 (ITI2)"]
    #[inline(always)]
    pub fn iti2(self) -> &'a mut W {
        self.variant(TRGS_A::ITI2)
    }
    #[doc = "CI0 Edge Detector (CI0F_ED)"]
    #[inline(always)]
    pub fn ci0f_ed(self) -> &'a mut W {
        self.variant(TRGS_A::CI0F_ED)
    }
    #[doc = "Filtered Timer Input 0 (CI0FE0)"]
    #[inline(always)]
    pub fn ci0fe0(self) -> &'a mut W {
        self.variant(TRGS_A::CI0FE0)
    }
    #[doc = "Filtered Timer Input 1 (CI1FE1)"]
    #[inline(always)]
    pub fn ci1fe1(self) -> &'a mut W {
        self.variant(TRGS_A::CI1FE1)
    }
    #[doc = "External Trigger input (ETIFP)"]
    #[inline(always)]
    pub fn etifp(self) -> &'a mut W {
        self.variant(TRGS_A::ETIFP)
    }
}
#[doc = "Slave mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMC_A {
    #[doc = "0: Slave mode disabled - if CEN=1 then the prescaler is clocked directly by the internal clock."]
    DISABLED = 0,
    #[doc = "1: Quadrature decoder mode 0 - Counter counts up/down on CI1FE1 edge depending on CI0FE0 level."]
    QUADRATUREDECODERMODE0 = 1,
    #[doc = "2: Quadrature decoder mode 1 - Counter counts up/down on CI0FE0 edge depending on CI1FE1 level."]
    QUADRATUREDECODERMODE1 = 2,
    #[doc = "3: Quadrature decoder mode 2 - Counter counts up/down on both CI0FE0 and CI1FE1 edges depending on the level of the other input."]
    QUADRATUREDECODERMODE2 = 3,
    #[doc = "4: Restart Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    RESTARTMODE = 4,
    #[doc = "5: Pause Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    PAUSEMODE = 5,
    #[doc = "6: Event Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    EVENTMODE = 6,
    #[doc = "7: External Clock Mode 0 - Rising edges of the selected trigger (TRGI) clock the counter."]
    EXTERNALCLOCKMODE = 7,
}
impl From<SMC_A> for u8 {
    #[inline(always)]
    fn from(variant: SMC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMC` reader - Slave mode control"]
pub type SMC_R = crate::FieldReader<u8, SMC_A>;
impl SMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMC_A {
        match self.bits {
            0 => SMC_A::DISABLED,
            1 => SMC_A::QUADRATUREDECODERMODE0,
            2 => SMC_A::QUADRATUREDECODERMODE1,
            3 => SMC_A::QUADRATUREDECODERMODE2,
            4 => SMC_A::RESTARTMODE,
            5 => SMC_A::PAUSEMODE,
            6 => SMC_A::EVENTMODE,
            7 => SMC_A::EXTERNALCLOCKMODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `QUADRATUREDECODERMODE0`"]
    #[inline(always)]
    pub fn is_quadrature_decoder_mode0(&self) -> bool {
        *self == SMC_A::QUADRATUREDECODERMODE0
    }
    #[doc = "Checks if the value of the field is `QUADRATUREDECODERMODE1`"]
    #[inline(always)]
    pub fn is_quadrature_decoder_mode1(&self) -> bool {
        *self == SMC_A::QUADRATUREDECODERMODE1
    }
    #[doc = "Checks if the value of the field is `QUADRATUREDECODERMODE2`"]
    #[inline(always)]
    pub fn is_quadrature_decoder_mode2(&self) -> bool {
        *self == SMC_A::QUADRATUREDECODERMODE2
    }
    #[doc = "Checks if the value of the field is `RESTARTMODE`"]
    #[inline(always)]
    pub fn is_restart_mode(&self) -> bool {
        *self == SMC_A::RESTARTMODE
    }
    #[doc = "Checks if the value of the field is `PAUSEMODE`"]
    #[inline(always)]
    pub fn is_pause_mode(&self) -> bool {
        *self == SMC_A::PAUSEMODE
    }
    #[doc = "Checks if the value of the field is `EVENTMODE`"]
    #[inline(always)]
    pub fn is_event_mode(&self) -> bool {
        *self == SMC_A::EVENTMODE
    }
    #[doc = "Checks if the value of the field is `EXTERNALCLOCKMODE`"]
    #[inline(always)]
    pub fn is_external_clock_mode(&self) -> bool {
        *self == SMC_A::EXTERNALCLOCKMODE
    }
}
#[doc = "Field `SMC` writer - Slave mode control"]
pub type SMC_W<'a> = crate::FieldWriterSafe<'a, u32, SMCFG_SPEC, u8, SMC_A, 3, 0>;
impl<'a> SMC_W<'a> {
    #[doc = "Slave mode disabled - if CEN=1 then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMC_A::DISABLED)
    }
    #[doc = "Quadrature decoder mode 0 - Counter counts up/down on CI1FE1 edge depending on CI0FE0 level."]
    #[inline(always)]
    pub fn quadrature_decoder_mode0(self) -> &'a mut W {
        self.variant(SMC_A::QUADRATUREDECODERMODE0)
    }
    #[doc = "Quadrature decoder mode 1 - Counter counts up/down on CI0FE0 edge depending on CI1FE1 level."]
    #[inline(always)]
    pub fn quadrature_decoder_mode1(self) -> &'a mut W {
        self.variant(SMC_A::QUADRATUREDECODERMODE1)
    }
    #[doc = "Quadrature decoder mode 2 - Counter counts up/down on both CI0FE0 and CI1FE1 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn quadrature_decoder_mode2(self) -> &'a mut W {
        self.variant(SMC_A::QUADRATUREDECODERMODE2)
    }
    #[doc = "Restart Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn restart_mode(self) -> &'a mut W {
        self.variant(SMC_A::RESTARTMODE)
    }
    #[doc = "Pause Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn pause_mode(self) -> &'a mut W {
        self.variant(SMC_A::PAUSEMODE)
    }
    #[doc = "Event Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn event_mode(self) -> &'a mut W {
        self.variant(SMC_A::EVENTMODE)
    }
    #[doc = "External Clock Mode 0 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn external_clock_mode(self) -> &'a mut W {
        self.variant(SMC_A::EXTERNALCLOCKMODE)
    }
}
impl R {
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Part of SMC for enable External clock mode1"]
    #[inline(always)]
    pub fn smc1(&self) -> SMC1_R {
        SMC1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etpsc(&self) -> ETPSC_R {
        ETPSC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 8:11 - External trigger filter control"]
    #[inline(always)]
    pub fn etfc(&self) -> ETFC_R {
        ETFC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Master-slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&self) -> TRGS_R {
        TRGS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 0:2 - Slave mode control"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W {
        ETP_W::new(self)
    }
    #[doc = "Bit 14 - Part of SMC for enable External clock mode1"]
    #[inline(always)]
    pub fn smc1(&mut self) -> SMC1_W {
        SMC1_W::new(self)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etpsc(&mut self) -> ETPSC_W {
        ETPSC_W::new(self)
    }
    #[doc = "Bits 8:11 - External trigger filter control"]
    #[inline(always)]
    pub fn etfc(&mut self) -> ETFC_W {
        ETFC_W::new(self)
    }
    #[doc = "Bit 7 - Master-slave mode"]
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W {
        MSM_W::new(self)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&mut self) -> TRGS_W {
        TRGS_W::new(self)
    }
    #[doc = "Bits 0:2 - Slave mode control"]
    #[inline(always)]
    pub fn smc(&mut self) -> SMC_W {
        SMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcfg](index.html) module"]
pub struct SMCFG_SPEC;
impl crate::RegisterSpec for SMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smcfg::R](R) reader structure"]
impl crate::Readable for SMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcfg::W](W) writer structure"]
impl crate::Writable for SMCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMCFG to value 0"]
impl crate::Resettable for SMCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
