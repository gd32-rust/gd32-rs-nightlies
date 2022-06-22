#[doc = "Register `BDCTL` reader"]
pub struct R(crate::R<BDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDCTL` writer"]
pub struct W(crate::W<BDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCTL_SPEC>;
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
impl From<crate::W<BDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LXTAL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTALEN_A {
    #[doc = "0: LXTAL oscillator Off"]
    OFF = 0,
    #[doc = "1: LXTAL oscillator On"]
    ON = 1,
}
impl From<LXTALEN_A> for bool {
    #[inline(always)]
    fn from(variant: LXTALEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALEN` reader - LXTAL enable"]
pub type LXTALEN_R = crate::BitReader<LXTALEN_A>;
impl LXTALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTALEN_A {
        match self.bits {
            false => LXTALEN_A::OFF,
            true => LXTALEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LXTALEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LXTALEN_A::ON
    }
}
#[doc = "Field `LXTALEN` writer - LXTAL enable"]
pub type LXTALEN_W<'a> = crate::BitWriter<'a, u32, BDCTL_SPEC, LXTALEN_A, 0>;
impl<'a> LXTALEN_W<'a> {
    #[doc = "LXTAL oscillator Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LXTALEN_A::OFF)
    }
    #[doc = "LXTAL oscillator On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LXTALEN_A::ON)
    }
}
#[doc = "External low-speed oscillator stabilization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTALSTB_A {
    #[doc = "0: LXTAL oscillator not ready"]
    NOTREADY = 0,
    #[doc = "1: LXTAL oscillator ready"]
    READY = 1,
}
impl From<LXTALSTB_A> for bool {
    #[inline(always)]
    fn from(variant: LXTALSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALSTB` reader - External low-speed oscillator stabilization"]
pub type LXTALSTB_R = crate::BitReader<LXTALSTB_A>;
impl LXTALSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTALSTB_A {
        match self.bits {
            false => LXTALSTB_A::NOTREADY,
            true => LXTALSTB_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LXTALSTB_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LXTALSTB_A::READY
    }
}
#[doc = "LXTAL bypass mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTALBPS_A {
    #[doc = "0: LXTAL crystal oscillator not bypassed"]
    NOTBYPASSED = 0,
    #[doc = "1: LXTAL crystal oscillator bypassed with external clock"]
    BYPASSED = 1,
}
impl From<LXTALBPS_A> for bool {
    #[inline(always)]
    fn from(variant: LXTALBPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALBPS` reader - LXTAL bypass mode enable"]
pub type LXTALBPS_R = crate::BitReader<LXTALBPS_A>;
impl LXTALBPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTALBPS_A {
        match self.bits {
            false => LXTALBPS_A::NOTBYPASSED,
            true => LXTALBPS_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBYPASSED`"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LXTALBPS_A::NOTBYPASSED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == LXTALBPS_A::BYPASSED
    }
}
#[doc = "Field `LXTALBPS` writer - LXTAL bypass mode enable"]
pub type LXTALBPS_W<'a> = crate::BitWriter<'a, u32, BDCTL_SPEC, LXTALBPS_A, 2>;
impl<'a> LXTALBPS_W<'a> {
    #[doc = "LXTAL crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(LXTALBPS_A::NOTBYPASSED)
    }
    #[doc = "LXTAL crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(LXTALBPS_A::BYPASSED)
    }
}
#[doc = "LXTAL drive capability\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LXTALDRI_A {
    #[doc = "0: Low driving capability"]
    LOW = 0,
    #[doc = "1: Medium low driving capability"]
    MEDIUMLOW = 1,
    #[doc = "2: Medium high driving capability"]
    MEDIUMHIGH = 2,
    #[doc = "3: High driving capability (reset value)"]
    HIGH = 3,
}
impl From<LXTALDRI_A> for u8 {
    #[inline(always)]
    fn from(variant: LXTALDRI_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LXTALDRI` reader - LXTAL drive capability"]
pub type LXTALDRI_R = crate::FieldReader<u8, LXTALDRI_A>;
impl LXTALDRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTALDRI_A {
        match self.bits {
            0 => LXTALDRI_A::LOW,
            1 => LXTALDRI_A::MEDIUMLOW,
            2 => LXTALDRI_A::MEDIUMHIGH,
            3 => LXTALDRI_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LXTALDRI_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUMLOW`"]
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == LXTALDRI_A::MEDIUMLOW
    }
    #[doc = "Checks if the value of the field is `MEDIUMHIGH`"]
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == LXTALDRI_A::MEDIUMHIGH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LXTALDRI_A::HIGH
    }
}
#[doc = "Field `LXTALDRI` writer - LXTAL drive capability"]
pub type LXTALDRI_W<'a> = crate::FieldWriterSafe<'a, u32, BDCTL_SPEC, u8, LXTALDRI_A, 2, 3>;
impl<'a> LXTALDRI_W<'a> {
    #[doc = "Low driving capability"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LXTALDRI_A::LOW)
    }
    #[doc = "Medium low driving capability"]
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut W {
        self.variant(LXTALDRI_A::MEDIUMLOW)
    }
    #[doc = "Medium high driving capability"]
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut W {
        self.variant(LXTALDRI_A::MEDIUMHIGH)
    }
    #[doc = "High driving capability (reset value)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LXTALDRI_A::HIGH)
    }
}
#[doc = "RTC clock entry selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCSRC_A {
    #[doc = "0: No clock"]
    NOCLOCK = 0,
    #[doc = "1: LXTAL oscillator clock used as RTC clock"]
    LXTAL = 1,
    #[doc = "2: IRC40K oscillator clock used as RTC clock"]
    IRC40K = 2,
    #[doc = "3: HXTAL oscillator / 32 used as RTC clock"]
    HXTAL = 3,
}
impl From<RTCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCSRC` reader - RTC clock entry selection"]
pub type RTCSRC_R = crate::FieldReader<u8, RTCSRC_A>;
impl RTCSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSRC_A {
        match self.bits {
            0 => RTCSRC_A::NOCLOCK,
            1 => RTCSRC_A::LXTAL,
            2 => RTCSRC_A::IRC40K,
            3 => RTCSRC_A::HXTAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSRC_A::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `LXTAL`"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == RTCSRC_A::LXTAL
    }
    #[doc = "Checks if the value of the field is `IRC40K`"]
    #[inline(always)]
    pub fn is_irc40k(&self) -> bool {
        *self == RTCSRC_A::IRC40K
    }
    #[doc = "Checks if the value of the field is `HXTAL`"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == RTCSRC_A::HXTAL
    }
}
#[doc = "Field `RTCSRC` writer - RTC clock entry selection"]
pub type RTCSRC_W<'a> = crate::FieldWriterSafe<'a, u32, BDCTL_SPEC, u8, RTCSRC_A, 2, 8>;
impl<'a> RTCSRC_W<'a> {
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSRC_A::NOCLOCK)
    }
    #[doc = "LXTAL oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut W {
        self.variant(RTCSRC_A::LXTAL)
    }
    #[doc = "IRC40K oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn irc40k(self) -> &'a mut W {
        self.variant(RTCSRC_A::IRC40K)
    }
    #[doc = "HXTAL oscillator / 32 used as RTC clock"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut W {
        self.variant(RTCSRC_A::HXTAL)
    }
}
#[doc = "RTC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEN_A {
    #[doc = "0: RTC clock disabled"]
    DISABLED = 0,
    #[doc = "1: RTC clock enabled"]
    ENABLED = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
