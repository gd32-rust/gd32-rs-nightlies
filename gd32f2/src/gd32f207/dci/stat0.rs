#[doc = "Register `STAT0` reader"]
pub type R = crate::R<STAT0_SPEC>;
#[doc = "Field `HS` reader - HS line status"]
pub type HS_R = crate::BitReader;
#[doc = "Field `VS` reader - VS line status"]
pub type VS_R = crate::BitReader;
#[doc = "Field `FV` reader - FIFO Valid"]
pub type FV_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HS line status"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VS line status"]
    #[inline(always)]
    pub fn vs(&self) -> VS_R {
        VS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Valid"]
    #[inline(always)]
    pub fn fv(&self) -> FV_R {
        FV_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "DCI Status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT0_SPEC;
impl crate::RegisterSpec for STAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat0::R`](R) reader structure"]
impl crate::Readable for STAT0_SPEC {}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for STAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
