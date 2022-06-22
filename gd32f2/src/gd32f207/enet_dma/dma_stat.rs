#[doc = "Register `DMA_STAT` reader"]
pub struct R(crate::R<DMA_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_STAT` writer"]
pub struct W(crate::W<DMA_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS` reader - Transmit status"]
pub type TS_R = crate::BitReader<bool>;
#[doc = "Field `TS` writer - Transmit status"]
pub type TS_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 0>;
#[doc = "Field `TPS` reader - Transmit process stopped status"]
pub type TPS_R = crate::BitReader<bool>;
#[doc = "Field `TPS` writer - Transmit process stopped status"]
pub type TPS_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 1>;
#[doc = "Field `TBU` reader - Transmit buffer unavailable status"]
pub type TBU_R = crate::BitReader<bool>;
#[doc = "Field `TBU` writer - Transmit buffer unavailable status"]
pub type TBU_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 2>;
#[doc = "Field `TJT` reader - Transmit jabber timeout status"]
pub type TJT_R = crate::BitReader<bool>;
#[doc = "Field `TJT` writer - Transmit jabber timeout status"]
pub type TJT_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 3>;
#[doc = "Field `RO` reader - Receive overflow status"]
pub type RO_R = crate::BitReader<bool>;
#[doc = "Field `RO` writer - Receive overflow status"]
pub type RO_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 4>;
#[doc = "Field `TU` reader - Transmit underflow status"]
pub type TU_R = crate::BitReader<bool>;
#[doc = "Field `TU` writer - Transmit underflow status"]
pub type TU_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 5>;
#[doc = "Field `RS` reader - Receive status"]
pub type RS_R = crate::BitReader<bool>;
#[doc = "Field `RS` writer - Receive status"]
pub type RS_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 6>;
#[doc = "Field `RBU` reader - Receive buffer unavailable status"]
pub type RBU_R = crate::BitReader<bool>;
#[doc = "Field `RBU` writer - Receive buffer unavailable status"]
pub type RBU_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 7>;
#[doc = "Field `RPS` reader - Receive process stopped status"]
pub type RPS_R = crate::BitReader<bool>;
#[doc = "Field `RPS` writer - Receive process stopped status"]
pub type RPS_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 8>;
#[doc = "Field `RWT` reader - Receive watchdog timeout status"]
pub type RWT_R = crate::BitReader<bool>;
#[doc = "Field `RWT` writer - Receive watchdog timeout status"]
pub type RWT_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 9>;
#[doc = "Field `ET` reader - Early transmit status"]
pub type ET_R = crate::BitReader<bool>;
#[doc = "Field `ET` writer - Early transmit status"]
pub type ET_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 10>;
#[doc = "Field `FBE` reader - Fatal bus error status"]
pub type FBE_R = crate::BitReader<bool>;
#[doc = "Field `FBE` writer - Fatal bus error status"]
pub type FBE_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 13>;
#[doc = "Field `ER` reader - Early receive status"]
pub type ER_R = crate::BitReader<bool>;
#[doc = "Field `ER` writer - Early receive status"]
pub type ER_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 14>;
#[doc = "Field `AI` reader - Abnormal interrupt summary"]
pub type AI_R = crate::BitReader<bool>;
#[doc = "Field `AI` writer - Abnormal interrupt summary"]
pub type AI_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 15>;
#[doc = "Field `NI` reader - Normal interrupt summary"]
pub type NI_R = crate::BitReader<bool>;
#[doc = "Field `NI` writer - Normal interrupt summary"]
pub type NI_W<'a> = crate::BitWriter<'a, u32, DMA_STAT_SPEC, bool, 16>;
#[doc = "Field `RP` reader - Receive process state"]
pub type RP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TP` reader - Transmit process state"]
pub type TP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EB` reader - Error bits status"]
pub type EB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSC` reader - MSC status"]
pub type MSC_R = crate::BitReader<bool>;
#[doc = "Field `WUM` reader - WUM status"]
pub type WUM_R = crate::BitReader<bool>;
#[doc = "Field `TST` reader - Time stamp trigger status"]
pub type TST_R = crate::BitReader<bool>;
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
    pub fn ts(&mut self) -> TS_W {
        TS_W::new(self)
    }
    #[doc = "Bit 1 - Transmit process stopped status"]
    #[inline(always)]
    pub fn tps(&mut self) -> TPS_W {
        TPS_W::new(self)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable status"]
    #[inline(always)]
    pub fn tbu(&mut self) -> TBU_W {
        TBU_W::new(self)
    }
    #[doc = "Bit 3 - Transmit jabber timeout status"]
    #[inline(always)]
    pub fn tjt(&mut self) -> TJT_W {
        TJT_W::new(self)
    }
    #[doc = "Bit 4 - Receive overflow status"]
    #[inline(always)]
    pub fn ro(&mut self) -> RO_W {
        RO_W::new(self)
    }
    #[doc = "Bit 5 - Transmit underflow status"]
    #[inline(always)]
    pub fn tu(&mut self) -> TU_W {
        TU_W::new(self)
    }
    #[doc = "Bit 6 - Receive status"]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W {
        RS_W::new(self)
    }
    #[doc = "Bit 7 - Receive buffer unavailable status"]
    #[inline(always)]
    pub fn rbu(&mut self) -> RBU_W {
        RBU_W::new(self)
    }
    #[doc = "Bit 8 - Receive process stopped status"]
    #[inline(always)]
    pub fn rps(&mut self) -> RPS_W {
        RPS_W::new(self)
    }
    #[doc = "Bit 9 - Receive watchdog timeout status"]
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W {
        RWT_W::new(self)
    }
    #[doc = "Bit 10 - Early transmit status"]
    #[inline(always)]
    pub fn et(&mut self) -> ET_W {
        ET_W::new(self)
    }
    #[doc = "Bit 13 - Fatal bus error status"]
    #[inline(always)]
    pub fn fbe(&mut self) -> FBE_W {
        FBE_W::new(self)
    }
    #[doc = "Bit 14 - Early receive status"]
    #[inline(always)]
    pub fn er(&mut self) -> ER_W {
        ER_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W {
        AI_W::new(self)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn ni(&mut self) -> NI_W {
        NI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_stat](index.html) module"]
pub struct DMA_STAT_SPEC;
impl crate::RegisterSpec for DMA_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_stat::R](R) reader structure"]
impl crate::Readable for DMA_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_stat::W](W) writer structure"]
impl crate::Writable for DMA_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_STAT to value 0"]
impl crate::Resettable for DMA_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
