#[doc = "Register `DO3` reader"]
pub type R = crate::R<DO3_SPEC>;
#[doc = "Field `DO3` reader - message digest result of hash algorithm"]
pub type DO3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do3(&self) -> DO3_R {
        DO3_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DO3_SPEC;
impl crate::RegisterSpec for DO3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do3::R`](R) reader structure"]
impl crate::Readable for DO3_SPEC {}
#[doc = "`reset()` method sets DO3 to value 0"]
impl crate::Resettable for DO3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
