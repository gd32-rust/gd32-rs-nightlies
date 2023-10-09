#[doc = "Register `OSPD` reader"]
pub type R = crate::R<OSPD_SPEC>;
#[doc = "Register `OSPD` writer"]
pub type W = crate::W<OSPD_SPEC>;
#[doc = "Field `OSPD0` reader - Port x configuration bits (y = 0..15)"]
pub type OSPD0_R = crate::FieldReader<OSPD0_A>;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPD0_A {
    #[doc = "0: Max output speed 2 MHz"]
    SPEED2M = 0,
    #[doc = "1: Max output speed 10 MHz"]
    SPEED10M = 1,
    #[doc = "3: Max output speed 50 MHz"]
    SPEED50M = 3,
}
impl From<OSPD0_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPD0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPD0_A {
    type Ux = u8;
}
impl OSPD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSPD0_A> {
        match self.bits {
            0 => Some(OSPD0_A::SPEED2M),
            1 => Some(OSPD0_A::SPEED10M),
            3 => Some(OSPD0_A::SPEED50M),
            _ => None,
        }
    }
    #[doc = "Max output speed 2 MHz"]
    #[inline(always)]
    pub fn is_speed2m(&self) -> bool {
        *self == OSPD0_A::SPEED2M
    }
    #[doc = "Max output speed 10 MHz"]
    #[inline(always)]
    pub fn is_speed10m(&self) -> bool {
        *self == OSPD0_A::SPEED10M
    }
    #[doc = "Max output speed 50 MHz"]
    #[inline(always)]
    pub fn is_speed50m(&self) -> bool {
        *self == OSPD0_A::SPEED50M
    }
}
#[doc = "Field `OSPD0` writer - Port x configuration bits (y = 0..15)"]
pub type OSPD0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, OSPD0_A>;
impl<'a, REG, const O: u8> OSPD0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Max output speed 2 MHz"]
    #[inline(always)]
    pub fn speed2m(self) -> &'a mut crate::W<REG> {
        self.variant(OSPD0_A::SPEED2M)
    }
    #[doc = "Max output speed 10 MHz"]
    #[inline(always)]
    pub fn speed10m(self) -> &'a mut crate::W<REG> {
        self.variant(OSPD0_A::SPEED10M)
    }
    #[doc = "Max output speed 50 MHz"]
    #[inline(always)]
    pub fn speed50m(self) -> &'a mut crate::W<REG> {
        self.variant(OSPD0_A::SPEED50M)
    }
}
#[doc = "Field `OSPD1` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD1_R;
#[doc = "Field `OSPD2` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD2_R;
#[doc = "Field `OSPD3` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD3_R;
#[doc = "Field `OSPD4` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD4_R;
#[doc = "Field `OSPD5` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD5_R;
#[doc = "Field `OSPD6` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD6_R;
#[doc = "Field `OSPD7` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD7_R;
#[doc = "Field `OSPD8` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD8_R;
#[doc = "Field `OSPD9` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD9_R;
#[doc = "Field `OSPD10` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD10_R;
#[doc = "Field `OSPD11` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD11_R;
#[doc = "Field `OSPD12` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD12_R;
#[doc = "Field `OSPD13` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD13_R;
#[doc = "Field `OSPD14` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD14_R;
#[doc = "Field `OSPD15` reader - Port x configuration bits (y = 0..15)"]
pub use OSPD0_R as OSPD15_R;
#[doc = "Field `OSPD1` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD1_W;
#[doc = "Field `OSPD2` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD2_W;
#[doc = "Field `OSPD3` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD3_W;
#[doc = "Field `OSPD4` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD4_W;
#[doc = "Field `OSPD5` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD5_W;
#[doc = "Field `OSPD6` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD6_W;
#[doc = "Field `OSPD7` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD7_W;
#[doc = "Field `OSPD8` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD8_W;
#[doc = "Field `OSPD9` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD9_W;
#[doc = "Field `OSPD10` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD10_W;
#[doc = "Field `OSPD11` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD11_W;
#[doc = "Field `OSPD12` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD12_W;
#[doc = "Field `OSPD13` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD13_W;
#[doc = "Field `OSPD14` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD14_W;
#[doc = "Field `OSPD15` writer - Port x configuration bits (y = 0..15)"]
pub use OSPD0_W as OSPD15_W;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd0(&self) -> OSPD0_R {
        OSPD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd1(&self) -> OSPD1_R {
        OSPD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd2(&self) -> OSPD2_R {
        OSPD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd3(&self) -> OSPD3_R {
        OSPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd4(&self) -> OSPD4_R {
        OSPD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd5(&self) -> OSPD5_R {
        OSPD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd6(&self) -> OSPD6_R {
        OSPD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd7(&self) -> OSPD7_R {
        OSPD7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd8(&self) -> OSPD8_R {
        OSPD8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd9(&self) -> OSPD9_R {
        OSPD9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd10(&self) -> OSPD10_R {
        OSPD10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd11(&self) -> OSPD11_R {
        OSPD11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd12(&self) -> OSPD12_R {
        OSPD12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd13(&self) -> OSPD13_R {
        OSPD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd14(&self) -> OSPD14_R {
        OSPD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd15(&self) -> OSPD15_R {
        OSPD15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd0(&mut self) -> OSPD0_W<OSPD_SPEC, 0> {
        OSPD0_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd1(&mut self) -> OSPD1_W<OSPD_SPEC, 2> {
        OSPD1_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd2(&mut self) -> OSPD2_W<OSPD_SPEC, 4> {
        OSPD2_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd3(&mut self) -> OSPD3_W<OSPD_SPEC, 6> {
        OSPD3_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd4(&mut self) -> OSPD4_W<OSPD_SPEC, 8> {
        OSPD4_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd5(&mut self) -> OSPD5_W<OSPD_SPEC, 10> {
        OSPD5_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd6(&mut self) -> OSPD6_W<OSPD_SPEC, 12> {
        OSPD6_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd7(&mut self) -> OSPD7_W<OSPD_SPEC, 14> {
        OSPD7_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd8(&mut self) -> OSPD8_W<OSPD_SPEC, 16> {
        OSPD8_W::new(self)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd9(&mut self) -> OSPD9_W<OSPD_SPEC, 18> {
        OSPD9_W::new(self)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd10(&mut self) -> OSPD10_W<OSPD_SPEC, 20> {
        OSPD10_W::new(self)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd11(&mut self) -> OSPD11_W<OSPD_SPEC, 22> {
        OSPD11_W::new(self)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd12(&mut self) -> OSPD12_W<OSPD_SPEC, 24> {
        OSPD12_W::new(self)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd13(&mut self) -> OSPD13_W<OSPD_SPEC, 26> {
        OSPD13_W::new(self)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd14(&mut self) -> OSPD14_W<OSPD_SPEC, 28> {
        OSPD14_W::new(self)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospd15(&mut self) -> OSPD15_W<OSPD_SPEC, 30> {
        OSPD15_W::new(self)
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
#[doc = "GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSPD_SPEC;
impl crate::RegisterSpec for OSPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospd::R`](R) reader structure"]
impl crate::Readable for OSPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ospd::W`](W) writer structure"]
impl crate::Writable for OSPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSPD to value 0"]
impl crate::Resettable for OSPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
