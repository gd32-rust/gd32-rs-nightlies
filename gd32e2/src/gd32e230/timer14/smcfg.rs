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
    pub fn variant(&self) -> Option<SMC_A> {
        match self.bits {
            0 => Some(SMC_A::DISABLED),
            4 => Some(SMC_A::RESTART_MODE),
            5 => Some(SMC_A::PAUSE_MODE),
            6 => Some(SMC_A::EVENT_MODE),
            7 => Some(SMC_A::EXTERNAL_CLOCK_MODE),
            _ => None,
        }
    }
    #[doc = "Slave mode disabled - if CEN=1 then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMC_A::DISABLED
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
pub type SMC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SMC_A>;
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
    #[doc = "4: CI0 Edge Detector (CI0F_ED)"]
    CI0F_ED = 4,
    #[doc = "5: Filtered Timer Input 0 (CI0FE0)"]
    CI0FE0 = 5,
    #[doc = "6: Filtered Timer Input 1 (CI1FE1)"]
    CI1FE1 = 6,
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
            4 => Some(TRGS_A::CI0F_ED),
            5 => Some(TRGS_A::CI0FE0),
            6 => Some(TRGS_A::CI1FE1),
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
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new((self.bits & 7) as u8)
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
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn smc(&mut self) -> SMC_W<SMCFG_SPEC, 0> {
        SMC_W::new(self)
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
#[doc = "slave mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
