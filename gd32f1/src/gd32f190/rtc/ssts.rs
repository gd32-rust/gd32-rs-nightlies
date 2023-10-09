#[doc = "Register `SSTS` reader"]
pub type R = crate::R<SSTS_SPEC>;
#[doc = "Field `SSC` reader - Sub second value"]
pub type SSC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Sub second of time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSTS_SPEC;
impl crate::RegisterSpec for SSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssts::R`](R) reader structure"]
impl crate::Readable for SSTS_SPEC {}
#[doc = "`reset()` method sets SSTS to value 0"]
impl crate::Resettable for SSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
