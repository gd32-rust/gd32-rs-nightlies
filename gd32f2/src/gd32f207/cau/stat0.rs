#[doc = "Register `STAT0` reader"]
pub struct R(crate::R<STAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - BUSY flag"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `OFU` reader - OUT FIFO full flag"]
pub type OFU_R = crate::BitReader<bool>;
#[doc = "Field `ONE` reader - OUT FIFO not empty flag"]
pub type ONE_R = crate::BitReader<bool>;
#[doc = "Field `INF` reader - IN FIFO not full flag"]
pub type INF_R = crate::BitReader<bool>;
#[doc = "Field `IEM` reader - IN FIFO empty flag"]
pub type IEM_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 4 - BUSY flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - OUT FIFO full flag"]
    #[inline(always)]
    pub fn ofu(&self) -> OFU_R {
        OFU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - OUT FIFO not empty flag"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - IN FIFO not full flag"]
    #[inline(always)]
    pub fn inf(&self) -> INF_R {
        INF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - IN FIFO empty flag"]
    #[inline(always)]
    pub fn iem(&self) -> IEM_R {
        IEM_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CAU status register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat0](index.html) module"]
pub struct STAT0_SPEC;
impl crate::RegisterSpec for STAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat0::R](R) reader structure"]
impl crate::Readable for STAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT0 to value 0x03"]
impl crate::Resettable for STAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
