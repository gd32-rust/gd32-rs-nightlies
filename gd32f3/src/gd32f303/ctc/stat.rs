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
#[doc = "Field `CKOKIF` reader - Clock trim OK interrupt flag"]
pub type CKOKIF_R = crate::BitReader<bool>;
#[doc = "Field `CKWARNIF` reader - Clock trim warning interrupt flag"]
pub type CKWARNIF_R = crate::BitReader<bool>;
#[doc = "Field `ERRIF` reader - Error interrupt flag"]
pub type ERRIF_R = crate::BitReader<bool>;
#[doc = "Field `EREFIF` reader - Expect reference interrupt flag"]
pub type EREFIF_R = crate::BitReader<bool>;
#[doc = "Field `CKERR` reader - Clock trim error bit"]
pub type CKERR_R = crate::BitReader<bool>;
#[doc = "Field `REFMISS` reader - Reference sync pulse miss"]
pub type REFMISS_R = crate::BitReader<bool>;
#[doc = "Field `TRIMERR` reader - Trim value error bit"]
pub type TRIMERR_R = crate::BitReader<bool>;
#[doc = "Field `REFDIR` reader - CTC trim counter direction when reference sync pulse"]
pub type REFDIR_R = crate::BitReader<bool>;
#[doc = "Field `REFCAP` reader - CTC counter capture when reference sync pulse"]
pub type REFCAP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Clock trim OK interrupt flag"]
    #[inline(always)]
    pub fn ckokif(&self) -> CKOKIF_R {
        CKOKIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock trim warning interrupt flag"]
    #[inline(always)]
    pub fn ckwarnif(&self) -> CKWARNIF_R {
        CKWARNIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&self) -> ERRIF_R {
        ERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expect reference interrupt flag"]
    #[inline(always)]
    pub fn erefif(&self) -> EREFIF_R {
        EREFIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock trim error bit"]
    #[inline(always)]
    pub fn ckerr(&self) -> CKERR_R {
        CKERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reference sync pulse miss"]
    #[inline(always)]
    pub fn refmiss(&self) -> REFMISS_R {
        REFMISS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Trim value error bit"]
    #[inline(always)]
    pub fn trimerr(&self) -> TRIMERR_R {
        TRIMERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - CTC trim counter direction when reference sync pulse"]
    #[inline(always)]
    pub fn refdir(&self) -> REFDIR_R {
        REFDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - CTC counter capture when reference sync pulse"]
    #[inline(always)]
    pub fn refcap(&self) -> REFCAP_R {
        REFCAP_R::new(((self.bits >> 16) & 0xffff) as u16)
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
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
