#[doc = "Register `DO7` reader"]
pub type R = crate::R<DO7_SPEC>;
#[doc = "Field `DO7` reader - message digest result of hash algorithm"]
pub type DO7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do7(&self) -> DO7_R {
        DO7_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DO7_SPEC;
impl crate::RegisterSpec for DO7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do7::R`](R) reader structure"]
impl crate::Readable for DO7_SPEC {}
#[doc = "`reset()` method sets DO7 to value 0"]
impl crate::Resettable for DO7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
