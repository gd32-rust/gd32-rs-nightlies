#[doc = "Register `ST0INTF` reader"]
pub type R = crate::R<ST0INTF_SPEC>;
#[doc = "Field `CMP0IF` reader - Compare 0 interrupt flag"]
pub type CMP0IF_R = crate::BitReader;
#[doc = "Field `CMP1IF` reader - Compare 1 interrupt flag"]
pub type CMP1IF_R = crate::BitReader;
#[doc = "Field `CMP2IF` reader - Compare 2 interrupt flag"]
pub type CMP2IF_R = crate::BitReader;
#[doc = "Field `CMP3IF` reader - Compare 3 interrupt flag"]
pub type CMP3IF_R = crate::BitReader;
#[doc = "Field `REPIF` reader - Repetition interrupt flag"]
pub type REPIF_R = crate::BitReader;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub type UPIF_R = crate::BitReader;
#[doc = "Field `CAP0IF` reader - Capture 0 interrupt flag"]
pub type CAP0IF_R = crate::BitReader;
#[doc = "Field `CAP1IF` reader - Capture 1 interrupt flag"]
pub type CAP1IF_R = crate::BitReader;
#[doc = "Field `CH0OAIF` reader - Channel 0 output active interrupt flag"]
pub type CH0OAIF_R = crate::BitReader;
#[doc = "Field `CH0ONAIF` reader - Channel 0 output inactive interrupt flag"]
pub type CH0ONAIF_R = crate::BitReader;
#[doc = "Field `CH1OAIF` reader - Channel 1 output active interrupt flag"]
pub type CH1OAIF_R = crate::BitReader;
#[doc = "Field `CH1ONAIF` reader - Channel 1 output inactive interrupt flag"]
pub type CH1ONAIF_R = crate::BitReader;
#[doc = "Field `RSTIF` reader - Counter reset interrupt flag"]
pub type RSTIF_R = crate::BitReader;
#[doc = "Field `DLYIIF` reader - Delayed IDLE mode entry interrupt flag"]
pub type DLYIIF_R = crate::BitReader;
#[doc = "Field `CBLNF` reader - Current balanced flag"]
pub type CBLNF_R = crate::BitReader;
#[doc = "Field `BLNIF` reader - Balanced IDLE flag"]
pub type BLNIF_R = crate::BitReader;
#[doc = "Field `CH0F` reader - Channel 0 output flag"]
pub type CH0F_R = crate::BitReader;
#[doc = "Field `CH1F` reader - Channel 1 output flag"]
pub type CH1F_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Compare 0 interrupt flag"]
    #[inline(always)]
    pub fn cmp0if(&self) -> CMP0IF_R {
        CMP0IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cmp1if(&self) -> CMP1IF_R {
        CMP1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cmp2if(&self) -> CMP2IF_R {
        CMP2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cmp3if(&self) -> CMP3IF_R {
        CMP3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Repetition interrupt flag"]
    #[inline(always)]
    pub fn repif(&self) -> REPIF_R {
        REPIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UPIF_R {
        UPIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture 0 interrupt flag"]
    #[inline(always)]
    pub fn cap0if(&self) -> CAP0IF_R {
        CAP0IF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture 1 interrupt flag"]
    #[inline(always)]
    pub fn cap1if(&self) -> CAP1IF_R {
        CAP1IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 output active interrupt flag"]
    #[inline(always)]
    pub fn ch0oaif(&self) -> CH0OAIF_R {
        CH0OAIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 0 output inactive interrupt flag"]
    #[inline(always)]
    pub fn ch0onaif(&self) -> CH0ONAIF_R {
        CH0ONAIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 1 output active interrupt flag"]
    #[inline(always)]
    pub fn ch1oaif(&self) -> CH1OAIF_R {
        CH1OAIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 1 output inactive interrupt flag"]
    #[inline(always)]
    pub fn ch1onaif(&self) -> CH1ONAIF_R {
        CH1ONAIF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Counter reset interrupt flag"]
    #[inline(always)]
    pub fn rstif(&self) -> RSTIF_R {
        RSTIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Delayed IDLE mode entry interrupt flag"]
    #[inline(always)]
    pub fn dlyiif(&self) -> DLYIIF_R {
        DLYIIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Current balanced flag"]
    #[inline(always)]
    pub fn cblnf(&self) -> CBLNF_R {
        CBLNF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Balanced IDLE flag"]
    #[inline(always)]
    pub fn blnif(&self) -> BLNIF_R {
        BLNIF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 0 output flag"]
    #[inline(always)]
    pub fn ch0f(&self) -> CH0F_R {
        CH0F_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 1 output flag"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "SHRTIMER Slave_TIMER0 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0intf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST0INTF_SPEC;
impl crate::RegisterSpec for ST0INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0intf::R`](R) reader structure"]
impl crate::Readable for ST0INTF_SPEC {}
#[doc = "`reset()` method sets ST0INTF to value 0"]
impl crate::Resettable for ST0INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
