#[doc = "Register `INTF` reader"]
pub struct R(crate::R<INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LCRF` reader - Layer Configuration Reloaded Flag"]
pub type LCRF_R = crate::BitReader<bool>;
#[doc = "Field `TEF` reader - Transaction Error Flag"]
pub type TEF_R = crate::BitReader<bool>;
#[doc = "Field `FEF` reader - FIFO Error Flag"]
pub type FEF_R = crate::BitReader<bool>;
#[doc = "Field `LMF` reader - Line Mark Flag"]
pub type LMF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 3 - Layer Configuration Reloaded Flag"]
    #[inline(always)]
    pub fn lcrf(&self) -> LCRF_R {
        LCRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transaction Error Flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Line Mark Flag"]
    #[inline(always)]
    pub fn lmf(&self) -> LMF_R {
        LMF_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](index.html) module"]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intf::R](R) reader structure"]
impl crate::Readable for INTF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
