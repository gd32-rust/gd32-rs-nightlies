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
#[doc = "Field `ELF` reader - End of line flag"]
pub type ELF_R = crate::BitReader<bool>;
#[doc = "Field `VSF` reader - Vsync Flag"]
pub type VSF_R = crate::BitReader<bool>;
#[doc = "Field `ESEF` reader - Embedded Synchronous Error Flag"]
pub type ESEF_R = crate::BitReader<bool>;
#[doc = "Field `OVRF` reader - FIFO Overrun flag"]
pub type OVRF_R = crate::BitReader<bool>;
#[doc = "Field `EFF` reader - End of Frame Flag"]
pub type EFF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 4 - End of line flag"]
    #[inline(always)]
    pub fn elf(&self) -> ELF_R {
        ELF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Vsync Flag"]
    #[inline(always)]
    pub fn vsf(&self) -> VSF_R {
        VSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded Synchronous Error Flag"]
    #[inline(always)]
    pub fn esef(&self) -> ESEF_R {
        ESEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Overrun flag"]
    #[inline(always)]
    pub fn ovrf(&self) -> OVRF_R {
        OVRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - End of Frame Flag"]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DCI Status register1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat1](index.html) module"]
pub struct STAT1_SPEC;
impl crate::RegisterSpec for STAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat1::R](R) reader structure"]
impl crate::Readable for STAT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT1 to value 0"]
impl crate::Resettable for STAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
