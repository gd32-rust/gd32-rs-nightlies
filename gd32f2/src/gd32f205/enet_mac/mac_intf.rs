#[doc = "Register `MAC_INTF` reader"]
pub struct R(crate::R<MAC_INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WUM` reader - WUM status"]
pub type WUM_R = crate::BitReader<bool>;
#[doc = "Field `MSC` reader - MSC status"]
pub type MSC_R = crate::BitReader<bool>;
#[doc = "Field `MSCR` reader - MSC receive status"]
pub type MSCR_R = crate::BitReader<bool>;
#[doc = "Field `MSCT` reader - MSC transmit status"]
pub type MSCT_R = crate::BitReader<bool>;
#[doc = "Field `TMST` reader - Time stamp trigger status"]
pub type TMST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 3 - WUM status"]
    #[inline(always)]
    pub fn wum(&self) -> WUM_R {
        WUM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MSC status"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MSC receive status"]
    #[inline(always)]
    pub fn mscr(&self) -> MSCR_R {
        MSCR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MSC transmit status"]
    #[inline(always)]
    pub fn msct(&self) -> MSCT_R {
        MSCT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Time stamp trigger status"]
    #[inline(always)]
    pub fn tmst(&self) -> TMST_R {
        TMST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Ethernet MAC interrupt flag register (MAC_INTF)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_intf](index.html) module"]
pub struct MAC_INTF_SPEC;
impl crate::RegisterSpec for MAC_INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_intf::R](R) reader structure"]
impl crate::Readable for MAC_INTF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAC_INTF to value 0"]
impl crate::Resettable for MAC_INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
