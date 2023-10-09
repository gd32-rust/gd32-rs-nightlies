#[doc = "Register `MSC_MSCCNT` reader"]
pub type R = crate::R<MSC_MSCCNT_SPEC>;
#[doc = "Field `MSCC` reader - Transmitted good frames after more than a single collision counter"]
pub type MSCC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames after more than a single collision counter"]
    #[inline(always)]
    pub fn mscc(&self) -> MSCC_R {
        MSCC_R::new(self.bits)
    }
}
#[doc = "Ethernet MSC transmitted good frames after more than a single collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_msccnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSC_MSCCNT_SPEC;
impl crate::RegisterSpec for MSC_MSCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_msccnt::R`](R) reader structure"]
impl crate::Readable for MSC_MSCCNT_SPEC {}
#[doc = "`reset()` method sets MSC_MSCCNT to value 0"]
impl crate::Resettable for MSC_MSCCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
