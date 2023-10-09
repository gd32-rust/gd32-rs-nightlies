#[doc = "Register `BDCTL` reader"]
pub type R = crate::R<BDCTL_SPEC>;
#[doc = "Register `BDCTL` writer"]
pub type W = crate::W<BDCTL_SPEC>;
#[doc = "Field `LXTALEN` reader - LXTAL enable"]
pub type LXTALEN_R = crate::BitReader<LXTALEN_A>;
#[doc = "LXTAL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LXTALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTALEN_A {
        match self.bits {
            false => LXTALEN_A::OFF,
            true => LXTALEN_A::ON,
        }
    }
    #[doc = "LXTAL oscillator Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LXTALEN_A::OFF
    }
    #[doc = "LXTAL oscillator On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LXTALEN_A::ON
    }
}
#[doc = "Field `LXTALEN` writer - LXTAL enable"]
pub type LXTALEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LXTALEN_A>;
impl<'a, REG, const O: u8> LXTALEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LXTAL oscillator Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LXTALEN_A::OFF)
    }
    #[doc = "LXTAL oscillator On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LXTALEN_A::ON)
    }
}
#[doc = "Field `LXTALSTB` reader - External low-speed oscillator stabilization"]
pub type LXTALSTB_R = crate::BitReader<LXTALSTBR_A>;
#[doc = "External low-speed oscillator stabilization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LXTALSTBR_A {
    #[doc = "0: LXTAL oscillator not ready"]
    NOT_READY = 0,
    #[doc = "1: LXTAL oscillator ready"]
    READY = 1,
}
impl From<LXTALSTBR_A> for bool {
    #[inline(always)]
    fn from(variant: LXTALSTBR_A) -> Self {
        variant as u8 != 0
    }
}
impl LXTALSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTALSTBR_A {
        match self.bits {
            false => LXTALSTBR_A::NOT_READY,
            true => LXTALSTBR_A::READY,
        }
    }
    #[doc = "LXTAL oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LXTALSTBR_A::NOT_READY
    }
    #[doc = "LXTAL oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LXTALSTBR_A::READY
    }
}
#[doc = "Field `LXTALBPS` reader - LXTAL bypass mode enable"]
pub type LXTALBPS_R = crate::BitReader<LXTALBPS_A>;
#[doc = "LXTAL bypass mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LXTALBPS_A {
    #[doc = "0: LXTAL crystal oscillator not bypassed"]
    NOT_BYPASSED = 0,
    #[doc = "1: LXTAL crystal oscillator bypassed with external clock"]
    BYPASSED = 1,
}
impl From<LXTALBPS_A> for bool {
    #[inline(always)]
    fn from(variant: LXTALBPS_A) -> Self {
        variant as u8 != 0
    }
}
impl LXTALBPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTALBPS_A {
        match self.bits {
            false => LXTALBPS_A::NOT_BYPASSED,
            true => LXTALBPS_A::BYPASSED,
        }
    }
    #[doc = "LXTAL crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LXTALBPS_A::NOT_BYPASSED
    }
    #[doc = "LXTAL crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == LXTALBPS_A::BYPASSED
    }
}
#[doc = "Field `LXTALBPS` writer - LXTAL bypass mode enable"]
pub type LXTALBPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LXTALBPS_A>;
impl<'a, REG, const O: u8> LXTALBPS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LXTAL crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(LXTALBPS_A::NOT_BYPASSED)
    }
    #[doc = "LXTAL crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(LXTALBPS_A::BYPASSED)
    }
}
#[doc = "Field `LXTALDRI` reader - LXTAL drive capability"]
pub type LXTALDRI_R = crate::FieldReader<LXTALDRI_A>;
#[doc = "LXTAL drive capability\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LXTALDRI_A {
    #[doc = "0: Low driving capability"]
    LOW = 0,
    #[doc = "1: Medium low driving capability"]
    MEDIUM_LOW = 1,
    #[doc = "2: Medium high driving capability"]
    MEDIUM_HIGH = 2,
    #[doc = "3: High driving capability (reset value)"]
    HIGH = 3,
}
impl From<LXTALDRI_A> for u8 {
    #[inline(always)]
    fn from(variant: LXTALDRI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LXTALDRI_A {
    type Ux = u8;
}
impl LXTALDRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTALDRI_A {
        match self.bits {
            0 => LXTALDRI_A::LOW,
            1 => LXTALDRI_A::MEDIUM_LOW,
            2 => LXTALDRI_A::MEDIUM_HIGH,
            3 => LXTALDRI_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Low driving capability"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LXTALDRI_A::LOW
    }
    #[doc = "Medium low driving capability"]
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == LXTALDRI_A::MEDIUM_LOW
    }
    #[doc = "Medium high driving capability"]
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == LXTALDRI_A::MEDIUM_HIGH
    }
    #[doc = "High driving capability (reset value)"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LXTALDRI_A::HIGH
    }
}
#[doc = "Field `LXTALDRI` writer - LXTAL drive capability"]
pub type LXTALDRI_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, LXTALDRI_A>;
impl<'a, REG, const O: u8> LXTALDRI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low driving capability"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(LXTALDRI_A::LOW)
    }
    #[doc = "Medium low driving capability"]
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut crate::W<REG> {
        self.variant(LXTALDRI_A::MEDIUM_LOW)
    }
    #[doc = "Medium high driving capability"]
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut crate::W<REG> {
        self.variant(LXTALDRI_A::MEDIUM_HIGH)
    }
    #[doc = "High driving capability (reset value)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(LXTALDRI_A::HIGH)
    }
}
#[doc = "Field `RTCSRC` reader - RTC clock entry selection"]
pub type RTCSRC_R = crate::FieldReader<RTCSRC_A>;
#[doc = "RTC clock entry selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSRC_A {
    #[doc = "0: No clock"]
    NO_CLOCK = 0,
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
impl crate::FieldSpec for RTCSRC_A {
    type Ux = u8;
}
impl RTCSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSRC_A {
        match self.bits {
            0 => RTCSRC_A::NO_CLOCK,
            1 => RTCSRC_A::LXTAL,
            2 => RTCSRC_A::IRC40K,
            3 => RTCSRC_A::HXTAL,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSRC_A::NO_CLOCK
    }
    #[doc = "LXTAL oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == RTCSRC_A::LXTAL
    }
    #[doc = "IRC40K oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn is_irc40k(&self) -> bool {
        *self == RTCSRC_A::IRC40K
    }
    #[doc = "HXTAL oscillator / 32 used as RTC clock"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == RTCSRC_A::HXTAL
    }
}
#[doc = "Field `RTCSRC` writer - RTC clock entry selection"]
pub type RTCSRC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, RTCSRC_A>;
impl<'a, REG, const O: u8> RTCSRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSRC_A::NO_CLOCK)
    }
    #[doc = "LXTAL oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSRC_A::LXTAL)
    }
    #[doc = "IRC40K oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn irc40k(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSRC_A::IRC40K)
    }
    #[doc = "HXTAL oscillator / 32 used as RTC clock"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSRC_A::HXTAL)
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
#[doc = "RTC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::DISABLED,
            true => RTCEN_A::ENABLED,
        }
    }
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN_A::DISABLED
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN_A::ENABLED
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTCEN_A>;
impl<'a, REG, const O: u8> RTCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN_A::DISABLED)
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN_A::ENABLED)
    }
}
#[doc = "Field `BKPRST` reader - Backup domain reset"]
pub type BKPRST_R = crate::BitReader<BKPRST_A>;
#[doc = "Backup domain reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKPRST_A {
    #[doc = "0: Reset not activated"]
    NO_RESET = 0,
    #[doc = "1: Reset the entire RTC domain"]
    RESET = 1,
}
impl From<BKPRST_A> for bool {
    #[inline(always)]
    fn from(variant: BKPRST_A) -> Self {
        variant as u8 != 0
    }
}
impl BKPRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKPRST_A {
        match self.bits {
            false => BKPRST_A::NO_RESET,
            true => BKPRST_A::RESET,
        }
    }
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == BKPRST_A::NO_RESET
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BKPRST_A::RESET
    }
}
#[doc = "Field `BKPRST` writer - Backup domain reset"]
pub type BKPRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BKPRST_A>;
impl<'a, REG, const O: u8> BKPRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(BKPRST_A::NO_RESET)
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
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
    #[must_use]
    pub fn lxtalen(&mut self) -> LXTALEN_W<BDCTL_SPEC, 0> {
        LXTALEN_W::new(self)
    }
    #[doc = "Bit 2 - LXTAL bypass mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalbps(&mut self) -> LXTALBPS_W<BDCTL_SPEC, 2> {
        LXTALBPS_W::new(self)
    }
    #[doc = "Bits 3:4 - LXTAL drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn lxtaldri(&mut self) -> LXTALDRI_W<BDCTL_SPEC, 3> {
        LXTALDRI_W::new(self)
    }
    #[doc = "Bits 8:9 - RTC clock entry selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsrc(&mut self) -> RTCSRC_W<BDCTL_SPEC, 8> {
        RTCSRC_W::new(self)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<BDCTL_SPEC, 15> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 16 - Backup domain reset"]
    #[inline(always)]
    #[must_use]
    pub fn bkprst(&mut self) -> BKPRST_W<BDCTL_SPEC, 16> {
        BKPRST_W::new(self)
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
#[doc = "Backup domain control register (RCU_BDCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDCTL_SPEC;
impl crate::RegisterSpec for BDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdctl::R`](R) reader structure"]
impl crate::Readable for BDCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bdctl::W`](W) writer structure"]
impl crate::Writable for BDCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDCTL to value 0x18"]
impl crate::Resettable for BDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}
