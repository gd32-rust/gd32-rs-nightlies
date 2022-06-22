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
#[doc = "Field `OINTF` reader - Out FIFO enabled interrupt flag"]
pub type OINTF_R = crate::BitReader<bool>;
#[doc = "Field `IINTF` reader - In FIFO enabled interrupt flag"]
pub type IINTF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Out FIFO enabled interrupt flag"]
    #[inline(always)]
    pub fn ointf(&self) -> OINTF_R {
        OINTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - In FIFO enabled interrupt flag"]
    #[inline(always)]
    pub fn iintf(&self) -> IINTF_R {
        IINTF_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CAU enable interrupt status flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](index.html) module"]
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
