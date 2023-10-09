#[doc = "Register `DO0` reader"]
pub type R = crate::R<DO0_SPEC>;
#[doc = "Field `DO0` reader - message digest result of hash algorithm"]
pub type DO0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do0(&self) -> DO0_R {
        DO0_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DO0_SPEC;
impl crate::RegisterSpec for DO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do0::R`](R) reader structure"]
impl crate::Readable for DO0_SPEC {}
#[doc = "`reset()` method sets DO0 to value 0"]
impl crate::Resettable for DO0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
