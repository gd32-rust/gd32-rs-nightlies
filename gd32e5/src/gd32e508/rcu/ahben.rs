#[doc = "Register `AHBEN` reader"]
pub type R = crate::R<AHBEN_SPEC>;
#[doc = "Register `AHBEN` writer"]
pub type W = crate::W<AHBEN_SPEC>;
#[doc = "Field `DMA0EN` reader - DMA0 clock enable"]
pub type DMA0EN_R = crate::BitReader;
#[doc = "Field `DMA0EN` writer - DMA0 clock enable"]
pub type DMA0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type DMA1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAMSPEN` reader - SRAM interface clock enable when sleep mode"]
pub type SRAMSPEN_R = crate::BitReader;
#[doc = "Field `SRAMSPEN` writer - SRAM interface clock enable when sleep mode"]
pub type SRAMSPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMCSPEN` reader - FMC clock enable when sleep mode"]
pub type FMCSPEN_R = crate::BitReader;
#[doc = "Field `FMCSPEN` writer - FMC clock enable when sleep mode"]
pub type FMCSPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CRCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXMCEN` reader - EXMC clock enable"]
pub type EXMCEN_R = crate::BitReader;
#[doc = "Field `EXMCEN` writer - EXMC clock enable"]
pub type EXMCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHSEN` reader - USBFS clock enable"]
pub type USBHSEN_R = crate::BitReader;
#[doc = "Field `USBHSEN` writer - USBFS clock enable"]
pub type USBHSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ULPIEN` reader - ULPI clock enable"]
pub type ULPIEN_R = crate::BitReader;
#[doc = "Field `ULPIEN` writer - ULPI clock enable"]
pub type ULPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENETEN` reader - Ethernet clock enable"]
pub type ENETEN_R = crate::BitReader;
#[doc = "Field `ENETEN` writer - Ethernet clock enable"]
pub type ENETEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENETTXEN` reader - Ethernet TX clock enable"]
pub type ENETTXEN_R = crate::BitReader;
#[doc = "Field `ENETTXEN` writer - Ethernet TX clock enable"]
pub type ENETTXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENETRXEN` reader - Ethernet RX clock enable"]
pub type ENETRXEN_R = crate::BitReader;
#[doc = "Field `ENETRXEN` writer - Ethernet RX clock enable"]
pub type ENETRXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMUEN` reader - TMU clock enable"]
pub type TMUEN_R = crate::BitReader;
#[doc = "Field `TMUEN` writer - TMU clock enable"]
pub type TMUEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SQPIEN` reader - SQPI clock enable"]
pub type SQPIEN_R = crate::BitReader;
#[doc = "Field `SQPIEN` writer - SQPI clock enable"]
pub type SQPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DMA0 clock enable"]
    #[inline(always)]
    pub fn dma0en(&self) -> DMA0EN_R {
        DMA0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable when sleep mode"]
    #[inline(always)]
    pub fn sramspen(&self) -> SRAMSPEN_R {
        SRAMSPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FMC clock enable when sleep mode"]
    #[inline(always)]
    pub fn fmcspen(&self) -> FMCSPEN_R {
        FMCSPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - EXMC clock enable"]
    #[inline(always)]
    pub fn exmcen(&self) -> EXMCEN_R {
        EXMCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - USBFS clock enable"]
    #[inline(always)]
    pub fn usbhsen(&self) -> USBHSEN_R {
        USBHSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ULPI clock enable"]
    #[inline(always)]
    pub fn ulpien(&self) -> ULPIEN_R {
        ULPIEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Ethernet clock enable"]
    #[inline(always)]
    pub fn eneten(&self) -> ENETEN_R {
        ENETEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ethernet TX clock enable"]
    #[inline(always)]
    pub fn enettxen(&self) -> ENETTXEN_R {
        ENETTXEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ethernet RX clock enable"]
    #[inline(always)]
    pub fn enetrxen(&self) -> ENETRXEN_R {
        ENETRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - TMU clock enable"]
    #[inline(always)]
    pub fn tmuen(&self) -> TMUEN_R {
        TMUEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SQPI clock enable"]
    #[inline(always)]
    pub fn sqpien(&self) -> SQPIEN_R {
        SQPIEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma0en(&mut self) -> DMA0EN_W<AHBEN_SPEC, 0> {
        DMA0EN_W::new(self)
    }
    #[doc = "Bit 1 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHBEN_SPEC, 1> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 2 - SRAM interface clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sramspen(&mut self) -> SRAMSPEN_W<AHBEN_SPEC, 2> {
        SRAMSPEN_W::new(self)
    }
    #[doc = "Bit 4 - FMC clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmcspen(&mut self) -> FMCSPEN_W<AHBEN_SPEC, 4> {
        FMCSPEN_W::new(self)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHBEN_SPEC, 6> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 8 - EXMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn exmcen(&mut self) -> EXMCEN_W<AHBEN_SPEC, 8> {
        EXMCEN_W::new(self)
    }
    #[doc = "Bit 12 - USBFS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbhsen(&mut self) -> USBHSEN_W<AHBEN_SPEC, 12> {
        USBHSEN_W::new(self)
    }
    #[doc = "Bit 13 - ULPI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ulpien(&mut self) -> ULPIEN_W<AHBEN_SPEC, 13> {
        ULPIEN_W::new(self)
    }
    #[doc = "Bit 14 - Ethernet clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn eneten(&mut self) -> ENETEN_W<AHBEN_SPEC, 14> {
        ENETEN_W::new(self)
    }
    #[doc = "Bit 15 - Ethernet TX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn enettxen(&mut self) -> ENETTXEN_W<AHBEN_SPEC, 15> {
        ENETTXEN_W::new(self)
    }
    #[doc = "Bit 16 - Ethernet RX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn enetrxen(&mut self) -> ENETRXEN_W<AHBEN_SPEC, 16> {
        ENETRXEN_W::new(self)
    }
    #[doc = "Bit 30 - TMU clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmuen(&mut self) -> TMUEN_W<AHBEN_SPEC, 30> {
        TMUEN_W::new(self)
    }
    #[doc = "Bit 31 - SQPI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sqpien(&mut self) -> SQPIEN_W<AHBEN_SPEC, 31> {
        SQPIEN_W::new(self)
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
#[doc = "AHB enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN_SPEC;
impl crate::RegisterSpec for AHBEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben::R`](R) reader structure"]
impl crate::Readable for AHBEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben::W`](W) writer structure"]
impl crate::Writable for AHBEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBEN to value 0x14"]
impl crate::Resettable for AHBEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
