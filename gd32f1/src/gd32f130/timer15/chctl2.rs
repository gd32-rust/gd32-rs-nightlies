#[doc = "Register `CHCTL2` reader"]
pub type R = crate::R<CHCTL2_SPEC>;
#[doc = "Register `CHCTL2` writer"]
pub type W = crate::W<CHCTL2_SPEC>;
#[doc = "Channel 0 enable"]
pub use crate::gd32f130::timer0::chctl2::CH0EN_A;
#[doc = "Field `CH0EN` reader - Channel 0 enable"]
pub use crate::gd32f130::timer0::chctl2::CH0EN_R;
#[doc = "Field `CH0NEN` reader - Channel 0 complementary output enable"]
pub use crate::gd32f130::timer0::chctl2::CH0EN_R as CH0NEN_R;
#[doc = "Field `CH0EN` writer - Channel 0 enable"]
pub use crate::gd32f130::timer0::chctl2::CH0EN_W;
#[doc = "Field `CH0NEN` writer - Channel 0 complementary output enable"]
pub use crate::gd32f130::timer0::chctl2::CH0EN_W as CH0NEN_W;
#[doc = "Channel 0 polarity"]
pub use crate::gd32f130::timer0::chctl2::CH0P_A;
#[doc = "Field `CH0P` reader - Channel 0 polarity"]
pub use crate::gd32f130::timer0::chctl2::CH0P_R;
#[doc = "Field `CH0NP` reader - Channel 0 complementary output polarity"]
pub use crate::gd32f130::timer0::chctl2::CH0P_R as CH0NP_R;
#[doc = "Field `CH0P` writer - Channel 0 polarity"]
pub use crate::gd32f130::timer0::chctl2::CH0P_W;
#[doc = "Field `CH0NP` writer - Channel 0 complementary output polarity"]
pub use crate::gd32f130::timer0::chctl2::CH0P_W as CH0NP_W;
impl R {
    #[doc = "Bit 0 - Channel 0 enable"]
    #[inline(always)]
    pub fn ch0en(&self) -> CH0EN_R {
        CH0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 polarity"]
    #[inline(always)]
    pub fn ch0p(&self) -> CH0P_R {
        CH0P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 complementary output enable"]
    #[inline(always)]
    pub fn ch0nen(&self) -> CH0NEN_R {
        CH0NEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    pub fn ch0np(&self) -> CH0NP_R {
        CH0NP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0en(&mut self) -> CH0EN_W<CHCTL2_SPEC, 0> {
        CH0EN_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch0p(&mut self) -> CH0P_W<CHCTL2_SPEC, 1> {
        CH0P_W::new(self)
    }
    #[doc = "Bit 2 - Channel 0 complementary output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0nen(&mut self) -> CH0NEN_W<CHCTL2_SPEC, 2> {
        CH0NEN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch0np(&mut self) -> CH0NP_W<CHCTL2_SPEC, 3> {
        CH0NP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTL2_SPEC;
impl crate::RegisterSpec for CHCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chctl2::R`](R) reader structure"]
impl crate::Readable for CHCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctl2::W`](W) writer structure"]
impl crate::Writable for CHCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTL2 to value 0"]
impl crate::Resettable for CHCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
