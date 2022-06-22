#[doc = "Register `BPSZ` reader"]
pub struct R(crate::R<BPSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPSZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPSZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPSZ` writer"]
pub struct W(crate::W<BPSZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPSZ_SPEC>;
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
impl From<crate::W<BPSZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPSZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBPSZ` reader - Size of the vertical back porch plus synchronous pulse"]
pub type VBPSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VBPSZ` writer - Size of the vertical back porch plus synchronous pulse"]
pub type VBPSZ_W<'a> = crate::FieldWriter<'a, u32, BPSZ_SPEC, u16, u16, 12, 0>;
#[doc = "Field `HBPSZ` reader - Size of the horizontal back porch plus synchronous pulse"]
pub type HBPSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HBPSZ` writer - Size of the horizontal back porch plus synchronous pulse"]
pub type HBPSZ_W<'a> = crate::FieldWriter<'a, u32, BPSZ_SPEC, u16, u16, 12, 16>;
impl R {
    #[doc = "Bits 0:11 - Size of the vertical back porch plus synchronous pulse"]
    #[inline(always)]
    pub fn vbpsz(&self) -> VBPSZ_R {
        VBPSZ_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Size of the horizontal back porch plus synchronous pulse"]
    #[inline(always)]
    pub fn hbpsz(&self) -> HBPSZ_R {
        HBPSZ_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Size of the vertical back porch plus synchronous pulse"]
    #[inline(always)]
    pub fn vbpsz(&mut self) -> VBPSZ_W {
        VBPSZ_W::new(self)
    }
    #[doc = "Bits 16:27 - Size of the horizontal back porch plus synchronous pulse"]
    #[inline(always)]
    pub fn hbpsz(&mut self) -> HBPSZ_W {
        HBPSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Back-porch size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpsz](index.html) module"]
pub struct BPSZ_SPEC;
impl crate::RegisterSpec for BPSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpsz::R](R) reader structure"]
impl crate::Readable for BPSZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpsz::W](W) writer structure"]
impl crate::Writable for BPSZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPSZ to value 0"]
impl crate::Resettable for BPSZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
