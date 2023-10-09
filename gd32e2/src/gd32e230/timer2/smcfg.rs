#[doc = "Register `SMCFG` reader"]
pub type R = crate::R<SMCFG_SPEC>;
#[doc = "Register `SMCFG` writer"]
pub type W = crate::W<SMCFG_SPEC>;
#[doc = "Field `SMC` reader - Slave mode selection"]
pub type SMC_R = crate::FieldReader<SMC_A>;
#[doc = "Slave mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMC_A {
    #[doc = "0: Slave mode disabled - if CEN=1 then the prescaler is clocked directly by the internal clock."]
    DISABLED = 0,
    #[doc = "1: Quadrature decoder mode 0 - Counter counts up/down on CI1FE1 edge depending on CI0FE0 level."]
    QUADRATURE_DECODER_MODE0 = 1,
    #[doc = "2: Quadrature decoder mode 1 - Counter counts up/down on CI0FE0 edge depending on CI1FE1 level."]
    QUADRATURE_DECODER_MODE1 = 2,
    #[doc = "3: Quadrature decoder mode 2 - Counter counts up/down on both CI0FE0 and CI1FE1 edges depending on the level of the other input."]
    QUADRATURE_DECODER_MODE2 = 3,
    #[doc = "4: Restart Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    RESTART_MODE = 4,
    #[doc = "5: Pause Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    PAUSE_MODE = 5,
    #[doc = "6: Event Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    EVENT_MODE = 6,
    #[doc = "7: External Clock Mode 0 - Rising edges of the selected trigger (TRGI) clock the counter."]
    EXTERNAL_CLOCK_MODE = 7,
}
impl From<SMC_A> for u8 {
    #[inline(always)]
    fn from(variant: SMC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMC_A {
    type Ux = u8;
}
impl SMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMC_A {
        match self.bits {
            0 => SMC_A::DISABLED,
            1 => SMC_A::QUADRATURE_DECODER_MODE0,
            2 => SMC_A::QUADRATURE_DECODER_MODE1,
            3 => SMC_A::QUADRATURE_DECODER_MODE2,
            4 => SMC_A::RESTART_MODE,
            5 => SMC_A::PAUSE_MODE,
            6 => SMC_A::EVENT_MODE,
            7 => SMC_A::EXTERNAL_CLOCK_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave mode disabled - if CEN=1 then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMC_A::DISABLED
    }
    #[doc = "Quadrature decoder mode 0 - Counter counts up/down on CI1FE1 edge depending on CI0FE0 level."]
    #[inline(always)]
    pub fn is_quadrature_decoder_mode0(&self) -> bool {
        *self == SMC_A::QUADRATURE_DECODER_MODE0
    }
    #[doc = "Quadrature decoder mode 1 - Counter counts up/down on CI0FE0 edge depending on CI1FE1 level."]
    #[inline(always)]
    pub fn is_quadrature_decoder_mode1(&self) -> bool {
        *self == SMC_A::QUADRATURE_DECODER_MODE1
    }
    #[doc = "Quadrature decoder mode 2 - Counter counts up/down on both CI0FE0 and CI1FE1 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn is_quadrature_decoder_mode2(&self) -> bool {
        *self == SMC_A::QUADRATURE_DECODER_MODE2
    }
    #[doc = "Restart Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn is_restart_mode(&self) -> bool {
        *self == SMC_A::RESTART_MODE
    }
    #[doc = "Pause Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn is_pause_mode(&self) -> bool {
        *self == SMC_A::PAUSE_MODE
    }
    #[doc = "Event Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn is_event_mode(&self) -> bool {
        *self == SMC_A::EVENT_MODE
    }
    #[doc = "External Clock Mode 0 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn is_external_clock_mode(&self) -> bool {
        *self == SMC_A::EXTERNAL_CLOCK_MODE
    }
}
#[doc = "Field `SMC` writer - Slave mode selection"]
pub type SMC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, SMC_A>;
impl<'a, REG, const O: u8> SMC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave mode disabled - if CEN=1 then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMC_A::DISABLED)
    }
    #[doc = "Quadrature decoder mode 0 - Counter counts up/down on CI1FE1 edge depending on CI0FE0 level."]
    #[inline(always)]
    pub fn quadrature_decoder_mode0(self) -> &'a mut crate::W<REG> {
        self.variant(SMC_A::QUADRATURE_DECODER_MODE0)
    }
    #[doc = "Quadrature decoder mode 1 - Counter counts up/down on CI0FE0 edge depending on CI1FE1 level."]
    #[inline(always)]
    pub fn quadrature_decoder_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(SMC_A::QUADRATURE_DECODER_MODE1)
    }
    #[doc = "Quadrature decoder mode 2 - Counter counts up/down on both CI0FE0 and CI1FE1 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn quadrature_decoder_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(SMC_A::QUADRATURE_DECODER_MODE2)
    }
    #[doc = "Restart Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn restart_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SMC_A::RESTART_MODE)
    }
    #[doc = "Pause Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn pause_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SMC_A::PAUSE_MODE)
    }
    #[doc = "Event Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn event_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SMC_A::EVENT_MODE)
    }
    #[doc = "External Clock Mode 0 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn external_clock_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SMC_A::EXTERNAL_CLOCK_MODE)
    }
}
#[doc = "Field `OCRC` reader - OCREF clear source selection"]
pub type OCRC_R = crate::BitReader<OCRC_A>;
#[doc = "OCREF clear source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCRC_A {
    #[doc = "0: OCPRE_CLR_INT is connected to the OCPRE_CLR input"]
    INPUT = 0,
    #[doc = "1: OCPRE_CLR_INT is connected to ETIF"]
    ETIF = 1,
}
impl From<OCRC_A> for bool {
    #[inline(always)]
    fn from(variant: OCRC_A) -> Self {
        variant as u8 != 0
    }
}
impl OCRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCRC_A {
        match self.bits {
            false => OCRC_A::INPUT,
            true => OCRC_A::ETIF,
        }
    }
    #[doc = "OCPRE_CLR_INT is connected to the OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == OCRC_A::INPUT
    }
    #[doc = "OCPRE_CLR_INT is connected to ETIF"]
    #[inline(always)]
    pub fn is_etif(&self) -> bool {
        *self == OCRC_A::ETIF
    }
}
#[doc = "Field `OCRC` writer - OCREF clear source selection"]
pub type OCRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OCRC_A>;
impl<'a, REG, const O: u8> OCRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OCPRE_CLR_INT is connected to the OCPRE_CLR input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(OCRC_A::INPUT)
    }
    #[doc = "OCPRE_CLR_INT is connected to ETIF"]
    #[inline(always)]
    pub fn etif(self) -> &'a mut crate::W<REG> {
        self.variant(OCRC_A::ETIF)
    }
}
#[doc = "Field `TRGS` reader - Trigger selection"]
pub type TRGS_R = crate::FieldReader<TRGS_A>;
#[doc = "Trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for TRGS_A {
    type Ux = u8;
}
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
    #[doc = "Internal Trigger 0 (ITI0)"]
    #[inline(always)]
    pub fn is_iti0(&self) -> bool {
        *self == TRGS_A::ITI0
    }
    #[doc = "Internal Trigger 1 (ITI1)"]
    #[inline(always)]
    pub fn is_iti1(&self) -> bool {
        *self == TRGS_A::ITI1
    }
    #[doc = "Internal Trigger 2 (ITI2)"]
    #[inline(always)]
    pub fn is_iti2(&self) -> bool {
        *self == TRGS_A::ITI2
    }
    #[doc = "CI0 Edge Detector (CI0F_ED)"]
    #[inline(always)]
    pub fn is_ci0f_ed(&self) -> bool {
        *self == TRGS_A::CI0F_ED
    }
    #[doc = "Filtered Timer Input 0 (CI0FE0)"]
    #[inline(always)]
    pub fn is_ci0fe0(&self) -> bool {
        *self == TRGS_A::CI0FE0
    }
    #[doc = "Filtered Timer Input 1 (CI1FE1)"]
    #[inline(always)]
    pub fn is_ci1fe1(&self) -> bool {
        *self == TRGS_A::CI1FE1
    }
    #[doc = "External Trigger input (ETIFP)"]
    #[inline(always)]
    pub fn is_etifp(&self) -> bool {
        *self == TRGS_A::ETIFP
    }
}
#[doc = "Field `TRGS` writer - Trigger selection"]
pub type TRGS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, TRGS_A>;
impl<'a, REG, const O: u8> TRGS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal Trigger 0 (ITI0)"]
    #[inline(always)]
    pub fn iti0(self) -> &'a mut crate::W<REG> {
        self.variant(TRGS_A::ITI0)
    }
    #[doc = "Internal Trigger 1 (ITI1)"]
    #[inline(always)]
    pub fn iti1(self) -> &'a mut crate::W<REG> {
        self.variant(TRGS_A::ITI1)
    }
    #[doc = "Internal Trigger 2 (ITI2)"]
    #[inline(always)]
    pub fn iti2(self) -> &'a mut crate::W<REG> {
        self.variant(TRGS_A::ITI2)
    }
    #[doc = "CI0 Edge Detector (CI0F_ED)"]
    #[inline(always)]
    pub fn ci0f_ed(self) -> &'a mut crate::W<REG> {
        self.variant(TRGS_A::CI0F_ED)
    }
    #[doc = "Filtered Timer Input 0 (CI0FE0)"]
    #[inline(always)]
    pub fn ci0fe0(self) -> &'a mut crate::W<REG> {
        self.variant(TRGS_A::CI0FE0)
    }
    #[doc = "Filtered Timer Input 1 (CI1FE1)"]
    #[inline(always)]
    pub fn ci1fe1(self) -> &'a mut crate::W<REG> {
        self.variant(TRGS_A::CI1FE1)
    }
    #[doc = "External Trigger input (ETIFP)"]
    #[inline(always)]
    pub fn etifp(self) -> &'a mut crate::W<REG> {
        self.variant(TRGS_A::ETIFP)
    }
}
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MSM_R = crate::BitReader<MSM_A>;
#[doc = "Master/Slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSM_A {
    #[doc = "0: No action"]
    NO_SYNC = 0,
    #[doc = "1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    SYNC = 1,
}
impl From<MSM_A> for bool {
    #[inline(always)]
    fn from(variant: MSM_A) -> Self {
        variant as u8 != 0
    }
}
impl MSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSM_A {
        match self.bits {
            false => MSM_A::NO_SYNC,
            true => MSM_A::SYNC,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        *self == MSM_A::NO_SYNC
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == MSM_A::SYNC
    }
}
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSM_A>;
impl<'a, REG, const O: u8> MSM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_sync(self) -> &'a mut crate::W<REG> {
        self.variant(MSM_A::NO_SYNC)
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(MSM_A::SYNC)
    }
}
#[doc = "Field `ETFC` reader - External trigger filter"]
pub type ETFC_R = crate::FieldReader<ETFC_A>;
#[doc = "External trigger filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETFC_A {
    #[doc = "0: Filter disabled. fSAMP=fDTS, N=1"]
    NO_FILTER = 0,
    #[doc = "1: fSAMP=fTIMER_CK, N=2"]
    TIMER_CK_N2 = 1,
    #[doc = "2: fSAMP=fTIMER_CK, N=4"]
    TIMER_CK_N4 = 2,
    #[doc = "3: fSAMP=fTIMER_CK, N=8"]
    TIMER_CK_N8 = 3,
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
impl crate::FieldSpec for ETFC_A {
    type Ux = u8;
}
impl ETFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETFC_A {
        match self.bits {
            0 => ETFC_A::NO_FILTER,
            1 => ETFC_A::TIMER_CK_N2,
            2 => ETFC_A::TIMER_CK_N4,
            3 => ETFC_A::TIMER_CK_N8,
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
    #[doc = "Filter disabled. fSAMP=fDTS, N=1"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == ETFC_A::NO_FILTER
    }
    #[doc = "fSAMP=fTIMER_CK, N=2"]
    #[inline(always)]
    pub fn is_timer_ck_n2(&self) -> bool {
        *self == ETFC_A::TIMER_CK_N2
    }
    #[doc = "fSAMP=fTIMER_CK, N=4"]
    #[inline(always)]
    pub fn is_timer_ck_n4(&self) -> bool {
        *self == ETFC_A::TIMER_CK_N4
    }
    #[doc = "fSAMP=fTIMER_CK, N=8"]
    #[inline(always)]
    pub fn is_timer_ck_n8(&self) -> bool {
        *self == ETFC_A::TIMER_CK_N8
    }
    #[doc = "fSAMP=fDTS/2, N=6"]
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == ETFC_A::FDTS_DIV2_N6
    }
    #[doc = "fSAMP=fDTS/2, N=8"]
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == ETFC_A::FDTS_DIV2_N8
    }
    #[doc = "fSAMP=fDTS/4, N=6"]
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == ETFC_A::FDTS_DIV4_N6
    }
    #[doc = "fSAMP=fDTS/4, N=8"]
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == ETFC_A::FDTS_DIV4_N8
    }
    #[doc = "fSAMP=fDTS/8, N=6"]
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == ETFC_A::FDTS_DIV8_N6
    }
    #[doc = "fSAMP=fDTS/8, N=8"]
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == ETFC_A::FDTS_DIV8_N8
    }
    #[doc = "fSAMP=fDTS/16, N=5"]
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == ETFC_A::FDTS_DIV16_N5
    }
    #[doc = "fSAMP=fDTS/16, N=6"]
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == ETFC_A::FDTS_DIV16_N6
    }
    #[doc = "fSAMP=fDTS/16, N=8"]
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == ETFC_A::FDTS_DIV16_N8
    }
    #[doc = "fSAMP=fDTS/32, N=5"]
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == ETFC_A::FDTS_DIV32_N5
    }
    #[doc = "fSAMP=fDTS/32, N=6"]
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == ETFC_A::FDTS_DIV32_N6
    }
    #[doc = "fSAMP=fDTS/32, N=8"]
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == ETFC_A::FDTS_DIV32_N8
    }
}
#[doc = "Field `ETFC` writer - External trigger filter"]
pub type ETFC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, ETFC_A>;
impl<'a, REG, const O: u8> ETFC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Filter disabled. fSAMP=fDTS, N=1"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::NO_FILTER)
    }
    #[doc = "fSAMP=fTIMER_CK, N=2"]
    #[inline(always)]
    pub fn timer_ck_n2(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::TIMER_CK_N2)
    }
    #[doc = "fSAMP=fTIMER_CK, N=4"]
    #[inline(always)]
    pub fn timer_ck_n4(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::TIMER_CK_N4)
    }
    #[doc = "fSAMP=fTIMER_CK, N=8"]
    #[inline(always)]
    pub fn timer_ck_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::TIMER_CK_N8)
    }
    #[doc = "fSAMP=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::FDTS_DIV2_N6)
    }
    #[doc = "fSAMP=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::FDTS_DIV2_N8)
    }
    #[doc = "fSAMP=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::FDTS_DIV4_N6)
    }
    #[doc = "fSAMP=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::FDTS_DIV4_N8)
    }
    #[doc = "fSAMP=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::FDTS_DIV8_N6)
    }
    #[doc = "fSAMP=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::FDTS_DIV8_N8)
    }
    #[doc = "fSAMP=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::FDTS_DIV16_N5)
    }
    #[doc = "fSAMP=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::FDTS_DIV16_N6)
    }
    #[doc = "fSAMP=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::FDTS_DIV16_N8)
    }
    #[doc = "fSAMP=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::FDTS_DIV32_N5)
    }
    #[doc = "fSAMP=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::FDTS_DIV32_N6)
    }
    #[doc = "fSAMP=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETFC_A::FDTS_DIV32_N8)
    }
}
#[doc = "Field `ETPSC` reader - External trigger prescaler"]
pub type ETPSC_R = crate::FieldReader<ETPSC_A>;
#[doc = "External trigger prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for ETPSC_A {
    type Ux = u8;
}
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
    #[doc = "External trigger prescaler disabled"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == ETPSC_A::DIV1
    }
    #[doc = "ETI frequency divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ETPSC_A::DIV2
    }
    #[doc = "ETI frequency divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ETPSC_A::DIV4
    }
    #[doc = "ETI frequency divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ETPSC_A::DIV8
    }
}
#[doc = "Field `ETPSC` writer - External trigger prescaler"]
pub type ETPSC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, ETPSC_A>;
impl<'a, REG, const O: u8> ETPSC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External trigger prescaler disabled"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(ETPSC_A::DIV1)
    }
    #[doc = "ETI frequency divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(ETPSC_A::DIV2)
    }
    #[doc = "ETI frequency divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(ETPSC_A::DIV4)
    }
    #[doc = "ETI frequency divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(ETPSC_A::DIV8)
    }
}
#[doc = "Field `SMC1` reader - External clock enable"]
pub type SMC1_R = crate::BitReader<SMC1_A>;
#[doc = "External clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SMC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMC1_A {
        match self.bits {
            false => SMC1_A::DISABLED,
            true => SMC1_A::ENABLED,
        }
    }
    #[doc = "External clock mode 1 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMC1_A::DISABLED
    }
    #[doc = "External clock mode 1 enabled. The counter is clocked by any active edge on the ETIF signal."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMC1_A::ENABLED
    }
}
#[doc = "Field `SMC1` writer - External clock enable"]
pub type SMC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SMC1_A>;
impl<'a, REG, const O: u8> SMC1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External clock mode 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMC1_A::DISABLED)
    }
    #[doc = "External clock mode 1 enabled. The counter is clocked by any active edge on the ETIF signal."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMC1_A::ENABLED)
    }
}
#[doc = "Field `ETP` reader - External trigger polarity"]
pub type ETP_R = crate::BitReader<ETP_A>;
#[doc = "External trigger polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETP_A {
    #[doc = "0: ETI is noninverted, active at high level or rising edge"]
    NOT_INVERTED = 0,
    #[doc = "1: ETI is inverted, active at low level or falling edge"]
    INVERTED = 1,
}
impl From<ETP_A> for bool {
    #[inline(always)]
    fn from(variant: ETP_A) -> Self {
        variant as u8 != 0
    }
}
impl ETP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETP_A {
        match self.bits {
            false => ETP_A::NOT_INVERTED,
            true => ETP_A::INVERTED,
        }
    }
    #[doc = "ETI is noninverted, active at high level or rising edge"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == ETP_A::NOT_INVERTED
    }
    #[doc = "ETI is inverted, active at low level or falling edge"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == ETP_A::INVERTED
    }
}
#[doc = "Field `ETP` writer - External trigger polarity"]
pub type ETP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ETP_A>;
impl<'a, REG, const O: u8> ETP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ETI is noninverted, active at high level or rising edge"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(ETP_A::NOT_INVERTED)
    }
    #[doc = "ETI is inverted, active at low level or falling edge"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(ETP_A::INVERTED)
    }
}
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - OCREF clear source selection"]
    #[inline(always)]
    pub fn ocrc(&self) -> OCRC_R {
        OCRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&self) -> TRGS_R {
        TRGS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etfc(&self) -> ETFC_R {
        ETFC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etpsc(&self) -> ETPSC_R {
        ETPSC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn smc1(&self) -> SMC1_R {
        SMC1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn smc(&mut self) -> SMC_W<SMCFG_SPEC, 0> {
        SMC_W::new(self)
    }
    #[doc = "Bit 3 - OCREF clear source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ocrc(&mut self) -> OCRC_W<SMCFG_SPEC, 3> {
        OCRC_W::new(self)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgs(&mut self) -> TRGS_W<SMCFG_SPEC, 4> {
        TRGS_W::new(self)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<SMCFG_SPEC, 7> {
        MSM_W::new(self)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    #[must_use]
    pub fn etfc(&mut self) -> ETFC_W<SMCFG_SPEC, 8> {
        ETFC_W::new(self)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn etpsc(&mut self) -> ETPSC_W<SMCFG_SPEC, 12> {
        ETPSC_W::new(self)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn smc1(&mut self) -> SMC1_W<SMCFG_SPEC, 14> {
        SMC1_W::new(self)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<SMCFG_SPEC, 15> {
        ETP_W::new(self)
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
#[doc = "slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMCFG_SPEC;
impl crate::RegisterSpec for SMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcfg::R`](R) reader structure"]
impl crate::Readable for SMCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smcfg::W`](W) writer structure"]
impl crate::Writable for SMCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCFG to value 0"]
impl crate::Resettable for SMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