impl RTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::DISABLED,
            true => RTCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN_A::ENABLED
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a> = crate::BitWriter<'a, u32, BDCTL_SPEC, RTCEN_A, 15>;
impl<'a> RTCEN_W<'a> {
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCEN_A::DISABLED)
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCEN_A::ENABLED)
    }
}
#[doc = "Backup domain reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKPRST_A {
    #[doc = "0: Reset not activated"]
    NORESET = 0,
    #[doc = "1: Reset the entire RTC domain"]
    RESET = 1,
}
impl From<BKPRST_A> for bool {
    #[inline(always)]
    fn from(variant: BKPRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKPRST` reader - Backup domain reset"]
pub type BKPRST_R = crate::BitReader<BKPRST_A>;
impl BKPRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKPRST_A {
        match self.bits {
            false => BKPRST_A::NORESET,
            true => BKPRST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == BKPRST_A::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BKPRST_A::RESET
    }
}
#[doc = "Field `BKPRST` writer - Backup domain reset"]
pub type BKPRST_W<'a> = crate::BitWriter<'a, u32, BDCTL_SPEC, BKPRST_A, 16>;
impl<'a> BKPRST_W<'a> {
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(BKPRST_A::NORESET)
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BKPRST_A::RESET)
    }
}
impl R {
    #[doc = "Bit 0 - LXTAL enable"]
    #[inline(always)]
    pub fn lxtalen(&self) -> LXTALEN_R {
        LXTALEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External low-speed oscillator stabilization"]
    #[inline(always)]
    pub fn lxtalstb(&self) -> LXTALSTB_R {
        LXTALSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LXTAL bypass mode enable"]
    #[inline(always)]
    pub fn lxtalbps(&self) -> LXTALBPS_R {
        LXTALBPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LXTAL drive capability"]
    #[inline(always)]
    pub fn lxtaldri(&self) -> LXTALDRI_R {
        LXTALDRI_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 8:9 - RTC clock entry selection"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RTCSRC_R {
        RTCSRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup domain reset"]
    #[inline(always)]
    pub fn bkprst(&self) -> BKPRST_R {
        BKPRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LXTAL enable"]
    #[inline(always)]
    pub fn lxtalen(&mut self) -> LXTALEN_W {
        LXTALEN_W::new(self)
    }
    #[doc = "Bit 2 - LXTAL bypass mode enable"]
    #[inline(always)]
    pub fn lxtalbps(&mut self) -> LXTALBPS_W {
        LXTALBPS_W::new(self)
    }
    #[doc = "Bits 3:4 - LXTAL drive capability"]
    #[inline(always)]
    pub fn lxtaldri(&mut self) -> LXTALDRI_W {
        LXTALDRI_W::new(self)
    }
    #[doc = "Bits 8:9 - RTC clock entry selection"]
    #[inline(always)]
    pub fn rtcsrc(&mut self) -> RTCSRC_W {
        RTCSRC_W::new(self)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 16 - Backup domain reset"]
    #[inline(always)]
    pub fn bkprst(&mut self) -> BKPRST_W {
        BKPRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup domain control register (RCU_BDCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdctl](index.html) module"]
pub struct BDCTL_SPEC;
impl crate::RegisterSpec for BDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdctl::R](R) reader structure"]
impl crate::Readable for BDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdctl::W](W) writer structure"]
impl crate::Writable for BDCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDCTL to value 0x18"]
impl crate::Resettable for BDCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x18
    }
}
