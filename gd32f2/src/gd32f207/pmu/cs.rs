#[doc = "Register `CS` reader"]
pub struct R(crate::R<CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS` writer"]
pub struct W(crate::W<CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS_SPEC>;
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
impl From<crate::W<CS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable WKUP pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `WUPEN0` reader - Enable WKUP pin"]
pub type WUPEN0_R = crate::BitReader<WUPEN0_A>;
impl WUPEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPEN0_A {
        match self.bits {
            false => WUPEN0_A::DISABLED,
            true => WUPEN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUPEN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUPEN0_A::ENABLED
    }
}
#[doc = "Field `WUPEN0` writer - Enable WKUP pin"]
pub type WUPEN0_W<'a> = crate::BitWriter<'a, u32, CS_SPEC, WUPEN0_A, 8>;
impl<'a> WUPEN0_W<'a> {
    #[doc = "WKUP pin 0 is used for general purpose I/Os. An event on the WKUP pin 0 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUPEN0_A::DISABLED)
    }
    #[doc = "WKUP pin 0 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 0 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUPEN0_A::ENABLED)
    }
}
#[doc = "Low Voltage Detector Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDF_A {
    #[doc = "0: VDD is higher than the LVD threshold"]
    ABOVETHRESHOLD = 0,
    #[doc = "1: VDD is lower than or equal to the LVD threshold"]
    BELOWTHRESHOLD = 1,
}
impl From<LVDF_A> for bool {
    #[inline(always)]
    fn from(variant: LVDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDF` reader - Low Voltage Detector Status Flag"]
pub type LVDF_R = crate::BitReader<LVDF_A>;
impl LVDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDF_A {
        match self.bits {
            false => LVDF_A::ABOVETHRESHOLD,
            true => LVDF_A::BELOWTHRESHOLD,
        }
    }
    #[doc = "Checks if the value of the field is `ABOVETHRESHOLD`"]
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == LVDF_A::ABOVETHRESHOLD
    }
    #[doc = "Checks if the value of the field is `BELOWTHRESHOLD`"]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == LVDF_A::BELOWTHRESHOLD
    }
}
#[doc = "Standby flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBF_A {
    #[doc = "0: Device has not been in Standby mode"]
    NOSTANDBYEVENT = 0,
    #[doc = "1: Device has been in Standby mode"]
    STANDBYEVENT = 1,
}
impl From<STBF_A> for bool {
    #[inline(always)]
    fn from(variant: STBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBF` reader - Standby flag"]
pub type STBF_R = crate::BitReader<STBF_A>;
impl STBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBF_A {
        match self.bits {
            false => STBF_A::NOSTANDBYEVENT,
            true => STBF_A::STANDBYEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTANDBYEVENT`"]
    #[inline(always)]
    pub fn is_no_standby_event(&self) -> bool {
        *self == STBF_A::NOSTANDBYEVENT
    }
    #[doc = "Checks if the value of the field is `STANDBYEVENT`"]
    #[inline(always)]
    pub fn is_standby_event(&self) -> bool {
        *self == STBF_A::STANDBYEVENT
    }
}
#[doc = "Wakeup flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF_A {
    #[doc = "0: No wakeup event occurred"]
    NOWAKEUPEVENT = 0,
    #[doc = "1: A wakeup event was received from the WKUP pin or from the RTC wakeup event (RTC Tamper event, RTC TimeStamp event or RTC alarm)"]
    WAKEUPEVENT = 1,
}
impl From<WUF_A> for bool {
    #[inline(always)]
    fn from(variant: WUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WUF_R = crate::BitReader<WUF_A>;
impl WUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF_A {
        match self.bits {
            false => WUF_A::NOWAKEUPEVENT,
            true => WUF_A::WAKEUPEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOWAKEUPEVENT`"]
    #[inline(always)]
    pub fn is_no_wakeup_event(&self) -> bool {
        *self == WUF_A::NOWAKEUPEVENT
    }
    #[doc = "Checks if the value of the field is `WAKEUPEVENT`"]
    #[inline(always)]
    pub fn is_wakeup_event(&self) -> bool {
        *self == WUF_A::WAKEUPEVENT
    }
}
impl R {
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    pub fn wupen0(&self) -> WUPEN0_R {
        WUPEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Voltage Detector Status Flag"]
    #[inline(always)]
    pub fn lvdf(&self) -> LVDF_R {
        LVDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn stbf(&self) -> STBF_R {
        STBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    pub fn wupen0(&mut self) -> WUPEN0_W {
        WUPEN0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs](index.html) module"]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs::R](R) reader structure"]
impl crate::Readable for CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs::W](W) writer structure"]
impl crate::Writable for CS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
