#[doc = "Register `CS` reader"]
pub type R = crate::R<CS_SPEC>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CS_SPEC>;
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WUF_R = crate::BitReader<WUFR_A>;
#[doc = "Wakeup flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFR_A {
    #[doc = "0: No wakeup event occurred"]
    NO_WAKEUP_EVENT = 0,
    #[doc = "1: A wakeup event was received from the WKUP pin or from the RTC wakeup event (RTC Tamper event, RTC TimeStamp event or RTC alarm)"]
    WAKEUP_EVENT = 1,
}
impl From<WUFR_A> for bool {
    #[inline(always)]
    fn from(variant: WUFR_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUFR_A {
        match self.bits {
            false => WUFR_A::NO_WAKEUP_EVENT,
            true => WUFR_A::WAKEUP_EVENT,
        }
    }
    #[doc = "No wakeup event occurred"]
    #[inline(always)]
    pub fn is_no_wakeup_event(&self) -> bool {
        *self == WUFR_A::NO_WAKEUP_EVENT
    }
    #[doc = "A wakeup event was received from the WKUP pin or from the RTC wakeup event (RTC Tamper event, RTC TimeStamp event or RTC alarm)"]
    #[inline(always)]
    pub fn is_wakeup_event(&self) -> bool {
        *self == WUFR_A::WAKEUP_EVENT
    }
}
#[doc = "Field `STBF` reader - Standby flag"]
pub type STBF_R = crate::BitReader<STBFR_A>;
#[doc = "Standby flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STBFR_A {
    #[doc = "0: Device has not been in Standby mode"]
    NO_STANDBY_EVENT = 0,
    #[doc = "1: Device has been in Standby mode"]
    STANDBY_EVENT = 1,
}
impl From<STBFR_A> for bool {
    #[inline(always)]
    fn from(variant: STBFR_A) -> Self {
        variant as u8 != 0
    }
}
impl STBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBFR_A {
        match self.bits {
            false => STBFR_A::NO_STANDBY_EVENT,
            true => STBFR_A::STANDBY_EVENT,
        }
    }
    #[doc = "Device has not been in Standby mode"]
    #[inline(always)]
    pub fn is_no_standby_event(&self) -> bool {
        *self == STBFR_A::NO_STANDBY_EVENT
    }
    #[doc = "Device has been in Standby mode"]
    #[inline(always)]
    pub fn is_standby_event(&self) -> bool {
        *self == STBFR_A::STANDBY_EVENT
    }
}
#[doc = "Field `LVDF` reader - Low Voltage Detector Status Flag"]
pub type LVDF_R = crate::BitReader<LVDFR_A>;
#[doc = "Low Voltage Detector Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDFR_A {
    #[doc = "0: VDD is higher than the LVD threshold"]
    ABOVE_THRESHOLD = 0,
    #[doc = "1: VDD is lower than or equal to the LVD threshold"]
    BELOW_THRESHOLD = 1,
}
impl From<LVDFR_A> for bool {
    #[inline(always)]
    fn from(variant: LVDFR_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDFR_A {
        match self.bits {
            false => LVDFR_A::ABOVE_THRESHOLD,
            true => LVDFR_A::BELOW_THRESHOLD,
        }
    }
    #[doc = "VDD is higher than the LVD threshold"]
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == LVDFR_A::ABOVE_THRESHOLD
    }
    #[doc = "VDD is lower than or equal to the LVD threshold"]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == LVDFR_A::BELOW_THRESHOLD
    }
}
#[doc = "Field `WUPEN0` reader - Enable WKUP pin"]
pub type WUPEN0_R = crate::BitReader<WUPEN0_A>;
#[doc = "Enable WKUP pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN0_A {
    #[doc = "0: WKUP pin 0 is used for general purpose I/Os. An event on the WKUP pin 0 does not wakeup the device from Standby mode"]
    DISABLED = 0,
    #[doc = "1: WKUP pin 0 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 0 wakes-up the system from Standby mode)"]
    ENABLED = 1,
}
impl From<WUPEN0_A> for bool {
    #[inline(always)]
    fn from(variant: WUPEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl WUPEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPEN0_A {
        match self.bits {
            false => WUPEN0_A::DISABLED,
            true => WUPEN0_A::ENABLED,
        }
    }
    #[doc = "WKUP pin 0 is used for general purpose I/Os. An event on the WKUP pin 0 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUPEN0_A::DISABLED
    }
    #[doc = "WKUP pin 0 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 0 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUPEN0_A::ENABLED
    }
}
#[doc = "Field `WUPEN0` writer - Enable WKUP pin"]
pub type WUPEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WUPEN0_A>;
impl<'a, REG, const O: u8> WUPEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WKUP pin 0 is used for general purpose I/Os. An event on the WKUP pin 0 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN0_A::DISABLED)
    }
    #[doc = "WKUP pin 0 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 0 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN0_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn stbf(&self) -> STBF_R {
        STBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Voltage Detector Status Flag"]
    #[inline(always)]
    pub fn lvdf(&self) -> LVDF_R {
        LVDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    pub fn wupen0(&self) -> WUPEN0_R {
        WUPEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    #[must_use]
    pub fn wupen0(&mut self) -> WUPEN0_W<CS_SPEC, 8> {
        WUPEN0_W::new(self)
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
#[doc = "power control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
