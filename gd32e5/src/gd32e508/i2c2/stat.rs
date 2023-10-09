#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `TBE` reader - I2C_TDATA is empty during transmitting"]
pub type TBE_R = crate::BitReader;
#[doc = "Field `TBE` writer - I2C_TDATA is empty during transmitting"]
pub type TBE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TI` reader - Transmit interrupt"]
pub type TI_R = crate::BitReader;
#[doc = "Field `TI` writer - Transmit interrupt"]
pub type TI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBNE` reader - I2C_RDATA is not empty during receiving"]
pub type RBNE_R = crate::BitReader;
#[doc = "Field `ADDSEND` reader - Address received matches in slave mode"]
pub type ADDSEND_R = crate::BitReader;
#[doc = "Field `NACK` reader - Not Acknowledge flag"]
pub type NACK_R = crate::BitReader;
#[doc = "Field `STPDET` reader - STOP condition is detected on the bus"]
pub type STPDET_R = crate::BitReader;
#[doc = "Field `TC` reader - Transfer complete in master mode"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TCR` reader - Transfer complete reload"]
pub type TCR_R = crate::BitReader;
#[doc = "Field `BERR` reader - Bus error"]
pub type BERR_R = crate::BitReader;
#[doc = "Field `LOSTARB` reader - Arbitration Lost"]
pub type LOSTARB_R = crate::BitReader;
#[doc = "Field `OUERR` reader - Overrun/Underrun error in slave mode"]
pub type OUERR_R = crate::BitReader;
#[doc = "Field `PECERR` reader - PEC error"]
pub type PECERR_R = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - TIMEOUT flag"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `SMBALT` reader - SMBus Alert"]
pub type SMBALT_R = crate::BitReader;
#[doc = "Field `I2CBSY` reader - Busy flag"]
pub type I2CBSY_R = crate::BitReader;
#[doc = "Field `TR` reader - Whether the I2C is a transmitter or a receiver in slave mode"]
pub type TR_R = crate::BitReader;
#[doc = "Field `READDR` reader - Received match address in slave mode"]
pub type READDR_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - I2C_TDATA is empty during transmitting"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C_RDATA is not empty during receiving"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address received matches in slave mode"]
    #[inline(always)]
    pub fn addsend(&self) -> ADDSEND_R {
        ADDSEND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not Acknowledge flag"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - STOP condition is detected on the bus"]
    #[inline(always)]
    pub fn stpdet(&self) -> STPDET_R {
        STPDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer complete in master mode"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer complete reload"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost"]
    #[inline(always)]
    pub fn lostarb(&self) -> LOSTARB_R {
        LOSTARB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun/Underrun error in slave mode"]
    #[inline(always)]
    pub fn ouerr(&self) -> OUERR_R {
        OUERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PEC error"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIMEOUT flag"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus Alert"]
    #[inline(always)]
    pub fn smbalt(&self) -> SMBALT_R {
        SMBALT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Busy flag"]
    #[inline(always)]
    pub fn i2cbsy(&self) -> I2CBSY_R {
        I2CBSY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Whether the I2C is a transmitter or a receiver in slave mode"]
    #[inline(always)]
    pub fn tr(&self) -> TR_R {
        TR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Received match address in slave mode"]
    #[inline(always)]
    pub fn readdr(&self) -> READDR_R {
        READDR_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - I2C_TDATA is empty during transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn tbe(&mut self) -> TBE_W<STAT_SPEC, 0> {
        TBE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TI_W<STAT_SPEC, 1> {
        TI_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transfer status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0x01"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
