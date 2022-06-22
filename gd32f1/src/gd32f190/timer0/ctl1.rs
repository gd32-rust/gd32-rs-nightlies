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
#[doc = "Idle state of channel 1 output"]
pub use crate::gd32f190::timer0::ctl1::ISO0_A as ISO1_A;
#[doc = "Field `ISO1` reader - Idle state of channel 1 output"]
pub use crate::gd32f190::timer0::ctl1::ISO0_R as ISO1_R;
#[doc = "Idle state of channel 2 complementary output"]
pub use ISO0N_A as ISO2N_A;
#[doc = "Idle state of channel 1 complementary output"]
pub use ISO0N_A as ISO1N_A;
#[doc = "Field `ISO2N` reader - Idle state of channel 2 complementary output"]
pub use ISO0N_R as ISO2N_R;
#[doc = "Field `ISO1N` reader - Idle state of channel 1 complementary output"]
pub use ISO0N_R as ISO1N_R;
#[doc = "Field `ISO2N` writer - Idle state of channel 2 complementary output"]
pub use ISO0N_W as ISO2N_W;
#[doc = "Field `ISO1N` writer - Idle state of channel 1 complementary output"]
pub use ISO0N_W as ISO1N_W;
#[doc = "Idle state of channel 3 output"]
pub use ISO0_A as ISO3_A;
#[doc = "Idle state of channel 2 output"]
pub use ISO0_A as ISO2_A;
#[doc = "Field `ISO3` reader - Idle state of channel 3 output"]
pub use ISO0_R as ISO3_R;
#[doc = "Field `ISO2` reader - Idle state of channel 2 output"]
pub use ISO0_R as ISO2_R;
#[doc = "Field `ISO3` writer - Idle state of channel 3 output"]
pub use ISO0_W as ISO3_W;
#[doc = "Field `ISO2` writer - Idle state of channel 2 output"]
pub use ISO0_W as ISO2_W;
#[doc = "Field `ISO1` writer - Idle state of channel 1 output"]
pub type ISO1_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, ISO1_A, 10>;
impl<'a> ISO1_W<'a> {
    #[doc = "CHn_O=0 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ISO1_A::LOW)
    }
    #[doc = "CHn_O=1 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ISO1_A::HIGH)
    }
}
#[doc = "Idle state of channel 0 complementary output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ISO0N` reader - Idle state of channel 0 complementary output"]
pub type ISO0N_R = crate::BitReader<ISO0N_A>;
impl ISO0N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISO0N_A {
        match self.bits {
            false => ISO0N_A::LOW,
            true => ISO0N_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ISO0N_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ISO0N_A::HIGH
    }
}
#[doc = "Field `ISO0N` writer - Idle state of channel 0 complementary output"]
pub type ISO0N_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, ISO0N_A, 9>;
impl<'a> ISO0N_W<'a> {
    #[doc = "CHn_ON=0 when POEN=0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ISO0N_A::LOW)
    }
    #[doc = "CHn_ON=1 when POEN=0"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ISO0N_A::HIGH)
    }
}
#[doc = "Idle state of channel 0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ISO0` reader - Idle state of channel 0 output"]
pub type ISO0_R = crate::BitReader<ISO0_A>;
impl ISO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISO0_A {
        match self.bits {
            false => ISO0_A::LOW,
            true => ISO0_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ISO0_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ISO0_A::HIGH
    }
}
#[doc = "Field `ISO0` writer - Idle state of channel 0 output"]
pub type ISO0_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, ISO0_A, 8>;
impl<'a> ISO0_W<'a> {
    #[doc = "CHn_O=0 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ISO0_A::LOW)
    }
    #[doc = "CHn_O=1 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ISO0_A::HIGH)
    }
}
#[doc = "Channel 0 trigger input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `TI0S` reader - Channel 0 trigger input selection"]
pub type TI0S_R = crate::BitReader<TI0S_A>;
impl TI0S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI0S_A {
        match self.bits {
            false => TI0S_A::NORMAL,
            true => TI0S_A::XOR,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TI0S_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `XOR`"]
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == TI0S_A::XOR
    }
}
#[doc = "Field `TI0S` writer - Channel 0 trigger input selection"]
pub type TI0S_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, TI0S_A, 7>;
impl<'a> TI0S_W<'a> {
    #[doc = "The CH0 pin input is selected as channel 0 trigger input"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TI0S_A::NORMAL)
    }
    #[doc = "The XOR of CH0, CH1 and CH2 pins are selected as channel 0 trigger input"]
    #[inline(always)]
    pub fn xor(self) -> &'a mut W {
        self.variant(TI0S_A::XOR)
    }
}
#[doc = "Master mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MMC_A {
    #[doc = "0: Use UPG bit from SWEVG register"]
    RESET = 0,
    #[doc = "1: Use CEN bit from CTL0 register"]
    ENABLE = 1,
    #[doc = "2: Use the update event"]
    UPDATE = 2,
    #[doc = "3: The trigger output send a positive pulse when a capture or a compare match occurred in channel 0"]
    CAPTURECOMPAREPULSE = 3,
    #[doc = "4: O0CPRE signal is used as trigger output"]
    COMPAREO0C = 4,
    #[doc = "5: O1CPRE signal is used as trigger output"]
    COMPAREO1C = 5,
    #[doc = "6: O2CPRE signal is used as trigger output"]
    COMPAREO2C = 6,
    #[doc = "7: O3CPRE signal is used as trigger output"]
    COMPAREO3C = 7,
}
impl From<MMC_A> for u8 {
    #[inline(always)]
    fn from(variant: MMC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MMC` reader - Master mode control"]
pub type MMC_R = crate::FieldReader<u8, MMC_A>;
impl MMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMC_A {
        match self.bits {
            0 => MMC_A::RESET,
            1 => MMC_A::ENABLE,
            2 => MMC_A::UPDATE,
            3 => MMC_A::CAPTURECOMPAREPULSE,
            4 => MMC_A::COMPAREO0C,
            5 => MMC_A::COMPAREO1C,
            6 => MMC_A::COMPAREO2C,
            7 => MMC_A::COMPAREO3C,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMC_A::RESET
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMC_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMC_A::UPDATE
    }
    #[doc = "Checks if the value of the field is `CAPTURECOMPAREPULSE`"]
    #[inline(always)]
    pub fn is_capture_compare_pulse(&self) -> bool {
        *self == MMC_A::CAPTURECOMPAREPULSE
    }
    #[doc = "Checks if the value of the field is `COMPAREO0C`"]
    #[inline(always)]
    pub fn is_compare_o0c(&self) -> bool {
        *self == MMC_A::COMPAREO0C
    }
    #[doc = "Checks if the value of the field is `COMPAREO1C`"]
    #[inline(always)]
    pub fn is_compare_o1c(&self) -> bool {
        *self == MMC_A::COMPAREO1C
    }
    #[doc = "Checks if the value of the field is `COMPAREO2C`"]
    #[inline(always)]
    pub fn is_compare_o2c(&self) -> bool {
        *self == MMC_A::COMPAREO2C
    }
    #[doc = "Checks if the value of the field is `COMPAREO3C`"]
    #[inline(always)]
    pub fn is_compare_o3c(&self) -> bool {
        *self == MMC_A::COMPAREO3C
    }
}
#[doc = "Field `MMC` writer - Master mode control"]
pub type MMC_W<'a> = crate::FieldWriterSafe<'a, u16, CTL1_SPEC, u8, MMC_A, 3, 4>;
impl<'a> MMC_W<'a> {
    #[doc = "Use UPG bit from SWEVG register"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMC_A::RESET)
    }
    #[doc = "Use CEN bit from CTL0 register"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMC_A::ENABLE)
    }
    #[doc = "Use the update event"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMC_A::UPDATE)
    }
    #[doc = "The trigger output send a positive pulse when a capture or a compare match occurred in channel 0"]
    #[inline(always)]
    pub fn capture_compare_pulse(self) -> &'a mut W {
        self.variant(MMC_A::CAPTURECOMPAREPULSE)
    }
    #[doc = "O0CPRE signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_o0c(self) -> &'a mut W {
        self.variant(MMC_A::COMPAREO0C)
    }
    #[doc = "O1CPRE signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_o1c(self) -> &'a mut W {
        self.variant(MMC_A::COMPAREO1C)
    }
    #[doc = "O2CPRE signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_o2c(self) -> &'a mut W {
        self.variant(MMC_A::COMPAREO2C)
    }
    #[doc = "O3CPRE signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_o3c(self) -> &'a mut W {
        self.variant(MMC_A::COMPAREO3C)
    }
}
#[doc = "DMA request source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAS_A {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    ONCOMPARE = 0,
    #[doc = "1: CCx DMA request sent when update event occurs"]
    ONUPDATE = 1,
}
impl From<DMAS_A> for bool {
    #[inline(always)]
    fn from(variant: DMAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAS` reader - DMA request source selection"]
pub type DMAS_R = crate::BitReader<DMAS_A>;
impl DMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAS_A {
        match self.bits {
            false => DMAS_A::ONCOMPARE,
            true => DMAS_A::ONUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `ONCOMPARE`"]
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == DMAS_A::ONCOMPARE
    }
    #[doc = "Checks if the value of the field is `ONUPDATE`"]
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == DMAS_A::ONUPDATE
    }
}
#[doc = "Field `DMAS` writer - DMA request source selection"]
pub type DMAS_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, DMAS_A, 3>;
impl<'a> DMAS_W<'a> {
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut W {
        self.variant(DMAS_A::ONCOMPARE)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn on_update(self) -> &'a mut W {
        self.variant(DMAS_A::ONUPDATE)
    }
}
#[doc = "Commutation control shadow register update control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUC_A {
    #[doc = "0: Capture/compare are updated only by setting the CMTG bit"]
    DEFAULT = 0,
    #[doc = "1: Capture/compare are updated by setting the CMTG bit or when an rising edge occurs on TRGI"]
    WITHRISINGEDGE = 1,
}
impl From<CCUC_A> for bool {
    #[inline(always)]
    fn from(variant: CCUC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUC` reader - Commutation control shadow register update control"]
pub type CCUC_R = crate::BitReader<CCUC_A>;
impl CCUC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCUC_A {
        match self.bits {
            false => CCUC_A::DEFAULT,
            true => CCUC_A::WITHRISINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == CCUC_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `WITHRISINGEDGE`"]
    #[inline(always)]
    pub fn is_with_rising_edge(&self) -> bool {
        *self == CCUC_A::WITHRISINGEDGE
    }
}
#[doc = "Field `CCUC` writer - Commutation control shadow register update control"]
pub type CCUC_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, CCUC_A, 2>;
impl<'a> CCUC_W<'a> {
    #[doc = "Capture/compare are updated only by setting the CMTG bit"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(CCUC_A::DEFAULT)
    }
    #[doc = "Capture/compare are updated by setting the CMTG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn with_rising_edge(self) -> &'a mut W {
        self.variant(CCUC_A::WITHRISINGEDGE)
    }
}
#[doc = "Commutation control shadow register enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCSE_A {
    #[doc = "0: The shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits are disabled"]
    NOTPRELOADED = 0,
    #[doc = "1: The shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits are enabled"]
    PRELOADED = 1,
}
impl From<CCSE_A> for bool {
    #[inline(always)]
    fn from(variant: CCSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCSE` reader - Commutation control shadow register enable"]
pub type CCSE_R = crate::BitReader<CCSE_A>;
impl CCSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCSE_A {
        match self.bits {
            false => CCSE_A::NOTPRELOADED,
            true => CCSE_A::PRELOADED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRELOADED`"]
    #[inline(always)]
    pub fn is_not_preloaded(&self) -> bool {
        *self == CCSE_A::NOTPRELOADED
    }
    #[doc = "Checks if the value of the field is `PRELOADED`"]
    #[inline(always)]
    pub fn is_preloaded(&self) -> bool {
        *self == CCSE_A::PRELOADED
    }
}
#[doc = "Field `CCSE` writer - Commutation control shadow register enable"]
pub type CCSE_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, CCSE_A, 0>;
impl<'a> CCSE_W<'a> {
    #[doc = "The shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits are disabled"]
    #[inline(always)]
    pub fn not_preloaded(self) -> &'a mut W {
        self.variant(CCSE_A::NOTPRELOADED)
    }
    #[doc = "The shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits are enabled"]
    #[inline(always)]
    pub fn preloaded(self) -> &'a mut W {
        self.variant(CCSE_A::PRELOADED)
    }
}
impl R {
    #[doc = "Bit 14 - Idle state of channel 3 output"]
    #[inline(always)]
    pub fn iso3(&self) -> ISO3_R {
        ISO3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Idle state of channel 2 complementary output"]
    #[inline(always)]
    pub fn iso2n(&self) -> ISO2N_R {
        ISO2N_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Idle state of channel 2 output"]
    #[inline(always)]
    pub fn iso2(&self) -> ISO2_R {
        ISO2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Idle state of channel 1 complementary output"]
    #[inline(always)]
    pub fn iso1n(&self) -> ISO1N_R {
        ISO1N_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso1(&self) -> ISO1_R {
        ISO1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Idle state of channel 0 complementary output"]
    #[inline(always)]
    pub fn iso0n(&self) -> ISO0N_R {
        ISO0N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    pub fn iso0(&self) -> ISO0_R {
        ISO0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 0 trigger input selection"]
    #[inline(always)]
    pub fn ti0s(&self) -> TI0S_R {
        TI0S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    pub fn ccuc(&self) -> CCUC_R {
        CCUC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - Commutation control shadow register enable"]
    #[inline(always)]
    pub fn ccse(&self) -> CCSE_R {
        CCSE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Idle state of channel 3 output"]
    #[inline(always)]
    pub fn iso3(&mut self) -> ISO3_W {
        ISO3_W::new(self)
    }
    #[doc = "Bit 13 - Idle state of channel 2 complementary output"]
    #[inline(always)]
    pub fn iso2n(&mut self) -> ISO2N_W {
        ISO2N_W::new(self)
    }
    #[doc = "Bit 12 - Idle state of channel 2 output"]
    #[inline(always)]
    pub fn iso2(&mut self) -> ISO2_W {
        ISO2_W::new(self)
    }
    #[doc = "Bit 11 - Idle state of channel 1 complementary output"]
    #[inline(always)]
    pub fn iso1n(&mut self) -> ISO1N_W {
        ISO1N_W::new(self)
    }
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso1(&mut self) -> ISO1_W {
        ISO1_W::new(self)
    }
    #[doc = "Bit 9 - Idle state of channel 0 complementary output"]
    #[inline(always)]
    pub fn iso0n(&mut self) -> ISO0N_W {
        ISO0N_W::new(self)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    pub fn iso0(&mut self) -> ISO0_W {
        ISO0_W::new(self)
    }
    #[doc = "Bit 7 - Channel 0 trigger input selection"]
    #[inline(always)]
    pub fn ti0s(&mut self) -> TI0S_W {
        TI0S_W::new(self)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    pub fn mmc(&mut self) -> MMC_W {
        MMC_W::new(self)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&mut self) -> DMAS_W {
        DMAS_W::new(self)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    pub fn ccuc(&mut self) -> CCUC_W {
        CCUC_W::new(self)
    }
    #[doc = "Bit 0 - Commutation control shadow register enable"]
    #[inline(always)]
    pub fn ccse(&mut self) -> CCSE_W {
        CCSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u16;
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
