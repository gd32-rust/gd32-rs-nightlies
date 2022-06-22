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
#[doc = "Field `PFEN` reader - GPIO port F clock enable"]
pub type PFEN_R = crate::BitReader<bool>;
#[doc = "Field `PFEN` writer - GPIO port F clock enable"]
pub type PFEN_W<'a> = crate::BitWriter<'a, u32, AHBEN_SPEC, bool, 22>;
#[doc = "Field `PCEN` reader - GPIO port C clock enable"]
pub type PCEN_R = crate::BitReader<bool>;
#[doc = "Field `PCEN` writer - GPIO port C clock enable"]
pub type PCEN_W<'a> = crate::BitWriter<'a, u32, AHBEN_SPEC, bool, 19>;
#[doc = "Field `PBEN` reader - GPIO port B clock enable"]
pub type PBEN_R = crate::BitReader<bool>;
#[doc = "Field `PBEN` writer - GPIO port B clock enable"]
pub type PBEN_W<'a> = crate::BitWriter<'a, u32, AHBEN_SPEC, bool, 18>;
#[doc = "Field `PAEN` reader - GPIO port A clock enable"]
pub type PAEN_R = crate::BitReader<bool>;
#[doc = "Field `PAEN` writer - GPIO port A clock enable"]
pub type PAEN_W<'a> = crate::BitWriter<'a, u32, AHBEN_SPEC, bool, 17>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CRCEN_W<'a> = crate::BitWriter<'a, u32, AHBEN_SPEC, bool, 6>;
#[doc = "Field `FMCSPEN` reader - FMC clock during sleep mode enable"]
pub type FMCSPEN_R = crate::BitReader<bool>;
#[doc = "Field `FMCSPEN` writer - FMC clock during sleep mode enable"]
pub type FMCSPEN_W<'a> = crate::BitWriter<'a, u32, AHBEN_SPEC, bool, 4>;
#[doc = "Field `SRAMSPEN` reader - SRAM interface clock during sleep mode enable"]
pub type SRAMSPEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAMSPEN` writer - SRAM interface clock during sleep mode enable"]
pub type SRAMSPEN_W<'a> = crate::BitWriter<'a, u32, AHBEN_SPEC, bool, 2>;
#[doc = "Field `DMAEN` reader - DMA clock enable"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA clock enable"]
pub type DMAEN_W<'a> = crate::BitWriter<'a, u32, AHBEN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 22 - GPIO port F clock enable"]
    #[inline(always)]
    pub fn pfen(&self) -> PFEN_R {
        PFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&self) -> PBEN_R {
        PBEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&self) -> PAEN_R {
        PAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - FMC clock during sleep mode enable"]
    #[inline(always)]
    pub fn fmcspen(&self) -> FMCSPEN_R {
        FMCSPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock during sleep mode enable"]
    #[inline(always)]
    pub fn sramspen(&self) -> SRAMSPEN_R {
        SRAMSPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - GPIO port F clock enable"]
    #[inline(always)]
    pub fn pfen(&mut self) -> PFEN_W {
        PFEN_W::new(self)
    }
    #[doc = "Bit 19 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PCEN_W {
        PCEN_W::new(self)
    }
    #[doc = "Bit 18 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&mut self) -> PBEN_W {
        PBEN_W::new(self)
    }
    #[doc = "Bit 17 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&mut self) -> PAEN_W {
        PAEN_W::new(self)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 4 - FMC clock during sleep mode enable"]
    #[inline(always)]
    pub fn fmcspen(&mut self) -> FMCSPEN_W {
        FMCSPEN_W::new(self)
    }
    #[doc = "Bit 2 - SRAM interface clock during sleep mode enable"]
    #[inline(always)]
    pub fn sramspen(&mut self) -> SRAMSPEN_W {
        SRAMSPEN_W::new(self)
    }
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W::new(self)
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
