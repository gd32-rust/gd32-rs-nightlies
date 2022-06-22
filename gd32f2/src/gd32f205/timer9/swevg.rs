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
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub type CH0G_W<'a> = crate::BitWriter<'a, u32, SWEVG_SPEC, bool, 1>;
#[doc = "Update generation"]
pub use crate::gd32f205::timer0::swevg::UPG_AW;
#[doc = "Field `UPG` writer - Update generation"]
pub type UPG_W<'a> = crate::BitWriter<'a, u32, SWEVG_SPEC, UPG_AW, 0>;
impl<'a> UPG_W<'a> {
    #[doc = "Re-initializes the timer counter and generates an update of the registers."]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(UPG_AW::UPDATE)
    }
}
impl W {
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevg](index.html) module"]
pub struct SWEVG_SPEC;
impl crate::RegisterSpec for SWEVG_SPEC {
    type Ux = u32;
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
