#[doc = "Register `CMDAGMT` reader"]
pub struct R(crate::R<CMDAGMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDAGMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDAGMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDAGMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDAGMT` writer"]
pub struct W(crate::W<CMDAGMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDAGMT_SPEC>;
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
impl From<crate::W<CMDAGMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDAGMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDAGMT` reader - SDIO card command argument"]
pub type CMDAGMT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMDAGMT` writer - SDIO card command argument"]
pub type CMDAGMT_W<'a> = crate::FieldWriter<'a, u32, CMDAGMT_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - SDIO card command argument"]
    #[inline(always)]
    pub fn cmdagmt(&self) -> CMDAGMT_R {
        CMDAGMT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SDIO card command argument"]
    #[inline(always)]
    pub fn cmdagmt(&mut self) -> CMDAGMT_W {
        CMDAGMT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdagmt](index.html) module"]
pub struct CMDAGMT_SPEC;
impl crate::RegisterSpec for CMDAGMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdagmt::R](R) reader structure"]
impl crate::Readable for CMDAGMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdagmt::W](W) writer structure"]
impl crate::Writable for CMDAGMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDAGMT to value 0"]
impl crate::Resettable for CMDAGMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
