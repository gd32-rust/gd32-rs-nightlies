#[doc = "Register `RESP3` reader"]
pub type R = crate::R<RESP3_SPEC>;
#[doc = "Field `RESP3` reader - Card state"]
pub type RESP3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Card state"]
    #[inline(always)]
    pub fn resp3(&self) -> RESP3_R {
        RESP3_R::new(self.bits)
    }
}
#[doc = "Response register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP3_SPEC;
impl crate::RegisterSpec for RESP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp3::R`](R) reader structure"]
impl crate::Readable for RESP3_SPEC {}
#[doc = "`reset()` method sets RESP3 to value 0"]
impl crate::Resettable for RESP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
