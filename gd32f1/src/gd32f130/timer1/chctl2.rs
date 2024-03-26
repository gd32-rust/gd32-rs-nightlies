#[doc = "Register `CHCTL2` reader"]
pub type R = crate::R<Chctl2Spec>;
#[doc = "Register `CHCTL2` writer"]
pub type W = crate::W<Chctl2Spec>;
#[doc = "Channel 0 enable"]
pub use crate::gd32f130::timer0::chctl2::Ch0en;
#[doc = "Field `CH0EN` reader - Channel 0 enable"]
pub use crate::gd32f130::timer0::chctl2::Ch0enR;
#[doc = "Field `CH1EN` reader - Channel 1 enable"]
pub use crate::gd32f130::timer0::chctl2::Ch0enR as Ch1enR;
#[doc = "Field `CH2EN` reader - Channel 2 enable"]
pub use crate::gd32f130::timer0::chctl2::Ch0enR as Ch2enR;
#[doc = "Field `CH3EN` reader - Channel 3 enable"]
pub use crate::gd32f130::timer0::chctl2::Ch0enR as Ch3enR;
#[doc = "Field `CH0EN` writer - Channel 0 enable"]
pub use crate::gd32f130::timer0::chctl2::Ch0enW;
#[doc = "Field `CH1EN` writer - Channel 1 enable"]
pub use crate::gd32f130::timer0::chctl2::Ch0enW as Ch1enW;
#[doc = "Field `CH2EN` writer - Channel 2 enable"]
pub use crate::gd32f130::timer0::chctl2::Ch0enW as Ch2enW;
#[doc = "Field `CH3EN` writer - Channel 3 enable"]
pub use crate::gd32f130::timer0::chctl2::Ch0enW as Ch3enW;
#[doc = "Channel 0 polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0p;
#[doc = "Field `CH0P` reader - Channel 0 polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pR;
#[doc = "Field `CH0NP` reader - Channel 0 complementary output polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pR as Ch0npR;
#[doc = "Field `CH1P` reader - Channel 1 polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pR as Ch1pR;
#[doc = "Field `CH1NP` reader - Channel 1 complementary output polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pR as Ch1npR;
#[doc = "Field `CH2P` reader - Channel 2 polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pR as Ch2pR;
#[doc = "Field `CH2NP` reader - Channel 2 complementary output polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pR as Ch2npR;
#[doc = "Field `CH3P` reader - Channel 3 polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pR as Ch3pR;
#[doc = "Field `CH3NP` reader - Channel 3 complementary output polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pR as Ch3npR;
#[doc = "Field `CH0P` writer - Channel 0 polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pW;
#[doc = "Field `CH0NP` writer - Channel 0 complementary output polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pW as Ch0npW;
#[doc = "Field `CH1P` writer - Channel 1 polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pW as Ch1pW;
#[doc = "Field `CH1NP` writer - Channel 1 complementary output polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pW as Ch1npW;
#[doc = "Field `CH2P` writer - Channel 2 polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pW as Ch2pW;
#[doc = "Field `CH2NP` writer - Channel 2 complementary output polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pW as Ch2npW;
#[doc = "Field `CH3P` writer - Channel 3 polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pW as Ch3pW;
#[doc = "Field `CH3NP` writer - Channel 3 complementary output polarity"]
pub use crate::gd32f130::timer0::chctl2::Ch0pW as Ch3npW;
impl R {
    #[doc = "Bit 0 - Channel 0 enable"]
    #[inline(always)]
    pub fn ch0en(&self) -> Ch0enR {
        Ch0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 polarity"]
    #[inline(always)]
    pub fn ch0p(&self) -> Ch0pR {
        Ch0pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    pub fn ch0np(&self) -> Ch0npR {
        Ch0npR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 enable"]
    #[inline(always)]
    pub fn ch1en(&self) -> Ch1enR {
        Ch1enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 polarity"]
    #[inline(always)]
    pub fn ch1p(&self) -> Ch1pR {
        Ch1pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 1 complementary output polarity"]
    #[inline(always)]
    pub fn ch1np(&self) -> Ch1npR {
        Ch1npR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 2 enable"]
    #[inline(always)]
    pub fn ch2en(&self) -> Ch2enR {
        Ch2enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 2 polarity"]
    #[inline(always)]
    pub fn ch2p(&self) -> Ch2pR {
        Ch2pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 complementary output polarity"]
    #[inline(always)]
    pub fn ch2np(&self) -> Ch2npR {
        Ch2npR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 enable"]
    #[inline(always)]
    pub fn ch3en(&self) -> Ch3enR {
        Ch3enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 3 polarity"]
    #[inline(always)]
    pub fn ch3p(&self) -> Ch3pR {
        Ch3pR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 3 complementary output polarity"]
    #[inline(always)]
    pub fn ch3np(&self) -> Ch3npR {
        Ch3npR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0en(&mut self) -> Ch0enW<Chctl2Spec> {
        Ch0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch0p(&mut self) -> Ch0pW<Chctl2Spec> {
        Ch0pW::new(self, 1)
    }
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch0np(&mut self) -> Ch0npW<Chctl2Spec> {
        Ch0npW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1en(&mut self) -> Ch1enW<Chctl2Spec> {
        Ch1enW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch1p(&mut self) -> Ch1pW<Chctl2Spec> {
        Ch1pW::new(self, 5)
    }
    #[doc = "Bit 7 - Channel 1 complementary output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch1np(&mut self) -> Ch1npW<Chctl2Spec> {
        Ch1npW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2en(&mut self) -> Ch2enW<Chctl2Spec> {
        Ch2enW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch2p(&mut self) -> Ch2pW<Chctl2Spec> {
        Ch2pW::new(self, 9)
    }
    #[doc = "Bit 11 - Channel 2 complementary output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch2np(&mut self) -> Ch2npW<Chctl2Spec> {
        Ch2npW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3en(&mut self) -> Ch3enW<Chctl2Spec> {
        Ch3enW::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 3 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch3p(&mut self) -> Ch3pW<Chctl2Spec> {
        Ch3pW::new(self, 13)
    }
    #[doc = "Bit 15 - Channel 3 complementary output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch3np(&mut self) -> Ch3npW<Chctl2Spec> {
        Ch3npW::new(self, 15)
    }
}
#[doc = "Channel control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chctl2Spec;
impl crate::RegisterSpec for Chctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chctl2::R`](R) reader structure"]
impl crate::Readable for Chctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`chctl2::W`](W) writer structure"]
impl crate::Writable for Chctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CHCTL2 to value 0"]
impl crate::Resettable for Chctl2Spec {
    const RESET_VALUE: u16 = 0;
}
