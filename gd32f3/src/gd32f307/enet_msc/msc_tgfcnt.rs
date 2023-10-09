#[doc = "Register `MSC_TGFCNT` reader"]
pub type R = crate::R<MSC_TGFCNT_SPEC>;
#[doc = "Field `TGF` reader - Transmitted good frames counter"]
pub type TGF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames counter"]
    #[inline(always)]
    pub fn tgf(&self) -> TGF_R {
        TGF_R::new(self.bits)
    }
}
#[doc = "Ethernet MSC transmitted good frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_tgfcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSC_TGFCNT_SPEC;
impl crate::RegisterSpec for MSC_TGFCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_tgfcnt::R`](R) reader structure"]
impl crate::Readable for MSC_TGFCNT_SPEC {}
#[doc = "`reset()` method sets MSC_TGFCNT to value 0"]
impl crate::Resettable for MSC_TGFCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
