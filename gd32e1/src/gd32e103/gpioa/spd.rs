#[doc = "Register `SPD` reader"]
pub type R = crate::R<SPD_SPEC>;
#[doc = "Register `SPD` writer"]
pub type W = crate::W<SPD_SPEC>;
#[doc = "Field `SPD0` reader - Port 0 output max speed bits"]
pub type SPD0_R = crate::BitReader<SPD0_A>;
#[doc = "Port 0 output max speed bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPD0_A {
    #[doc = "0: Max output speed 50 MHz"]
    SPEED50M = 0,
    #[doc = "1: Max output speed 120 MHz"]
    SPEED120M = 1,
}
impl From<SPD0_A> for bool {
    #[inline(always)]
    fn from(variant: SPD0_A) -> Self {
        variant as u8 != 0
    }
}
impl SPD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPD0_A {
        match self.bits {
            false => SPD0_A::SPEED50M,
            true => SPD0_A::SPEED120M,
        }
    }
    #[doc = "Max output speed 50 MHz"]
    #[inline(always)]
    pub fn is_speed50m(&self) -> bool {
        *self == SPD0_A::SPEED50M
    }
    #[doc = "Max output speed 120 MHz"]
    #[inline(always)]
    pub fn is_speed120m(&self) -> bool {
        *self == SPD0_A::SPEED120M
    }
}
#[doc = "Field `SPD0` writer - Port 0 output max speed bits"]
pub type SPD0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPD0_A>;
impl<'a, REG, const O: u8> SPD0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Max output speed 50 MHz"]
    #[inline(always)]
    pub fn speed50m(self) -> &'a mut crate::W<REG> {
        self.variant(SPD0_A::SPEED50M)
    }
    #[doc = "Max output speed 120 MHz"]
    #[inline(always)]
    pub fn speed120m(self) -> &'a mut crate::W<REG> {
        self.variant(SPD0_A::SPEED120M)
    }
}
#[doc = "Field `SPD1` reader - Port 1 output max speed bits"]
pub use SPD0_R as SPD1_R;
#[doc = "Field `SPD2` reader - Port 2 output max speed bits"]
pub use SPD0_R as SPD2_R;
#[doc = "Field `SPD3` reader - Port 3 output max speed bits"]
pub use SPD0_R as SPD3_R;
#[doc = "Field `SPD4` reader - Port 4 output max speed bits"]
pub use SPD0_R as SPD4_R;
#[doc = "Field `SPD5` reader - Port 5 output max speed bits"]
pub use SPD0_R as SPD5_R;
#[doc = "Field `SPD6` reader - Port 6 output max speed bits"]
pub use SPD0_R as SPD6_R;
#[doc = "Field `SPD7` reader - Port 7 output max speed bits"]
pub use SPD0_R as SPD7_R;
#[doc = "Field `SPD8` reader - Port 8 output max speed bits"]
pub use SPD0_R as SPD8_R;
#[doc = "Field `SPD9` reader - Port 9 output max speed bits"]
pub use SPD0_R as SPD9_R;
#[doc = "Field `SPD10` reader - Port 10 output max speed bits"]
pub use SPD0_R as SPD10_R;
#[doc = "Field `SPD11` reader - Port 11 output max speed bits"]
pub use SPD0_R as SPD11_R;
#[doc = "Field `SPD12` reader - Port 12 output max speed bits"]
pub use SPD0_R as SPD12_R;
#[doc = "Field `SPD13` reader - Port 13 output max speed bits"]
pub use SPD0_R as SPD13_R;
#[doc = "Field `SPD14` reader - Port 14 output max speed bits"]
pub use SPD0_R as SPD14_R;
#[doc = "Field `SPD15` reader - Port 15 output max speed bits"]
pub use SPD0_R as SPD15_R;
#[doc = "Field `SPD1` writer - Port 1 output max speed bits"]
pub use SPD0_W as SPD1_W;
#[doc = "Field `SPD2` writer - Port 2 output max speed bits"]
pub use SPD0_W as SPD2_W;
#[doc = "Field `SPD3` writer - Port 3 output max speed bits"]
pub use SPD0_W as SPD3_W;
#[doc = "Field `SPD4` writer - Port 4 output max speed bits"]
pub use SPD0_W as SPD4_W;
#[doc = "Field `SPD5` writer - Port 5 output max speed bits"]
pub use SPD0_W as SPD5_W;
#[doc = "Field `SPD6` writer - Port 6 output max speed bits"]
pub use SPD0_W as SPD6_W;
#[doc = "Field `SPD7` writer - Port 7 output max speed bits"]
pub use SPD0_W as SPD7_W;
#[doc = "Field `SPD8` writer - Port 8 output max speed bits"]
pub use SPD0_W as SPD8_W;
#[doc = "Field `SPD9` writer - Port 9 output max speed bits"]
pub use SPD0_W as SPD9_W;
#[doc = "Field `SPD10` writer - Port 10 output max speed bits"]
pub use SPD0_W as SPD10_W;
#[doc = "Field `SPD11` writer - Port 11 output max speed bits"]
pub use SPD0_W as SPD11_W;
#[doc = "Field `SPD12` writer - Port 12 output max speed bits"]
pub use SPD0_W as SPD12_W;
#[doc = "Field `SPD13` writer - Port 13 output max speed bits"]
pub use SPD0_W as SPD13_W;
#[doc = "Field `SPD14` writer - Port 14 output max speed bits"]
pub use SPD0_W as SPD14_W;
#[doc = "Field `SPD15` writer - Port 15 output max speed bits"]
pub use SPD0_W as SPD15_W;
impl R {
    #[doc = "Bit 0 - Port 0 output max speed bits"]
    #[inline(always)]
    pub fn spd0(&self) -> SPD0_R {
        SPD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port 1 output max speed bits"]
    #[inline(always)]
    pub fn spd1(&self) -> SPD1_R {
        SPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port 2 output max speed bits"]
    #[inline(always)]
    pub fn spd2(&self) -> SPD2_R {
        SPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port 3 output max speed bits"]
    #[inline(always)]
    pub fn spd3(&self) -> SPD3_R {
        SPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port 4 output max speed bits"]
    #[inline(always)]
    pub fn spd4(&self) -> SPD4_R {
        SPD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port 5 output max speed bits"]
    #[inline(always)]
    pub fn spd5(&self) -> SPD5_R {
        SPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port 6 output max speed bits"]
    #[inline(always)]
    pub fn spd6(&self) -> SPD6_R {
        SPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port 7 output max speed bits"]
    #[inline(always)]
    pub fn spd7(&self) -> SPD7_R {
        SPD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port 8 output max speed bits"]
    #[inline(always)]
    pub fn spd8(&self) -> SPD8_R {
        SPD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port 9 output max speed bits"]
    #[inline(always)]
    pub fn spd9(&self) -> SPD9_R {
        SPD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port 10 output max speed bits"]
    #[inline(always)]
    pub fn spd10(&self) -> SPD10_R {
        SPD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port 11 output max speed bits"]
    #[inline(always)]
    pub fn spd11(&self) -> SPD11_R {
        SPD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port 12 output max speed bits"]
    #[inline(always)]
    pub fn spd12(&self) -> SPD12_R {
        SPD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port 13 output max speed bits"]
    #[inline(always)]
    pub fn spd13(&self) -> SPD13_R {
        SPD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port 14 output max speed bits"]
    #[inline(always)]
    pub fn spd14(&self) -> SPD14_R {
        SPD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port 15 output max speed bits"]
    #[inline(always)]
    pub fn spd15(&self) -> SPD15_R {
        SPD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 0 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd0(&mut self) -> SPD0_W<SPD_SPEC, 0> {
        SPD0_W::new(self)
    }
    #[doc = "Bit 1 - Port 1 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd1(&mut self) -> SPD1_W<SPD_SPEC, 1> {
        SPD1_W::new(self)
    }
    #[doc = "Bit 2 - Port 2 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd2(&mut self) -> SPD2_W<SPD_SPEC, 2> {
        SPD2_W::new(self)
    }
    #[doc = "Bit 3 - Port 3 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd3(&mut self) -> SPD3_W<SPD_SPEC, 3> {
        SPD3_W::new(self)
    }
    #[doc = "Bit 4 - Port 4 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd4(&mut self) -> SPD4_W<SPD_SPEC, 4> {
        SPD4_W::new(self)
    }
    #[doc = "Bit 5 - Port 5 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd5(&mut self) -> SPD5_W<SPD_SPEC, 5> {
        SPD5_W::new(self)
    }
    #[doc = "Bit 6 - Port 6 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd6(&mut self) -> SPD6_W<SPD_SPEC, 6> {
        SPD6_W::new(self)
    }
    #[doc = "Bit 7 - Port 7 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd7(&mut self) -> SPD7_W<SPD_SPEC, 7> {
        SPD7_W::new(self)
    }
    #[doc = "Bit 8 - Port 8 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd8(&mut self) -> SPD8_W<SPD_SPEC, 8> {
        SPD8_W::new(self)
    }
    #[doc = "Bit 9 - Port 9 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd9(&mut self) -> SPD9_W<SPD_SPEC, 9> {
        SPD9_W::new(self)
    }
    #[doc = "Bit 10 - Port 10 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd10(&mut self) -> SPD10_W<SPD_SPEC, 10> {
        SPD10_W::new(self)
    }
    #[doc = "Bit 11 - Port 11 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd11(&mut self) -> SPD11_W<SPD_SPEC, 11> {
        SPD11_W::new(self)
    }
    #[doc = "Bit 12 - Port 12 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd12(&mut self) -> SPD12_W<SPD_SPEC, 12> {
        SPD12_W::new(self)
    }
    #[doc = "Bit 13 - Port 13 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd13(&mut self) -> SPD13_W<SPD_SPEC, 13> {
        SPD13_W::new(self)
    }
    #[doc = "Bit 14 - Port 14 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd14(&mut self) -> SPD14_W<SPD_SPEC, 14> {
        SPD14_W::new(self)
    }
    #[doc = "Bit 15 - Port 15 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd15(&mut self) -> SPD15_W<SPD_SPEC, 15> {
        SPD15_W::new(self)
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
#[doc = "Port bit speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPD_SPEC;
impl crate::RegisterSpec for SPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spd::R`](R) reader structure"]
impl crate::Readable for SPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spd::W`](W) writer structure"]
impl crate::Writable for SPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPD to value 0"]
impl crate::Resettable for SPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
