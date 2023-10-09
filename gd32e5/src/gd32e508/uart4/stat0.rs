#[doc = "Register `STAT0` reader"]
pub type R = crate::R<STAT0_SPEC>;
#[doc = "Field `PERR` reader - Parity error flag"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `FERR` reader - Frame error flag"]
pub type FERR_R = crate::BitReader;
#[doc = "Field `NERR` reader - Noise error flag"]
pub type NERR_R = crate::BitReader;
#[doc = "Field `ORERR` reader - Overrun error"]
pub type ORERR_R = crate::BitReader;
#[doc = "Field `IDLEF` reader - IDLE frame detected flag"]
pub type IDLEF_R = crate::BitReader;
#[doc = "Field `RBNE` reader - Read data buffer not empty"]
pub type RBNE_R = crate::BitReader;
#[doc = "Field `TC` reader - Transmission complete"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TBE` reader - Transmit data buffer empty"]
pub type TBE_R = crate::BitReader;
#[doc = "Field `LBDF` reader - LIN break detection flag"]
pub type LBDF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parity error flag"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame error flag"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise error flag"]
    #[inline(always)]
    pub fn nerr(&self) -> NERR_R {
        NERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn orerr(&self) -> ORERR_R {
        ORERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE frame detected flag"]
    #[inline(always)]
    pub fn idlef(&self) -> IDLEF_R {
        IDLEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data buffer not empty"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data buffer empty"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT0_SPEC;
impl crate::RegisterSpec for STAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat0::R`](R) reader structure"]
impl crate::Readable for STAT0_SPEC {}
#[doc = "`reset()` method sets STAT0 to value 0xc0"]
impl crate::Resettable for STAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
