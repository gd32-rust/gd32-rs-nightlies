#[doc = "Register `SWEVG` writer"]
pub type W = crate::W<SwevgSpec>;
#[doc = "Channel 0 capture or compare event generation"]
pub use crate::gd32f130::timer0::swevg::Ch0g;
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub use crate::gd32f130::timer0::swevg::Ch0gW;
#[doc = "Update generation"]
pub use crate::gd32f130::timer0::swevg::Upg;
#[doc = "Field `UPG` writer - Update generation"]
pub use crate::gd32f130::timer0::swevg::UpgW;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn upg(&mut self) -> UpgW<SwevgSpec> {
        UpgW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch0g(&mut self) -> Ch0gW<SwevgSpec> {
        Ch0gW::new(self, 1)
    }
}
#[doc = "Software event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwevgSpec;
impl crate::RegisterSpec for SwevgSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`swevg::W`](W) writer structure"]
impl crate::Writable for SwevgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SWEVG to value 0"]
impl crate::Resettable for SwevgSpec {
    const RESET_VALUE: u16 = 0;
}
