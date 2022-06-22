#[doc = "Register `SPSZ` reader"]
pub struct R(crate::R<SPSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPSZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPSZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPSZ` writer"]
pub struct W(crate::W<SPSZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPSZ_SPEC>;
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
impl From<crate::W<SPSZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPSZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VPSZ` reader - size of vertical synchronous pluse"]
pub type VPSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VPSZ` writer - size of vertical synchronous pluse"]
pub type VPSZ_W<'a> = crate::FieldWriter<'a, u32, SPSZ_SPEC, u16, u16, 12, 0>;
#[doc = "Field `HPSZ` reader - size of horizontal synchronous pluse"]
pub type HPSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPSZ` writer - size of horizontal synchronous pluse"]
pub type HPSZ_W<'a> = crate::FieldWriter<'a, u32, SPSZ_SPEC, u16, u16, 12, 16>;
impl R {
    #[doc = "Bits 0:11 - size of vertical synchronous pluse"]
    #[inline(always)]
    pub fn vpsz(&self) -> VPSZ_R {
        VPSZ_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - size of horizontal synchronous pluse"]
    #[inline(always)]
    pub fn hpsz(&self) -> HPSZ_R {
        HPSZ_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - size of vertical synchronous pluse"]
    #[inline(always)]
    pub fn vpsz(&mut self) -> VPSZ_W {
        VPSZ_W::new(self)
    }
    #[doc = "Bits 16:27 - size of horizontal synchronous pluse"]
    #[inline(always)]
    pub fn hpsz(&mut self) -> HPSZ_W {
        HPSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous pulse size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spsz](index.html) module"]
pub struct SPSZ_SPEC;
impl crate::RegisterSpec for SPSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spsz::R](R) reader structure"]
impl crate::Readable for SPSZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spsz::W](W) writer structure"]
impl crate::Writable for SPSZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPSZ to value 0"]
impl crate::Resettable for SPSZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
