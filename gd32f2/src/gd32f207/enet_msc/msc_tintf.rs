#[doc = "Register `MSC_TINTF` reader"]
pub type R = crate::R<MSC_TINTF_SPEC>;
#[doc = "Field `TGFSC` reader - Transmitted good frames single collision"]
pub type TGFSC_R = crate::BitReader;
#[doc = "Field `TGFMSC` reader - Transmitted good frames more single collision"]
pub type TGFMSC_R = crate::BitReader;
#[doc = "Field `TGF` reader - Transmitted good frames"]
pub type TGF_R = crate::BitReader;
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision"]
    #[inline(always)]
    pub fn tgfsc(&self) -> TGFSC_R {
        TGFSC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision"]
    #[inline(always)]
    pub fn tgfmsc(&self) -> TGFMSC_R {
        TGFMSC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames"]
    #[inline(always)]
    pub fn tgf(&self) -> TGF_R {
        TGF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Ethernet MSC transmit interrupt flag register (MSC_TINTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_tintf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSC_TINTF_SPEC;
impl crate::RegisterSpec for MSC_TINTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_tintf::R`](R) reader structure"]
impl crate::Readable for MSC_TINTF_SPEC {}
#[doc = "`reset()` method sets MSC_TINTF to value 0"]
impl crate::Resettable for MSC_TINTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
