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
#[doc = "Trigger generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGG_AW {
    #[doc = "1: Generate a trigger event"]
    TRIGGER = 1,
}
impl From<TRGG_AW> for bool {
    #[inline(always)]
    fn from(variant: TRGG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGG` writer - Trigger generation"]
pub type TRGG_W<'a> = crate::BitWriter<'a, u16, SWEVG_SPEC, TRGG_AW, 6>;
impl<'a> TRGG_W<'a> {
    #[doc = "Generate a trigger event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TRGG_AW::TRIGGER)
    }
}
#[doc = "Channel 3 capture or compare event generation"]
pub use crate::gd32f150::timer0::swevg::CH0G_AW as CH3G_AW;
#[doc = "Field `CH3G` writer - Channel 3 capture or compare event generation"]
pub type CH3G_W<'a> = crate::BitWriter<'a, u16, SWEVG_SPEC, CH3G_AW, 4>;
impl<'a> CH3G_W<'a> {
    #[doc = "Generate a capture or compare event"]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut W {
        self.variant(CH3G_AW::CAPTURECOMPARE)
    }
}
#[doc = "Channel 2 capture or compare event generation"]
pub use crate::gd32f150::timer0::swevg::CH0G_AW as CH2G_AW;
#[doc = "Field `CH2G` writer - Channel 2 capture or compare event generation"]
pub type CH2G_W<'a> = crate::BitWriter<'a, u16, SWEVG_SPEC, CH2G_AW, 3>;
impl<'a> CH2G_W<'a> {
    #[doc = "Generate a capture or compare event"]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut W {
        self.variant(CH2G_AW::CAPTURECOMPARE)
    }
}
#[doc = "Channel 1 capture or compare event generation"]
pub use crate::gd32f150::timer0::swevg::CH0G_AW as CH1G_AW;
#[doc = "Field `CH1G` writer - Channel 1 capture or compare event generation"]
pub type CH1G_W<'a> = crate::BitWriter<'a, u16, SWEVG_SPEC, CH1G_AW, 2>;
impl<'a> CH1G_W<'a> {
    #[doc = "Generate a capture or compare event"]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut W {
        self.variant(CH1G_AW::CAPTURECOMPARE)
    }
}
#[doc = "Channel 0 capture or compare event generation"]
pub use crate::gd32f150::timer0::swevg::CH0G_AW;
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
pub use crate::gd32f150::timer0::swevg::UPG_AW;
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
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn trgg(&mut self) -> TRGG_W {
        TRGG_W::new(self)
    }
    #[doc = "Bit 4 - Channel 3 capture or compare event generation"]
    #[inline(always)]
    pub fn ch3g(&mut self) -> CH3G_W {
        CH3G_W::new(self)
    }
    #[doc = "Bit 3 - Channel 2 capture or compare event generation"]
    #[inline(always)]
    pub fn ch2g(&mut self) -> CH2G_W {
        CH2G_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 capture or compare event generation"]
    #[inline(always)]
    pub fn ch1g(&mut self) -> CH1G_W {
        CH1G_W::new(self)
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
#[doc = "event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevg](index.html) module"]
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
