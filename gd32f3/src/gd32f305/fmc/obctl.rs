#[doc = "Register `OBCTL` reader"]
pub struct R(crate::R<OBCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RERR` reader - Option bytes read error bit"]
pub type RERR_R = crate::BitReader<bool>;
#[doc = "Field `SPC` reader - Option bytes security protection code"]
pub type SPC_R = crate::BitReader<bool>;
#[doc = "Field `USER` reader - Store USER of option bytes block after system reset"]
pub type USER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` reader - Store DATA\\[15:0\\]
of option bytes block after system reset"]
pub type DATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Option bytes read error bit"]
    #[inline(always)]
    pub fn rerr(&self) -> RERR_R {
        RERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option bytes security protection code"]
    #[inline(always)]
    pub fn spc(&self) -> SPC_R {
        SPC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - Store USER of option bytes block after system reset"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 10:25 - Store DATA\\[15:0\\]
of option bytes block after system reset"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
}
#[doc = "Option byte control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obctl](index.html) module"]
pub struct OBCTL_SPEC;
impl crate::RegisterSpec for OBCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [obctl::R](R) reader structure"]
impl crate::Readable for OBCTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OBCTL to value 0"]
impl crate::Resettable for OBCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
