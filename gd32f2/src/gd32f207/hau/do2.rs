#[doc = "Register `DO2` reader"]
pub type R = crate::R<DO2_SPEC>;
#[doc = "Field `DO2` reader - message digest result of hash algorithm"]
pub type DO2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do2(&self) -> DO2_R {
        DO2_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DO2_SPEC;
impl crate::RegisterSpec for DO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do2::R`](R) reader structure"]
impl crate::Readable for DO2_SPEC {}
#[doc = "`reset()` method sets DO2 to value 0"]
impl crate::Resettable for DO2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
