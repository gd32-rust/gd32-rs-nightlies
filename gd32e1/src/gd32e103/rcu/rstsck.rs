#[doc = "Register `RSTSCK` reader"]
pub type R = crate::R<RSTSCK_SPEC>;
#[doc = "Register `RSTSCK` writer"]
pub type W = crate::W<RSTSCK_SPEC>;
#[doc = "Field `IRC40KEN` reader - IRC40K enable"]
pub type IRC40KEN_R = crate::BitReader<IRC40KEN_A>;
#[doc = "IRC40K enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl IRC40KEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC40KEN_A {
        match self.bits {
            false => IRC40KEN_A::OFF,
            true => IRC40KEN_A::ON,
        }
    }
    #[doc = "IRC40K oscillator disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IRC40KEN_A::OFF
    }
    #[doc = "IRC40K oscillator enabled"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == IRC40KEN_A::ON
    }
}
#[doc = "Field `IRC40KEN` writer - IRC40K enable"]
pub type IRC40KEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRC40KEN_A>;
impl<'a, REG, const O: u8> IRC40KEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IRC40K oscillator disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(IRC40KEN_A::OFF)
    }
    #[doc = "IRC40K oscillator enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(IRC40KEN_A::ON)
    }
}
#[doc = "Field `IRC40KSTB` reader - IRC40K stabilization"]
pub type IRC40KSTB_R = crate::BitReader<IRC40KSTBR_A>;
#[doc = "IRC40K stabilization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRC40KSTBR_A {
    #[doc = "0: IRC40K oscillator is not stable"]
    NOT_READY = 0,
    #[doc = "1: IRC40K oscillator is stable"]
    READY = 1,
}
impl From<IRC40KSTBR_A> for bool {
    #[inline(always)]
    fn from(variant: IRC40KSTBR_A) -> Self {
        variant as u8 != 0
    }
}
impl IRC40KSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC40KSTBR_A {
        match self.bits {
            false => IRC40KSTBR_A::NOT_READY,
            true => IRC40KSTBR_A::READY,
        }
    }
    #[doc = "IRC40K oscillator is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == IRC40KSTBR_A::NOT_READY
    }
    #[doc = "IRC40K oscillator is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == IRC40KSTBR_A::READY
    }
}
#[doc = "Field `RSTFC` reader - Reset flag clear"]
pub type RSTFC_R = crate::BitReader<RSTFCW_A>;
#[doc = "Reset flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTFCW_A {
    #[doc = "1: Clears reset flags"]
    CLEAR = 1,
}
impl From<RSTFCW_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFCW_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSTFCW_A> {
        match self.bits {
            true => Some(RSTFCW_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Clears reset flags"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RSTFCW_A::CLEAR
    }
}
#[doc = "Field `RSTFC` writer - Reset flag clear"]
pub type RSTFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RSTFCW_A>;
impl<'a, REG, const O: u8> RSTFC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears reset flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RSTFCW_A::CLEAR)
    }
}
#[doc = "Field `EPRSTF` reader - External PIN reset flag"]
pub type EPRSTF_R = crate::BitReader<EPRSTFR_A>;
#[doc = "External PIN reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPRSTFR_A {
    #[doc = "0: No reset has occured"]
    NO_RESET = 0,
    #[doc = "1: A reset has occured"]
    RESET = 1,
}
impl From<EPRSTFR_A> for bool {
    #[inline(always)]
    fn from(variant: EPRSTFR_A) -> Self {
        variant as u8 != 0
    }
}
impl EPRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPRSTFR_A {
        match self.bits {
            false => EPRSTFR_A::NO_RESET,
            true => EPRSTFR_A::RESET,
        }
    }
    #[doc = "No reset has occured"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == EPRSTFR_A::NO_RESET
    }
    #[doc = "A reset has occured"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == EPRSTFR_A::RESET
    }
}
#[doc = "Field `PORRSTF` reader - Power reset flag"]
pub use EPRSTF_R as PORRSTF_R;
#[doc = "Field `SWRSTF` reader - Software reset flag"]
pub use EPRSTF_R as SWRSTF_R;
#[doc = "Field `FWDGTRSTF` reader - Free Watchdog timer reset flag"]
pub use EPRSTF_R as FWDGTRSTF_R;
#[doc = "Field `WWDGTRSTF` reader - Window watchdog timer reset flag"]
pub use EPRSTF_R as WWDGTRSTF_R;
#[doc = "Field `LPRSTF` reader - Low-power reset flag"]
pub use EPRSTF_R as LPRSTF_R;
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
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    pub fn rstfc(&self) -> RSTFC_R {
        RSTFC_R::new(((self.bits >> 24) & 1) != 0)
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
    #[must_use]
    pub fn irc40ken(&mut self) -> IRC40KEN_W<RSTSCK_SPEC, 0> {
        IRC40KEN_W::new(self)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstfc(&mut self) -> RSTFC_W<RSTSCK_SPEC, 24> {
        RSTFC_W::new(self)
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
#[doc = "Reset source /clock register (RCU_RSTSCK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstsck::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstsck::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTSCK_SPEC;
impl crate::RegisterSpec for RSTSCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstsck::R`](R) reader structure"]
impl crate::Readable for RSTSCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rstsck::W`](W) writer structure"]
impl crate::Writable for RSTSCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTSCK to value 0x0c00_0000"]
impl crate::Resettable for RSTSCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_0000;
}
