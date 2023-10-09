#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Field `LMF` reader - Line Mark Flag"]
pub type LMF_R = crate::BitReader;
#[doc = "Field `FEF` reader - FIFO Error Flag"]
pub type FEF_R = crate::BitReader;
#[doc = "Field `TEF` reader - Transaction Error Flag"]
pub type TEF_R = crate::BitReader;
#[doc = "Field `LCRF` reader - Layer Configuration Reloaded Flag"]
pub type LCRF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Line Mark Flag"]
    #[inline(always)]
    pub fn lmf(&self) -> LMF_R {
        LMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transaction Error Flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Layer Configuration Reloaded Flag"]
    #[inline(always)]
    pub fn lcrf(&self) -> LCRF_R {
        LCRF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for INTF_SPEC {}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
