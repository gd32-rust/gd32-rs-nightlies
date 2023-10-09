#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `LDOLP` reader - LDO Low Power Mode"]
pub type LDOLP_R = crate::BitReader<LDOLP_A>;
#[doc = "LDO Low Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDOLP_A {
    #[doc = "0: LDO operates normally during Deepsleep mode"]
    NORMAL = 0,
    #[doc = "1: LDO in low-power mode during Deepsleep mode"]
    LOW_POWER = 1,
}
impl From<LDOLP_A> for bool {
    #[inline(always)]
    fn from(variant: LDOLP_A) -> Self {
        variant as u8 != 0
    }
}
impl LDOLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDOLP_A {
        match self.bits {
            false => LDOLP_A::NORMAL,
            true => LDOLP_A::LOW_POWER,
        }
    }
    #[doc = "LDO operates normally during Deepsleep mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LDOLP_A::NORMAL
    }
    #[doc = "LDO in low-power mode during Deepsleep mode"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == LDOLP_A::LOW_POWER
    }
}
#[doc = "Field `LDOLP` writer - LDO Low Power Mode"]
pub type LDOLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LDOLP_A>;
impl<'a, REG, const O: u8> LDOLP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LDO operates normally during Deepsleep mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(LDOLP_A::NORMAL)
    }
    #[doc = "LDO in low-power mode during Deepsleep mode"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(LDOLP_A::LOW_POWER)
    }
}
#[doc = "Field `STBMOD` reader - Standby Mode"]
pub type STBMOD_R = crate::BitReader<STBMOD_A>;
#[doc = "Standby Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STBMOD_A {
    #[doc = "0: Enter Deep-sleep mode when the CPU enters deepsleep"]
    DEEP_SLEEP = 0,
    #[doc = "1: Enter Standby mode when the CPU enters deepsleep"]
    STANDBY = 1,
}
impl From<STBMOD_A> for bool {
    #[inline(always)]
    fn from(variant: STBMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl STBMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBMOD_A {
        match self.bits {
            false => STBMOD_A::DEEP_SLEEP,
            true => STBMOD_A::STANDBY,
        }
    }
    #[doc = "Enter Deep-sleep mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn is_deep_sleep(&self) -> bool {
        *self == STBMOD_A::DEEP_SLEEP
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == STBMOD_A::STANDBY
    }
}
#[doc = "Field `STBMOD` writer - Standby Mode"]
pub type STBMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STBMOD_A>;
impl<'a, REG, const O: u8> STBMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enter Deep-sleep mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn deep_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(STBMOD_A::DEEP_SLEEP)
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(STBMOD_A::STANDBY)
    }
}
#[doc = "Field `WURST` reader - Wakeup Flag Reset"]
pub type WURST_R = crate::BitReader<WURSTW_A>;
#[doc = "Wakeup Flag Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WURSTW_A {
    #[doc = "1: Clear the wakeup flag"]
    CLEAR = 1,
}
impl From<WURSTW_A> for bool {
    #[inline(always)]
    fn from(variant: WURSTW_A) -> Self {
        variant as u8 != 0
    }
}
impl WURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WURSTW_A> {
        match self.bits {
            true => Some(WURSTW_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Clear the wakeup flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WURSTW_A::CLEAR
    }
}
#[doc = "Field `WURST` writer - Wakeup Flag Reset"]
pub type WURST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WURSTW_A>;
impl<'a, REG, const O: u8> WURST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the wakeup flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WURSTW_A::CLEAR)
    }
}
#[doc = "Field `STBRST` reader - Standby Flag Reset"]
pub type STBRST_R = crate::BitReader<STBRSTW_A>;
#[doc = "Standby Flag Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STBRSTW_A {
    #[doc = "1: Clear the standby flag"]
    CLEAR = 1,
}
impl From<STBRSTW_A> for bool {
    #[inline(always)]
    fn from(variant: STBRSTW_A) -> Self {
        variant as u8 != 0
    }
}
impl STBRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STBRSTW_A> {
        match self.bits {
            true => Some(STBRSTW_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Clear the standby flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == STBRSTW_A::CLEAR
    }
}
#[doc = "Field `STBRST` writer - Standby Flag Reset"]
pub type STBRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STBRSTW_A>;
impl<'a, REG, const O: u8> STBRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the standby flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(STBRSTW_A::CLEAR)
    }
}
#[doc = "Field `LVDEN` reader - Low Voltage Detector Enable"]
pub type LVDEN_R = crate::BitReader<LVDEN_A>;
#[doc = "Low Voltage Detector Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDEN_A {
    #[doc = "0: Low voltage detector disabled"]
    DISABLED = 0,
    #[doc = "1: Low voltage detector enabled"]
    ENABLED = 1,
}
impl From<LVDEN_A> for bool {
    #[inline(always)]
    fn from(variant: LVDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDEN_A {
        match self.bits {
            false => LVDEN_A::DISABLED,
            true => LVDEN_A::ENABLED,
        }
    }
    #[doc = "Low voltage detector disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LVDEN_A::DISABLED
    }
    #[doc = "Low voltage detector enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LVDEN_A::ENABLED
    }
}
#[doc = "Field `LVDEN` writer - Low Voltage Detector Enable"]
pub type LVDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LVDEN_A>;
impl<'a, REG, const O: u8> LVDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low voltage detector disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LVDEN_A::DISABLED)
    }
    #[doc = "Low voltage detector enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LVDEN_A::ENABLED)
    }
}
#[doc = "Field `LVDT` reader - Low Voltage Detector Threshold"]
pub type LVDT_R = crate::FieldReader<LVDT_A>;
#[doc = "Low Voltage Detector Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVDT_A {
    #[doc = "0: 2.2 V"]
    V2_2 = 0,
    #[doc = "1: 2.3 V"]
    V2_3 = 1,
    #[doc = "2: 2.4 V"]
    V2_4 = 2,
    #[doc = "3: 2.5 V"]
    V2_5 = 3,
    #[doc = "4: 2.6 V"]
    V2_6 = 4,
    #[doc = "5: 2.7 V"]
    V2_7 = 5,
    #[doc = "6: 2.8 V"]
    V2_8 = 6,
    #[doc = "7: 2.9 V"]
    V2_9 = 7,
}
impl From<LVDT_A> for u8 {
    #[inline(always)]
    fn from(variant: LVDT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LVDT_A {
    type Ux = u8;
}
impl LVDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDT_A {
        match self.bits {
            0 => LVDT_A::V2_2,
            1 => LVDT_A::V2_3,
            2 => LVDT_A::V2_4,
            3 => LVDT_A::V2_5,
            4 => LVDT_A::V2_6,
            5 => LVDT_A::V2_7,
            6 => LVDT_A::V2_8,
            7 => LVDT_A::V2_9,
            _ => unreachable!(),
        }
    }
    #[doc = "2.2 V"]
    #[inline(always)]
    pub fn is_v2_2(&self) -> bool {
        *self == LVDT_A::V2_2
    }
    #[doc = "2.3 V"]
    #[inline(always)]
    pub fn is_v2_3(&self) -> bool {
        *self == LVDT_A::V2_3
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn is_v2_4(&self) -> bool {
        *self == LVDT_A::V2_4
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        *self == LVDT_A::V2_5
    }
    #[doc = "2.6 V"]
    #[inline(always)]
    pub fn is_v2_6(&self) -> bool {
        *self == LVDT_A::V2_6
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn is_v2_7(&self) -> bool {
        *self == LVDT_A::V2_7
    }
    #[doc = "2.8 V"]
    #[inline(always)]
    pub fn is_v2_8(&self) -> bool {
        *self == LVDT_A::V2_8
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn is_v2_9(&self) -> bool {
        *self == LVDT_A::V2_9
    }
}
#[doc = "Field `LVDT` writer - Low Voltage Detector Threshold"]
pub type LVDT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, LVDT_A>;
impl<'a, REG, const O: u8> LVDT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.2 V"]
    #[inline(always)]
    pub fn v2_2(self) -> &'a mut crate::W<REG> {
        self.variant(LVDT_A::V2_2)
    }
    #[doc = "2.3 V"]
    #[inline(always)]
    pub fn v2_3(self) -> &'a mut crate::W<REG> {
        self.variant(LVDT_A::V2_3)
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn v2_4(self) -> &'a mut crate::W<REG> {
        self.variant(LVDT_A::V2_4)
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut crate::W<REG> {
        self.variant(LVDT_A::V2_5)
    }
    #[doc = "2.6 V"]
    #[inline(always)]
    pub fn v2_6(self) -> &'a mut crate::W<REG> {
        self.variant(LVDT_A::V2_6)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn v2_7(self) -> &'a mut crate::W<REG> {
        self.variant(LVDT_A::V2_7)
    }
    #[doc = "2.8 V"]
    #[inline(always)]
    pub fn v2_8(self) -> &'a mut crate::W<REG> {
        self.variant(LVDT_A::V2_8)
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn v2_9(self) -> &'a mut crate::W<REG> {
        self.variant(LVDT_A::V2_9)
    }
}
#[doc = "Field `BKPWEN` reader - Backup Domain Write Enable"]
pub type BKPWEN_R = crate::BitReader<BKPWEN_A>;
#[doc = "Backup Domain Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKPWEN_A {
    #[doc = "0: Access to backup domain registers disabled"]
    DISABLED = 0,
    #[doc = "1: Access to backup domain registers enabled"]
    ENABLED = 1,
}
impl From<BKPWEN_A> for bool {
    #[inline(always)]
    fn from(variant: BKPWEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BKPWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKPWEN_A {
        match self.bits {
            false => BKPWEN_A::DISABLED,
            true => BKPWEN_A::ENABLED,
        }
    }
    #[doc = "Access to backup domain registers disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKPWEN_A::DISABLED
    }
    #[doc = "Access to backup domain registers enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKPWEN_A::ENABLED
    }
}
#[doc = "Field `BKPWEN` writer - Backup Domain Write Enable"]
pub type BKPWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BKPWEN_A>;
impl<'a, REG, const O: u8> BKPWEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to backup domain registers disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKPWEN_A::DISABLED)
    }
    #[doc = "Access to backup domain registers enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKPWEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - LDO Low Power Mode"]
    #[inline(always)]
    pub fn ldolp(&self) -> LDOLP_R {
        LDOLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby Mode"]
    #[inline(always)]
    pub fn stbmod(&self) -> STBMOD_R {
        STBMOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    pub fn wurst(&self) -> WURST_R {
        WURST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    pub fn stbrst(&self) -> STBRST_R {
        STBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Voltage Detector Enable"]
    #[inline(always)]
    pub fn lvden(&self) -> LVDEN_R {
        LVDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Low Voltage Detector Threshold"]
    #[inline(always)]
    pub fn lvdt(&self) -> LVDT_R {
        LVDT_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Backup Domain Write Enable"]
    #[inline(always)]
    pub fn bkpwen(&self) -> BKPWEN_R {
        BKPWEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LDO Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ldolp(&mut self) -> LDOLP_W<CTL_SPEC, 0> {
        LDOLP_W::new(self)
    }
    #[doc = "Bit 1 - Standby Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stbmod(&mut self) -> STBMOD_W<CTL_SPEC, 1> {
        STBMOD_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wurst(&mut self) -> WURST_W<CTL_SPEC, 2> {
        WURST_W::new(self)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    #[must_use]
    pub fn stbrst(&mut self) -> STBRST_W<CTL_SPEC, 3> {
        STBRST_W::new(self)
    }
    #[doc = "Bit 4 - Low Voltage Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvden(&mut self) -> LVDEN_W<CTL_SPEC, 4> {
        LVDEN_W::new(self)
    }
    #[doc = "Bits 5:7 - Low Voltage Detector Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lvdt(&mut self) -> LVDT_W<CTL_SPEC, 5> {
        LVDT_W::new(self)
    }
    #[doc = "Bit 8 - Backup Domain Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpwen(&mut self) -> BKPWEN_W<CTL_SPEC, 8> {
        BKPWEN_W::new(self)
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
#[doc = "power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
