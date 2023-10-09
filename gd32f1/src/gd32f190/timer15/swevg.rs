#[doc = "Register `SWEVG` writer"]
pub type W = crate::W<SWEVG_SPEC>;
#[doc = "Break event generation"]
pub use crate::gd32f190::timer0::swevg::BRKG_AW;
#[doc = "Field `BRKG` writer - Break event generation"]
pub use crate::gd32f190::timer0::swevg::BRKG_W;
#[doc = "Channel 0 capture or compare event generation"]
pub use crate::gd32f190::timer0::swevg::CH0G_AW;
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub use crate::gd32f190::timer0::swevg::CH0G_W;
#[doc = "Channel commutation event generation"]
pub use crate::gd32f190::timer0::swevg::CMTG_AW;
#[doc = "Field `CMTG` writer - Channel commutation event generation"]
pub use crate::gd32f190::timer0::swevg::CMTG_W;
#[doc = "Update generation"]
pub use crate::gd32f190::timer0::swevg::UPG_AW;
#[doc = "Field `UPG` writer - Update generation"]
pub use crate::gd32f190::timer0::swevg::UPG_W;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn upg(&mut self) -> UPG_W<SWEVG_SPEC, 0> {
        UPG_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch0g(&mut self) -> CH0G_W<SWEVG_SPEC, 1> {
        CH0G_W::new(self)
    }
    #[doc = "Bit 5 - Channel commutation event generation"]
    #[inline(always)]
    #[must_use]
    pub fn cmtg(&mut self) -> CMTG_W<SWEVG_SPEC, 5> {
        CMTG_W::new(self)
    }
    #[doc = "Bit 7 - Break event generation"]
    #[inline(always)]
    #[must_use]
    pub fn brkg(&mut self) -> BRKG_W<SWEVG_SPEC, 7> {
        BRKG_W::new(self)
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
#[doc = "Software event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVG_SPEC;
impl crate::RegisterSpec for SWEVG_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`swevg::W`](W) writer structure"]
impl crate::Writable for SWEVG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWEVG to value 0"]
impl crate::Resettable for SWEVG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
