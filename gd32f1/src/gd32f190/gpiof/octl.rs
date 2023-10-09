#[doc = "Register `OCTL` reader"]
pub type R = crate::R<OCTL_SPEC>;
#[doc = "Register `OCTL` writer"]
pub type W = crate::W<OCTL_SPEC>;
#[doc = "Field `OCTL0` reader - Port output data 0"]
pub type OCTL0_R = crate::BitReader<OCTL0_A>;
#[doc = "Port output data 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCTL0_A {
    #[doc = "0: Set output to logic low"]
    LOW = 0,
    #[doc = "1: Set output to logic high"]
    HIGH = 1,
}
impl From<OCTL0_A> for bool {
    #[inline(always)]
    fn from(variant: OCTL0_A) -> Self {
        variant as u8 != 0
    }
}
impl OCTL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCTL0_A {
        match self.bits {
            false => OCTL0_A::LOW,
            true => OCTL0_A::HIGH,
        }
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OCTL0_A::LOW
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OCTL0_A::HIGH
    }
}
#[doc = "Field `OCTL0` writer - Port output data 0"]
pub type OCTL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OCTL0_A>;
impl<'a, REG, const O: u8> OCTL0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(OCTL0_A::LOW)
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(OCTL0_A::HIGH)
    }
}
#[doc = "Field `OCTL1` reader - Port output data 1"]
pub use OCTL0_R as OCTL1_R;
#[doc = "Field `OCTL2` reader - Port output data 2"]
pub use OCTL0_R as OCTL2_R;
#[doc = "Field `OCTL3` reader - Port output data 3"]
pub use OCTL0_R as OCTL3_R;
#[doc = "Field `OCTL4` reader - Port output data 4"]
pub use OCTL0_R as OCTL4_R;
#[doc = "Field `OCTL5` reader - Port output data 5"]
pub use OCTL0_R as OCTL5_R;
#[doc = "Field `OCTL6` reader - Port output data 6"]
pub use OCTL0_R as OCTL6_R;
#[doc = "Field `OCTL7` reader - Port output data 7"]
pub use OCTL0_R as OCTL7_R;
#[doc = "Field `OCTL8` reader - Port output data 8"]
pub use OCTL0_R as OCTL8_R;
#[doc = "Field `OCTL9` reader - Port output data 9"]
pub use OCTL0_R as OCTL9_R;
#[doc = "Field `OCTL10` reader - Port output data 10"]
pub use OCTL0_R as OCTL10_R;
#[doc = "Field `OCTL11` reader - Port output data 11"]
pub use OCTL0_R as OCTL11_R;
#[doc = "Field `OCTL12` reader - Port output data 12"]
pub use OCTL0_R as OCTL12_R;
#[doc = "Field `OCTL13` reader - Port output data 13"]
pub use OCTL0_R as OCTL13_R;
#[doc = "Field `OCTL14` reader - Port output data 14"]
pub use OCTL0_R as OCTL14_R;
#[doc = "Field `OCTL15` reader - Port output data 15"]
pub use OCTL0_R as OCTL15_R;
#[doc = "Field `OCTL1` writer - Port output data 1"]
pub use OCTL0_W as OCTL1_W;
#[doc = "Field `OCTL2` writer - Port output data 2"]
pub use OCTL0_W as OCTL2_W;
#[doc = "Field `OCTL3` writer - Port output data 3"]
pub use OCTL0_W as OCTL3_W;
#[doc = "Field `OCTL4` writer - Port output data 4"]
pub use OCTL0_W as OCTL4_W;
#[doc = "Field `OCTL5` writer - Port output data 5"]
pub use OCTL0_W as OCTL5_W;
#[doc = "Field `OCTL6` writer - Port output data 6"]
pub use OCTL0_W as OCTL6_W;
#[doc = "Field `OCTL7` writer - Port output data 7"]
pub use OCTL0_W as OCTL7_W;
#[doc = "Field `OCTL8` writer - Port output data 8"]
pub use OCTL0_W as OCTL8_W;
#[doc = "Field `OCTL9` writer - Port output data 9"]
pub use OCTL0_W as OCTL9_W;
#[doc = "Field `OCTL10` writer - Port output data 10"]
pub use OCTL0_W as OCTL10_W;
#[doc = "Field `OCTL11` writer - Port output data 11"]
pub use OCTL0_W as OCTL11_W;
#[doc = "Field `OCTL12` writer - Port output data 12"]
pub use OCTL0_W as OCTL12_W;
#[doc = "Field `OCTL13` writer - Port output data 13"]
pub use OCTL0_W as OCTL13_W;
#[doc = "Field `OCTL14` writer - Port output data 14"]
pub use OCTL0_W as OCTL14_W;
#[doc = "Field `OCTL15` writer - Port output data 15"]
pub use OCTL0_W as OCTL15_W;
impl R {
    #[doc = "Bit 0 - Port output data 0"]
    #[inline(always)]
    pub fn octl0(&self) -> OCTL0_R {
        OCTL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data 1"]
    #[inline(always)]
    pub fn octl1(&self) -> OCTL1_R {
        OCTL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data 2"]
    #[inline(always)]
    pub fn octl2(&self) -> OCTL2_R {
        OCTL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data 3"]
    #[inline(always)]
    pub fn octl3(&self) -> OCTL3_R {
        OCTL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data 4"]
    #[inline(always)]
    pub fn octl4(&self) -> OCTL4_R {
        OCTL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data 5"]
    #[inline(always)]
    pub fn octl5(&self) -> OCTL5_R {
        OCTL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data 6"]
    #[inline(always)]
    pub fn octl6(&self) -> OCTL6_R {
        OCTL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data 7"]
    #[inline(always)]
    pub fn octl7(&self) -> OCTL7_R {
        OCTL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data 8"]
    #[inline(always)]
    pub fn octl8(&self) -> OCTL8_R {
        OCTL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output data 9"]
    #[inline(always)]
    pub fn octl9(&self) -> OCTL9_R {
        OCTL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output data 10"]
    #[inline(always)]
    pub fn octl10(&self) -> OCTL10_R {
        OCTL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output data 11"]
    #[inline(always)]
    pub fn octl11(&self) -> OCTL11_R {
        OCTL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output data 12"]
    #[inline(always)]
    pub fn octl12(&self) -> OCTL12_R {
        OCTL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output data 13"]
    #[inline(always)]
    pub fn octl13(&self) -> OCTL13_R {
        OCTL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output data 14"]
    #[inline(always)]
    pub fn octl14(&self) -> OCTL14_R {
        OCTL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port output data 15"]
    #[inline(always)]
    pub fn octl15(&self) -> OCTL15_R {
        OCTL15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port output data 0"]
    #[inline(always)]
    #[must_use]
    pub fn octl0(&mut self) -> OCTL0_W<OCTL_SPEC, 0> {
        OCTL0_W::new(self)
    }
    #[doc = "Bit 1 - Port output data 1"]
    #[inline(always)]
    #[must_use]
    pub fn octl1(&mut self) -> OCTL1_W<OCTL_SPEC, 1> {
        OCTL1_W::new(self)
    }
    #[doc = "Bit 2 - Port output data 2"]
    #[inline(always)]
    #[must_use]
    pub fn octl2(&mut self) -> OCTL2_W<OCTL_SPEC, 2> {
        OCTL2_W::new(self)
    }
    #[doc = "Bit 3 - Port output data 3"]
    #[inline(always)]
    #[must_use]
    pub fn octl3(&mut self) -> OCTL3_W<OCTL_SPEC, 3> {
        OCTL3_W::new(self)
    }
    #[doc = "Bit 4 - Port output data 4"]
    #[inline(always)]
    #[must_use]
    pub fn octl4(&mut self) -> OCTL4_W<OCTL_SPEC, 4> {
        OCTL4_W::new(self)
    }
    #[doc = "Bit 5 - Port output data 5"]
    #[inline(always)]
    #[must_use]
    pub fn octl5(&mut self) -> OCTL5_W<OCTL_SPEC, 5> {
        OCTL5_W::new(self)
    }
    #[doc = "Bit 6 - Port output data 6"]
    #[inline(always)]
    #[must_use]
    pub fn octl6(&mut self) -> OCTL6_W<OCTL_SPEC, 6> {
        OCTL6_W::new(self)
    }
    #[doc = "Bit 7 - Port output data 7"]
    #[inline(always)]
    #[must_use]
    pub fn octl7(&mut self) -> OCTL7_W<OCTL_SPEC, 7> {
        OCTL7_W::new(self)
    }
    #[doc = "Bit 8 - Port output data 8"]
    #[inline(always)]
    #[must_use]
    pub fn octl8(&mut self) -> OCTL8_W<OCTL_SPEC, 8> {
        OCTL8_W::new(self)
    }
    #[doc = "Bit 9 - Port output data 9"]
    #[inline(always)]
    #[must_use]
    pub fn octl9(&mut self) -> OCTL9_W<OCTL_SPEC, 9> {
        OCTL9_W::new(self)
    }
    #[doc = "Bit 10 - Port output data 10"]
    #[inline(always)]
    #[must_use]
    pub fn octl10(&mut self) -> OCTL10_W<OCTL_SPEC, 10> {
        OCTL10_W::new(self)
    }
    #[doc = "Bit 11 - Port output data 11"]
    #[inline(always)]
    #[must_use]
    pub fn octl11(&mut self) -> OCTL11_W<OCTL_SPEC, 11> {
        OCTL11_W::new(self)
    }
    #[doc = "Bit 12 - Port output data 12"]
    #[inline(always)]
    #[must_use]
    pub fn octl12(&mut self) -> OCTL12_W<OCTL_SPEC, 12> {
        OCTL12_W::new(self)
    }
    #[doc = "Bit 13 - Port output data 13"]
    #[inline(always)]
    #[must_use]
    pub fn octl13(&mut self) -> OCTL13_W<OCTL_SPEC, 13> {
        OCTL13_W::new(self)
    }
    #[doc = "Bit 14 - Port output data 14"]
    #[inline(always)]
    #[must_use]
    pub fn octl14(&mut self) -> OCTL14_W<OCTL_SPEC, 14> {
        OCTL14_W::new(self)
    }
    #[doc = "Bit 15 - Port output data 15"]
    #[inline(always)]
    #[must_use]
    pub fn octl15(&mut self) -> OCTL15_W<OCTL_SPEC, 15> {
        OCTL15_W::new(self)
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
#[doc = "GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCTL_SPEC;
impl crate::RegisterSpec for OCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octl::R`](R) reader structure"]
impl crate::Readable for OCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`octl::W`](W) writer structure"]
impl crate::Writable for OCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTL to value 0"]
impl crate::Resettable for OCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
