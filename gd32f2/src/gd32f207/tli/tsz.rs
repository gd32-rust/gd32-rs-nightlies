#[doc = "Register `TSZ` reader"]
pub struct R(crate::R<TSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSZ` writer"]
pub struct W(crate::W<TSZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSZ_SPEC>;
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
impl From<crate::W<TSZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTSZ` reader - Vertical total size of the display"]
pub type VTSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VTSZ` writer - Vertical total size of the display"]
pub type VTSZ_W<'a> = crate::FieldWriter<'a, u32, TSZ_SPEC, u16, u16, 12, 0>;
#[doc = "Field `HTSZ` reader - Horizontal total size of the display"]
pub type HTSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HTSZ` writer - Horizontal total size of the display"]
pub type HTSZ_W<'a> = crate::FieldWriter<'a, u32, TSZ_SPEC, u16, u16, 12, 16>;
impl R {
    #[doc = "Bits 0:11 - Vertical total size of the display"]
    #[inline(always)]
    pub fn vtsz(&self) -> VTSZ_R {
        VTSZ_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Horizontal total size of the display"]
    #[inline(always)]
    pub fn htsz(&self) -> HTSZ_R {
        HTSZ_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Vertical total size of the display"]
    #[inline(always)]
    pub fn vtsz(&mut self) -> VTSZ_W {
        VTSZ_W::new(self)
    }
    #[doc = "Bits 16:27 - Horizontal total size of the display"]
    #[inline(always)]
    pub fn htsz(&mut self) -> HTSZ_W {
        HTSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Total size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsz](index.html) module"]
pub struct TSZ_SPEC;
impl crate::RegisterSpec for TSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsz::R](R) reader structure"]
impl crate::Readable for TSZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsz::W](W) writer structure"]
impl crate::Writable for TSZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSZ to value 0"]
impl crate::Resettable for TSZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
