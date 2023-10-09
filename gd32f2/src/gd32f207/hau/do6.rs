#[doc = "Register `DO6` reader"]
pub type R = crate::R<DO6_SPEC>;
#[doc = "Field `DO6` reader - message digest result of hash algorithm"]
pub type DO6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do6(&self) -> DO6_R {
        DO6_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DO6_SPEC;
impl crate::RegisterSpec for DO6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do6::R`](R) reader structure"]
impl crate::Readable for DO6_SPEC {}
#[doc = "`reset()` method sets DO6 to value 0"]
impl crate::Resettable for DO6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
