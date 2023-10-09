#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `DMAREN` reader - Rx buffer DMA enable"]
pub type DMAREN_R = crate::BitReader;
#[doc = "Field `DMAREN` writer - Rx buffer DMA enable"]
pub type DMAREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMATEN` reader - Tx buffer DMA enable"]
pub type DMATEN_R = crate::BitReader;
#[doc = "Field `DMATEN` writer - Tx buffer DMA enable"]
pub type DMATEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NSSDRV` reader - NSS output enable"]
pub type NSSDRV_R = crate::BitReader;
#[doc = "Field `NSSDRV` writer - NSS output enable"]
pub type NSSDRV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NSSP` reader - SPI NSS Pulse Mode Enable"]
pub type NSSP_R = crate::BitReader;
#[doc = "Field `NSSP` writer - SPI NSS Pulse Mode Enable"]
pub type NSSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMOD` reader - SPI TI Mode Enable"]
pub type TMOD_R = crate::BitReader;
#[doc = "Field `TMOD` writer - SPI TI Mode Enable"]
pub type TMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBNEIE` reader - Receive Buffer Not Empty Interrupt Enable"]
pub type RBNEIE_R = crate::BitReader;
#[doc = "Field `RBNEIE` writer - Receive Buffer Not Empty Interrupt Enable"]
pub type RBNEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TBEIE` reader - Transmit Buffer Empty Interrupt Enable"]
pub type TBEIE_R = crate::BitReader;
#[doc = "Field `TBEIE` writer - Transmit Buffer Empty Interrupt Enable"]
pub type TBEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DZ` reader - Date size"]
pub type DZ_R = crate::FieldReader;
#[doc = "Field `DZ` writer - Date size"]
pub type DZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `BYTEN` reader - Byte access enable"]
pub type BYTEN_R = crate::BitReader;
#[doc = "Field `BYTEN` writer - Byte access enable"]
pub type BYTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXDMA_ODD` reader - Odd bytes in RX DMA channel"]
pub type RXDMA_ODD_R = crate::BitReader;
#[doc = "Field `RXDMA_ODD` writer - Odd bytes in RX DMA channel"]
pub type RXDMA_ODD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDMA_ODD` reader - Odd bytes in TX DMA channel"]
pub type TXDMA_ODD_R = crate::BitReader;
#[doc = "Field `TXDMA_ODD` writer - Odd bytes in TX DMA channel"]
pub type TXDMA_ODD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DMATEN_R {
        DMATEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NSS output enable"]
    #[inline(always)]
    pub fn nssdrv(&self) -> NSSDRV_R {
        NSSDRV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI NSS Pulse Mode Enable"]
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI TI Mode Enable"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Buffer Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RBNEIE_R {
        RBNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TBEIE_R {
        TBEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Date size"]
    #[inline(always)]
    pub fn dz(&self) -> DZ_R {
        DZ_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Byte access enable"]
    #[inline(always)]
    pub fn byten(&self) -> BYTEN_R {
        BYTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Odd bytes in RX DMA channel"]
    #[inline(always)]
    pub fn rxdma_odd(&self) -> RXDMA_ODD_R {
        RXDMA_ODD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Odd bytes in TX DMA channel"]
    #[inline(always)]
    pub fn txdma_odd(&self) -> TXDMA_ODD_R {
        TXDMA_ODD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<CTL1_SPEC, 0> {
        DMAREN_W::new(self)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaten(&mut self) -> DMATEN_W<CTL1_SPEC, 1> {
        DMATEN_W::new(self)
    }
    #[doc = "Bit 2 - NSS output enable"]
    #[inline(always)]
    #[must_use]
    pub fn nssdrv(&mut self) -> NSSDRV_W<CTL1_SPEC, 2> {
        NSSDRV_W::new(self)
    }
    #[doc = "Bit 3 - SPI NSS Pulse Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nssp(&mut self) -> NSSP_W<CTL1_SPEC, 3> {
        NSSP_W::new(self)
    }
    #[doc = "Bit 4 - SPI TI Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmod(&mut self) -> TMOD_W<CTL1_SPEC, 4> {
        TMOD_W::new(self)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTL1_SPEC, 5> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 6 - Receive Buffer Not Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbneie(&mut self) -> RBNEIE_W<CTL1_SPEC, 6> {
        RBNEIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbeie(&mut self) -> TBEIE_W<CTL1_SPEC, 7> {
        TBEIE_W::new(self)
    }
    #[doc = "Bits 8:11 - Date size"]
    #[inline(always)]
    #[must_use]
    pub fn dz(&mut self) -> DZ_W<CTL1_SPEC, 8> {
        DZ_W::new(self)
    }
    #[doc = "Bit 12 - Byte access enable"]
    #[inline(always)]
    #[must_use]
    pub fn byten(&mut self) -> BYTEN_W<CTL1_SPEC, 12> {
        BYTEN_W::new(self)
    }
    #[doc = "Bit 13 - Odd bytes in RX DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn rxdma_odd(&mut self) -> RXDMA_ODD_W<CTL1_SPEC, 13> {
        RXDMA_ODD_W::new(self)
    }
    #[doc = "Bit 14 - Odd bytes in TX DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn txdma_odd(&mut self) -> TXDMA_ODD_W<CTL1_SPEC, 14> {
        TXDMA_ODD_W::new(self)
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
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
