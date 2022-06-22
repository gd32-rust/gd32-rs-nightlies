#[doc = "Register `MAC_DBG` reader"]
pub struct R(crate::R<MAC_DBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_DBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_DBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_DBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MRNI` reader - MAC receive state not idle"]
pub type MRNI_R = crate::BitReader<bool>;
#[doc = "Field `RXAFS` reader - Rx asynchronous FIFO status"]
pub type RXAFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFW` reader - RxFIFO is writing"]
pub type RXFW_R = crate::BitReader<bool>;
#[doc = "Field `RXFRS` reader - RxFIFO read operation status"]
pub type RXFRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFS` reader - RxFIFO state"]
pub type RXFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MTNI` reader - MAC transmit state not idle"]
pub type MTNI_R = crate::BitReader<bool>;
#[doc = "Field `SOMT` reader - Status of MAC transmitter"]
pub type SOMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCS` reader - Pause condition status"]
pub type PCS_R = crate::BitReader<bool>;
#[doc = "Field `TXFRS` reader - TxFIFO read operation status"]
pub type TXFRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFW` reader - TxFIFO is writing"]
pub type TXFW_R = crate::BitReader<bool>;
#[doc = "Field `TXFNE` reader - TxFIFO not empty flag"]
pub type TXFNE_R = crate::BitReader<bool>;
#[doc = "Field `TXFF` reader - TxFIFO Full flag"]
pub type TXFF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - MAC receive state not idle"]
    #[inline(always)]
    pub fn mrni(&self) -> MRNI_R {
        MRNI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Rx asynchronous FIFO status"]
    #[inline(always)]
    pub fn rxafs(&self) -> RXAFS_R {
        RXAFS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - RxFIFO is writing"]
    #[inline(always)]
    pub fn rxfw(&self) -> RXFW_R {
        RXFW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - RxFIFO read operation status"]
    #[inline(always)]
    pub fn rxfrs(&self) -> RXFRS_R {
        RXFRS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - RxFIFO state"]
    #[inline(always)]
    pub fn rxfs(&self) -> RXFS_R {
        RXFS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - MAC transmit state not idle"]
    #[inline(always)]
    pub fn mtni(&self) -> MTNI_R {
        MTNI_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Status of MAC transmitter"]
    #[inline(always)]
    pub fn somt(&self) -> SOMT_R {
        SOMT_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Pause condition status"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - TxFIFO read operation status"]
    #[inline(always)]
    pub fn txfrs(&self) -> TXFRS_R {
        TXFRS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - TxFIFO is writing"]
    #[inline(always)]
    pub fn txfw(&self) -> TXFW_R {
        TXFW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - TxFIFO not empty flag"]
    #[inline(always)]
    pub fn txfne(&self) -> TXFNE_R {
        TXFNE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TxFIFO Full flag"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Ethernet MAC debug register (MAC_DBG)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_dbg](index.html) module"]
pub struct MAC_DBG_SPEC;
impl crate::RegisterSpec for MAC_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_dbg::R](R) reader structure"]
impl crate::Readable for MAC_DBG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAC_DBG to value 0"]
impl crate::Resettable for MAC_DBG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
