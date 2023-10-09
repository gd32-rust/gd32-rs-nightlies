#[doc = "Register `G4CYCN` reader"]
pub type R = crate::R<G4CYCN_SPEC>;
#[doc = "Field `CYCN` reader - Cycle number"]
pub type CYCN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Cycle number"]
    #[inline(always)]
    pub fn cycn(&self) -> CYCN_R {
        CYCN_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g4cycn::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct G4CYCN_SPEC;
impl crate::RegisterSpec for G4CYCN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`g4cycn::R`](R) reader structure"]
impl crate::Readable for G4CYCN_SPEC {}
#[doc = "`reset()` method sets G4CYCN to value 0"]
impl crate::Resettable for G4CYCN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
