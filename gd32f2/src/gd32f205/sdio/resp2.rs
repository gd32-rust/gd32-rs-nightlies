#[doc = "Register `RESP2` reader"]
pub type R = crate::R<RESP2_SPEC>;
#[doc = "Field `RESP2` reader - Card state"]
pub type RESP2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Card state"]
    #[inline(always)]
    pub fn resp2(&self) -> RESP2_R {
        RESP2_R::new(self.bits)
    }
}
#[doc = "Response register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP2_SPEC;
impl crate::RegisterSpec for RESP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp2::R`](R) reader structure"]
impl crate::Readable for RESP2_SPEC {}
#[doc = "`reset()` method sets RESP2 to value 0"]
impl crate::Resettable for RESP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
