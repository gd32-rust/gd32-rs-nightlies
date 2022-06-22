#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALEN` writer - Digest calculation enable"]
pub type CALEN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 8>;
#[doc = "Field `VBL` reader - Valid bits length in the last word"]
pub type VBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBL` writer - Valid bits length in the last word"]
pub type VBL_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 0:4 - Valid bits length in the last word"]
    #[inline(always)]
    pub fn vbl(&self) -> VBL_R {
        VBL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Digest calculation enable"]
    #[inline(always)]
    pub fn calen(&mut self) -> CALEN_W {
        CALEN_W::new(self)
    }
    #[doc = "Bits 0:4 - Valid bits length in the last word"]
    #[inline(always)]
    pub fn vbl(&mut self) -> VBL_W {
        VBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HAU configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
