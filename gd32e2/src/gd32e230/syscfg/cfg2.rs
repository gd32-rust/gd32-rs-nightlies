#[doc = "Register `CFG2` reader"]
pub type R = crate::R<CFG2_SPEC>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<CFG2_SPEC>;
#[doc = "Field `LOCKUP_LOCK` reader - Cortex-M4 LOCKUP output lock"]
pub type LOCKUP_LOCK_R = crate::BitReader<LOCKUP_LOCK_A>;
#[doc = "Cortex-M4 LOCKUP output lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LOCKUP_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_LOCK_A {
        match self.bits {
            false => LOCKUP_LOCK_A::UNLOCKED,
            true => LOCKUP_LOCK_A::LOCKED,
        }
    }
    #[doc = "The Cortex-M3 LOCKUP output is disconnected from the break input"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKUP_LOCK_A::UNLOCKED
    }
    #[doc = "The Cortex-M3 LOCKUP output is connected to the break input"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKUP_LOCK_A::LOCKED
    }
}
#[doc = "Field `LOCKUP_LOCK` writer - Cortex-M4 LOCKUP output lock"]
pub type LOCKUP_LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LOCKUP_LOCK_A>;
impl<'a, REG, const O: u8> LOCKUP_LOCK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Cortex-M3 LOCKUP output is disconnected from the break input"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUP_LOCK_A::UNLOCKED)
    }
    #[doc = "The Cortex-M3 LOCKUP output is connected to the break input"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUP_LOCK_A::LOCKED)
    }
}
#[doc = "Field `SRAM_PARITY_ERROR_LOCK` reader - SRAM parity check error lock"]
pub type SRAM_PARITY_ERROR_LOCK_R = crate::BitReader<SRAM_PARITY_ERROR_LOCK_A>;
#[doc = "SRAM parity check error lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SRAM_PARITY_ERROR_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_PARITY_ERROR_LOCK_A {
        match self.bits {
            false => SRAM_PARITY_ERROR_LOCK_A::UNLOCKED,
            true => SRAM_PARITY_ERROR_LOCK_A::LOCKED,
        }
    }
    #[doc = "The SRAM parity check error is disconnected from the break input"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == SRAM_PARITY_ERROR_LOCK_A::UNLOCKED
    }
    #[doc = "The SRAM parity check error is connected to the break input"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == SRAM_PARITY_ERROR_LOCK_A::LOCKED
    }
}
#[doc = "Field `SRAM_PARITY_ERROR_LOCK` writer - SRAM parity check error lock"]
pub type SRAM_PARITY_ERROR_LOCK_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, SRAM_PARITY_ERROR_LOCK_A>;
impl<'a, REG, const O: u8> SRAM_PARITY_ERROR_LOCK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SRAM parity check error is disconnected from the break input"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM_PARITY_ERROR_LOCK_A::UNLOCKED)
    }
    #[doc = "The SRAM parity check error is connected to the break input"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM_PARITY_ERROR_LOCK_A::LOCKED)
    }
}
#[doc = "Field `LVD_LOCK` reader - LVD lock"]
pub type LVD_LOCK_R = crate::BitReader<LVD_LOCK_A>;
#[doc = "LVD lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LVD_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD_LOCK_A {
        match self.bits {
            false => LVD_LOCK_A::UNLOCKED,
            true => LVD_LOCK_A::LOCKED,
        }
    }
    #[doc = "The LVD interrupt is disconnected from the break input"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LVD_LOCK_A::UNLOCKED
    }
    #[doc = "The LVD interrupt is connected to the break input"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LVD_LOCK_A::LOCKED
    }
}
#[doc = "Field `LVD_LOCK` writer - LVD lock"]
pub type LVD_LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LVD_LOCK_A>;
impl<'a, REG, const O: u8> LVD_LOCK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The LVD interrupt is disconnected from the break input"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LVD_LOCK_A::UNLOCKED)
    }
    #[doc = "The LVD interrupt is connected to the break input"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LVD_LOCK_A::LOCKED)
    }
}
#[doc = "Field `SRAM_PCEF` reader - SRAM parity check error flag"]
pub type SRAM_PCEF_R = crate::BitReader<SRAM_PCEFR_A>;
#[doc = "SRAM parity check error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_PCEFR_A {
    #[doc = "0: No SRAM parity check error detected"]
    NO_ERROR = 0,
    #[doc = "1: SRAM parity check error detected"]
    ERROR = 1,
}
impl From<SRAM_PCEFR_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PCEFR_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_PCEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_PCEFR_A {
        match self.bits {
            false => SRAM_PCEFR_A::NO_ERROR,
            true => SRAM_PCEFR_A::ERROR,
        }
    }
    #[doc = "No SRAM parity check error detected"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == SRAM_PCEFR_A::NO_ERROR
    }
    #[doc = "SRAM parity check error detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SRAM_PCEFR_A::ERROR
    }
}
#[doc = "SRAM parity check error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_PCEFW_AW {
    #[doc = "1: Clear error flag"]
    CLEAR = 1,
}
impl From<SRAM_PCEFW_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PCEFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PCEF` writer - SRAM parity check error flag"]
pub type SRAM_PCEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SRAM_PCEFW_AW>;
impl<'a, REG, const O: u8> SRAM_PCEF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM_PCEFW_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Cortex-M4 LOCKUP output lock"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM parity check error lock"]
    #[inline(always)]
    pub fn sram_parity_error_lock(&self) -> SRAM_PARITY_ERROR_LOCK_R {
        SRAM_PARITY_ERROR_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LVD lock"]
    #[inline(always)]
    pub fn lvd_lock(&self) -> LVD_LOCK_R {
        LVD_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM parity check error flag"]
    #[inline(always)]
    pub fn sram_pcef(&self) -> SRAM_PCEF_R {
        SRAM_PCEF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M4 LOCKUP output lock"]
    #[inline(always)]
    #[must_use]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W<CFG2_SPEC, 0> {
        LOCKUP_LOCK_W::new(self)
    }
    #[doc = "Bit 1 - SRAM parity check error lock"]
    #[inline(always)]
    #[must_use]
    pub fn sram_parity_error_lock(&mut self) -> SRAM_PARITY_ERROR_LOCK_W<CFG2_SPEC, 1> {
        SRAM_PARITY_ERROR_LOCK_W::new(self)
    }
    #[doc = "Bit 2 - LVD lock"]
    #[inline(always)]
    #[must_use]
    pub fn lvd_lock(&mut self) -> LVD_LOCK_W<CFG2_SPEC, 2> {
        LVD_LOCK_W::new(self)
    }
    #[doc = "Bit 8 - SRAM parity check error flag"]
    #[inline(always)]
    #[must_use]
    pub fn sram_pcef(&mut self) -> SRAM_PCEF_W<CFG2_SPEC, 8> {
        SRAM_PCEF_W::new(self)
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
#[doc = "System configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
