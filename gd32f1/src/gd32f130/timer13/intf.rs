#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<IntfSpec>;
#[doc = "Channel 0 interrupt flag"]
pub use crate::gd32f130::timer0::intf::Ch0if;
#[doc = "Field `CH0IF` reader - Channel 0 interrupt flag"]
pub use crate::gd32f130::timer0::intf::Ch0ifR;
#[doc = "Field `CH0IF` writer - Channel 0 interrupt flag"]
pub use crate::gd32f130::timer0::intf::Ch0ifW;
#[doc = "Channel 0 Capture overflow flag"]
pub use crate::gd32f130::timer0::intf::Ch0of;
#[doc = "Field `CH0OF` reader - Channel 0 Capture overflow flag"]
pub use crate::gd32f130::timer0::intf::Ch0ofR;
#[doc = "Field `CH0OF` writer - Channel 0 Capture overflow flag"]
pub use crate::gd32f130::timer0::intf::Ch0ofW;
#[doc = "Update interrupt flag"]
pub use crate::gd32f130::timer0::intf::Upif;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub use crate::gd32f130::timer0::intf::UpifR;
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub use crate::gd32f130::timer0::intf::UpifW;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UpifR {
        UpifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&self) -> Ch0ifR {
        Ch0ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> Ch0ofR {
        Ch0ofR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn upif(&mut self) -> UpifW<IntfSpec> {
        UpifW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0if(&mut self) -> Ch0ifW<IntfSpec> {
        Ch0ifW::new(self, 1)
    }
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0of(&mut self) -> Ch0ofW<IntfSpec> {
        Ch0ofW::new(self, 9)
    }
}
#[doc = "interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for IntfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {
    const RESET_VALUE: u16 = 0;
}
