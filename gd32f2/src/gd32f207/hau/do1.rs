#[doc = "Register `DO1` reader"]
pub type R = crate::R<DO1_SPEC>;
#[doc = "Field `DO1` reader - message digest result of hash algorithm"]
pub type DO1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do1(&self) -> DO1_R {
        DO1_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DO1_SPEC;
impl crate::RegisterSpec for DO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do1::R`](R) reader structure"]
impl crate::Readable for DO1_SPEC {}
#[doc = "`reset()` method sets DO1 to value 0"]
impl crate::Resettable for DO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
