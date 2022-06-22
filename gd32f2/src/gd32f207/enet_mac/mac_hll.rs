#[doc = "Register `MAC_HLL` reader"]
pub struct R(crate::R<MAC_HLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_HLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_HLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_HLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_HLL` writer"]
pub struct W(crate::W<MAC_HLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_HLL_SPEC>;
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
impl From<crate::W<MAC_HLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_HLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HLL` reader - Hash list low"]
pub type HLL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HLL` writer - Hash list low"]
pub type HLL_W<'a> = crate::FieldWriter<'a, u32, MAC_HLL_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Hash list low"]
    #[inline(always)]
    pub fn hll(&self) -> HLL_R {
        HLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash list low"]
    #[inline(always)]
    pub fn hll(&mut self) -> HLL_W {
        HLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC hash list low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_hll](index.html) module"]
pub struct MAC_HLL_SPEC;
impl crate::RegisterSpec for MAC_HLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_hll::R](R) reader structure"]
impl crate::Readable for MAC_HLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_hll::W](W) writer structure"]
impl crate::Writable for MAC_HLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_HLL to value 0"]
impl crate::Resettable for MAC_HLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
