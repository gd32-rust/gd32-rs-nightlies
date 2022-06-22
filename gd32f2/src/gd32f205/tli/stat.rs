#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HS` reader - Current HS staus of the TLI"]
pub type HS_R = crate::BitReader<bool>;
#[doc = "Field `VS` reader - Current VS staus of the TLI"]
pub type VS_R = crate::BitReader<bool>;
#[doc = "Field `HDE` reader - Current HDE status"]
pub type HDE_R = crate::BitReader<bool>;
#[doc = "Field `VDE` reader - Current VDE status"]
pub type VDE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 3 - Current HS staus of the TLI"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Current VS staus of the TLI"]
    #[inline(always)]
    pub fn vs(&self) -> VS_R {
        VS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Current HDE status"]
    #[inline(always)]
    pub fn hde(&self) -> HDE_R {
        HDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Current VDE status"]
    #[inline(always)]
    pub fn vde(&self) -> VDE_R {
        VDE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0x0f"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
