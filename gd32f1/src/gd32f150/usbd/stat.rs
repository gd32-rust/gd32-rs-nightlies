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
#[doc = "Field `FCNT` reader - Frame number counter"]
pub type FCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SOFLN` reader - SOF lost number"]
pub type SOFLN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK` reader - Locked the USB"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `RX_DM` reader - Receive data - line status"]
pub type RX_DM_R = crate::BitReader<bool>;
#[doc = "Field `RX_DP` reader - Receive data + line status"]
pub type RX_DP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:10 - Frame number counter"]
    #[inline(always)]
    pub fn fcnt(&self) -> FCNT_R {
        FCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - SOF lost number"]
    #[inline(always)]
    pub fn sofln(&self) -> SOFLN_R {
        SOFLN_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Locked the USB"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive data - line status"]
    #[inline(always)]
    pub fn rx_dm(&self) -> RX_DM_R {
        RX_DM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive data + line status"]
    #[inline(always)]
    pub fn rx_dp(&self) -> RX_DP_R {
        RX_DP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
