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
#[doc = "Field `BRKG` writer - Break generation"]
pub type BRKG_W<'a> = crate::BitWriter<'a, u32, SWEVG_SPEC, bool, 7>;
#[doc = "Field `CMTG` writer - Capture/Compare control update generation"]
pub type CMTG_W<'a> = crate::BitWriter<'a, u32, SWEVG_SPEC, bool, 5>;
#[doc = "Field `CH0G` writer - Capture/compare 0 generation"]
pub type CH0G_W<'a> = crate::BitWriter<'a, u32, SWEVG_SPEC, bool, 1>;
#[doc = "Field `UPG` writer - Update generation"]
pub type UPG_W<'a> = crate::BitWriter<'a, u32, SWEVG_SPEC, bool, 0>;
impl W {
    #[doc = "Bit 7 - Break generation"]
    #[inline(always)]
    pub fn brkg(&mut self) -> BRKG_W {
        BRKG_W::new(self)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation"]
    #[inline(always)]
    pub fn cmtg(&mut self) -> CMTG_W {
        CMTG_W::new(self)
    }
    #[doc = "Bit 1 - Capture/compare 0 generation"]
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
