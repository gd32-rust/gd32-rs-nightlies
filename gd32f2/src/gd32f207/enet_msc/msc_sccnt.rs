#[doc = "Register `MSC_SCCNT` reader"]
pub type R = crate::R<MSC_SCCNT_SPEC>;
#[doc = "Field `SCC` reader - Transmitted good frames after a single collision counter"]
pub type SCC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames after a single collision counter"]
    #[inline(always)]
    pub fn scc(&self) -> SCC_R {
        SCC_R::new(self.bits)
    }
}
#[doc = "Ethernet MSC transmitted good frames after a single collision counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_sccnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSC_SCCNT_SPEC;
impl crate::RegisterSpec for MSC_SCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_sccnt::R`](R) reader structure"]
impl crate::Readable for MSC_SCCNT_SPEC {}
#[doc = "`reset()` method sets MSC_SCCNT to value 0"]
impl crate::Resettable for MSC_SCCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
