#[doc = "Register `RSTSCK` reader"]
pub struct R(crate::R<RSTSCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTSCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTSCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTSCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTSCK` writer"]
pub struct W(crate::W<RSTSCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTSCK_SPEC>;
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
impl From<crate::W<RSTSCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTSCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IRC40K enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC40KEN_A {
    #[doc = "0: IRC40K oscillator disabled"]
    OFF = 0,
    #[doc = "1: IRC40K oscillator enabled"]
    ON = 1,
}
impl From<IRC40KEN_A> for bool {
    #[inline(always)]
    fn from(variant: IRC40KEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC40KEN` reader - IRC40K enable"]
pub type IRC40KEN_R = crate::BitReader<IRC40KEN_A>;
impl IRC40KEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC40KEN_A {
        match self.bits {
            false => IRC40KEN_A::OFF,
            true => IRC40KEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IRC40KEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == IRC40KEN_A::ON
    }
}
#[doc = "Field `IRC40KEN` writer - IRC40K enable"]
pub type IRC40KEN_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, IRC40KEN_A, 0>;
impl<'a> IRC40KEN_W<'a> {
    #[doc = "IRC40K oscillator disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IRC40KEN_A::OFF)
    }
    #[doc = "IRC40K oscillator enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(IRC40KEN_A::ON)
    }
}
#[doc = "IRC40K stabilization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC40KSTB_A {
    #[doc = "0: IRC40K oscillator is not stable"]
    NOTREADY = 0,
    #[doc = "1: IRC40K oscillator is stable"]
    READY = 1,
}
impl From<IRC40KSTB_A> for bool {
    #[inline(always)]
    fn from(variant: IRC40KSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC40KSTB` reader - IRC40K stabilization"]
pub type IRC40KSTB_R = crate::BitReader<IRC40KSTB_A>;
impl IRC40KSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC40KSTB_A {
        match self.bits {
            false => IRC40KSTB_A::NOTREADY,
            true => IRC40KSTB_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == IRC40KSTB_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == IRC40KSTB_A::READY
    }
}
#[doc = "V12 domain Power reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V12RSTF_A {
    #[doc = "0: No reset has occured"]
    NORESET = 0,
    #[doc = "1: A reset has occured"]
    RESET = 1,
}
impl From<V12RSTF_A> for bool {
    #[inline(always)]
    fn from(variant: V12RSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V12RSTF` reader - V12 domain Power reset flag"]
pub type V12RSTF_R = crate::BitReader<V12RSTF_A>;
impl V12RSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V12RSTF_A {
        match self.bits {
            false => V12RSTF_A::NORESET,
            true => V12RSTF_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == V12RSTF_A::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == V12RSTF_A::RESET
    }
}
#[doc = "Field `V12RSTF` writer - V12 domain Power reset flag"]
pub type V12RSTF_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, V12RSTF_A, 23>;
impl<'a> V12RSTF_W<'a> {
    #[doc = "No reset has occured"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(V12RSTF_A::NORESET)
    }
    #[doc = "A reset has occured"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(V12RSTF_A::RESET)
    }
}
#[doc = "Reset flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFC_A {
    #[doc = "1: Clears reset flags"]
    CLEAR = 1,
}
impl From<RSTFC_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTFC` reader - Reset flag clear"]
pub type RSTFC_R = crate::BitReader<RSTFC_A>;
impl RSTFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSTFC_A> {
        match self.bits {
            true => Some(RSTFC_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RSTFC_A::CLEAR
    }
}
#[doc = "Field `RSTFC` writer - Reset flag clear"]
pub type RSTFC_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, RSTFC_A, 24>;
impl<'a> RSTFC_W<'a> {
    #[doc = "Clears reset flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSTFC_A::CLEAR)
    }
}
#[doc = "Option byte loader reset flag"]
pub use V12RSTF_A as OBLRSTF_A;
#[doc = "External PIN reset flag"]
pub use V12RSTF_A as EPRSTF_A;
#[doc = "Power reset flag"]
pub use V12RSTF_A as PORRSTF_A;
#[doc = "Software reset flag"]
pub use V12RSTF_A as SWRSTF_A;
#[doc = "Free Watchdog timer reset flag"]
pub use V12RSTF_A as FWDGTRSTF_A;
#[doc = "Window watchdog timer reset flag"]
pub use V12RSTF_A as WWDGTRSTF_A;
#[doc = "Low-power reset flag"]
pub use V12RSTF_A as LPRSTF_A;
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub use V12RSTF_R as OBLRSTF_R;
#[doc = "Field `EPRSTF` reader - External PIN reset flag"]
pub use V12RSTF_R as EPRSTF_R;
#[doc = "Field `PORRSTF` reader - Power reset flag"]
pub use V12RSTF_R as PORRSTF_R;
#[doc = "Field `SWRSTF` reader - Software reset flag"]
pub use V12RSTF_R as SWRSTF_R;
#[doc = "Field `FWDGTRSTF` reader - Free Watchdog timer reset flag"]
pub use V12RSTF_R as FWDGTRSTF_R;
#[doc = "Field `WWDGTRSTF` reader - Window watchdog timer reset flag"]
pub use V12RSTF_R as WWDGTRSTF_R;
#[doc = "Field `LPRSTF` reader - Low-power reset flag"]
pub use V12RSTF_R as LPRSTF_R;
#[doc = "Field `OBLRSTF` writer - Option byte loader reset flag"]
pub use V12RSTF_W as OBLRSTF_W;
#[doc = "Field `EPRSTF` writer - External PIN reset flag"]
pub use V12RSTF_W as EPRSTF_W;
#[doc = "Field `PORRSTF` writer - Power reset flag"]
pub use V12RSTF_W as PORRSTF_W;
#[doc = "Field `SWRSTF` writer - Software reset flag"]
pub use V12RSTF_W as SWRSTF_W;
#[doc = "Field `FWDGTRSTF` writer - Free Watchdog timer reset flag"]
pub use V12RSTF_W as FWDGTRSTF_W;
#[doc = "Field `WWDGTRSTF` writer - Window watchdog timer reset flag"]
pub use V12RSTF_W as WWDGTRSTF_W;
#[doc = "Field `LPRSTF` writer - Low-power reset flag"]
pub use V12RSTF_W as LPRSTF_W;
impl R {
    #[doc = "Bit 0 - IRC40K enable"]
    #[inline(always)]
    pub fn irc40ken(&self) -> IRC40KEN_R {
        IRC40KEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC40K stabilization"]
    #[inline(always)]
    pub fn irc40kstb(&self) -> IRC40KSTB_R {
        IRC40KSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 23 - V12 domain Power reset flag"]
    #[inline(always)]
    pub fn v12rstf(&self) -> V12RSTF_R {
        V12RSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    pub fn rstfc(&self) -> RSTFC_R {
        RSTFC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - External PIN reset flag"]
    #[inline(always)]
    pub fn eprstf(&self) -> EPRSTF_R {
        EPRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Power reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&self) -> SWRSTF_R {
        SWRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Free Watchdog timer reset flag"]
    #[inline(always)]
    pub fn fwdgtrstf(&self) -> FWDGTRSTF_R {
        FWDGTRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    pub fn wwdgtrstf(&self) -> WWDGTRSTF_R {
        WWDGTRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lprstf(&self) -> LPRSTF_R {
        LPRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRC40K enable"]
    #[inline(always)]
    pub fn irc40ken(&mut self) -> IRC40KEN_W {
        IRC40KEN_W::new(self)
    }
    #[doc = "Bit 23 - V12 domain Power reset flag"]
    #[inline(always)]
    pub fn v12rstf(&mut self) -> V12RSTF_W {
        V12RSTF_W::new(self)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    pub fn rstfc(&mut self) -> RSTFC_W {
        RSTFC_W::new(self)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&mut self) -> OBLRSTF_W {
        OBLRSTF_W::new(self)
    }
    #[doc = "Bit 26 - External PIN reset flag"]
    #[inline(always)]
    pub fn eprstf(&mut self) -> EPRSTF_W {
        EPRSTF_W::new(self)
    }
    #[doc = "Bit 27 - Power reset flag"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W {
        PORRSTF_W::new(self)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&mut self) -> SWRSTF_W {
        SWRSTF_W::new(self)
    }
    #[doc = "Bit 29 - Free Watchdog timer reset flag"]
    #[inline(always)]
    pub fn fwdgtrstf(&mut self) -> FWDGTRSTF_W {
        FWDGTRSTF_W::new(self)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    pub fn wwdgtrstf(&mut self) -> WWDGTRSTF_W {
        WWDGTRSTF_W::new(self)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lprstf(&mut self) -> LPRSTF_W {
        LPRSTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset source /clock register (RCU_RSTSCK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstsck](index.html) module"]
pub struct RSTSCK_SPEC;
impl crate::RegisterSpec for RSTSCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstsck::R](R) reader structure"]
impl crate::Readable for RSTSCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstsck::W](W) writer structure"]
impl crate::Writable for RSTSCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTSCK to value 0x0c00_0000"]
impl crate::Resettable for RSTSCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c00_0000
    }
}
