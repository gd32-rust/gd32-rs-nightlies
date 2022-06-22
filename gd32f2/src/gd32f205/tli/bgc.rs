#[doc = "Register `BGC` reader"]
pub struct R(crate::R<BGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGC` writer"]
pub struct W(crate::W<BGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGC_SPEC>;
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
impl From<crate::W<BGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BVR` reader - Background value red"]
pub type BVR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BVR` writer - Background value red"]
pub type BVR_W<'a> = crate::FieldWriter<'a, u32, BGC_SPEC, u8, u8, 8, 16>;
#[doc = "Field `BVG` reader - Background value green"]
pub type BVG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BVG` writer - Background value green"]
pub type BVG_W<'a> = crate::FieldWriter<'a, u32, BGC_SPEC, u8, u8, 8, 8>;
#[doc = "Field `BVB` reader - Background value blue"]
pub type BVB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BVB` writer - Background value blue"]
pub type BVB_W<'a> = crate::FieldWriter<'a, u32, BGC_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 16:23 - Background value red"]
    #[inline(always)]
    pub fn bvr(&self) -> BVR_R {
        BVR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Background value green"]
    #[inline(always)]
    pub fn bvg(&self) -> BVG_R {
        BVG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Background value blue"]
    #[inline(always)]
    pub fn bvb(&self) -> BVB_R {
        BVB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Background value red"]
    #[inline(always)]
    pub fn bvr(&mut self) -> BVR_W {
        BVR_W::new(self)
    }
    #[doc = "Bits 8:15 - Background value green"]
    #[inline(always)]
    pub fn bvg(&mut self) -> BVG_W {
        BVG_W::new(self)
    }
    #[doc = "Bits 0:7 - Background value blue"]
    #[inline(always)]
    pub fn bvb(&mut self) -> BVB_W {
        BVB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Background color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgc](index.html) module"]
pub struct BGC_SPEC;
impl crate::RegisterSpec for BGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgc::R](R) reader structure"]
impl crate::Readable for BGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgc::W](W) writer structure"]
impl crate::Writable for BGC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGC to value 0"]
impl crate::Resettable for BGC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
