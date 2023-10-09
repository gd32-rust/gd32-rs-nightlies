#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `CCSE` reader - Commutation control shadow enable"]
pub type CCSE_R = crate::BitReader<CCSE_A>;
#[doc = "Commutation control shadow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCSE_A {
    #[doc = "0: The shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits are disabled"]
    NOT_PRELOADED = 0,
    #[doc = "1: The shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits are enabled"]
    PRELOADED = 1,
}
impl From<CCSE_A> for bool {
    #[inline(always)]
    fn from(variant: CCSE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCSE_A {
        match self.bits {
            false => CCSE_A::NOT_PRELOADED,
            true => CCSE_A::PRELOADED,
        }
    }
    #[doc = "The shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits are disabled"]
    #[inline(always)]
    pub fn is_not_preloaded(&self) -> bool {
        *self == CCSE_A::NOT_PRELOADED
    }
    #[doc = "The shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits are enabled"]
    #[inline(always)]
    pub fn is_preloaded(&self) -> bool {
        *self == CCSE_A::PRELOADED
    }
}
#[doc = "Field `CCSE` writer - Commutation control shadow enable"]
pub type CCSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCSE_A>;
impl<'a, REG, const O: u8> CCSE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits are disabled"]
    #[inline(always)]
    pub fn not_preloaded(self) -> &'a mut crate::W<REG> {
        self.variant(CCSE_A::NOT_PRELOADED)
    }
    #[doc = "The shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits are enabled"]
    #[inline(always)]
    pub fn preloaded(self) -> &'a mut crate::W<REG> {
        self.variant(CCSE_A::PRELOADED)
    }
}
#[doc = "Field `CCUC` reader - Commutation control shadow register update control"]
pub type CCUC_R = crate::BitReader<CCUC_A>;
#[doc = "Commutation control shadow register update control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUC_A {
    #[doc = "0: Capture/compare are updated only by setting the CMTG bit"]
    DEFAULT = 0,
    #[doc = "1: Capture/compare are updated by setting the CMTG bit or when an rising edge occurs on TRGI"]
    WITH_RISING_EDGE = 1,
}
impl From<CCUC_A> for bool {
    #[inline(always)]
    fn from(variant: CCUC_A) -> Self {
        variant as u8 != 0
    }
}
impl CCUC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCUC_A {
        match self.bits {
            false => CCUC_A::DEFAULT,
            true => CCUC_A::WITH_RISING_EDGE,
        }
    }
    #[doc = "Capture/compare are updated only by setting the CMTG bit"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == CCUC_A::DEFAULT
    }
    #[doc = "Capture/compare are updated by setting the CMTG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn is_with_rising_edge(&self) -> bool {
        *self == CCUC_A::WITH_RISING_EDGE
    }
}
#[doc = "Field `CCUC` writer - Commutation control shadow register update control"]
pub type CCUC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCUC_A>;
impl<'a, REG, const O: u8> CCUC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare are updated only by setting the CMTG bit"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(CCUC_A::DEFAULT)
    }
    #[doc = "Capture/compare are updated by setting the CMTG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn with_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CCUC_A::WITH_RISING_EDGE)
    }
}
#[doc = "Field `DMAS` reader - DMA request source selection"]
pub type DMAS_R = crate::BitReader<DMAS_A>;
#[doc = "DMA request source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAS_A {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    ON_COMPARE = 0,
    #[doc = "1: CCx DMA request sent when update event occurs"]
    ON_UPDATE = 1,
}
impl From<DMAS_A> for bool {
    #[inline(always)]
    fn from(variant: DMAS_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAS_A {
        match self.bits {
            false => DMAS_A::ON_COMPARE,
            true => DMAS_A::ON_UPDATE,
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == DMAS_A::ON_COMPARE
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == DMAS_A::ON_UPDATE
    }
}
#[doc = "Field `DMAS` writer - DMA request source selection"]
pub type DMAS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMAS_A>;
impl<'a, REG, const O: u8> DMAS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut crate::W<REG> {
        self.variant(DMAS_A::ON_COMPARE)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn on_update(self) -> &'a mut crate::W<REG> {
        self.variant(DMAS_A::ON_UPDATE)
    }
}
#[doc = "Field `MMC` reader - Master mode control"]
pub type MMC_R = crate::FieldReader<MMC_A>;
#[doc = "Master mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMC_A {
    #[doc = "0: Use UPG bit from SWEVG register"]
    RESET = 0,
    #[doc = "1: Use CEN bit from CTL0 register"]
    ENABLE = 1,
    #[doc = "2: Use the update event"]
    UPDATE = 2,
    #[doc = "3: The trigger output send a positive pulse when a capture or a compare match occurred in channel 0"]
    CAPTURE_COMPARE_PULSE = 3,
    #[doc = "4: O0CPRE signal is used as trigger output"]
    COMPARE_O0C = 4,
    #[doc = "5: O1CPRE signal is used as trigger output"]
    COMPARE_O1C = 5,
    #[doc = "6: O2CPRE signal is used as trigger output"]
    COMPARE_O2C = 6,
    #[doc = "7: O3CPRE signal is used as trigger output"]
    COMPARE_O3C = 7,
}
impl From<MMC_A> for u8 {
    #[inline(always)]
    fn from(variant: MMC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMC_A {
    type Ux = u8;
}
impl MMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMC_A {
        match self.bits {
            0 => MMC_A::RESET,
            1 => MMC_A::ENABLE,
            2 => MMC_A::UPDATE,
            3 => MMC_A::CAPTURE_COMPARE_PULSE,
            4 => MMC_A::COMPARE_O0C,
            5 => MMC_A::COMPARE_O1C,
            6 => MMC_A::COMPARE_O2C,
            7 => MMC_A::COMPARE_O3C,
            _ => unreachable!(),
        }
    }
    #[doc = "Use UPG bit from SWEVG register"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMC_A::RESET
    }
    #[doc = "Use CEN bit from CTL0 register"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMC_A::ENABLE
    }
    #[doc = "Use the update event"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMC_A::UPDATE
    }
    #[doc = "The trigger output send a positive pulse when a capture or a compare match occurred in channel 0"]
    #[inline(always)]
    pub fn is_capture_compare_pulse(&self) -> bool {
        *self == MMC_A::CAPTURE_COMPARE_PULSE
    }
    #[doc = "O0CPRE signal is used as trigger output"]
    #[inline(always)]
    pub fn is_compare_o0c(&self) -> bool {
        *self == MMC_A::COMPARE_O0C
    }
    #[doc = "O1CPRE signal is used as trigger output"]
    #[inline(always)]
    pub fn is_compare_o1c(&self) -> bool {
        *self == MMC_A::COMPARE_O1C
    }
    #[doc = "O2CPRE signal is used as trigger output"]
    #[inline(always)]
    pub fn is_compare_o2c(&self) -> bool {
        *self == MMC_A::COMPARE_O2C
    }
    #[doc = "O3CPRE signal is used as trigger output"]
    #[inline(always)]
    pub fn is_compare_o3c(&self) -> bool {
        *self == MMC_A::COMPARE_O3C
    }
}
#[doc = "Field `MMC` writer - Master mode control"]
pub type MMC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, MMC_A>;
impl<'a, REG, const O: u8> MMC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use UPG bit from SWEVG register"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(MMC_A::RESET)
    }
    #[doc = "Use CEN bit from CTL0 register"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MMC_A::ENABLE)
    }
    #[doc = "Use the update event"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(MMC_A::UPDATE)
    }
    #[doc = "The trigger output send a positive pulse when a capture or a compare match occurred in channel 0"]
    #[inline(always)]
    pub fn capture_compare_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(MMC_A::CAPTURE_COMPARE_PULSE)
    }
    #[doc = "O0CPRE signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_o0c(self) -> &'a mut crate::W<REG> {
        self.variant(MMC_A::COMPARE_O0C)
    }
    #[doc = "O1CPRE signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_o1c(self) -> &'a mut crate::W<REG> {
        self.variant(MMC_A::COMPARE_O1C)
    }
    #[doc = "O2CPRE signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_o2c(self) -> &'a mut crate::W<REG> {
        self.variant(MMC_A::COMPARE_O2C)
    }
    #[doc = "O3CPRE signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_o3c(self) -> &'a mut crate::W<REG> {
        self.variant(MMC_A::COMPARE_O3C)
    }
}
#[doc = "Field `TI0S` reader - Channel 0 trigger input selection"]
pub type TI0S_R = crate::BitReader<TI0S_A>;
#[doc = "Channel 0 trigger input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI0S_A {
    #[doc = "0: The CH0 pin input is selected as channel 0 trigger input"]
    NORMAL = 0,
    #[doc = "1: The XOR of CH0, CH1 and CH2 pins are selected as channel 0 trigger input"]
    XOR = 1,
}
impl From<TI0S_A> for bool {
    #[inline(always)]
    fn from(variant: TI0S_A) -> Self {
        variant as u8 != 0
    }
}
impl TI0S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI0S_A {
        match self.bits {
            false => TI0S_A::NORMAL,
            true => TI0S_A::XOR,
        }
    }
    #[doc = "The CH0 pin input is selected as channel 0 trigger input"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TI0S_A::NORMAL
    }
    #[doc = "The XOR of CH0, CH1 and CH2 pins are selected as channel 0 trigger input"]
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == TI0S_A::XOR
    }
}
#[doc = "Field `TI0S` writer - Channel 0 trigger input selection"]
pub type TI0S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TI0S_A>;
impl<'a, REG, const O: u8> TI0S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CH0 pin input is selected as channel 0 trigger input"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(TI0S_A::NORMAL)
    }
    #[doc = "The XOR of CH0, CH1 and CH2 pins are selected as channel 0 trigger input"]
    #[inline(always)]
    pub fn xor(self) -> &'a mut crate::W<REG> {
        self.variant(TI0S_A::XOR)
    }
}
#[doc = "Field `ISO0` reader - Idle state of channel 0 output"]
pub type ISO0_R = crate::BitReader<ISO0_A>;
#[doc = "Idle state of channel 0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISO0_A {
    #[doc = "0: CHn_O=0 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    LOW = 0,
    #[doc = "1: CHn_O=1 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    HIGH = 1,
}
impl From<ISO0_A> for bool {
    #[inline(always)]
    fn from(variant: ISO0_A) -> Self {
        variant as u8 != 0
    }
}
impl ISO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISO0_A {
        match self.bits {
            false => ISO0_A::LOW,
            true => ISO0_A::HIGH,
        }
    }
    #[doc = "CHn_O=0 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ISO0_A::LOW
    }
    #[doc = "CHn_O=1 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ISO0_A::HIGH
    }
}
#[doc = "Field `ISO0` writer - Idle state of channel 0 output"]
pub type ISO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ISO0_A>;
impl<'a, REG, const O: u8> ISO0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CHn_O=0 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(ISO0_A::LOW)
    }
    #[doc = "CHn_O=1 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(ISO0_A::HIGH)
    }
}
#[doc = "Field `ISO0N` reader - Idle state of channel 0 complementary output"]
pub type ISO0N_R = crate::BitReader<ISO0N_A>;
#[doc = "Idle state of channel 0 complementary output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISO0N_A {
    #[doc = "0: CHn_ON=0 when POEN=0"]
    LOW = 0,
    #[doc = "1: CHn_ON=1 when POEN=0"]
    HIGH = 1,
}
impl From<ISO0N_A> for bool {
    #[inline(always)]
    fn from(variant: ISO0N_A) -> Self {
        variant as u8 != 0
    }
}
impl ISO0N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISO0N_A {
        match self.bits {
            false => ISO0N_A::LOW,
            true => ISO0N_A::HIGH,
        }
    }
    #[doc = "CHn_ON=0 when POEN=0"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ISO0N_A::LOW
    }
    #[doc = "CHn_ON=1 when POEN=0"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ISO0N_A::HIGH
    }
}
#[doc = "Field `ISO0N` writer - Idle state of channel 0 complementary output"]
pub type ISO0N_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ISO0N_A>;
impl<'a, REG, const O: u8> ISO0N_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CHn_ON=0 when POEN=0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(ISO0N_A::LOW)
    }
    #[doc = "CHn_ON=1 when POEN=0"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(ISO0N_A::HIGH)
    }
}
#[doc = "Field `ISO1N` reader - Idle state of channel 1 complementary output"]
pub use ISO0N_R as ISO1N_R;
#[doc = "Field `ISO2N` reader - Idle state of channel 2 complementary output"]
pub use ISO0N_R as ISO2N_R;
#[doc = "Field `ISO1N` writer - Idle state of channel 1 complementary output"]
pub use ISO0N_W as ISO1N_W;
#[doc = "Field `ISO2N` writer - Idle state of channel 2 complementary output"]
pub use ISO0N_W as ISO2N_W;
#[doc = "Field `ISO1` reader - Idle state of channel 1 output"]
pub use ISO0_R as ISO1_R;
#[doc = "Field `ISO2` reader - Idle state of channel 2 output"]
pub use ISO0_R as ISO2_R;
#[doc = "Field `ISO3` reader - Idle state of channel 3 output"]
pub use ISO0_R as ISO3_R;
#[doc = "Field `ISO1` writer - Idle state of channel 1 output"]
pub use ISO0_W as ISO1_W;
#[doc = "Field `ISO2` writer - Idle state of channel 2 output"]
pub use ISO0_W as ISO2_W;
#[doc = "Field `ISO3` writer - Idle state of channel 3 output"]
pub use ISO0_W as ISO3_W;
impl R {
    #[doc = "Bit 0 - Commutation control shadow enable"]
    #[inline(always)]
    pub fn ccse(&self) -> CCSE_R {
        CCSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    pub fn ccuc(&self) -> CCUC_R {
        CCUC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 0 trigger input selection"]
    #[inline(always)]
    pub fn ti0s(&self) -> TI0S_R {
        TI0S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    pub fn iso0(&self) -> ISO0_R {
        ISO0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Idle state of channel 0 complementary output"]
    #[inline(always)]
    pub fn iso0n(&self) -> ISO0N_R {
        ISO0N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso1(&self) -> ISO1_R {
        ISO1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Idle state of channel 1 complementary output"]
    #[inline(always)]
    pub fn iso1n(&self) -> ISO1N_R {
        ISO1N_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Idle state of channel 2 output"]
    #[inline(always)]
    pub fn iso2(&self) -> ISO2_R {
        ISO2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Idle state of channel 2 complementary output"]
    #[inline(always)]
    pub fn iso2n(&self) -> ISO2N_R {
        ISO2N_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Idle state of channel 3 output"]
    #[inline(always)]
    pub fn iso3(&self) -> ISO3_R {
        ISO3_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Commutation control shadow enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccse(&mut self) -> CCSE_W<CTL1_SPEC, 0> {
        CCSE_W::new(self)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    #[must_use]
    pub fn ccuc(&mut self) -> CCUC_W<CTL1_SPEC, 2> {
        CCUC_W::new(self)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    #[must_use]
    pub fn dmas(&mut self) -> DMAS_W<CTL1_SPEC, 3> {
        DMAS_W::new(self)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    #[must_use]
    pub fn mmc(&mut self) -> MMC_W<CTL1_SPEC, 4> {
        MMC_W::new(self)
    }
    #[doc = "Bit 7 - Channel 0 trigger input selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti0s(&mut self) -> TI0S_W<CTL1_SPEC, 7> {
        TI0S_W::new(self)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso0(&mut self) -> ISO0_W<CTL1_SPEC, 8> {
        ISO0_W::new(self)
    }
    #[doc = "Bit 9 - Idle state of channel 0 complementary output"]
    #[inline(always)]
    #[must_use]
    pub fn iso0n(&mut self) -> ISO0N_W<CTL1_SPEC, 9> {
        ISO0N_W::new(self)
    }
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso1(&mut self) -> ISO1_W<CTL1_SPEC, 10> {
        ISO1_W::new(self)
    }
    #[doc = "Bit 11 - Idle state of channel 1 complementary output"]
    #[inline(always)]
    #[must_use]
    pub fn iso1n(&mut self) -> ISO1N_W<CTL1_SPEC, 11> {
        ISO1N_W::new(self)
    }
    #[doc = "Bit 12 - Idle state of channel 2 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso2(&mut self) -> ISO2_W<CTL1_SPEC, 12> {
        ISO2_W::new(self)
    }
    #[doc = "Bit 13 - Idle state of channel 2 complementary output"]
    #[inline(always)]
    #[must_use]
    pub fn iso2n(&mut self) -> ISO2N_W<CTL1_SPEC, 13> {
        ISO2N_W::new(self)
    }
    #[doc = "Bit 14 - Idle state of channel 3 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso3(&mut self) -> ISO3_W<CTL1_SPEC, 14> {
        ISO3_W::new(self)
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
