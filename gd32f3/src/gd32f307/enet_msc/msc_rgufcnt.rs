#[doc = "Register `MSC_RGUFCNT` reader"]
pub type R = crate::R<MSC_RGUFCNT_SPEC>;
#[doc = "Field `RGUF` reader - Received good unicast frames counter"]
pub type RGUF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received good unicast frames counter"]
    #[inline(always)]
    pub fn rguf(&self) -> RGUF_R {
        RGUF_R::new(self.bits)
    }
}
#[doc = "MSC received good unicast frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_rgufcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSC_RGUFCNT_SPEC;
impl crate::RegisterSpec for MSC_RGUFCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_rgufcnt::R`](R) reader structure"]
impl crate::Readable for MSC_RGUFCNT_SPEC {}
#[doc = "`reset()` method sets MSC_RGUFCNT to value 0"]
impl crate::Resettable for MSC_RGUFCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
