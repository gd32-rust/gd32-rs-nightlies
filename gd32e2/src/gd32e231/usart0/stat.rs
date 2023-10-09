#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Field `PERR` reader - Parity error"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `FERR` reader - Framing error"]
pub type FERR_R = crate::BitReader;
#[doc = "Field `NERR` reader - Noise detected flag"]
pub type NERR_R = crate::BitReader;
#[doc = "Field `ORERR` reader - Overrun error"]
pub type ORERR_R = crate::BitReader;
#[doc = "Field `IDLEF` reader - Idle line detected"]
pub type IDLEF_R = crate::BitReader;
#[doc = "Field `RBNE` reader - Read data register not empty"]
pub type RBNE_R = crate::BitReader;
#[doc = "Field `TC` reader - Transmission complete"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TBE` reader - Transmit data register empty"]
pub type TBE_R = crate::BitReader;
#[doc = "Field `LBDF` reader - LIN break detection flag"]
pub type LBDF_R = crate::BitReader;
#[doc = "Field `CTSF` reader - CTS interrupt flag"]
pub type CTSF_R = crate::BitReader;
#[doc = "Field `CTS` reader - CTS flag"]
pub type CTS_R = crate::BitReader;
#[doc = "Field `RTF` reader - Receiver timeout"]
pub type RTF_R = crate::BitReader;
#[doc = "Field `EBF` reader - End of block flag"]
pub type EBF_R = crate::BitReader;
#[doc = "Field `ABDE` reader - Auto baud rate error"]
pub type ABDE_R = crate::BitReader;
#[doc = "Field `ABDF` reader - Auto baud rate flag"]
pub type ABDF_R = crate::BitReader;
#[doc = "Field `BSY` reader - Busy flag"]
pub type BSY_R = crate::BitReader;
#[doc = "Field `AMF` reader - character match flag"]
pub type AMF_R = crate::BitReader;
#[doc = "Field `SBF` reader - Send break flag"]
pub type SBF_R = crate::BitReader;
#[doc = "Field `RWU` reader - Receiver wakeup from Mute mode"]
pub type RWU_R = crate::BitReader;
#[doc = "Field `WUF` reader - Wakeup from Stop mode flag"]
pub type WUF_R = crate::BitReader;
#[doc = "Field `TEA` reader - Transmit enable acknowledge flag"]
pub type TEA_R = crate::BitReader;
#[doc = "Field `REA` reader - Receive enable acknowledge flag"]
pub type REA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parity error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise detected flag"]
    #[inline(always)]
    pub fn nerr(&self) -> NERR_R {
        NERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn orerr(&self) -> ORERR_R {
        ORERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle line detected"]
    #[inline(always)]
    pub fn idlef(&self) -> IDLEF_R {
        IDLEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data register not empty"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data register empty"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS interrupt flag"]
    #[inline(always)]
    pub fn ctsf(&self) -> CTSF_R {
        CTSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS flag"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver timeout"]
    #[inline(always)]
    pub fn rtf(&self) -> RTF_R {
        RTF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of block flag"]
    #[inline(always)]
    pub fn ebf(&self) -> EBF_R {
        EBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto baud rate error"]
    #[inline(always)]
    pub fn abde(&self) -> ABDE_R {
        ABDE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto baud rate flag"]
    #[inline(always)]
    pub fn abdf(&self) -> ABDF_R {
        ABDF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - character match flag"]
    #[inline(always)]
    pub fn amf(&self) -> AMF_R {
        AMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Send break flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receiver wakeup from Mute mode"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wakeup from Stop mode flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit enable acknowledge flag"]
    #[inline(always)]
    pub fn tea(&self) -> TEA_R {
        TEA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive enable acknowledge flag"]
    #[inline(always)]
    pub fn rea(&self) -> REA_R {
        REA_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Interrupt &amp; status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`reset()` method sets STAT to value 0xc0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
