#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDEN` reader - Low-driver mode enable in Deep-sleep mode"]
pub type LDEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LDEN` writer - Low-driver mode enable in Deep-sleep mode"]
pub type LDEN_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, 18>;
#[doc = "Field `HDS` reader - High-driver mode switch"]
pub type HDS_R = crate::BitReader<bool>;
#[doc = "Field `HDS` writer - High-driver mode switch"]
pub type HDS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 17>;
#[doc = "Field `HDEN` reader - High-driver mode enable"]
pub type HDEN_R = crate::BitReader<bool>;
#[doc = "Field `HDEN` writer - High-driver mode enable"]
pub type HDEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 16>;
#[doc = "Field `LDOVS` reader - LDO output voltage select"]
pub type LDOVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LDOVS` writer - LDO output voltage select"]
pub type LDOVS_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, 14>;
#[doc = "Field `LDNP` reader - Low-driver mode when use normal power LDO"]
pub type LDNP_R = crate::BitReader<bool>;
#[doc = "Field `LDNP` writer - Low-driver mode when use normal power LDO"]
pub type LDNP_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 11>;
#[doc = "Field `LDLP` reader - Low-driver mode when use low power LDO."]
pub type LDLP_R = crate::BitReader<bool>;
#[doc = "Field `LDLP` writer - Low-driver mode when use low power LDO."]
pub type LDLP_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 10>;
#[doc = "Backup Domain Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `BKPWEN` reader - Backup Domain Write Enable"]
pub type BKPWEN_R = crate::BitReader<BKPWEN_A>;
impl BKPWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKPWEN_A {
        match self.bits {
            false => BKPWEN_A::DISABLED,
            true => BKPWEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKPWEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKPWEN_A::ENABLED
    }
}
#[doc = "Field `BKPWEN` writer - Backup Domain Write Enable"]
pub type BKPWEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, BKPWEN_A, 8>;
impl<'a> BKPWEN_W<'a> {
    #[doc = "Access to backup domain registers disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BKPWEN_A::DISABLED)
    }
    #[doc = "Access to backup domain registers enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BKPWEN_A::ENABLED)
    }
}
#[doc = "Low Voltage Detector Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LVDT` reader - Low Voltage Detector Threshold"]
pub type LVDT_R = crate::FieldReader<u8, LVDT_A>;
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
    #[doc = "Checks if the value of the field is `V2_2`"]
    #[inline(always)]
    pub fn is_v2_2(&self) -> bool {
        *self == LVDT_A::V2_2
    }
    #[doc = "Checks if the value of the field is `V2_3`"]
    #[inline(always)]
    pub fn is_v2_3(&self) -> bool {
        *self == LVDT_A::V2_3
    }
    #[doc = "Checks if the value of the field is `V2_4`"]
    #[inline(always)]
    pub fn is_v2_4(&self) -> bool {
        *self == LVDT_A::V2_4
    }
    #[doc = "Checks if the value of the field is `V2_5`"]
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        *self == LVDT_A::V2_5
    }
    #[doc = "Checks if the value of the field is `V2_6`"]
    #[inline(always)]
    pub fn is_v2_6(&self) -> bool {
        *self == LVDT_A::V2_6
    }
    #[doc = "Checks if the value of the field is `V2_7`"]
    #[inline(always)]
    pub fn is_v2_7(&self) -> bool {
        *self == LVDT_A::V2_7
    }
    #[doc = "Checks if the value of the field is `V2_8`"]
    #[inline(always)]
    pub fn is_v2_8(&self) -> bool {
        *self == LVDT_A::V2_8
    }
    #[doc = "Checks if the value of the field is `V2_9`"]
    #[inline(always)]
    pub fn is_v2_9(&self) -> bool {
        *self == LVDT_A::V2_9
    }
}
#[doc = "Field `LVDT` writer - Low Voltage Detector Threshold"]
pub type LVDT_W<'a> = crate::FieldWriterSafe<'a, u32, CTL_SPEC, u8, LVDT_A, 3, 5>;
impl<'a> LVDT_W<'a> {
    #[doc = "2.2 V"]
    #[inline(always)]
    pub fn v2_2(self) -> &'a mut W {
        self.variant(LVDT_A::V2_2)
    }
    #[doc = "2.3 V"]
    #[inline(always)]
    pub fn v2_3(self) -> &'a mut W {
        self.variant(LVDT_A::V2_3)
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn v2_4(self) -> &'a mut W {
        self.variant(LVDT_A::V2_4)
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut W {
        self.variant(LVDT_A::V2_5)
    }
    #[doc = "2.6 V"]
    #[inline(always)]
    pub fn v2_6(self) -> &'a mut W {
        self.variant(LVDT_A::V2_6)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn v2_7(self) -> &'a mut W {
        self.variant(LVDT_A::V2_7)
    }
    #[doc = "2.8 V"]
    #[inline(always)]
    pub fn v2_8(self) -> &'a mut W {
        self.variant(LVDT_A::V2_8)
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn v2_9(self) -> &'a mut W {
        self.variant(LVDT_A::V2_9)
    }
}
#[doc = "Low Voltage Detector Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LVDEN` reader - Low Voltage Detector Enable"]
pub type LVDEN_R = crate::BitReader<LVDEN_A>;
impl LVDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDEN_A {
        match self.bits {
            false => LVDEN_A::DISABLED,
            true => LVDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LVDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LVDEN_A::ENABLED
    }
}
#[doc = "Field `LVDEN` writer - Low Voltage Detector Enable"]
pub type LVDEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, LVDEN_A, 4>;
impl<'a> LVDEN_W<'a> {
    #[doc = "Low voltage detector disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LVDEN_A::DISABLED)
    }
    #[doc = "Low voltage detector enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LVDEN_A::ENABLED)
    }
}
#[doc = "Standby Flag Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBRST_A {
    #[doc = "1: Clear the standby flag"]
    CLEAR = 1,
}
impl From<STBRST_A> for bool {
    #[inline(always)]
    fn from(variant: STBRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBRST` reader - Standby Flag Reset"]
pub type STBRST_R = crate::BitReader<STBRST_A>;
impl STBRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STBRST_A> {
        match self.bits {
            true => Some(STBRST_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == STBRST_A::CLEAR
    }
}
#[doc = "Field `STBRST` writer - Standby Flag Reset"]
pub type STBRST_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, STBRST_A, 3>;
impl<'a> STBRST_W<'a> {
    #[doc = "Clear the standby flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STBRST_A::CLEAR)
    }
}
#[doc = "Wakeup Flag Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WURST_A {
    #[doc = "1: Clear the wakeup flag"]
    CLEAR = 1,
}
impl From<WURST_A> for bool {
    #[inline(always)]
    fn from(variant: WURST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WURST` reader - Wakeup Flag Reset"]
pub type WURST_R = crate::BitReader<WURST_A>;
impl WURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WURST_A> {
        match self.bits {
            true => Some(WURST_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WURST_A::CLEAR
    }
}
#[doc = "Field `WURST` writer - Wakeup Flag Reset"]
pub type WURST_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, WURST_A, 2>;
impl<'a> WURST_W<'a> {
    #[doc = "Clear the wakeup flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WURST_A::CLEAR)
    }
}
#[doc = "Standby Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBMOD_A {
    #[doc = "0: Enter Deep-sleep mode when the CPU enters deepsleep"]
    DEEPSLEEP = 0,
    #[doc = "1: Enter Standby mode when the CPU enters deepsleep"]
    STANDBY = 1,
}
impl From<STBMOD_A> for bool {
    #[inline(always)]
    fn from(variant: STBMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBMOD` reader - Standby Mode"]
pub type STBMOD_R = crate::BitReader<STBMOD_A>;
impl STBMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBMOD_A {
        match self.bits {
            false => STBMOD_A::DEEPSLEEP,
            true => STBMOD_A::STANDBY,
        }
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deep_sleep(&self) -> bool {
        *self == STBMOD_A::DEEPSLEEP
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == STBMOD_A::STANDBY
    }
}
#[doc = "Field `STBMOD` writer - Standby Mode"]
pub type STBMOD_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, STBMOD_A, 1>;
impl<'a> STBMOD_W<'a> {
    #[doc = "Enter Deep-sleep mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn deep_sleep(self) -> &'a mut W {
        self.variant(STBMOD_A::DEEPSLEEP)
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(STBMOD_A::STANDBY)
    }
}
#[doc = "LDO Low Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDOLP_A {
    #[doc = "0: LDO operates normally during Deepsleep mode"]
    NORMAL = 0,
    #[doc = "1: LDO in low-power mode during Deepsleep mode"]
    LOWPOWER = 1,
}
impl From<LDOLP_A> for bool {
    #[inline(always)]
    fn from(variant: LDOLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDOLP` reader - LDO Low Power Mode"]
pub type LDOLP_R = crate::BitReader<LDOLP_A>;
impl LDOLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDOLP_A {
        match self.bits {
            false => LDOLP_A::NORMAL,
            true => LDOLP_A::LOWPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LDOLP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == LDOLP_A::LOWPOWER
    }
}
#[doc = "Field `LDOLP` writer - LDO Low Power Mode"]
pub type LDOLP_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, LDOLP_A, 0>;
impl<'a> LDOLP_W<'a> {
    #[doc = "LDO operates normally during Deepsleep mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LDOLP_A::NORMAL)
    }
    #[doc = "LDO in low-power mode during Deepsleep mode"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(LDOLP_A::LOWPOWER)
    }
}
impl R {
    #[doc = "Bits 18:19 - Low-driver mode enable in Deep-sleep mode"]
    #[inline(always)]
    pub fn lden(&self) -> LDEN_R {
        LDEN_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 17 - High-driver mode switch"]
    #[inline(always)]
    pub fn hds(&self) -> HDS_R {
        HDS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - High-driver mode enable"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 14:15 - LDO output voltage select"]
    #[inline(always)]
    pub fn ldovs(&self) -> LDOVS_R {
        LDOVS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 11 - Low-driver mode when use normal power LDO"]
    #[inline(always)]
    pub fn ldnp(&self) -> LDNP_R {
        LDNP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Low-driver mode when use low power LDO."]
    #[inline(always)]
    pub fn ldlp(&self) -> LDLP_R {
        LDLP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 8 - Backup Domain Write Enable"]
    #[inline(always)]
    pub fn bkpwen(&self) -> BKPWEN_R {
        BKPWEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Low Voltage Detector Threshold"]
    #[inline(always)]
    pub fn lvdt(&self) -> LVDT_R {
        LVDT_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 4 - Low Voltage Detector Enable"]
    #[inline(always)]
    pub fn lvden(&self) -> LVDEN_R {
        LVDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    pub fn stbrst(&self) -> STBRST_R {
        STBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    pub fn wurst(&self) -> WURST_R {
        WURST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Standby Mode"]
    #[inline(always)]
    pub fn stbmod(&self) -> STBMOD_R {
        STBMOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - LDO Low Power Mode"]
    #[inline(always)]
    pub fn ldolp(&self) -> LDOLP_R {
        LDOLP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:19 - Low-driver mode enable in Deep-sleep mode"]
    #[inline(always)]
    pub fn lden(&mut self) -> LDEN_W {
        LDEN_W::new(self)
    }
    #[doc = "Bit 17 - High-driver mode switch"]
    #[inline(always)]
    pub fn hds(&mut self) -> HDS_W {
        HDS_W::new(self)
    }
    #[doc = "Bit 16 - High-driver mode enable"]
    #[inline(always)]
    pub fn hden(&mut self) -> HDEN_W {
        HDEN_W::new(self)
    }
    #[doc = "Bits 14:15 - LDO output voltage select"]
    #[inline(always)]
    pub fn ldovs(&mut self) -> LDOVS_W {
        LDOVS_W::new(self)
    }
    #[doc = "Bit 11 - Low-driver mode when use normal power LDO"]
    #[inline(always)]
    pub fn ldnp(&mut self) -> LDNP_W {
        LDNP_W::new(self)
    }
    #[doc = "Bit 10 - Low-driver mode when use low power LDO."]
    #[inline(always)]
    pub fn ldlp(&mut self) -> LDLP_W {
        LDLP_W::new(self)
    }
    #[doc = "Bit 8 - Backup Domain Write Enable"]
    #[inline(always)]
    pub fn bkpwen(&mut self) -> BKPWEN_W {
        BKPWEN_W::new(self)
    }
    #[doc = "Bits 5:7 - Low Voltage Detector Threshold"]
    #[inline(always)]
    pub fn lvdt(&mut self) -> LVDT_W {
        LVDT_W::new(self)
    }
    #[doc = "Bit 4 - Low Voltage Detector Enable"]
    #[inline(always)]
    pub fn lvden(&mut self) -> LVDEN_W {
        LVDEN_W::new(self)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    pub fn stbrst(&mut self) -> STBRST_W {
        STBRST_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    pub fn wurst(&mut self) -> WURST_W {
        WURST_W::new(self)
    }
    #[doc = "Bit 1 - Standby Mode"]
    #[inline(always)]
    pub fn stbmod(&mut self) -> STBMOD_W {
        STBMOD_W::new(self)
    }
    #[doc = "Bit 0 - LDO Low Power Mode"]
    #[inline(always)]
    pub fn ldolp(&mut self) -> LDOLP_W {
        LDOLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0xc000"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000
    }
}
