#[doc = "Register `TDATA` reader"]
pub struct R(crate::R<TDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TDATA` writer"]
pub struct W(crate::W<TDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDATA_SPEC>;
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
impl From<crate::W<TDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDATA` reader - Transmit data value"]
pub type TDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TDATA` writer - Transmit data value"]
pub type TDATA_W<'a> = crate::FieldWriter<'a, u32, TDATA_SPEC, u16, u16, 9, 0>;
impl R {
    #[doc = "Bits 0:8 - Transmit data value"]
    #[inline(always)]
    pub fn tdata(&self) -> TDATA_R {
        TDATA_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit data value"]
    #[inline(always)]
    pub fn tdata(&mut self) -> TDATA_W {
        TDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdata](index.html) module"]
pub struct TDATA_SPEC;
impl crate::RegisterSpec for TDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdata::R](R) reader structure"]
impl crate::Readable for TDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tdata::W](W) writer structure"]
impl crate::Writable for TDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TDATA to value 0"]
impl crate::Resettable for TDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
