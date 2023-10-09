#[doc = "Register `DO` reader"]
pub type R = crate::R<DO_SPEC>;
#[doc = "Field `DO` reader - Data output"]
pub type DO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data output"]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(self.bits)
    }
}
#[doc = "CAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DO_SPEC;
impl crate::RegisterSpec for DO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do_::R`](R) reader structure"]
impl crate::Readable for DO_SPEC {}
#[doc = "`reset()` method sets DO to value 0"]
impl crate::Resettable for DO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
