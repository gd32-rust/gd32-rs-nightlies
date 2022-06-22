#[doc = "Register `STAT1` reader"]
pub struct R(crate::R<STAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OSTA` reader - Out FIFO interrupt flag"]
pub type OSTA_R = crate::BitReader<bool>;
#[doc = "Field `ISTA` reader - In FIFO interrupt flag"]
pub type ISTA_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Out FIFO interrupt flag"]
    #[inline(always)]
    pub fn osta(&self) -> OSTA_R {
        OSTA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - In FIFO interrupt flag"]
    #[inline(always)]
    pub fn ista(&self) -> ISTA_R {
        ISTA_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CAU interrupt status flag register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat1](index.html) module"]
pub struct STAT1_SPEC;
impl crate::RegisterSpec for STAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat1::R](R) reader structure"]
impl crate::Readable for STAT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT1 to value 0x01"]
impl crate::Resettable for STAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
