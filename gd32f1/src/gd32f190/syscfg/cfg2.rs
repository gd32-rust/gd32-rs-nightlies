#[doc = "Register `CFG2` reader"]
pub struct R(crate::R<CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG2` writer"]
pub struct W(crate::W<CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG2_SPEC>;
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
impl From<crate::W<CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SRAM parity check error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_PCEF_A {
    #[doc = "0: No SRAM parity check error detected"]
    NOERROR = 0,
    #[doc = "1: SRAM parity check error detected"]
    ERROR = 1,
}
impl From<SRAM_PCEF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PCEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PCEF` reader - SRAM parity check error flag"]
pub type SRAM_PCEF_R = crate::BitReader<SRAM_PCEF_A>;
impl SRAM_PCEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_PCEF_A {
        match self.bits {
            false => SRAM_PCEF_A::NOERROR,
            true => SRAM_PCEF_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == SRAM_PCEF_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SRAM_PCEF_A::ERROR
    }
}
#[doc = "SRAM parity check error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_PCEF_AW {
    #[doc = "1: Clear error flag"]
    CLEAR = 1,
}
impl From<SRAM_PCEF_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PCEF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PCEF` writer - SRAM parity check error flag"]
pub type SRAM_PCEF_W<'a> = crate::BitWriter<'a, u32, CFG2_SPEC, SRAM_PCEF_AW, 8>;
impl<'a> SRAM_PCEF_W<'a> {
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SRAM_PCEF_AW::CLEAR)
    }
}
#[doc = "LVD lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVD_LOCK_A {
    #[doc = "0: The LVD interrupt is disconnected from the break input"]
    UNLOCKED = 0,
    #[doc = "1: The LVD interrupt is connected to the break input"]
    LOCKED = 1,
}
impl From<LVD_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LVD_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD_LOCK` reader - LVD lock"]
pub type LVD_LOCK_R = crate::BitReader<LVD_LOCK_A>;
impl LVD_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD_LOCK_A {
        match self.bits {
            false => LVD_LOCK_A::UNLOCKED,
            true => LVD_LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LVD_LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LVD_LOCK_A::LOCKED
    }
}
#[doc = "Field `LVD_LOCK` writer - LVD lock"]
pub type LVD_LOCK_W<'a> = crate::BitWriter<'a, u32, CFG2_SPEC, LVD_LOCK_A, 2>;
impl<'a> LVD_LOCK_W<'a> {
    #[doc = "The LVD interrupt is disconnected from the break input"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LVD_LOCK_A::UNLOCKED)
    }
    #[doc = "The LVD interrupt is connected to the break input"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LVD_LOCK_A::LOCKED)
    }
}
#[doc = "SRAM parity check error lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_PARITY_ERROR_LOCK_A {
    #[doc = "0: The SRAM parity check error is disconnected from the break input"]
    UNLOCKED = 0,
    #[doc = "1: The SRAM parity check error is connected to the break input"]
    LOCKED = 1,
}
impl From<SRAM_PARITY_ERROR_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PARITY_ERROR_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PARITY_ERROR_LOCK` reader - SRAM parity check error lock"]
pub type SRAM_PARITY_ERROR_LOCK_R = crate::BitReader<SRAM_PARITY_ERROR_LOCK_A>;
impl SRAM_PARITY_ERROR_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_PARITY_ERROR_LOCK_A {
        match self.bits {
            false => SRAM_PARITY_ERROR_LOCK_A::UNLOCKED,
            true => SRAM_PARITY_ERROR_LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == SRAM_PARITY_ERROR_LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == SRAM_PARITY_ERROR_LOCK_A::LOCKED
    }
}
#[doc = "Field `SRAM_PARITY_ERROR_LOCK` writer - SRAM parity check error lock"]
pub type SRAM_PARITY_ERROR_LOCK_W<'a> =
    crate::BitWriter<'a, u32, CFG2_SPEC, SRAM_PARITY_ERROR_LOCK_A, 1>;
impl<'a> SRAM_PARITY_ERROR_LOCK_W<'a> {
    #[doc = "The SRAM parity check error is disconnected from the break input"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(SRAM_PARITY_ERROR_LOCK_A::UNLOCKED)
    }
    #[doc = "The SRAM parity check error is connected to the break input"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(SRAM_PARITY_ERROR_LOCK_A::LOCKED)
    }
}
#[doc = "Cortex-M3 LOCKUP output lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_LOCK_A {
    #[doc = "0: The Cortex-M3 LOCKUP output is disconnected from the break input"]
    UNLOCKED = 0,
    #[doc = "1: The Cortex-M3 LOCKUP output is connected to the break input"]
    LOCKED = 1,
}
impl From<LOCKUP_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUP_LOCK` reader - Cortex-M3 LOCKUP output lock"]
pub type LOCKUP_LOCK_R = crate::BitReader<LOCKUP_LOCK_A>;
impl LOCKUP_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_LOCK_A {
        match self.bits {
            false => LOCKUP_LOCK_A::UNLOCKED,
            true => LOCKUP_LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKUP_LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKUP_LOCK_A::LOCKED
    }
}
#[doc = "Field `LOCKUP_LOCK` writer - Cortex-M3 LOCKUP output lock"]
pub type LOCKUP_LOCK_W<'a> = crate::BitWriter<'a, u32, CFG2_SPEC, LOCKUP_LOCK_A, 0>;
impl<'a> LOCKUP_LOCK_W<'a> {
    #[doc = "The Cortex-M3 LOCKUP output is disconnected from the break input"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKUP_LOCK_A::UNLOCKED)
    }
    #[doc = "The Cortex-M3 LOCKUP output is connected to the break input"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKUP_LOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bit 8 - SRAM parity check error flag"]
    #[inline(always)]
    pub fn sram_pcef(&self) -> SRAM_PCEF_R {
        SRAM_PCEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 2 - LVD lock"]
    #[inline(always)]
    pub fn lvd_lock(&self) -> LVD_LOCK_R {
        LVD_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM parity check error lock"]
    #[inline(always)]
    pub fn sram_parity_error_lock(&self) -> SRAM_PARITY_ERROR_LOCK_R {
        SRAM_PARITY_ERROR_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Cortex-M3 LOCKUP output lock"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SRAM parity check error flag"]
    #[inline(always)]
    pub fn sram_pcef(&mut self) -> SRAM_PCEF_W {
        SRAM_PCEF_W::new(self)
    }
    #[doc = "Bit 2 - LVD lock"]
    #[inline(always)]
    pub fn lvd_lock(&mut self) -> LVD_LOCK_W {
        LVD_LOCK_W::new(self)
    }
    #[doc = "Bit 1 - SRAM parity check error lock"]
    #[inline(always)]
    pub fn sram_parity_error_lock(&mut self) -> SRAM_PARITY_ERROR_LOCK_W {
        SRAM_PARITY_ERROR_LOCK_W::new(self)
    }
    #[doc = "Bit 0 - Cortex-M3 LOCKUP output lock"]
    #[inline(always)]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W {
        LOCKUP_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](index.html) module"]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg2::R](R) reader structure"]
impl crate::Readable for CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg2::W](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
