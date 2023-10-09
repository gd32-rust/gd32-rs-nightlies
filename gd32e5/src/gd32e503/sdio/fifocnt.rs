#[doc = "Register `FIFOCNT` reader"]
pub type R = crate::R<FIFOCNT_SPEC>;
#[doc = "Field `FIFOCNT` reader - FIFO counter"]
pub type FIFOCNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - FIFO counter"]
    #[inline(always)]
    pub fn fifocnt(&self) -> FIFOCNT_R {
        FIFOCNT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "FIFO counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifocnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFOCNT_SPEC;
impl crate::RegisterSpec for FIFOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifocnt::R`](R) reader structure"]
impl crate::Readable for FIFOCNT_SPEC {}
#[doc = "`reset()` method sets FIFOCNT to value 0"]
impl crate::Resettable for FIFOCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
