#[doc = "Register `CFG0` reader"]
pub type R = crate::R<CFG0_SPEC>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<CFG0_SPEC>;
#[doc = "Field `BOOT_MODE` reader - Boot mode"]
pub type BOOT_MODE_R = crate::FieldReader<BOOT_MODE_A>;
#[doc = "Boot mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOT_MODE_A {
    #[doc = "0: Boot from main flash"]
    FLASH = 0,
    #[doc = "1: Boot from system memory"]
    SYSTEM_MEMORY = 1,
    #[doc = "3: Boot from embedded SRAM"]
    SRAM = 3,
}
impl From<BOOT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOOT_MODE_A {
    type Ux = u8;
}
impl BOOT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BOOT_MODE_A> {
        match self.bits {
            0 => Some(BOOT_MODE_A::FLASH),
            1 => Some(BOOT_MODE_A::SYSTEM_MEMORY),
            3 => Some(BOOT_MODE_A::SRAM),
            _ => None,
        }
    }
    #[doc = "Boot from main flash"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == BOOT_MODE_A::FLASH
    }
    #[doc = "Boot from system memory"]
    #[inline(always)]
    pub fn is_system_memory(&self) -> bool {
        *self == BOOT_MODE_A::SYSTEM_MEMORY
    }
    #[doc = "Boot from embedded SRAM"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == BOOT_MODE_A::SRAM
    }
}
#[doc = "Field `PA11_PA12_RMP` reader - PA11 and PA12 remapping bit for small packages"]
pub type PA11_PA12_RMP_R = crate::BitReader;
#[doc = "Field `PA11_PA12_RMP` writer - PA11 and PA12 remapping bit for small packages"]
pub type PA11_PA12_RMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DMA_RMP` reader - ADC DMA request remapping enable"]
pub type ADC_DMA_RMP_R = crate::BitReader<ADC_DMA_RMP_A>;
#[doc = "ADC DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ADC_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DMA_RMP_A {
        match self.bits {
            false => ADC_DMA_RMP_A::CHANNEL0,
            true => ADC_DMA_RMP_A::CHANNEL1,
        }
    }
    #[doc = "ADC DMA requests are mapped to DMA channel 0"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == ADC_DMA_RMP_A::CHANNEL0
    }
    #[doc = "ADC DMA requests are remapped to DMA channel 1"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == ADC_DMA_RMP_A::CHANNEL1
    }
}
#[doc = "Field `ADC_DMA_RMP` writer - ADC DMA request remapping enable"]
pub type ADC_DMA_RMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADC_DMA_RMP_A>;
impl<'a, REG, const O: u8> ADC_DMA_RMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC DMA requests are mapped to DMA channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DMA_RMP_A::CHANNEL0)
    }
    #[doc = "ADC DMA requests are remapped to DMA channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DMA_RMP_A::CHANNEL1)
    }
}
#[doc = "Field `USART0_TX_DMA_RMP` reader - USART0_TX DMA request remapping enable"]
pub type USART0_TX_DMA_RMP_R = crate::BitReader<USART0_TX_DMA_RMP_A>;
#[doc = "USART0_TX DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl USART0_TX_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART0_TX_DMA_RMP_A {
        match self.bits {
            false => USART0_TX_DMA_RMP_A::CHANNEL1,
            true => USART0_TX_DMA_RMP_A::CHANNEL3,
        }
    }
    #[doc = "USART0 TX DMA requests are mapped to DMA channel 1"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == USART0_TX_DMA_RMP_A::CHANNEL1
    }
    #[doc = "USART0 TX DMA requests are remapped to DMA channel 3"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == USART0_TX_DMA_RMP_A::CHANNEL3
    }
}
#[doc = "Field `USART0_TX_DMA_RMP` writer - USART0_TX DMA request remapping enable"]
pub type USART0_TX_DMA_RMP_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, USART0_TX_DMA_RMP_A>;
impl<'a, REG, const O: u8> USART0_TX_DMA_RMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART0 TX DMA requests are mapped to DMA channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_TX_DMA_RMP_A::CHANNEL1)
    }
    #[doc = "USART0 TX DMA requests are remapped to DMA channel 3"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_TX_DMA_RMP_A::CHANNEL3)
    }
}
#[doc = "Field `USART0_RX_DMA_RMP` reader - USART0_RX DMA request remapping enable"]
pub type USART0_RX_DMA_RMP_R = crate::BitReader<USART0_RX_DMA_RMP_A>;
#[doc = "USART0_RX DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl USART0_RX_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART0_RX_DMA_RMP_A {
        match self.bits {
            false => USART0_RX_DMA_RMP_A::CHANNEL2,
            true => USART0_RX_DMA_RMP_A::CHANNEL4,
        }
    }
    #[doc = "USART0 RX DMA requests are mapped to DMA channel 2"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == USART0_RX_DMA_RMP_A::CHANNEL2
    }
    #[doc = "USART0 RX DMA requests are remapped to DMA channel 4"]
    #[inline(always)]
    pub fn is_channel4(&self) -> bool {
        *self == USART0_RX_DMA_RMP_A::CHANNEL4
    }
}
#[doc = "Field `USART0_RX_DMA_RMP` writer - USART0_RX DMA request remapping enable"]
pub type USART0_RX_DMA_RMP_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, USART0_RX_DMA_RMP_A>;
impl<'a, REG, const O: u8> USART0_RX_DMA_RMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART0 RX DMA requests are mapped to DMA channel 2"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_RX_DMA_RMP_A::CHANNEL2)
    }
    #[doc = "USART0 RX DMA requests are remapped to DMA channel 4"]
    #[inline(always)]
    pub fn channel4(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_RX_DMA_RMP_A::CHANNEL4)
    }
}
#[doc = "Field `TIMER15_DMA_RMP` reader - Timer 15 DMA request remapping enable"]
pub type TIMER15_DMA_RMP_R = crate::BitReader<TIMER15_DMA_RMP_A>;
#[doc = "Timer 15 DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TIMER15_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER15_DMA_RMP_A {
        match self.bits {
            false => TIMER15_DMA_RMP_A::CHANNEL2,
            true => TIMER15_DMA_RMP_A::CHANNEL3,
        }
    }
    #[doc = "Timer 15 DMA requests are mapped to DMA channel 2"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == TIMER15_DMA_RMP_A::CHANNEL2
    }
    #[doc = "Timer 15 DMA requests are remapped to DMA channel 3"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == TIMER15_DMA_RMP_A::CHANNEL3
    }
}
#[doc = "Field `TIMER15_DMA_RMP` writer - Timer 15 DMA request remapping enable"]
pub type TIMER15_DMA_RMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIMER15_DMA_RMP_A>;
impl<'a, REG, const O: u8> TIMER15_DMA_RMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer 15 DMA requests are mapped to DMA channel 2"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER15_DMA_RMP_A::CHANNEL2)
    }
    #[doc = "Timer 15 DMA requests are remapped to DMA channel 3"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER15_DMA_RMP_A::CHANNEL3)
    }
}
#[doc = "Field `TIMER16_DMA_RMP` reader - Timer 16 DMA request remapping enable"]
pub type TIMER16_DMA_RMP_R = crate::BitReader<TIMER16_DMA_RMP_A>;
#[doc = "Timer 16 DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TIMER16_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER16_DMA_RMP_A {
        match self.bits {
            false => TIMER16_DMA_RMP_A::CHANNEL0,
            true => TIMER16_DMA_RMP_A::CHANNEL1,
        }
    }
    #[doc = "Timer 16 DMA requests are mapped to DMA channel 0"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == TIMER16_DMA_RMP_A::CHANNEL0
    }
    #[doc = "Timer 16 DMA requests are remapped to DMA channel 1"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == TIMER16_DMA_RMP_A::CHANNEL1
    }
}
#[doc = "Field `TIMER16_DMA_RMP` writer - Timer 16 DMA request remapping enable"]
pub type TIMER16_DMA_RMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIMER16_DMA_RMP_A>;
impl<'a, REG, const O: u8> TIMER16_DMA_RMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer 16 DMA requests are mapped to DMA channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER16_DMA_RMP_A::CHANNEL0)
    }
    #[doc = "Timer 16 DMA requests are remapped to DMA channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER16_DMA_RMP_A::CHANNEL1)
    }
}
#[doc = "Field `PB9_HCCE` reader - PB9 pin high current capability enable"]
pub type PB9_HCCE_R = crate::BitReader<PB9_HCCE_A>;
#[doc = "PB9 pin high current capability enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PB9_HCCE_A {
    #[doc = "0: High current capability is disabled"]
    LOW_CURRENT = 0,
    #[doc = "1: High current capability is enabled, and speed control is bypassed"]
    HIGH_CURRENT = 1,
}
impl From<PB9_HCCE_A> for bool {
    #[inline(always)]
    fn from(variant: PB9_HCCE_A) -> Self {
        variant as u8 != 0
    }
}
impl PB9_HCCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB9_HCCE_A {
        match self.bits {
            false => PB9_HCCE_A::LOW_CURRENT,
            true => PB9_HCCE_A::HIGH_CURRENT,
        }
    }
    #[doc = "High current capability is disabled"]
    #[inline(always)]
    pub fn is_low_current(&self) -> bool {
        *self == PB9_HCCE_A::LOW_CURRENT
    }
    #[doc = "High current capability is enabled, and speed control is bypassed"]
    #[inline(always)]
    pub fn is_high_current(&self) -> bool {
        *self == PB9_HCCE_A::HIGH_CURRENT
    }
}
#[doc = "Field `PB9_HCCE` writer - PB9 pin high current capability enable"]
pub type PB9_HCCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PB9_HCCE_A>;
impl<'a, REG, const O: u8> PB9_HCCE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High current capability is disabled"]
    #[inline(always)]
    pub fn low_current(self) -> &'a mut crate::W<REG> {
        self.variant(PB9_HCCE_A::LOW_CURRENT)
    }
    #[doc = "High current capability is enabled, and speed control is bypassed"]
    #[inline(always)]
    pub fn high_current(self) -> &'a mut crate::W<REG> {
        self.variant(PB9_HCCE_A::HIGH_CURRENT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Boot mode"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit for small packages"]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&self) -> PA11_PA12_RMP_R {
        PA11_PA12_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC DMA request remapping enable"]
    #[inline(always)]
    pub fn adc_dma_rmp(&self) -> ADC_DMA_RMP_R {
        ADC_DMA_RMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USART0_TX DMA request remapping enable"]
    #[inline(always)]
    pub fn usart0_tx_dma_rmp(&self) -> USART0_TX_DMA_RMP_R {
        USART0_TX_DMA_RMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USART0_RX DMA request remapping enable"]
    #[inline(always)]
    pub fn usart0_rx_dma_rmp(&self) -> USART0_RX_DMA_RMP_R {
        USART0_RX_DMA_RMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer 15 DMA request remapping enable"]
    #[inline(always)]
    pub fn timer15_dma_rmp(&self) -> TIMER15_DMA_RMP_R {
        TIMER15_DMA_RMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer 16 DMA request remapping enable"]
    #[inline(always)]
    pub fn timer16_dma_rmp(&self) -> TIMER16_DMA_RMP_R {
        TIMER16_DMA_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 19 - PB9 pin high current capability enable"]
    #[inline(always)]
    pub fn pb9_hcce(&self) -> PB9_HCCE_R {
        PB9_HCCE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - PA11 and PA12 remapping bit for small packages"]
    #[inline(always)]
    #[must_use]
    pub fn pa11_pa12_rmp(&mut self) -> PA11_PA12_RMP_W<CFG0_SPEC, 4> {
        PA11_PA12_RMP_W::new(self)
    }
    #[doc = "Bit 8 - ADC DMA request remapping enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dma_rmp(&mut self) -> ADC_DMA_RMP_W<CFG0_SPEC, 8> {
        ADC_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 9 - USART0_TX DMA request remapping enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0_tx_dma_rmp(&mut self) -> USART0_TX_DMA_RMP_W<CFG0_SPEC, 9> {
        USART0_TX_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 10 - USART0_RX DMA request remapping enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0_rx_dma_rmp(&mut self) -> USART0_RX_DMA_RMP_W<CFG0_SPEC, 10> {
        USART0_RX_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 11 - Timer 15 DMA request remapping enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer15_dma_rmp(&mut self) -> TIMER15_DMA_RMP_W<CFG0_SPEC, 11> {
        TIMER15_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 12 - Timer 16 DMA request remapping enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer16_dma_rmp(&mut self) -> TIMER16_DMA_RMP_W<CFG0_SPEC, 12> {
        TIMER16_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 19 - PB9 pin high current capability enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb9_hcce(&mut self) -> PB9_HCCE_W<CFG0_SPEC, 19> {
        PB9_HCCE_W::new(self)
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
#[doc = "System configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
