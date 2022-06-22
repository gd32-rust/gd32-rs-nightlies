#[doc = "Register `AHBEN` reader"]
pub struct R(crate::R<AHBEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBEN` writer"]
pub struct W(crate::W<AHBEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBEN_SPEC>;
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
impl From<crate::W<AHBEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DMAEN` reader - DMA clock enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLED,
            true => DMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::ENABLED
    }
}
#[doc = "Field `DMAEN` writer - DMA clock enable"]
pub type DMAEN_W<'a> = crate::BitWriter<'a, u32, AHBEN_SPEC, DMAEN_A, 0>;
impl<'a> DMAEN_W<'a> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLED)
    }
}
#[doc = "SRAM interface clock enable"]
pub use DMAEN_A as SRAMEN_A;
#[doc = "FMC clock enable"]
pub use DMAEN_A as FMCEN_A;
#[doc = "CRC clock enable"]
pub use DMAEN_A as CRCEN_A;
#[doc = "GPIO port A clock enable"]
pub use DMAEN_A as PAEN_A;
#[doc = "GPIO port B clock enable"]
pub use DMAEN_A as PBEN_A;
#[doc = "GPIO port C clock enable"]
pub use DMAEN_A as PCEN_A;
#[doc = "GPIO port D clock enable"]
pub use DMAEN_A as PDEN_A;
#[doc = "GPIO port F clock enable"]
pub use DMAEN_A as PFEN_A;
#[doc = "TSI clock enable"]
pub use DMAEN_A as TSIEN_A;
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
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W {
        SRAMEN_W::new(self)
    }
    #[doc = "Bit 4 - FMC clock enable"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 17 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&mut self) -> PAEN_W {
        PAEN_W::new(self)
    }
    #[doc = "Bit 18 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&mut self) -> PBEN_W {
        PBEN_W::new(self)
    }
    #[doc = "Bit 19 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PCEN_W {
        PCEN_W::new(self)
    }
    #[doc = "Bit 20 - GPIO port D clock enable"]
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W::new(self)
    }
    #[doc = "Bit 22 - GPIO port F clock enable"]
    #[inline(always)]
    pub fn pfen(&mut self) -> PFEN_W {
        PFEN_W::new(self)
    }
    #[doc = "Bit 24 - TSI clock enable"]
    #[inline(always)]
    pub fn tsien(&mut self) -> TSIEN_W {
        TSIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB enable register (RCU_AHBEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahben](index.html) module"]
pub struct AHBEN_SPEC;
impl crate::RegisterSpec for AHBEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahben::R](R) reader structure"]
impl crate::Readable for AHBEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahben::W](W) writer structure"]
impl crate::Writable for AHBEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBEN to value 0x14"]
impl crate::Resettable for AHBEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}
