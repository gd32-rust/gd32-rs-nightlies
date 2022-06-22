#[doc = "Register `CPPOS` reader"]
pub struct R(crate::R<CPPOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPPOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPPOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPPOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HPOS` reader - Horizontal position"]
pub type HPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VPOS` reader - Vertical position"]
pub type VPOS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - Horizontal position"]
    #[inline(always)]
    pub fn hpos(&self) -> HPOS_R {
        HPOS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Vertical position"]
    #[inline(always)]
    pub fn vpos(&self) -> VPOS_R {
        VPOS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Current pixel position register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cppos](index.html) module"]
pub struct CPPOS_SPEC;
impl crate::RegisterSpec for CPPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cppos::R](R) reader structure"]
impl crate::Readable for CPPOS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPPOS to value 0"]
impl crate::Resettable for CPPOS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
