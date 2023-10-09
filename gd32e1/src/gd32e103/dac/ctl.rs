#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `DEN0` reader - DAC0 enable"]
pub type DEN0_R = crate::BitReader<DEN0_A>;
#[doc = "DAC0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEN0_A {
    #[doc = "0: DAC channel disabled"]
    DISABLED = 0,
    #[doc = "1: DAC channel enabled"]
    ENABLED = 1,
}
impl From<DEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl DEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN0_A {
        match self.bits {
            false => DEN0_A::DISABLED,
            true => DEN0_A::ENABLED,
        }
    }
    #[doc = "DAC channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEN0_A::DISABLED
    }
    #[doc = "DAC channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEN0_A::ENABLED
    }
}
#[doc = "Field `DEN0` writer - DAC0 enable"]
pub type DEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DEN0_A>;
impl<'a, REG, const O: u8> DEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEN0_A::DISABLED)
    }
    #[doc = "DAC channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEN0_A::ENABLED)
    }
}
#[doc = "Field `DBOFF0` reader - DAC0 output buffer turn off"]
pub type DBOFF0_R = crate::BitReader<DBOFF0_A>;
#[doc = "DAC0 output buffer turn off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBOFF0_A {
    #[doc = "0: DAC X output buffer enabled"]
    ENABLED = 0,
    #[doc = "1: DAC X output buffer disabled"]
    DISABLED = 1,
}
impl From<DBOFF0_A> for bool {
    #[inline(always)]
    fn from(variant: DBOFF0_A) -> Self {
        variant as u8 != 0
    }
}
impl DBOFF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBOFF0_A {
        match self.bits {
            false => DBOFF0_A::ENABLED,
            true => DBOFF0_A::DISABLED,
        }
    }
    #[doc = "DAC X output buffer enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBOFF0_A::ENABLED
    }
    #[doc = "DAC X output buffer disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBOFF0_A::DISABLED
    }
}
#[doc = "Field `DBOFF0` writer - DAC0 output buffer turn off"]
pub type DBOFF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DBOFF0_A>;
impl<'a, REG, const O: u8> DBOFF0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC X output buffer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBOFF0_A::ENABLED)
    }
    #[doc = "DAC X output buffer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBOFF0_A::DISABLED)
    }
}
#[doc = "Field `DTEN0` reader - DAC0 trigger enable"]
pub type DTEN0_R = crate::BitReader<DTEN0_A>;
#[doc = "DAC0 trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN0_A {
    #[doc = "0: DAC trigger disabled"]
    DISABLED = 0,
    #[doc = "1: DAC trigger enabled"]
    ENABLED = 1,
}
impl From<DTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl DTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN0_A {
        match self.bits {
            false => DTEN0_A::DISABLED,
            true => DTEN0_A::ENABLED,
        }
    }
    #[doc = "DAC trigger disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTEN0_A::DISABLED
    }
    #[doc = "DAC trigger enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTEN0_A::ENABLED
    }
}
#[doc = "Field `DTEN0` writer - DAC0 trigger enable"]
pub type DTEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DTEN0_A>;
impl<'a, REG, const O: u8> DTEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN0_A::DISABLED)
    }
    #[doc = "DAC trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN0_A::ENABLED)
    }
}
#[doc = "Field `DTSEL0` reader - DAC0 trigger selection"]
pub type DTSEL0_R = crate::FieldReader<DTSEL0_A>;
#[doc = "DAC0 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTSEL0_A {
    #[doc = "0: Timer 5 TRGO event"]
    TIMER5_TRGO = 0,
    #[doc = "1: Timer 2 TRGO event"]
    TIMER2_TRGO = 1,
    #[doc = "3: Timer 14 TRGO event"]
    TIMER14_TRGO = 3,
    #[doc = "4: Timer 1 TRGO event"]
    TIMER1_TRGO = 4,
    #[doc = "6: External line9"]
    EXTERNAL9 = 6,
    #[doc = "7: Software trigger"]
    SOFTWARE = 7,
}
impl From<DTSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: DTSEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTSEL0_A {
    type Ux = u8;
}
impl DTSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTSEL0_A> {
        match self.bits {
            0 => Some(DTSEL0_A::TIMER5_TRGO),
            1 => Some(DTSEL0_A::TIMER2_TRGO),
            3 => Some(DTSEL0_A::TIMER14_TRGO),
            4 => Some(DTSEL0_A::TIMER1_TRGO),
            6 => Some(DTSEL0_A::EXTERNAL9),
            7 => Some(DTSEL0_A::SOFTWARE),
            _ => None,
        }
    }
    #[doc = "Timer 5 TRGO event"]
    #[inline(always)]
    pub fn is_timer5_trgo(&self) -> bool {
        *self == DTSEL0_A::TIMER5_TRGO
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn is_timer2_trgo(&self) -> bool {
        *self == DTSEL0_A::TIMER2_TRGO
    }
    #[doc = "Timer 14 TRGO event"]
    #[inline(always)]
    pub fn is_timer14_trgo(&self) -> bool {
        *self == DTSEL0_A::TIMER14_TRGO
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn is_timer1_trgo(&self) -> bool {
        *self == DTSEL0_A::TIMER1_TRGO
    }
    #[doc = "External line9"]
    #[inline(always)]
    pub fn is_external9(&self) -> bool {
        *self == DTSEL0_A::EXTERNAL9
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == DTSEL0_A::SOFTWARE
    }
}
#[doc = "Field `DTSEL0` writer - DAC0 trigger selection"]
pub type DTSEL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, DTSEL0_A>;
impl<'a, REG, const O: u8> DTSEL0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 5 TRGO event"]
    #[inline(always)]
    pub fn timer5_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEL0_A::TIMER5_TRGO)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn timer2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEL0_A::TIMER2_TRGO)
    }
    #[doc = "Timer 14 TRGO event"]
    #[inline(always)]
    pub fn timer14_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEL0_A::TIMER14_TRGO)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn timer1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEL0_A::TIMER1_TRGO)
    }
    #[doc = "External line9"]
    #[inline(always)]
    pub fn external9(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEL0_A::EXTERNAL9)
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEL0_A::SOFTWARE)
    }
}
#[doc = "Field `DWM0` reader - DAC0 noise wave mode"]
pub type DWM0_R = crate::FieldReader<DWM0_A>;
#[doc = "DAC0 noise wave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DWM0_A {
    #[doc = "0: Wave disabled"]
    WAVE_DISABLED = 0,
    #[doc = "1: LFSR noise mode"]
    LFSR = 1,
    #[doc = "2: Triangle noise mode"]
    TRIANGLE = 2,
}
impl From<DWM0_A> for u8 {
    #[inline(always)]
    fn from(variant: DWM0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DWM0_A {
    type Ux = u8;
}
impl DWM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DWM0_A> {
        match self.bits {
            0 => Some(DWM0_A::WAVE_DISABLED),
            1 => Some(DWM0_A::LFSR),
            2 => Some(DWM0_A::TRIANGLE),
            _ => None,
        }
    }
    #[doc = "Wave disabled"]
    #[inline(always)]
    pub fn is_wave_disabled(&self) -> bool {
        *self == DWM0_A::WAVE_DISABLED
    }
    #[doc = "LFSR noise mode"]
    #[inline(always)]
    pub fn is_lfsr(&self) -> bool {
        *self == DWM0_A::LFSR
    }
    #[doc = "Triangle noise mode"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == DWM0_A::TRIANGLE
    }
}
#[doc = "Field `DWM0` writer - DAC0 noise wave mode"]
pub type DWM0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DWM0_A>;
impl<'a, REG, const O: u8> DWM0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Wave disabled"]
    #[inline(always)]
    pub fn wave_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DWM0_A::WAVE_DISABLED)
    }
    #[doc = "LFSR noise mode"]
    #[inline(always)]
    pub fn lfsr(self) -> &'a mut crate::W<REG> {
        self.variant(DWM0_A::LFSR)
    }
    #[doc = "Triangle noise mode"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(DWM0_A::TRIANGLE)
    }
}
#[doc = "Field `DWBW0` reader - DAC0 noise wave bit width"]
pub type DWBW0_R = crate::FieldReader<DWBW0_A>;
#[doc = "DAC0 noise wave bit width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DWBW0_A {
    #[doc = "0: The bit width of the wave signal is 1"]
    BIT_WIDTH1 = 0,
    #[doc = "1: The bit width of the wave signal is 2"]
    BIT_WIDTH2 = 1,
    #[doc = "2: The bit width of the wave signal is 3"]
    BIT_WIDTH3 = 2,
    #[doc = "3: The bit width of the wave signal is 4"]
    BIT_WIDTH4 = 3,
    #[doc = "4: The bit width of the wave signal is 5"]
    BIT_WIDTH5 = 4,
    #[doc = "5: The bit width of the wave signal is 6"]
    BIT_WIDTH6 = 5,
    #[doc = "6: The bit width of the wave signal is 7"]
    BIT_WIDTH7 = 6,
    #[doc = "7: The bit width of the wave signal is 8"]
    BIT_WIDTH8 = 7,
    #[doc = "8: The bit width of the wave signal is 9"]
    BIT_WIDTH9 = 8,
    #[doc = "9: The bit width of the wave signal is 10"]
    BIT_WIDTH10 = 9,
    #[doc = "10: The bit width of the wave signal is 11"]
    BIT_WIDTH11 = 10,
    #[doc = "11: The bit width of the wave signal is 12"]
    BIT_WIDTH12 = 11,
}
impl From<DWBW0_A> for u8 {
    #[inline(always)]
    fn from(variant: DWBW0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DWBW0_A {
    type Ux = u8;
}
impl DWBW0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DWBW0_A> {
        match self.bits {
            0 => Some(DWBW0_A::BIT_WIDTH1),
            1 => Some(DWBW0_A::BIT_WIDTH2),
            2 => Some(DWBW0_A::BIT_WIDTH3),
            3 => Some(DWBW0_A::BIT_WIDTH4),
            4 => Some(DWBW0_A::BIT_WIDTH5),
            5 => Some(DWBW0_A::BIT_WIDTH6),
            6 => Some(DWBW0_A::BIT_WIDTH7),
            7 => Some(DWBW0_A::BIT_WIDTH8),
            8 => Some(DWBW0_A::BIT_WIDTH9),
            9 => Some(DWBW0_A::BIT_WIDTH10),
            10 => Some(DWBW0_A::BIT_WIDTH11),
            11 => Some(DWBW0_A::BIT_WIDTH12),
            _ => None,
        }
    }
    #[doc = "The bit width of the wave signal is 1"]
    #[inline(always)]
    pub fn is_bit_width1(&self) -> bool {
        *self == DWBW0_A::BIT_WIDTH1
    }
    #[doc = "The bit width of the wave signal is 2"]
    #[inline(always)]
    pub fn is_bit_width2(&self) -> bool {
        *self == DWBW0_A::BIT_WIDTH2
    }
    #[doc = "The bit width of the wave signal is 3"]
    #[inline(always)]
    pub fn is_bit_width3(&self) -> bool {
        *self == DWBW0_A::BIT_WIDTH3
    }
    #[doc = "The bit width of the wave signal is 4"]
    #[inline(always)]
    pub fn is_bit_width4(&self) -> bool {
        *self == DWBW0_A::BIT_WIDTH4
    }
    #[doc = "The bit width of the wave signal is 5"]
    #[inline(always)]
    pub fn is_bit_width5(&self) -> bool {
        *self == DWBW0_A::BIT_WIDTH5
    }
    #[doc = "The bit width of the wave signal is 6"]
    #[inline(always)]
    pub fn is_bit_width6(&self) -> bool {
        *self == DWBW0_A::BIT_WIDTH6
    }
    #[doc = "The bit width of the wave signal is 7"]
    #[inline(always)]
    pub fn is_bit_width7(&self) -> bool {
        *self == DWBW0_A::BIT_WIDTH7
    }
    #[doc = "The bit width of the wave signal is 8"]
    #[inline(always)]
    pub fn is_bit_width8(&self) -> bool {
        *self == DWBW0_A::BIT_WIDTH8
    }
    #[doc = "The bit width of the wave signal is 9"]
    #[inline(always)]
    pub fn is_bit_width9(&self) -> bool {
        *self == DWBW0_A::BIT_WIDTH9
    }
    #[doc = "The bit width of the wave signal is 10"]
    #[inline(always)]
    pub fn is_bit_width10(&self) -> bool {
        *self == DWBW0_A::BIT_WIDTH10
    }
    #[doc = "The bit width of the wave signal is 11"]
    #[inline(always)]
    pub fn is_bit_width11(&self) -> bool {
        *self == DWBW0_A::BIT_WIDTH11
    }
    #[doc = "The bit width of the wave signal is 12"]
    #[inline(always)]
    pub fn is_bit_width12(&self) -> bool {
        *self == DWBW0_A::BIT_WIDTH12
    }
}
#[doc = "Field `DWBW0` writer - DAC0 noise wave bit width"]
pub type DWBW0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, DWBW0_A>;
impl<'a, REG, const O: u8> DWBW0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bit width of the wave signal is 1"]
    #[inline(always)]
    pub fn bit_width1(self) -> &'a mut crate::W<REG> {
        self.variant(DWBW0_A::BIT_WIDTH1)
    }
    #[doc = "The bit width of the wave signal is 2"]
    #[inline(always)]
    pub fn bit_width2(self) -> &'a mut crate::W<REG> {
        self.variant(DWBW0_A::BIT_WIDTH2)
    }
    #[doc = "The bit width of the wave signal is 3"]
    #[inline(always)]
    pub fn bit_width3(self) -> &'a mut crate::W<REG> {
        self.variant(DWBW0_A::BIT_WIDTH3)
    }
    #[doc = "The bit width of the wave signal is 4"]
    #[inline(always)]
    pub fn bit_width4(self) -> &'a mut crate::W<REG> {
        self.variant(DWBW0_A::BIT_WIDTH4)
    }
    #[doc = "The bit width of the wave signal is 5"]
    #[inline(always)]
    pub fn bit_width5(self) -> &'a mut crate::W<REG> {
        self.variant(DWBW0_A::BIT_WIDTH5)
    }
    #[doc = "The bit width of the wave signal is 6"]
    #[inline(always)]
    pub fn bit_width6(self) -> &'a mut crate::W<REG> {
        self.variant(DWBW0_A::BIT_WIDTH6)
    }
    #[doc = "The bit width of the wave signal is 7"]
    #[inline(always)]
    pub fn bit_width7(self) -> &'a mut crate::W<REG> {
        self.variant(DWBW0_A::BIT_WIDTH7)
    }
    #[doc = "The bit width of the wave signal is 8"]
    #[inline(always)]
    pub fn bit_width8(self) -> &'a mut crate::W<REG> {
        self.variant(DWBW0_A::BIT_WIDTH8)
    }
    #[doc = "The bit width of the wave signal is 9"]
    #[inline(always)]
    pub fn bit_width9(self) -> &'a mut crate::W<REG> {
        self.variant(DWBW0_A::BIT_WIDTH9)
    }
    #[doc = "The bit width of the wave signal is 10"]
    #[inline(always)]
    pub fn bit_width10(self) -> &'a mut crate::W<REG> {
        self.variant(DWBW0_A::BIT_WIDTH10)
    }
    #[doc = "The bit width of the wave signal is 11"]
    #[inline(always)]
    pub fn bit_width11(self) -> &'a mut crate::W<REG> {
        self.variant(DWBW0_A::BIT_WIDTH11)
    }
    #[doc = "The bit width of the wave signal is 12"]
    #[inline(always)]
    pub fn bit_width12(self) -> &'a mut crate::W<REG> {
        self.variant(DWBW0_A::BIT_WIDTH12)
    }
}
#[doc = "Field `DDMAEN0` reader - DAC0 DMA enable"]
pub type DDMAEN0_R = crate::BitReader<DDMAEN0_A>;
#[doc = "DAC0 DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDMAEN0_A {
    #[doc = "0: DAC DMA mode disabled"]
    DISABLED = 0,
    #[doc = "1: DAC DMA mode enabled"]
    ENABLED = 1,
}
impl From<DDMAEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DDMAEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl DDMAEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDMAEN0_A {
        match self.bits {
            false => DDMAEN0_A::DISABLED,
            true => DDMAEN0_A::ENABLED,
        }
    }
    #[doc = "DAC DMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDMAEN0_A::DISABLED
    }
    #[doc = "DAC DMA mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDMAEN0_A::ENABLED
    }
}
#[doc = "Field `DDMAEN0` writer - DAC0 DMA enable"]
pub type DDMAEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DDMAEN0_A>;
impl<'a, REG, const O: u8> DDMAEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDMAEN0_A::DISABLED)
    }
    #[doc = "DAC DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDMAEN0_A::ENABLED)
    }
}
#[doc = "Field `DBOFF1` reader - DAC1 output buffer turn off"]
pub use DBOFF0_R as DBOFF1_R;
#[doc = "Field `DBOFF1` writer - DAC1 output buffer turn off"]
pub use DBOFF0_W as DBOFF1_W;
#[doc = "Field `DDMAEN1` reader - DAC1 DMA enable"]
pub use DDMAEN0_R as DDMAEN1_R;
#[doc = "Field `DDMAEN1` writer - DAC1 DMA enable"]
pub use DDMAEN0_W as DDMAEN1_W;
#[doc = "Field `DEN1` reader - DAC1 enable"]
pub use DEN0_R as DEN1_R;
#[doc = "Field `DEN1` writer - DAC1 enable"]
pub use DEN0_W as DEN1_W;
#[doc = "Field `DTEN1` reader - DAC1 trigger enable"]
pub use DTEN0_R as DTEN1_R;
#[doc = "Field `DTEN1` writer - DAC1 trigger enable"]
pub use DTEN0_W as DTEN1_W;
#[doc = "Field `DTSEL1` reader - DAC1 trigger selection"]
pub use DTSEL0_R as DTSEL1_R;
#[doc = "Field `DTSEL1` writer - DAC1 trigger selection"]
pub use DTSEL0_W as DTSEL1_W;
#[doc = "Field `DWBW1` reader - DAC1 noise wave bit width"]
pub use DWBW0_R as DWBW1_R;
#[doc = "Field `DWBW1` writer - DAC1 noise wave bit width"]
pub use DWBW0_W as DWBW1_W;
#[doc = "Field `DWM1` reader - DAC1 noise wave mode"]
pub use DWM0_R as DWM1_R;
#[doc = "Field `DWM1` writer - DAC1 noise wave mode"]
pub use DWM0_W as DWM1_W;
impl R {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    pub fn den0(&self) -> DEN0_R {
        DEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    pub fn dboff0(&self) -> DBOFF0_R {
        DBOFF0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    pub fn dten0(&self) -> DTEN0_R {
        DTEN0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    pub fn dtsel0(&self) -> DTSEL0_R {
        DTSEL0_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DAC0 noise wave mode"]
    #[inline(always)]
    pub fn dwm0(&self) -> DWM0_R {
        DWM0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC0 noise wave bit width"]
    #[inline(always)]
    pub fn dwbw0(&self) -> DWBW0_R {
        DWBW0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    pub fn ddmaen0(&self) -> DDMAEN0_R {
        DDMAEN0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC1 enable"]
    #[inline(always)]
    pub fn den1(&self) -> DEN1_R {
        DEN1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC1 output buffer turn off"]
    #[inline(always)]
    pub fn dboff1(&self) -> DBOFF1_R {
        DBOFF1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC1 trigger enable"]
    #[inline(always)]
    pub fn dten1(&self) -> DTEN1_R {
        DTEN1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - DAC1 trigger selection"]
    #[inline(always)]
    pub fn dtsel1(&self) -> DTSEL1_R {
        DTSEL1_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - DAC1 noise wave mode"]
    #[inline(always)]
    pub fn dwm1(&self) -> DWM1_R {
        DWM1_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC1 noise wave bit width"]
    #[inline(always)]
    pub fn dwbw1(&self) -> DWBW1_R {
        DWBW1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn ddmaen1(&self) -> DDMAEN1_R {
        DDMAEN1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn den0(&mut self) -> DEN0_W<CTL_SPEC, 0> {
        DEN0_W::new(self)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    #[must_use]
    pub fn dboff0(&mut self) -> DBOFF0_W<CTL_SPEC, 1> {
        DBOFF0_W::new(self)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten0(&mut self) -> DTEN0_W<CTL_SPEC, 2> {
        DTEN0_W::new(self)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtsel0(&mut self) -> DTSEL0_W<CTL_SPEC, 3> {
        DTSEL0_W::new(self)
    }
    #[doc = "Bits 6:7 - DAC0 noise wave mode"]
    #[inline(always)]
    #[must_use]
    pub fn dwm0(&mut self) -> DWM0_W<CTL_SPEC, 6> {
        DWM0_W::new(self)
    }
    #[doc = "Bits 8:11 - DAC0 noise wave bit width"]
    #[inline(always)]
    #[must_use]
    pub fn dwbw0(&mut self) -> DWBW0_W<CTL_SPEC, 8> {
        DWBW0_W::new(self)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddmaen0(&mut self) -> DDMAEN0_W<CTL_SPEC, 12> {
        DDMAEN0_W::new(self)
    }
    #[doc = "Bit 16 - DAC1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn den1(&mut self) -> DEN1_W<CTL_SPEC, 16> {
        DEN1_W::new(self)
    }
    #[doc = "Bit 17 - DAC1 output buffer turn off"]
    #[inline(always)]
    #[must_use]
    pub fn dboff1(&mut self) -> DBOFF1_W<CTL_SPEC, 17> {
        DBOFF1_W::new(self)
    }
    #[doc = "Bit 18 - DAC1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten1(&mut self) -> DTEN1_W<CTL_SPEC, 18> {
        DTEN1_W::new(self)
    }
    #[doc = "Bits 19:21 - DAC1 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtsel1(&mut self) -> DTSEL1_W<CTL_SPEC, 19> {
        DTSEL1_W::new(self)
    }
    #[doc = "Bits 22:23 - DAC1 noise wave mode"]
    #[inline(always)]
    #[must_use]
    pub fn dwm1(&mut self) -> DWM1_W<CTL_SPEC, 22> {
        DWM1_W::new(self)
    }
    #[doc = "Bits 24:27 - DAC1 noise wave bit width"]
    #[inline(always)]
    #[must_use]
    pub fn dwbw1(&mut self) -> DWBW1_W<CTL_SPEC, 24> {
        DWBW1_W::new(self)
    }
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddmaen1(&mut self) -> DDMAEN1_W<CTL_SPEC, 28> {
        DDMAEN1_W::new(self)
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
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
