#[doc = "Register `DMA_STAT` reader"]
pub type R = crate::R<DMA_STAT_SPEC>;
#[doc = "Register `DMA_STAT` writer"]
pub type W = crate::W<DMA_STAT_SPEC>;
#[doc = "Field `TS` reader - Transmit status"]
pub type TS_R = crate::BitReader;
#[doc = "Field `TS` writer - Transmit status"]
pub type TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPS` reader - Transmit process stopped status"]
pub type TPS_R = crate::BitReader;
#[doc = "Field `TPS` writer - Transmit process stopped status"]
pub type TPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TBU` reader - Transmit buffer unavailable status"]
pub type TBU_R = crate::BitReader;
#[doc = "Field `TBU` writer - Transmit buffer unavailable status"]
pub type TBU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TJT` reader - Transmit jabber timeout status"]
pub type TJT_R = crate::BitReader;
#[doc = "Field `TJT` writer - Transmit jabber timeout status"]
pub type TJT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RO` reader - Receive overflow status"]
pub type RO_R = crate::BitReader;
#[doc = "Field `RO` writer - Receive overflow status"]
pub type RO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TU` reader - Transmit underflow status"]
pub type TU_R = crate::BitReader;
#[doc = "Field `TU` writer - Transmit underflow status"]
pub type TU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RS` reader - Receive status"]
pub type RS_R = crate::BitReader;
#[doc = "Field `RS` writer - Receive status"]
pub type RS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBU` reader - Receive buffer unavailable status"]
pub type RBU_R = crate::BitReader;
#[doc = "Field `RBU` writer - Receive buffer unavailable status"]
pub type RBU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RPS` reader - Receive process stopped status"]
pub type RPS_R = crate::BitReader;
#[doc = "Field `RPS` writer - Receive process stopped status"]
pub type RPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWT` reader - Receive watchdog timeout status"]
pub type RWT_R = crate::BitReader;
#[doc = "Field `RWT` writer - Receive watchdog timeout status"]
pub type RWT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ET` reader - Early transmit status"]
pub type ET_R = crate::BitReader;
#[doc = "Field `ET` writer - Early transmit status"]
pub type ET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBE` reader - Fatal bus error status"]
pub type FBE_R = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal bus error status"]
pub type FBE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ER` reader - Early receive status"]
pub type ER_R = crate::BitReader;
#[doc = "Field `ER` writer - Early receive status"]
pub type ER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AI` reader - Abnormal interrupt summary"]
pub type AI_R = crate::BitReader;
#[doc = "Field `AI` writer - Abnormal interrupt summary"]
pub type AI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NI` reader - Normal interrupt summary"]
pub type NI_R = crate::BitReader;
#[doc = "Field `NI` writer - Normal interrupt summary"]
pub type NI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP` reader - Receive process state"]
pub type RP_R = crate::FieldReader;
#[doc = "Field `TP` reader - Transmit process state"]
pub type TP_R = crate::FieldReader;
#[doc = "Field `EB` reader - Error bits status"]
pub type EB_R = crate::FieldReader;
#[doc = "Field `MSC` reader - MSC status"]
pub type MSC_R = crate::BitReader;
#[doc = "Field `WUM` reader - WUM status"]
pub type WUM_R = crate::BitReader;
#[doc = "Field `TST` reader - Time stamp trigger status"]
pub type TST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit status"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped status"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable status"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout status"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive overflow status"]
    #[inline(always)]
    pub fn ro(&self) -> RO_R {
        RO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit underflow status"]
    #[inline(always)]
    pub fn tu(&self) -> TU_R {
        TU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive status"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable status"]
    #[inline(always)]
    pub fn rbu(&self) -> RBU_R {
        RBU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped status"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive watchdog timeout status"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit status"]
    #[inline(always)]
    pub fn et(&self) -> ET_R {
        ET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error status"]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive status"]
    #[inline(always)]
    pub fn er(&self) -> ER_R {
        ER_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn ni(&self) -> NI_R {
        NI_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive process state"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit process state"]
    #[inline(always)]
    pub fn tp(&self) -> TP_R {
        TP_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error bits status"]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - MSC status"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - WUM status"]
    #[inline(always)]
    pub fn wum(&self) -> WUM_R {
        WUM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Time stamp trigger status"]
    #[inline(always)]
    pub fn tst(&self) -> TST_R {
        TST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit status"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<DMA_STAT_SPEC, 0> {
        TS_W::new(self)
    }
    #[doc = "Bit 1 - Transmit process stopped status"]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TPS_W<DMA_STAT_SPEC, 1> {
        TPS_W::new(self)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable status"]
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TBU_W<DMA_STAT_SPEC, 2> {
        TBU_W::new(self)
    }
    #[doc = "Bit 3 - Transmit jabber timeout status"]
    #[inline(always)]
    #[must_use]
    pub fn tjt(&mut self) -> TJT_W<DMA_STAT_SPEC, 3> {
        TJT_W::new(self)
    }
    #[doc = "Bit 4 - Receive overflow status"]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RO_W<DMA_STAT_SPEC, 4> {
        RO_W::new(self)
    }
    #[doc = "Bit 5 - Transmit underflow status"]
    #[inline(always)]
    #[must_use]
    pub fn tu(&mut self) -> TU_W<DMA_STAT_SPEC, 5> {
        TU_W::new(self)
    }
    #[doc = "Bit 6 - Receive status"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<DMA_STAT_SPEC, 6> {
        RS_W::new(self)
    }
    #[doc = "Bit 7 - Receive buffer unavailable status"]
    #[inline(always)]
    #[must_use]
    pub fn rbu(&mut self) -> RBU_W<DMA_STAT_SPEC, 7> {
        RBU_W::new(self)
    }
    #[doc = "Bit 8 - Receive process stopped status"]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RPS_W<DMA_STAT_SPEC, 8> {
        RPS_W::new(self)
    }
    #[doc = "Bit 9 - Receive watchdog timeout status"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<DMA_STAT_SPEC, 9> {
        RWT_W::new(self)
    }
    #[doc = "Bit 10 - Early transmit status"]
    #[inline(always)]
    #[must_use]
    pub fn et(&mut self) -> ET_W<DMA_STAT_SPEC, 10> {
        ET_W::new(self)
    }
    #[doc = "Bit 13 - Fatal bus error status"]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FBE_W<DMA_STAT_SPEC, 13> {
        FBE_W::new(self)
    }
    #[doc = "Bit 14 - Early receive status"]
    #[inline(always)]
    #[must_use]
    pub fn er(&mut self) -> ER_W<DMA_STAT_SPEC, 14> {
        ER_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<DMA_STAT_SPEC, 15> {
        AI_W::new(self)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    #[must_use]
    pub fn ni(&mut self) -> NI_W<DMA_STAT_SPEC, 16> {
        NI_W::new(self)
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
#[doc = "Ethernet DMA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_STAT_SPEC;
impl crate::RegisterSpec for DMA_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_stat::R`](R) reader structure"]
impl crate::Readable for DMA_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_stat::W`](W) writer structure"]
impl crate::Writable for DMA_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_STAT to value 0"]
impl crate::Resettable for DMA_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
