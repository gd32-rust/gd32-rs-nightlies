#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDMA_ODD` reader - Odd bytes in TX DMA channel"]
pub type TXDMA_ODD_R = crate::BitReader<bool>;
#[doc = "Field `TXDMA_ODD` writer - Odd bytes in TX DMA channel"]
pub type TXDMA_ODD_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 14>;
#[doc = "Field `RXDMA_ODD` reader - Odd bytes in RX DMA channel"]
pub type RXDMA_ODD_R = crate::BitReader<bool>;
#[doc = "Field `RXDMA_ODD` writer - Odd bytes in RX DMA channel"]
pub type RXDMA_ODD_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 13>;
#[doc = "Field `BYTEN` reader - Byte access enable"]
pub type BYTEN_R = crate::BitReader<bool>;
#[doc = "Field `BYTEN` writer - Byte access enable"]
pub type BYTEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 12>;
#[doc = "Field `DZ` reader - Date size"]
pub type DZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DZ` writer - Date size"]
pub type DZ_W<'a> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 4, 8>;
#[doc = "Field `TBEIE` reader - Transmit Buffer Empty Interrupt Enable"]
pub type TBEIE_R = crate::BitReader<bool>;
#[doc = "Field `TBEIE` writer - Transmit Buffer Empty Interrupt Enable"]
pub type TBEIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 7>;
#[doc = "Field `RBNEIE` reader - Receive Buffer Not Empty Interrupt Enable"]
pub type RBNEIE_R = crate::BitReader<bool>;
#[doc = "Field `RBNEIE` writer - Receive Buffer Not Empty Interrupt Enable"]
pub type RBNEIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 6>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 5>;
#[doc = "Field `TMOD` reader - SPI TI Mode Enable"]
pub type TMOD_R = crate::BitReader<bool>;
#[doc = "Field `TMOD` writer - SPI TI Mode Enable"]
pub type TMOD_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 4>;
#[doc = "Field `NSSP` reader - SPI NSS Pulse Mode Enable"]
pub type NSSP_R = crate::BitReader<bool>;
#[doc = "Field `NSSP` writer - SPI NSS Pulse Mode Enable"]
pub type NSSP_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 3>;
#[doc = "Field `NSSDRV` reader - NSS output enable"]
pub type NSSDRV_R = crate::BitReader<bool>;
#[doc = "Field `NSSDRV` writer - NSS output enable"]
pub type NSSDRV_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 2>;
#[doc = "Field `DMATEN` reader - Tx buffer DMA enable"]
pub type DMATEN_R = crate::BitReader<bool>;
#[doc = "Field `DMATEN` writer - Tx buffer DMA enable"]
pub type DMATEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 1>;
#[doc = "Field `DMAREN` reader - Rx buffer DMA enable"]
pub type DMAREN_R = crate::BitReader<bool>;
#[doc = "Field `DMAREN` writer - Rx buffer DMA enable"]
pub type DMAREN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 14 - Odd bytes in TX DMA channel"]
    #[inline(always)]
    pub fn txdma_odd(&self) -> TXDMA_ODD_R {
        TXDMA_ODD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Odd bytes in RX DMA channel"]
    #[inline(always)]
    pub fn rxdma_odd(&self) -> RXDMA_ODD_R {
        RXDMA_ODD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Byte access enable"]
    #[inline(always)]
    pub fn byten(&self) -> BYTEN_R {
        BYTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Date size"]
    #[inline(always)]
    pub fn dz(&self) -> DZ_R {
        DZ_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TBEIE_R {
        TBEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Buffer Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RBNEIE_R {
        RBNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI TI Mode Enable"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI NSS Pulse Mode Enable"]
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - NSS output enable"]
    #[inline(always)]
    pub fn nssdrv(&self) -> NSSDRV_R {
        NSSDRV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DMATEN_R {
        DMATEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Odd bytes in TX DMA channel"]
    #[inline(always)]
    pub fn txdma_odd(&mut self) -> TXDMA_ODD_W {
        TXDMA_ODD_W::new(self)
    }
    #[doc = "Bit 13 - Odd bytes in RX DMA channel"]
    #[inline(always)]
    pub fn rxdma_odd(&mut self) -> RXDMA_ODD_W {
        RXDMA_ODD_W::new(self)
    }
    #[doc = "Bit 12 - Byte access enable"]
    #[inline(always)]
    pub fn byten(&mut self) -> BYTEN_W {
        BYTEN_W::new(self)
    }
    #[doc = "Bits 8:11 - Date size"]
    #[inline(always)]
    pub fn dz(&mut self) -> DZ_W {
        DZ_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tbeie(&mut self) -> TBEIE_W {
        TBEIE_W::new(self)
    }
    #[doc = "Bit 6 - Receive Buffer Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rbneie(&mut self) -> RBNEIE_W {
        RBNEIE_W::new(self)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 4 - SPI TI Mode Enable"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TMOD_W {
        TMOD_W::new(self)
    }
    #[doc = "Bit 3 - SPI NSS Pulse Mode Enable"]
    #[inline(always)]
    pub fn nssp(&mut self) -> NSSP_W {
        NSSP_W::new(self)
    }
    #[doc = "Bit 2 - NSS output enable"]
    #[inline(always)]
    pub fn nssdrv(&mut self) -> NSSDRV_W {
        NSSDRV_W::new(self)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaten(&mut self) -> DMATEN_W {
        DMATEN_W::new(self)
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaren(&mut self) -> DMAREN_W {
        DMAREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
