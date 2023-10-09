#[doc = "Register `SS` reader"]
pub type R = crate::R<SS_SPEC>;
#[doc = "Field `SSC` reader - Sub second value"]
pub type SSC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SS_SPEC;
impl crate::RegisterSpec for SS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss::R`](R) reader structure"]
impl crate::Readable for SS_SPEC {}
#[doc = "`reset()` method sets SS to value 0"]
impl crate::Resettable for SS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
