#[doc = "Register `MAC_HLH` reader"]
pub struct R(crate::R<MAC_HLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_HLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_HLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_HLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_HLH` writer"]
pub struct W(crate::W<MAC_HLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_HLH_SPEC>;
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
impl From<crate::W<MAC_HLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_HLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HLH` reader - Hash list high"]
pub type HLH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HLH` writer - Hash list high"]
pub type HLH_W<'a> = crate::FieldWriter<'a, u32, MAC_HLH_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Hash list high"]
    #[inline(always)]
    pub fn hlh(&self) -> HLH_R {
        HLH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash list high"]
    #[inline(always)]
    pub fn hlh(&mut self) -> HLH_W {
        HLH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC hash list high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_hlh](index.html) module"]
pub struct MAC_HLH_SPEC;
impl crate::RegisterSpec for MAC_HLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_hlh::R](R) reader structure"]
impl crate::Readable for MAC_HLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_hlh::W](W) writer structure"]
impl crate::Writable for MAC_HLH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_HLH to value 0"]
impl crate::Resettable for MAC_HLH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
