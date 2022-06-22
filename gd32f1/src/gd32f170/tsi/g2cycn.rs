#[doc = "Register `G2CYCN` reader"]
pub struct R(crate::R<G2CYCN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<G2CYCN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<G2CYCN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<G2CYCN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CYCN` reader - Cycle number"]
pub type CYCN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - Cycle number"]
    #[inline(always)]
    pub fn cycn(&self) -> CYCN_R {
        CYCN_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "I/O group x cycle number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [g2cycn](index.html) module"]
pub struct G2CYCN_SPEC;
impl crate::RegisterSpec for G2CYCN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [g2cycn::R](R) reader structure"]
impl crate::Readable for G2CYCN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets G2CYCN to value 0"]
impl crate::Resettable for G2CYCN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
