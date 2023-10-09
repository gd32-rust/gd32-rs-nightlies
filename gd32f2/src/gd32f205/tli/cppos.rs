#[doc = "Register `CPPOS` reader"]
pub type R = crate::R<CPPOS_SPEC>;
#[doc = "Field `VPOS` reader - Vertical position"]
pub type VPOS_R = crate::FieldReader<u16>;
#[doc = "Field `HPOS` reader - Horizontal position"]
pub type HPOS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Vertical position"]
    #[inline(always)]
    pub fn vpos(&self) -> VPOS_R {
        VPOS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Horizontal position"]
    #[inline(always)]
    pub fn hpos(&self) -> HPOS_R {
        HPOS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Current pixel position register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cppos::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPPOS_SPEC;
impl crate::RegisterSpec for CPPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cppos::R`](R) reader structure"]
impl crate::Readable for CPPOS_SPEC {}
#[doc = "`reset()` method sets CPPOS to value 0"]
impl crate::Resettable for CPPOS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
