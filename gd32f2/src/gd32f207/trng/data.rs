#[doc = "Register `DATA` reader"]
pub type R = crate::R<DATA_SPEC>;
#[doc = "Field `TRNDATA` reader - 32-bit random data"]
pub type TRNDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit random data"]
    #[inline(always)]
    pub fn trndata(&self) -> TRNDATA_R {
        TRNDATA_R::new(self.bits)
    }
}
#[doc = "data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DATA_SPEC {}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
