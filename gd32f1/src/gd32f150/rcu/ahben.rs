#[doc = "Register `AHBEN` reader"]
pub type R = crate::R<AHBEN_SPEC>;
#[doc = "Register `AHBEN` writer"]
pub type W = crate::W<AHBEN_SPEC>;
#[doc = "Field `DMAEN` reader - DMA clock enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLED,
            true => DMAEN_A::ENABLED,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::DISABLED
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::ENABLED
    }
}
#[doc = "Field `DMAEN` writer - DMA clock enable"]
pub type DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMAEN_A>;
impl<'a, REG, const O: u8> DMAEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN_A::ENABLED)
    }
}
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub use DMAEN_R as SRAMEN_R;
#[doc = "Field `FMCEN` reader - FMC clock enable"]
pub use DMAEN_R as FMCEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub use DMAEN_R as CRCEN_R;
#[doc = "Field `PAEN` reader - GPIO port A clock enable"]
pub use DMAEN_R as PAEN_R;
#[doc = "Field `PBEN` reader - GPIO port B clock enable"]
pub use DMAEN_R as PBEN_R;
#[doc = "Field `PCEN` reader - GPIO port C clock enable"]
pub use DMAEN_R as PCEN_R;
#[doc = "Field `PDEN` reader - GPIO port D clock enable"]
pub use DMAEN_R as PDEN_R;
#[doc = "Field `PFEN` reader - GPIO port F clock enable"]
pub use DMAEN_R as PFEN_R;
#[doc = "Field `TSIEN` reader - TSI clock enable"]
pub use DMAEN_R as TSIEN_R;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub use DMAEN_W as SRAMEN_W;
#[doc = "Field `FMCEN` writer - FMC clock enable"]
pub use DMAEN_W as FMCEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub use DMAEN_W as CRCEN_W;
#[doc = "Field `PAEN` writer - GPIO port A clock enable"]
pub use DMAEN_W as PAEN_W;
#[doc = "Field `PBEN` writer - GPIO port B clock enable"]
pub use DMAEN_W as PBEN_W;
#[doc = "Field `PCEN` writer - GPIO port C clock enable"]
pub use DMAEN_W as PCEN_W;
#[doc = "Field `PDEN` writer - GPIO port D clock enable"]
pub use DMAEN_W as PDEN_W;
#[doc = "Field `PFEN` writer - GPIO port F clock enable"]
pub use DMAEN_W as PFEN_W;
#[doc = "Field `TSIEN` writer - TSI clock enable"]
pub use DMAEN_W as TSIEN_W;
impl R {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FMC clock enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&self) -> PAEN_R {
        PAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&self) -> PBEN_R {
        PBEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO port D clock enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO port F clock enable"]
    #[inline(always)]
    pub fn pfen(&self) -> PFEN_R {
        PFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - TSI clock enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TSIEN_R {
        TSIEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<AHBEN_SPEC, 0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SRAMEN_W<AHBEN_SPEC, 2> {
        SRAMEN_W::new(self)
    }
    #[doc = "Bit 4 - FMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<AHBEN_SPEC, 4> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHBEN_SPEC, 6> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 17 - GPIO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn paen(&mut self) -> PAEN_W<AHBEN_SPEC, 17> {
        PAEN_W::new(self)
    }
    #[doc = "Bit 18 - GPIO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pben(&mut self) -> PBEN_W<AHBEN_SPEC, 18> {
        PBEN_W::new(self)
    }
    #[doc = "Bit 19 - GPIO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<AHBEN_SPEC, 19> {
        PCEN_W::new(self)
    }
    #[doc = "Bit 20 - GPIO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<AHBEN_SPEC, 20> {
        PDEN_W::new(self)
    }
    #[doc = "Bit 22 - GPIO port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfen(&mut self) -> PFEN_W<AHBEN_SPEC, 22> {
        PFEN_W::new(self)
    }
    #[doc = "Bit 24 - TSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsien(&mut self) -> TSIEN_W<AHBEN_SPEC, 24> {
        TSIEN_W::new(self)
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
#[doc = "AHB enable register (RCU_AHBEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
