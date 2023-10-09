#[doc = "Register `PEC` reader"]
pub type R = crate::R<PEC_SPEC>;
#[doc = "Field `PECV` reader - Packet Error Checking Value"]
pub type PECV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Packet Error Checking Value"]
    #[inline(always)]
    pub fn pecv(&self) -> PECV_R {
        PECV_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Packet Error Checking\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEC_SPEC;
impl crate::RegisterSpec for PEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pec::R`](R) reader structure"]
impl crate::Readable for PEC_SPEC {}
#[doc = "`reset()` method sets PEC to value 0"]
impl crate::Resettable for PEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
