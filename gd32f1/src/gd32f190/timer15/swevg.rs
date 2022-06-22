#[doc = "Register `SWEVG` writer"]
pub struct W(crate::W<SWEVG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWEVG_SPEC>;
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
impl From<crate::W<SWEVG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWEVG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Break event generation"]
pub use crate::gd32f190::timer0::swevg::BRKG_AW;
#[doc = "Field `BRKG` writer - Break event generation"]
pub type BRKG_W<'a> = crate::BitWriter<'a, u16, SWEVG_SPEC, BRKG_AW, 7>;
impl<'a> BRKG_W<'a> {
    #[doc = "Generate a break event"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(BRKG_AW::BREAK)
    }
}
#[doc = "Channel commutation event generation"]
pub use crate::gd32f190::timer0::swevg::CMTG_AW;
#[doc = "Field `CMTG` writer - Channel commutation event generation"]
pub type CMTG_W<'a> = crate::BitWriter<'a, u16, SWEVG_SPEC, CMTG_AW, 5>;
impl<'a> CMTG_W<'a> {
    #[doc = "Generate a channel commutation event, updating capture/compare control registers based on the value of CCSE"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(CMTG_AW::UPDATE)
    }
}
#[doc = "Channel 0 capture or compare event generation"]
pub use crate::gd32f190::timer0::swevg::CH0G_AW;
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub type CH0G_W<'a> = crate::BitWriter<'a, u16, SWEVG_SPEC, CH0G_AW, 1>;
impl<'a> CH0G_W<'a> {
    #[doc = "Generate a capture or compare event"]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut W {
        self.variant(CH0G_AW::CAPTURECOMPARE)
    }
}
#[doc = "Update generation"]
pub use crate::gd32f190::timer0::swevg::UPG_AW;
#[doc = "Field `UPG` writer - Update generation"]
pub type UPG_W<'a> = crate::BitWriter<'a, u16, SWEVG_SPEC, UPG_AW, 0>;
impl<'a> UPG_W<'a> {
    #[doc = "Re-initializes the timer counter and generates an update of the registers."]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(UPG_AW::UPDATE)
    }
}
impl W {
    #[doc = "Bit 7 - Break event generation"]
    #[inline(always)]
    pub fn brkg(&mut self) -> BRKG_W {
        BRKG_W::new(self)
    }
    #[doc = "Bit 5 - Channel commutation event generation"]
    #[inline(always)]
    pub fn cmtg(&mut self) -> CMTG_W {
        CMTG_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 capture or compare event generation"]
    #[inline(always)]
    pub fn ch0g(&mut self) -> CH0G_W {
        CH0G_W::new(self)
    }
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn upg(&mut self) -> UPG_W {
        UPG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevg](index.html) module"]
pub struct SWEVG_SPEC;
impl crate::RegisterSpec for SWEVG_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [swevg::W](W) writer structure"]
impl crate::Writable for SWEVG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWEVG to value 0"]
impl crate::Resettable for SWEVG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
