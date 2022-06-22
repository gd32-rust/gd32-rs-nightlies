#[doc = "Register `RT` reader"]
pub struct R(crate::R<RT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RT` writer"]
pub struct W(crate::W<RT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RT_SPEC>;
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
impl From<crate::W<RT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BL` reader - Block Length"]
pub type BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BL` writer - Block Length"]
pub type BL_W<'a> = crate::FieldWriterSafe<'a, u32, RT_SPEC, u8, u8, 8, 24>;
#[doc = "Field `RT` reader - Receiver timeout threshold"]
pub type RT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RT` writer - Receiver timeout threshold"]
pub type RT_W<'a> = crate::FieldWriterSafe<'a, u32, RT_SPEC, u32, u32, 24, 0>;
impl R {
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 0:23 - Receiver timeout threshold"]
    #[inline(always)]
    pub fn rt(&self) -> RT_R {
        RT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W {
        BL_W::new(self)
    }
    #[doc = "Bits 0:23 - Receiver timeout threshold"]
    #[inline(always)]
    pub fn rt(&mut self) -> RT_W {
        RT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rt](index.html) module"]
pub struct RT_SPEC;
impl crate::RegisterSpec for RT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rt::R](R) reader structure"]
impl crate::Readable for RT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rt::W](W) writer structure"]
impl crate::Writable for RT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RT to value 0"]
impl crate::Resettable for RT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
