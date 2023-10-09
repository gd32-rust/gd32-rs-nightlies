#[doc = "Register `DO4` reader"]
pub type R = crate::R<DO4_SPEC>;
#[doc = "Field `DO4` reader - message digest result of hash algorithm"]
pub type DO4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do4(&self) -> DO4_R {
        DO4_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DO4_SPEC;
impl crate::RegisterSpec for DO4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do4::R`](R) reader structure"]
impl crate::Readable for DO4_SPEC {}
#[doc = "`reset()` method sets DO4 to value 0"]
impl crate::Resettable for DO4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
