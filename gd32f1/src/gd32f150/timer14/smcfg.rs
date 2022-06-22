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
#[doc = "Master/Slave mode\n\nValue on reset: 0"]
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
#[doc = "Field `MSM` reader - Master/Slave mode"]
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
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a> = crate::BitWriter<'a, u16, SMCFG_SPEC, MSM_A, 7>;
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
#[doc = "Field `TRGS` reader - Trigger selection"]
pub type TRGS_R = crate::FieldReader<u8, TRGS_A>;
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
}
#[doc = "Field `TRGS` writer - Trigger selection"]
pub type TRGS_W<'a> = crate::FieldWriter<'a, u16, SMCFG_SPEC, u8, TRGS_A, 3, 4>;
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
}
#[doc = "Slave mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMC_A {
    #[doc = "0: Slave mode disabled - if CEN=1 then the prescaler is clocked directly by the internal clock."]
    DISABLED = 0,
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
    pub fn variant(&self) -> Option<SMC_A> {
        match self.bits {
            0 => Some(SMC_A::DISABLED),
            4 => Some(SMC_A::RESTARTMODE),
            5 => Some(SMC_A::PAUSEMODE),
            6 => Some(SMC_A::EVENTMODE),
            7 => Some(SMC_A::EXTERNALCLOCKMODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMC_A::DISABLED
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
pub type SMC_W<'a> = crate::FieldWriter<'a, u16, SMCFG_SPEC, u8, SMC_A, 3, 0>;
impl<'a> SMC_W<'a> {
    #[doc = "Slave mode disabled - if CEN=1 then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMC_A::DISABLED)
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
    #[doc = "Bit 7 - Master/Slave mode"]
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
    #[doc = "Bit 7 - Master/Slave mode"]
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcfg](index.html) module"]
pub struct SMCFG_SPEC;
impl crate::RegisterSpec for SMCFG_SPEC {
    type Ux = u16;
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
