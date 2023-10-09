#[doc = "Register `STAT1` reader"]
pub type R = crate::R<STAT1_SPEC>;
#[doc = "Field `ISTA` reader - In FIFO interrupt flag"]
pub type ISTA_R = crate::BitReader;
#[doc = "Field `OSTA` reader - Out FIFO interrupt flag"]
pub type OSTA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - In FIFO interrupt flag"]
    #[inline(always)]
    pub fn ista(&self) -> ISTA_R {
        ISTA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Out FIFO interrupt flag"]
    #[inline(always)]
    pub fn osta(&self) -> OSTA_R {
        OSTA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "CAU interrupt status flag register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT1_SPEC;
impl crate::RegisterSpec for STAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat1::R`](R) reader structure"]
impl crate::Readable for STAT1_SPEC {}
#[doc = "`reset()` method sets STAT1 to value 0x01"]
impl crate::Resettable for STAT1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
