#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LOCK_SPEC>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LOCK_SPEC>;
#[doc = "Field `LK0` reader - Port Lock bit 0"]
pub type LK0_R = crate::BitReader<LK0_A>;
#[doc = "Port Lock bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LK0_A {
    #[doc = "0: Port configuration not locked"]
    UNLOCKED = 0,
    #[doc = "1: Port configuration locked"]
    LOCKED = 1,
}
impl From<LK0_A> for bool {
    #[inline(always)]
    fn from(variant: LK0_A) -> Self {
        variant as u8 != 0
    }
}
impl LK0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LK0_A {
        match self.bits {
            false => LK0_A::UNLOCKED,
            true => LK0_A::LOCKED,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LK0_A::UNLOCKED
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LK0_A::LOCKED
    }
}
#[doc = "Field `LK0` writer - Port Lock bit 0"]
pub type LK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LK0_A>;
impl<'a, REG, const O: u8> LK0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LK0_A::UNLOCKED)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LK0_A::LOCKED)
    }
}
#[doc = "Field `LK1` reader - Port Lock bit 1"]
pub use LK0_R as LK1_R;
#[doc = "Field `LK2` reader - Port Lock bit 2"]
pub use LK0_R as LK2_R;
#[doc = "Field `LK3` reader - Port Lock bit 3"]
pub use LK0_R as LK3_R;
#[doc = "Field `LK4` reader - Port Lock bit 4"]
pub use LK0_R as LK4_R;
#[doc = "Field `LK5` reader - Port Lock bit 5"]
pub use LK0_R as LK5_R;
#[doc = "Field `LK6` reader - Port Lock bit 6"]
pub use LK0_R as LK6_R;
#[doc = "Field `LK7` reader - Port Lock bit 7"]
pub use LK0_R as LK7_R;
#[doc = "Field `LK8` reader - Port Lock bit 8"]
pub use LK0_R as LK8_R;
#[doc = "Field `LK9` reader - Port Lock bit 9"]
pub use LK0_R as LK9_R;
#[doc = "Field `LK10` reader - Port Lock bit 10"]
pub use LK0_R as LK10_R;
#[doc = "Field `LK11` reader - Port Lock bit 11"]
pub use LK0_R as LK11_R;
#[doc = "Field `LK12` reader - Port Lock bit 12"]
pub use LK0_R as LK12_R;
#[doc = "Field `LK13` reader - Port Lock bit 13"]
pub use LK0_R as LK13_R;
#[doc = "Field `LK14` reader - Port Lock bit 14"]
pub use LK0_R as LK14_R;
#[doc = "Field `LK15` reader - Port Lock bit 15"]
pub use LK0_R as LK15_R;
#[doc = "Field `LK1` writer - Port Lock bit 1"]
pub use LK0_W as LK1_W;
#[doc = "Field `LK2` writer - Port Lock bit 2"]
pub use LK0_W as LK2_W;
#[doc = "Field `LK3` writer - Port Lock bit 3"]
pub use LK0_W as LK3_W;
#[doc = "Field `LK4` writer - Port Lock bit 4"]
pub use LK0_W as LK4_W;
#[doc = "Field `LK5` writer - Port Lock bit 5"]
pub use LK0_W as LK5_W;
#[doc = "Field `LK6` writer - Port Lock bit 6"]
pub use LK0_W as LK6_W;
#[doc = "Field `LK7` writer - Port Lock bit 7"]
pub use LK0_W as LK7_W;
#[doc = "Field `LK8` writer - Port Lock bit 8"]
pub use LK0_W as LK8_W;
#[doc = "Field `LK9` writer - Port Lock bit 9"]
pub use LK0_W as LK9_W;
#[doc = "Field `LK10` writer - Port Lock bit 10"]
pub use LK0_W as LK10_W;
#[doc = "Field `LK11` writer - Port Lock bit 11"]
pub use LK0_W as LK11_W;
#[doc = "Field `LK12` writer - Port Lock bit 12"]
pub use LK0_W as LK12_W;
#[doc = "Field `LK13` writer - Port Lock bit 13"]
pub use LK0_W as LK13_W;
#[doc = "Field `LK14` writer - Port Lock bit 14"]
pub use LK0_W as LK14_W;
#[doc = "Field `LK15` writer - Port Lock bit 15"]
pub use LK0_W as LK15_W;
#[doc = "Field `LKK` reader - Lock sequence key"]
pub type LKK_R = crate::BitReader<LKK_A>;
#[doc = "Lock sequence key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LKK_A {
    #[doc = "0: Port configuration lock key not active"]
    NOT_ACTIVE = 0,
    #[doc = "1: Port configuration lock key active"]
    ACTIVE = 1,
}
impl From<LKK_A> for bool {
    #[inline(always)]
    fn from(variant: LKK_A) -> Self {
        variant as u8 != 0
    }
}
impl LKK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LKK_A {
        match self.bits {
            false => LKK_A::NOT_ACTIVE,
            true => LKK_A::ACTIVE,
        }
    }
    #[doc = "Port configuration lock key not active"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == LKK_A::NOT_ACTIVE
    }
    #[doc = "Port configuration lock key active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == LKK_A::ACTIVE
    }
}
#[doc = "Field `LKK` writer - Lock sequence key"]
pub type LKK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LKK_A>;
impl<'a, REG, const O: u8> LKK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration lock key not active"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(LKK_A::NOT_ACTIVE)
    }
    #[doc = "Port configuration lock key active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(LKK_A::ACTIVE)
    }
}
impl R {
    #[doc = "Bit 0 - Port Lock bit 0"]
    #[inline(always)]
    pub fn lk0(&self) -> LK0_R {
        LK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Lock bit 1"]
    #[inline(always)]
    pub fn lk1(&self) -> LK1_R {
        LK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Lock bit 2"]
    #[inline(always)]
    pub fn lk2(&self) -> LK2_R {
        LK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Lock bit 3"]
    #[inline(always)]
    pub fn lk3(&self) -> LK3_R {
        LK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Lock bit 4"]
    #[inline(always)]
    pub fn lk4(&self) -> LK4_R {
        LK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Lock bit 5"]
    #[inline(always)]
    pub fn lk5(&self) -> LK5_R {
        LK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Lock bit 6"]
    #[inline(always)]
    pub fn lk6(&self) -> LK6_R {
        LK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Lock bit 7"]
    #[inline(always)]
    pub fn lk7(&self) -> LK7_R {
        LK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Lock bit 8"]
    #[inline(always)]
    pub fn lk8(&self) -> LK8_R {
        LK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Lock bit 9"]
    #[inline(always)]
    pub fn lk9(&self) -> LK9_R {
        LK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Lock bit 10"]
    #[inline(always)]
    pub fn lk10(&self) -> LK10_R {
        LK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Lock bit 11"]
    #[inline(always)]
    pub fn lk11(&self) -> LK11_R {
        LK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Lock bit 12"]
    #[inline(always)]
    pub fn lk12(&self) -> LK12_R {
        LK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Lock bit 13"]
    #[inline(always)]
    pub fn lk13(&self) -> LK13_R {
        LK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Lock bit 14"]
    #[inline(always)]
    pub fn lk14(&self) -> LK14_R {
        LK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Lock bit 15"]
    #[inline(always)]
    pub fn lk15(&self) -> LK15_R {
        LK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock sequence key"]
    #[inline(always)]
    pub fn lkk(&self) -> LKK_R {
        LKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Lock bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lk0(&mut self) -> LK0_W<LOCK_SPEC, 0> {
        LK0_W::new(self)
    }
    #[doc = "Bit 1 - Port Lock bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn lk1(&mut self) -> LK1_W<LOCK_SPEC, 1> {
        LK1_W::new(self)
    }
    #[doc = "Bit 2 - Port Lock bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn lk2(&mut self) -> LK2_W<LOCK_SPEC, 2> {
        LK2_W::new(self)
    }
    #[doc = "Bit 3 - Port Lock bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn lk3(&mut self) -> LK3_W<LOCK_SPEC, 3> {
        LK3_W::new(self)
    }
    #[doc = "Bit 4 - Port Lock bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn lk4(&mut self) -> LK4_W<LOCK_SPEC, 4> {
        LK4_W::new(self)
    }
    #[doc = "Bit 5 - Port Lock bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn lk5(&mut self) -> LK5_W<LOCK_SPEC, 5> {
        LK5_W::new(self)
    }
    #[doc = "Bit 6 - Port Lock bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn lk6(&mut self) -> LK6_W<LOCK_SPEC, 6> {
        LK6_W::new(self)
    }
    #[doc = "Bit 7 - Port Lock bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn lk7(&mut self) -> LK7_W<LOCK_SPEC, 7> {
        LK7_W::new(self)
    }
    #[doc = "Bit 8 - Port Lock bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lk8(&mut self) -> LK8_W<LOCK_SPEC, 8> {
        LK8_W::new(self)
    }
    #[doc = "Bit 9 - Port Lock bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn lk9(&mut self) -> LK9_W<LOCK_SPEC, 9> {
        LK9_W::new(self)
    }
    #[doc = "Bit 10 - Port Lock bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn lk10(&mut self) -> LK10_W<LOCK_SPEC, 10> {
        LK10_W::new(self)
    }
    #[doc = "Bit 11 - Port Lock bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn lk11(&mut self) -> LK11_W<LOCK_SPEC, 11> {
        LK11_W::new(self)
    }
    #[doc = "Bit 12 - Port Lock bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn lk12(&mut self) -> LK12_W<LOCK_SPEC, 12> {
        LK12_W::new(self)
    }
    #[doc = "Bit 13 - Port Lock bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn lk13(&mut self) -> LK13_W<LOCK_SPEC, 13> {
        LK13_W::new(self)
    }
    #[doc = "Bit 14 - Port Lock bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn lk14(&mut self) -> LK14_W<LOCK_SPEC, 14> {
        LK14_W::new(self)
    }
    #[doc = "Bit 15 - Port Lock bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn lk15(&mut self) -> LK15_W<LOCK_SPEC, 15> {
        LK15_W::new(self)
    }
    #[doc = "Bit 16 - Lock sequence key"]
    #[inline(always)]
    #[must_use]
    pub fn lkk(&mut self) -> LKK_W<LOCK_SPEC, 16> {
        LKK_W::new(self)
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
#[doc = "GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
