#[doc = "Register `DATACNT` reader"]
pub type R = crate::R<DATACNT_SPEC>;
#[doc = "Field `DATACNT` reader - Data count value"]
pub type DATACNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value"]
    #[inline(always)]
    pub fn datacnt(&self) -> DATACNT_R {
        DATACNT_R::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "Data counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datacnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATACNT_SPEC;
impl crate::RegisterSpec for DATACNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datacnt::R`](R) reader structure"]
impl crate::Readable for DATACNT_SPEC {}
#[doc = "`reset()` method sets DATACNT to value 0"]
impl crate::Resettable for DATACNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
