#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Field `CCRCERR` reader - Command response received (CRC check failed)"]
pub type CCRCERR_R = crate::BitReader;
#[doc = "Field `DTCRCERR` reader - Data block sent/received (CRC check failed)"]
pub type DTCRCERR_R = crate::BitReader;
#[doc = "Field `CMDTMOUT` reader - Command response timeout"]
pub type CMDTMOUT_R = crate::BitReader;
#[doc = "Field `DTTMOUT` reader - Data timeout"]
pub type DTTMOUT_R = crate::BitReader;
#[doc = "Field `TXURE` reader - Transmit FIFO underrun error occurs"]
pub type TXURE_R = crate::BitReader;
#[doc = "Field `RXORE` reader - Received FIFO overrun error occurs"]
pub type RXORE_R = crate::BitReader;
#[doc = "Field `CMDRECV` reader - Command response received (CRC check passed)"]
pub type CMDRECV_R = crate::BitReader;
#[doc = "Field `CMDSEND` reader - Command sent (no response required)"]
pub type CMDSEND_R = crate::BitReader;
#[doc = "Field `DTEND` reader - Data end (data counter, SDIO_DATACNT, is zero)"]
pub type DTEND_R = crate::BitReader;
#[doc = "Field `STBITE` reader - Start bit error in the bus"]
pub type STBITE_R = crate::BitReader;
#[doc = "Field `DTBLKEND` reader - Data block sent/received (CRC check passed)"]
pub type DTBLKEND_R = crate::BitReader;
#[doc = "Field `CMDRUN` reader - Command transmission in progress"]
pub type CMDRUN_R = crate::BitReader;
#[doc = "Field `TXRUN` reader - Data transmission in progress"]
pub type TXRUN_R = crate::BitReader;
#[doc = "Field `RXRUN` reader - Data reception in progress"]
pub type RXRUN_R = crate::BitReader;
#[doc = "Field `TFH` reader - Transmit FIFO is half empty: at least 8 words can be written into the FIFO"]
pub type TFH_R = crate::BitReader;
#[doc = "Field `RFH` reader - Receive FIFO is half full: at least 8 words can be read in the FIFO"]
pub type RFH_R = crate::BitReader;
#[doc = "Field `TFF` reader - Transmit FIFO is full"]
pub type TFF_R = crate::BitReader;
#[doc = "Field `RFF` reader - Receive FIFO is full"]
pub type RFF_R = crate::BitReader;
#[doc = "Field `TFE` reader - Transmit FIFO is empty"]
pub type TFE_R = crate::BitReader;
#[doc = "Field `RFE` reader - Receive FIFO is empty"]
pub type RFE_R = crate::BitReader;
#[doc = "Field `TXDTVAL` reader - Data is valid in transmit FIFO"]
pub type TXDTVAL_R = crate::BitReader;
#[doc = "Field `RXDTVAL` reader - Data is valid in receive FIFO"]
pub type RXDTVAL_R = crate::BitReader;
#[doc = "Field `SDIOINT` reader - SD I/O interrupt received"]
pub type SDIOINT_R = crate::BitReader;
#[doc = "Field `ATAEND` reader - CE-ATA command completion signal received (only for CMD61)"]
pub type ATAEND_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command response received (CRC check failed)"]
    #[inline(always)]
    pub fn ccrcerr(&self) -> CCRCERR_R {
        CCRCERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data block sent/received (CRC check failed)"]
    #[inline(always)]
    pub fn dtcrcerr(&self) -> DTCRCERR_R {
        DTCRCERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command response timeout"]
    #[inline(always)]
    pub fn cmdtmout(&self) -> CMDTMOUT_R {
        CMDTMOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout"]
    #[inline(always)]
    pub fn dttmout(&self) -> DTTMOUT_R {
        DTTMOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error occurs"]
    #[inline(always)]
    pub fn txure(&self) -> TXURE_R {
        TXURE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error occurs"]
    #[inline(always)]
    pub fn rxore(&self) -> RXORE_R {
        RXORE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received (CRC check passed)"]
    #[inline(always)]
    pub fn cmdrecv(&self) -> CMDRECV_R {
        CMDRECV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent (no response required)"]
    #[inline(always)]
    pub fn cmdsend(&self) -> CMDSEND_R {
        CMDSEND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end (data counter, SDIO_DATACNT, is zero)"]
    #[inline(always)]
    pub fn dtend(&self) -> DTEND_R {
        DTEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit error in the bus"]
    #[inline(always)]
    pub fn stbite(&self) -> STBITE_R {
        STBITE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received (CRC check passed)"]
    #[inline(always)]
    pub fn dtblkend(&self) -> DTBLKEND_R {
        DTBLKEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command transmission in progress"]
    #[inline(always)]
    pub fn cmdrun(&self) -> CMDRUN_R {
        CMDRUN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmission in progress"]
    #[inline(always)]
    pub fn txrun(&self) -> TXRUN_R {
        TXRUN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data reception in progress"]
    #[inline(always)]
    pub fn rxrun(&self) -> RXRUN_R {
        RXRUN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO is half empty: at least 8 words can be written into the FIFO"]
    #[inline(always)]
    pub fn tfh(&self) -> TFH_R {
        TFH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO is half full: at least 8 words can be read in the FIFO"]
    #[inline(always)]
    pub fn rfh(&self) -> RFH_R {
        RFH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO is full"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO is full"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO is empty"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data is valid in transmit FIFO"]
    #[inline(always)]
    pub fn txdtval(&self) -> TXDTVAL_R {
        TXDTVAL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data is valid in receive FIFO"]
    #[inline(always)]
    pub fn rxdtval(&self) -> RXDTVAL_R {
        RXDTVAL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SD I/O interrupt received"]
    #[inline(always)]
    pub fn sdioint(&self) -> SDIOINT_R {
        SDIOINT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received (only for CMD61)"]
    #[inline(always)]
    pub fn ataend(&self) -> ATAEND_R {
        ATAEND_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "SDIO status register (SDIO_STR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
