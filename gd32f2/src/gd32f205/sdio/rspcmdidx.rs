#[doc = "Register `RSPCMDIDX` reader"]
pub type R = crate::R<RSPCMDIDX_SPEC>;
#[doc = "Field `RSPCMDIDX` reader - Last response command index"]
pub type RSPCMDIDX_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Last response command index"]
    #[inline(always)]
    pub fn rspcmdidx(&self) -> RSPCMDIDX_R {
        RSPCMDIDX_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Command index response register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspcmdidx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSPCMDIDX_SPEC;
impl crate::RegisterSpec for RSPCMDIDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rspcmdidx::R`](R) reader structure"]
impl crate::Readable for RSPCMDIDX_SPEC {}
#[doc = "`reset()` method sets RSPCMDIDX to value 0"]
impl crate::Resettable for RSPCMDIDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
