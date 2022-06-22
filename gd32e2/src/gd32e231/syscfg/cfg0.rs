#[doc = "Register `CFG0` reader"]
pub struct R(crate::R<CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG0` writer"]
pub struct W(crate::W<CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG0_SPEC>;
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
impl From<crate::W<CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer 16 DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER16_DMA_RMP_A {
    #[doc = "0: Timer 16 DMA requests are mapped to DMA channel 0"]
    CHANNEL0 = 0,
    #[doc = "1: Timer 16 DMA requests are remapped to DMA channel 1"]
    CHANNEL1 = 1,
}
impl From<TIMER16_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER16_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER16_DMA_RMP` reader - Timer 16 DMA request remapping enable"]
pub type TIMER16_DMA_RMP_R = crate::BitReader<TIMER16_DMA_RMP_A>;
impl TIMER16_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER16_DMA_RMP_A {
        match self.bits {
            false => TIMER16_DMA_RMP_A::CHANNEL0,
            true => TIMER16_DMA_RMP_A::CHANNEL1,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL0`"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == TIMER16_DMA_RMP_A::CHANNEL0
    }
    #[doc = "Checks if the value of the field is `CHANNEL1`"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == TIMER16_DMA_RMP_A::CHANNEL1
    }
}
#[doc = "Field `TIMER16_DMA_RMP` writer - Timer 16 DMA request remapping enable"]
pub type TIMER16_DMA_RMP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, TIMER16_DMA_RMP_A, 12>;
impl<'a> TIMER16_DMA_RMP_W<'a> {
    #[doc = "Timer 16 DMA requests are mapped to DMA channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut W {
        self.variant(TIMER16_DMA_RMP_A::CHANNEL0)
    }
    #[doc = "Timer 16 DMA requests are remapped to DMA channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut W {
        self.variant(TIMER16_DMA_RMP_A::CHANNEL1)
    }
}
#[doc = "Timer 15 DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER15_DMA_RMP_A {
    #[doc = "0: Timer 15 DMA requests are mapped to DMA channel 2"]
    CHANNEL2 = 0,
    #[doc = "1: Timer 15 DMA requests are remapped to DMA channel 3"]
    CHANNEL3 = 1,
}
impl From<TIMER15_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER15_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER15_DMA_RMP` reader - Timer 15 DMA request remapping enable"]
pub type TIMER15_DMA_RMP_R = crate::BitReader<TIMER15_DMA_RMP_A>;
impl TIMER15_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER15_DMA_RMP_A {
        match self.bits {
            false => TIMER15_DMA_RMP_A::CHANNEL2,
            true => TIMER15_DMA_RMP_A::CHANNEL3,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL2`"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == TIMER15_DMA_RMP_A::CHANNEL2
    }
    #[doc = "Checks if the value of the field is `CHANNEL3`"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == TIMER15_DMA_RMP_A::CHANNEL3
    }
}
#[doc = "Field `TIMER15_DMA_RMP` writer - Timer 15 DMA request remapping enable"]
pub type TIMER15_DMA_RMP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, TIMER15_DMA_RMP_A, 11>;
impl<'a> TIMER15_DMA_RMP_W<'a> {
    #[doc = "Timer 15 DMA requests are mapped to DMA channel 2"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut W {
        self.variant(TIMER15_DMA_RMP_A::CHANNEL2)
    }
    #[doc = "Timer 15 DMA requests are remapped to DMA channel 3"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut W {
        self.variant(TIMER15_DMA_RMP_A::CHANNEL3)
    }
}
#[doc = "USART0_RX DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART0_RX_DMA_RMP_A {
    #[doc = "0: USART0 RX DMA requests are mapped to DMA channel 2"]
    CHANNEL2 = 0,
    #[doc = "1: USART0 RX DMA requests are remapped to DMA channel 4"]
    CHANNEL4 = 1,
}
impl From<USART0_RX_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USART0_RX_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART0_RX_DMA_RMP` reader - USART0_RX DMA request remapping enable"]
pub type USART0_RX_DMA_RMP_R = crate::BitReader<USART0_RX_DMA_RMP_A>;
impl USART0_RX_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART0_RX_DMA_RMP_A {
        match self.bits {
            false => USART0_RX_DMA_RMP_A::CHANNEL2,
            true => USART0_RX_DMA_RMP_A::CHANNEL4,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL2`"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == USART0_RX_DMA_RMP_A::CHANNEL2
    }
    #[doc = "Checks if the value of the field is `CHANNEL4`"]
    #[inline(always)]
    pub fn is_channel4(&self) -> bool {
        *self == USART0_RX_DMA_RMP_A::CHANNEL4
    }
}
#[doc = "Field `USART0_RX_DMA_RMP` writer - USART0_RX DMA request remapping enable"]
pub type USART0_RX_DMA_RMP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, USART0_RX_DMA_RMP_A, 10>;
impl<'a> USART0_RX_DMA_RMP_W<'a> {
    #[doc = "USART0 RX DMA requests are mapped to DMA channel 2"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut W {
        self.variant(USART0_RX_DMA_RMP_A::CHANNEL2)
    }
    #[doc = "USART0 RX DMA requests are remapped to DMA channel 4"]
    #[inline(always)]
    pub fn channel4(self) -> &'a mut W {
        self.variant(USART0_RX_DMA_RMP_A::CHANNEL4)
    }
}
#[doc = "USART0_TX DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART0_TX_DMA_RMP_A {
    #[doc = "0: USART0 TX DMA requests are mapped to DMA channel 1"]
    CHANNEL1 = 0,
    #[doc = "1: USART0 TX DMA requests are remapped to DMA channel 3"]
    CHANNEL3 = 1,
}
impl From<USART0_TX_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USART0_TX_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART0_TX_DMA_RMP` reader - USART0_TX DMA request remapping enable"]
pub type USART0_TX_DMA_RMP_R = crate::BitReader<USART0_TX_DMA_RMP_A>;
impl USART0_TX_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART0_TX_DMA_RMP_A {
        match self.bits {
            false => USART0_TX_DMA_RMP_A::CHANNEL1,
            true => USART0_TX_DMA_RMP_A::CHANNEL3,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL1`"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == USART0_TX_DMA_RMP_A::CHANNEL1
    }
    #[doc = "Checks if the value of the field is `CHANNEL3`"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == USART0_TX_DMA_RMP_A::CHANNEL3
    }
}
#[doc = "Field `USART0_TX_DMA_RMP` writer - USART0_TX DMA request remapping enable"]
pub type USART0_TX_DMA_RMP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, USART0_TX_DMA_RMP_A, 9>;
impl<'a> USART0_TX_DMA_RMP_W<'a> {
    #[doc = "USART0 TX DMA requests are mapped to DMA channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut W {
        self.variant(USART0_TX_DMA_RMP_A::CHANNEL1)
    }
    #[doc = "USART0 TX DMA requests are remapped to DMA channel 3"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut W {
        self.variant(USART0_TX_DMA_RMP_A::CHANNEL3)
    }
}
#[doc = "ADC DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DMA_RMP_A {
    #[doc = "0: ADC DMA requests are mapped to DMA channel 0"]
    CHANNEL0 = 0,
    #[doc = "1: ADC DMA requests are remapped to DMA channel 1"]
    CHANNEL1 = 1,
}
impl From<ADC_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_DMA_RMP` reader - ADC DMA request remapping enable"]
pub type ADC_DMA_RMP_R = crate::BitReader<ADC_DMA_RMP_A>;
impl ADC_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DMA_RMP_A {
        match self.bits {
            false => ADC_DMA_RMP_A::CHANNEL0,
            true => ADC_DMA_RMP_A::CHANNEL1,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL0`"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == ADC_DMA_RMP_A::CHANNEL0
    }
    #[doc = "Checks if the value of the field is `CHANNEL1`"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == ADC_DMA_RMP_A::CHANNEL1
    }
}
#[doc = "Field `ADC_DMA_RMP` writer - ADC DMA request remapping enable"]
pub type ADC_DMA_RMP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, ADC_DMA_RMP_A, 8>;
impl<'a> ADC_DMA_RMP_W<'a> {
    #[doc = "ADC DMA requests are mapped to DMA channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut W {
        self.variant(ADC_DMA_RMP_A::CHANNEL0)
    }
    #[doc = "ADC DMA requests are remapped to DMA channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut W {
        self.variant(ADC_DMA_RMP_A::CHANNEL1)
    }
}
#[doc = "Field `PA11_PA12_RMP` reader - PA11 and PA12 remapping bit for small packages"]
pub type PA11_PA12_RMP_R = crate::BitReader<bool>;
#[doc = "Field `PA11_PA12_RMP` writer - PA11 and PA12 remapping bit for small packages"]
pub type PA11_PA12_RMP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 4>;
#[doc = "Boot mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOT_MODE_A {
    #[doc = "0: Boot from main flash"]
    FLASH = 0,
    #[doc = "1: Boot from system memory"]
    SYSTEMMEMORY = 1,
    #[doc = "3: Boot from embedded SRAM"]
    SRAM = 3,
}
impl From<BOOT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BOOT_MODE` reader - Boot mode"]
pub type BOOT_MODE_R = crate::FieldReader<u8, BOOT_MODE_A>;
impl BOOT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BOOT_MODE_A> {
        match self.bits {
            0 => Some(BOOT_MODE_A::FLASH),
            1 => Some(BOOT_MODE_A::SYSTEMMEMORY),
            3 => Some(BOOT_MODE_A::SRAM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == BOOT_MODE_A::FLASH
    }
    #[doc = "Checks if the value of the field is `SYSTEMMEMORY`"]
    #[inline(always)]
    pub fn is_system_memory(&self) -> bool {
        *self == BOOT_MODE_A::SYSTEMMEMORY
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == BOOT_MODE_A::SRAM
    }
}
impl R {
    #[doc = "Bit 12 - Timer 16 DMA request remapping enable"]
    #[inline(always)]
    pub fn timer16_dma_rmp(&self) -> TIMER16_DMA_RMP_R {
        TIMER16_DMA_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer 15 DMA request remapping enable"]
    #[inline(always)]
    pub fn timer15_dma_rmp(&self) -> TIMER15_DMA_RMP_R {
        TIMER15_DMA_RMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - USART0_RX DMA request remapping enable"]
    #[inline(always)]
    pub fn usart0_rx_dma_rmp(&self) -> USART0_RX_DMA_RMP_R {
        USART0_RX_DMA_RMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - USART0_TX DMA request remapping enable"]
    #[inline(always)]
    pub fn usart0_tx_dma_rmp(&self) -> USART0_TX_DMA_RMP_R {
        USART0_TX_DMA_RMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC DMA request remapping enable"]
    #[inline(always)]
    pub fn adc_dma_rmp(&self) -> ADC_DMA_RMP_R {
        ADC_DMA_RMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit for small packages"]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&self) -> PA11_PA12_RMP_R {
        PA11_PA12_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Boot mode"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Timer 16 DMA request remapping enable"]
    #[inline(always)]
    pub fn timer16_dma_rmp(&mut self) -> TIMER16_DMA_RMP_W {
        TIMER16_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 11 - Timer 15 DMA request remapping enable"]
    #[inline(always)]
    pub fn timer15_dma_rmp(&mut self) -> TIMER15_DMA_RMP_W {
        TIMER15_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 10 - USART0_RX DMA request remapping enable"]
    #[inline(always)]
    pub fn usart0_rx_dma_rmp(&mut self) -> USART0_RX_DMA_RMP_W {
        USART0_RX_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 9 - USART0_TX DMA request remapping enable"]
    #[inline(always)]
    pub fn usart0_tx_dma_rmp(&mut self) -> USART0_TX_DMA_RMP_W {
        USART0_TX_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 8 - ADC DMA request remapping enable"]
    #[inline(always)]
    pub fn adc_dma_rmp(&mut self) -> ADC_DMA_RMP_W {
        ADC_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit for small packages"]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&mut self) -> PA11_PA12_RMP_W {
        PA11_PA12_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](index.html) module"]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg0::R](R) reader structure"]
impl crate::Readable for CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg0::W](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
