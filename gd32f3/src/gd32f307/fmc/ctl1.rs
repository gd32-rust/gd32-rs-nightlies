#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "End of operation interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIE_A {
    #[doc = "0: End of operation interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: End of operation interrupt enabled"]
    ENABLED = 1,
}
impl From<ENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIE` reader - End of operation interrupt enable bit"]
pub type ENDIE_R = crate::BitReader<ENDIE_A>;
impl ENDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIE_A {
        match self.bits {
            false => ENDIE_A::DISABLED,
            true => ENDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDIE_A::ENABLED
    }
}
#[doc = "Field `ENDIE` writer - End of operation interrupt enable bit"]
pub type ENDIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, ENDIE_A, 12>;
impl<'a> ENDIE_W<'a> {
    #[doc = "End of operation interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDIE_A::DISABLED)
    }
    #[doc = "End of operation interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDIE_A::ENABLED)
    }
}
#[doc = "Error interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt generation disabled"]
    DISABLED = 0,
    #[doc = "1: Error interrupt generation enabled"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable bit"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable bit"]
pub type ERRIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, ERRIE_A, 10>;
impl<'a> ERRIE_W<'a> {
    #[doc = "Error interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Error interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
    }
}
#[doc = "FMC_CTL0 lock bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
    #[doc = "0: CTL register is unlocked"]
    UNLOCKED = 0,
    #[doc = "1: CTL register is locked"]
    LOCKED = 1,
}
impl From<LK_A> for bool {
    #[inline(always)]
    fn from(variant: LK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LK` reader - FMC_CTL0 lock bit"]
pub type LK_R = crate::BitReader<LK_A>;
impl LK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LK_A {
        match self.bits {
            false => LK_A::UNLOCKED,
            true => LK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LK_A::LOCKED
    }
}
#[doc = "FMC_CTL0 lock bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_AW {
    #[doc = "1: Lock CTL register"]
    LOCK = 1,
}
impl From<LK_AW> for bool {
    #[inline(always)]
    fn from(variant: LK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LK` writer - FMC_CTL0 lock bit"]
pub type LK_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, LK_AW, 7>;
impl<'a> LK_W<'a> {
    #[doc = "Lock CTL register"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LK_AW::LOCK)
    }
}
#[doc = "Send erase command to FMC bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "1: Trigger an erase operation"]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Send erase command to FMC bit"]
pub type START_R = crate::BitReader<START_A>;
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<START_A> {
        match self.bits {
            true => Some(START_A::START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
#[doc = "Field `START` writer - Send erase command to FMC bit"]
pub type START_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, START_A, 6>;
impl<'a> START_W<'a> {
    #[doc = "Trigger an erase operation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
    }
}
#[doc = "Main flash mass erase for bank0 command bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MER_A {
    #[doc = "1: Erase activated for all user sectors"]
    MASSERASE = 1,
}
impl From<MER_A> for bool {
    #[inline(always)]
    fn from(variant: MER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MER` reader - Main flash mass erase for bank0 command bit"]
pub type MER_R = crate::BitReader<MER_A>;
impl MER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MER_A> {
        match self.bits {
            true => Some(MER_A::MASSERASE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASSERASE`"]
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER_A::MASSERASE
    }
}
#[doc = "Field `MER` writer - Main flash mass erase for bank0 command bit"]
pub type MER_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, MER_A, 2>;
impl<'a> MER_W<'a> {
    #[doc = "Erase activated for all user sectors"]
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut W {
        self.variant(MER_A::MASSERASE)
    }
}
#[doc = "Main flash page erase for bank0 command bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER_A {
    #[doc = "1: Erase activated for selected page"]
    PAGEERASE = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - Main flash page erase for bank0 command bit"]
pub type PER_R = crate::BitReader<PER_A>;
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PER_A> {
        match self.bits {
            true => Some(PER_A::PAGEERASE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PAGEERASE`"]
    #[inline(always)]
    pub fn is_page_erase(&self) -> bool {
        *self == PER_A::PAGEERASE
    }
}
#[doc = "Field `PER` writer - Main flash page erase for bank0 command bit"]
pub type PER_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, PER_A, 1>;
impl<'a> PER_W<'a> {
    #[doc = "Erase activated for selected page"]
    #[inline(always)]
    pub fn page_erase(self) -> &'a mut W {
        self.variant(PER_A::PAGEERASE)
    }
}
#[doc = "Main flash program for bank0 command bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PG_A {
    #[doc = "1: Flash programming activated"]
    PROGRAM = 1,
}
impl From<PG_A> for bool {
    #[inline(always)]
    fn from(variant: PG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PG` reader - Main flash program for bank0 command bit"]
pub type PG_R = crate::BitReader<PG_A>;
impl PG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG_A> {
        match self.bits {
            true => Some(PG_A::PROGRAM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PROGRAM`"]
    #[inline(always)]
    pub fn is_program(&self) -> bool {
        *self == PG_A::PROGRAM
    }
}
#[doc = "Field `PG` writer - Main flash program for bank0 command bit"]
pub type PG_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, PG_A, 0>;
impl<'a> PG_W<'a> {
    #[doc = "Flash programming activated"]
    #[inline(always)]
    pub fn program(self) -> &'a mut W {
        self.variant(PG_A::PROGRAM)
    }
}
impl R {
    #[doc = "Bit 12 - End of operation interrupt enable bit"]
    #[inline(always)]
    pub fn endie(&self) -> ENDIE_R {
        ENDIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable bit"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 7 - FMC_CTL0 lock bit"]
    #[inline(always)]
    pub fn lk(&self) -> LK_R {
        LK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Send erase command to FMC bit"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 2 - Main flash mass erase for bank0 command bit"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Main flash page erase for bank0 command bit"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Main flash program for bank0 command bit"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - End of operation interrupt enable bit"]
    #[inline(always)]
    pub fn endie(&mut self) -> ENDIE_W {
        ENDIE_W::new(self)
    }
    #[doc = "Bit 10 - Error interrupt enable bit"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 7 - FMC_CTL0 lock bit"]
    #[inline(always)]
    pub fn lk(&mut self) -> LK_W {
        LK_W::new(self)
    }
    #[doc = "Bit 6 - Send erase command to FMC bit"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bit 2 - Main flash mass erase for bank0 command bit"]
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W {
        MER_W::new(self)
    }
    #[doc = "Bit 1 - Main flash page erase for bank0 command bit"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W::new(self)
    }
    #[doc = "Bit 0 - Main flash program for bank0 command bit"]
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W {
        PG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0x80"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
