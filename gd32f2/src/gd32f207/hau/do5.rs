#[doc = "Register `DO5` reader"]
pub type R = crate::R<DO5_SPEC>;
#[doc = "Field `DO5` reader - message digest result of hash algorithm"]
pub type DO5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do5(&self) -> DO5_R {
        DO5_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DO5_SPEC;
impl crate::RegisterSpec for DO5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do5::R`](R) reader structure"]
impl crate::Readable for DO5_SPEC {}
#[doc = "`reset()` method sets DO5 to value 0"]
impl crate::Resettable for DO5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
