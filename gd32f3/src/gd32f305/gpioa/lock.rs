#[doc = "Register `LOCK` reader"]
pub struct R(crate::R<LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCK` writer"]
pub struct W(crate::W<LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCK_SPEC>;
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
impl From<crate::W<LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Lock sequence key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LKK_A {
    #[doc = "0: Port configuration lock key not active"]
    NOTACTIVE = 0,
    #[doc = "1: Port configuration lock key active"]
    ACTIVE = 1,
}
impl From<LKK_A> for bool {
    #[inline(always)]
    fn from(variant: LKK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LKK` reader - Lock sequence key"]
pub type LKK_R = crate::BitReader<LKK_A>;
impl LKK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LKK_A {
        match self.bits {
            false => LKK_A::NOTACTIVE,
            true => LKK_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == LKK_A::NOTACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == LKK_A::ACTIVE
    }
}
#[doc = "Field `LKK` writer - Lock sequence key"]
pub type LKK_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, LKK_A, 16>;
impl<'a> LKK_W<'a> {
    #[doc = "Port configuration lock key not active"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(LKK_A::NOTACTIVE)
    }
    #[doc = "Port configuration lock key active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(LKK_A::ACTIVE)
    }
}
#[doc = "Field `LK15` reader - Port Lock bit 15"]
pub type LK15_R = crate::BitReader<bool>;
#[doc = "Field `LK15` writer - Port Lock bit 15"]
pub type LK15_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 15>;
#[doc = "Field `LK14` reader - Port Lock bit 14"]
pub type LK14_R = crate::BitReader<bool>;
#[doc = "Field `LK14` writer - Port Lock bit 14"]
pub type LK14_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 14>;
#[doc = "Field `LK13` reader - Port Lock bit 13"]
pub type LK13_R = crate::BitReader<bool>;
#[doc = "Field `LK13` writer - Port Lock bit 13"]
pub type LK13_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 13>;
#[doc = "Field `LK12` reader - Port Lock bit 12"]
pub type LK12_R = crate::BitReader<bool>;
#[doc = "Field `LK12` writer - Port Lock bit 12"]
pub type LK12_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 12>;
#[doc = "Field `LK11` reader - Port Lock bit 11"]
pub type LK11_R = crate::BitReader<bool>;
#[doc = "Field `LK11` writer - Port Lock bit 11"]
pub type LK11_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 11>;
#[doc = "Field `LK10` reader - Port Lock bit 10"]
pub type LK10_R = crate::BitReader<bool>;
#[doc = "Field `LK10` writer - Port Lock bit 10"]
pub type LK10_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 10>;
#[doc = "Field `LK9` reader - Port Lock bit 9"]
pub type LK9_R = crate::BitReader<bool>;
#[doc = "Field `LK9` writer - Port Lock bit 9"]
pub type LK9_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 9>;
#[doc = "Field `LK8` reader - Port Lock bit 8"]
pub type LK8_R = crate::BitReader<bool>;
#[doc = "Field `LK8` writer - Port Lock bit 8"]
pub type LK8_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 8>;
#[doc = "Field `LK7` reader - Port Lock bit 7"]
pub type LK7_R = crate::BitReader<bool>;
#[doc = "Field `LK7` writer - Port Lock bit 7"]
pub type LK7_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 7>;
#[doc = "Field `LK6` reader - Port Lock bit 6"]
pub type LK6_R = crate::BitReader<bool>;
#[doc = "Field `LK6` writer - Port Lock bit 6"]
pub type LK6_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 6>;
#[doc = "Port Lock bit 5"]
pub use LK0_A as LK5_A;
#[doc = "Field `LK5` reader - Port Lock bit 5"]
pub use LK0_R as LK5_R;
#[doc = "Field `LK5` writer - Port Lock bit 5"]
pub use LK0_W as LK5_W;
#[doc = "Field `LK4` reader - Port Lock bit 4"]
pub type LK4_R = crate::BitReader<bool>;
#[doc = "Field `LK4` writer - Port Lock bit 4"]
pub type LK4_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 4>;
#[doc = "Field `LK3` reader - Port Lock bit 3"]
pub type LK3_R = crate::BitReader<bool>;
#[doc = "Field `LK3` writer - Port Lock bit 3"]
pub type LK3_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 3>;
#[doc = "Field `LK2` reader - Port Lock bit 2"]
pub type LK2_R = crate::BitReader<bool>;
#[doc = "Field `LK2` writer - Port Lock bit 2"]
pub type LK2_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, bool, 2>;
#[doc = "Port Lock bit 1"]
pub use LK0_A as LK1_A;
#[doc = "Field `LK1` reader - Port Lock bit 1"]
pub use LK0_R as LK1_R;
#[doc = "Field `LK1` writer - Port Lock bit 1"]
pub use LK0_W as LK1_W;
#[doc = "Port Lock bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LK0` reader - Port Lock bit 0"]
pub type LK0_R = crate::BitReader<LK0_A>;
impl LK0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LK0_A {
        match self.bits {
            false => LK0_A::UNLOCKED,
            true => LK0_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LK0_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LK0_A::LOCKED
    }
}
#[doc = "Field `LK0` writer - Port Lock bit 0"]
pub type LK0_W<'a> = crate::BitWriter<'a, u32, LOCK_SPEC, LK0_A, 0>;
impl<'a> LK0_W<'a> {
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LK0_A::UNLOCKED)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LK0_A::LOCKED)
    }
}
impl R {
    #[doc = "Bit 16 - Lock sequence key"]
    #[inline(always)]
    pub fn lkk(&self) -> LKK_R {
        LKK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Lock bit 15"]
    #[inline(always)]
    pub fn lk15(&self) -> LK15_R {
        LK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Lock bit 14"]
    #[inline(always)]
    pub fn lk14(&self) -> LK14_R {
        LK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Lock bit 13"]
    #[inline(always)]
    pub fn lk13(&self) -> LK13_R {
        LK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Lock bit 12"]
    #[inline(always)]
    pub fn lk12(&self) -> LK12_R {
        LK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Lock bit 11"]
    #[inline(always)]
    pub fn lk11(&self) -> LK11_R {
        LK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Lock bit 10"]
    #[inline(always)]
    pub fn lk10(&self) -> LK10_R {
        LK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Lock bit 9"]
    #[inline(always)]
    pub fn lk9(&self) -> LK9_R {
        LK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Lock bit 8"]
    #[inline(always)]
    pub fn lk8(&self) -> LK8_R {
        LK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Lock bit 7"]
    #[inline(always)]
    pub fn lk7(&self) -> LK7_R {
        LK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Lock bit 6"]
    #[inline(always)]
    pub fn lk6(&self) -> LK6_R {
        LK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Lock bit 5"]
    #[inline(always)]
    pub fn lk5(&self) -> LK5_R {
        LK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Lock bit 4"]
    #[inline(always)]
    pub fn lk4(&self) -> LK4_R {
        LK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Lock bit 3"]
    #[inline(always)]
    pub fn lk3(&self) -> LK3_R {
        LK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Lock bit 2"]
    #[inline(always)]
    pub fn lk2(&self) -> LK2_R {
        LK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Port Lock bit 1"]
    #[inline(always)]
    pub fn lk1(&self) -> LK1_R {
        LK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Port Lock bit 0"]
    #[inline(always)]
    pub fn lk0(&self) -> LK0_R {
        LK0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Lock sequence key"]
    #[inline(always)]
    pub fn lkk(&mut self) -> LKK_W {
        LKK_W::new(self)
    }
    #[doc = "Bit 15 - Port Lock bit 15"]
    #[inline(always)]
    pub fn lk15(&mut self) -> LK15_W {
        LK15_W::new(self)
    }
    #[doc = "Bit 14 - Port Lock bit 14"]
    #[inline(always)]
    pub fn lk14(&mut self) -> LK14_W {
        LK14_W::new(self)
    }
    #[doc = "Bit 13 - Port Lock bit 13"]
    #[inline(always)]
    pub fn lk13(&mut self) -> LK13_W {
        LK13_W::new(self)
    }
    #[doc = "Bit 12 - Port Lock bit 12"]
    #[inline(always)]
    pub fn lk12(&mut self) -> LK12_W {
        LK12_W::new(self)
    }
    #[doc = "Bit 11 - Port Lock bit 11"]
    #[inline(always)]
    pub fn lk11(&mut self) -> LK11_W {
        LK11_W::new(self)
    }
    #[doc = "Bit 10 - Port Lock bit 10"]
    #[inline(always)]
    pub fn lk10(&mut self) -> LK10_W {
        LK10_W::new(self)
    }
    #[doc = "Bit 9 - Port Lock bit 9"]
    #[inline(always)]
    pub fn lk9(&mut self) -> LK9_W {
        LK9_W::new(self)
    }
    #[doc = "Bit 8 - Port Lock bit 8"]
    #[inline(always)]
    pub fn lk8(&mut self) -> LK8_W {
        LK8_W::new(self)
    }
    #[doc = "Bit 7 - Port Lock bit 7"]
    #[inline(always)]
    pub fn lk7(&mut self) -> LK7_W {
        LK7_W::new(self)
    }
    #[doc = "Bit 6 - Port Lock bit 6"]
    #[inline(always)]
    pub fn lk6(&mut self) -> LK6_W {
        LK6_W::new(self)
    }
    #[doc = "Bit 5 - Port Lock bit 5"]
    #[inline(always)]
    pub fn lk5(&mut self) -> LK5_W {
        LK5_W::new(self)
    }
    #[doc = "Bit 4 - Port Lock bit 4"]
    #[inline(always)]
    pub fn lk4(&mut self) -> LK4_W {
        LK4_W::new(self)
    }
    #[doc = "Bit 3 - Port Lock bit 3"]
    #[inline(always)]
    pub fn lk3(&mut self) -> LK3_W {
        LK3_W::new(self)
    }
    #[doc = "Bit 2 - Port Lock bit 2"]
    #[inline(always)]
    pub fn lk2(&mut self) -> LK2_W {
        LK2_W::new(self)
    }
    #[doc = "Bit 1 - Port Lock bit 1"]
    #[inline(always)]
    pub fn lk1(&mut self) -> LK1_W {
        LK1_W::new(self)
    }
    #[doc = "Bit 0 - Port Lock bit 0"]
    #[inline(always)]
    pub fn lk0(&mut self) -> LK0_W {
        LK0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port configuration lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](index.html) module"]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lock::R](R) reader structure"]
impl crate::Readable for LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lock::W](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
