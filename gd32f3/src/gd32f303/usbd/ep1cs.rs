#[doc = "Register `EP1CS` reader"]
pub type R = crate::R<Ep1csSpec>;
#[doc = "Register `EP1CS` writer"]
pub type W = crate::W<Ep1csSpec>;
#[doc = "Field `EP_AR` reader - Endpoint address"]
pub type EpArR = crate::FieldReader;
#[doc = "Field `EP_AR` writer - Endpoint address"]
pub type EpArW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_STA` reader - Status bits, for transmission transfers"]
pub type TxStaR = crate::FieldReader;
#[doc = "Field `TX_STA` writer - Status bits, for transmission transfers"]
pub type TxStaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_DTG` reader - Data Toggle, for transmission transfers"]
pub type TxDtgR = crate::BitReader;
#[doc = "Field `TX_DTG` writer - Data Toggle, for transmission transfers"]
pub type TxDtgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ST` reader - Correct Transfer for transmission"]
pub type TxStR = crate::BitReader;
#[doc = "Field `TX_ST` writer - Correct Transfer for transmission"]
pub type TxStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_KCTL` reader - Endpoint kind"]
pub type EpKctlR = crate::BitReader;
#[doc = "Field `EP_KCTL` writer - Endpoint kind"]
pub type EpKctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_CTL` reader - Endpoint type"]
pub type EpCtlR = crate::FieldReader;
#[doc = "Field `EP_CTL` writer - Endpoint type"]
pub type EpCtlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SETUP` reader - Setup transaction completed"]
pub type SetupR = crate::BitReader;
#[doc = "Field `SETUP` writer - Setup transaction completed"]
pub type SetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_STA` reader - Status bits, for reception transfers"]
pub type RxStaR = crate::FieldReader;
#[doc = "Field `RX_STA` writer - Status bits, for reception transfers"]
pub type RxStaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_DTG` reader - Data Toggle, for reception transfers"]
pub type RxDtgR = crate::BitReader;
#[doc = "Field `RX_DTG` writer - Data Toggle, for reception transfers"]
pub type RxDtgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ST` reader - Correct transfer for reception"]
pub type RxStR = crate::BitReader;
#[doc = "Field `RX_ST` writer - Correct transfer for reception"]
pub type RxStW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn ep_ar(&self) -> EpArR {
        EpArR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    pub fn tx_sta(&self) -> TxStaR {
        TxStaR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    pub fn tx_dtg(&self) -> TxDtgR {
        TxDtgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    pub fn tx_st(&self) -> TxStR {
        TxStR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    pub fn ep_kctl(&self) -> EpKctlR {
        EpKctlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    pub fn ep_ctl(&self) -> EpCtlR {
        EpCtlR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    pub fn rx_sta(&self) -> RxStaR {
        RxStaR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    pub fn rx_dtg(&self) -> RxDtgR {
        RxDtgR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    pub fn rx_st(&self) -> RxStR {
        RxStR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    #[must_use]
    pub fn ep_ar(&mut self) -> EpArW<Ep1csSpec> {
        EpArW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    #[must_use]
    pub fn tx_sta(&mut self) -> TxStaW<Ep1csSpec> {
        TxStaW::new(self, 4)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dtg(&mut self) -> TxDtgW<Ep1csSpec> {
        TxDtgW::new(self, 6)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tx_st(&mut self) -> TxStW<Ep1csSpec> {
        TxStW::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    #[must_use]
    pub fn ep_kctl(&mut self) -> EpKctlW<Ep1csSpec> {
        EpKctlW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn ep_ctl(&mut self) -> EpCtlW<Ep1csSpec> {
        EpCtlW::new(self, 9)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SetupW<Ep1csSpec> {
        SetupW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sta(&mut self) -> RxStaW<Ep1csSpec> {
        RxStaW::new(self, 12)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dtg(&mut self) -> RxDtgW<Ep1csSpec> {
        RxDtgW::new(self, 14)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    #[must_use]
    pub fn rx_st(&mut self) -> RxStW<Ep1csSpec> {
        RxStW::new(self, 15)
    }
}
#[doc = "endpoint 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep1csSpec;
impl crate::RegisterSpec for Ep1csSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep1cs::R`](R) reader structure"]
impl crate::Readable for Ep1csSpec {}
#[doc = "`write(|w| ..)` method takes [`ep1cs::W`](W) writer structure"]
impl crate::Writable for Ep1csSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP1CS to value 0"]
impl crate::Resettable for Ep1csSpec {
    const RESET_VALUE: u32 = 0;
}
