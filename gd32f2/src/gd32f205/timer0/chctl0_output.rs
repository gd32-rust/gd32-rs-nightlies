#[doc = "Register `CHCTL0_Output` reader"]
pub struct R(crate::R<CHCTL0_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTL0_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTL0_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTL0_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTL0_Output` writer"]
pub struct W(crate::W<CHCTL0_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTL0_OUTPUT_SPEC>;
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
impl From<crate::W<CHCTL0_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTL0_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 1 output compare clear enable"]
pub use super::chctl0_output::CH0COMCEN_A as CH1COMCEN_A;
#[doc = "Field `CH1COMCEN` reader - Channel 1 output compare clear enable"]
pub use super::chctl0_output::CH0COMCEN_R as CH1COMCEN_R;
#[doc = "Field `CH1COMCEN` writer - Channel 1 output compare clear enable"]
pub type CH1COMCEN_W<'a> = crate::BitWriter<'a, u32, CHCTL0_OUTPUT_SPEC, CH1COMCEN_A, 15>;
impl<'a> CH1COMCEN_W<'a> {
    #[doc = "Output compare clear disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH1COMCEN_A::DISABLED)
    }
    #[doc = "Output compare clear enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH1COMCEN_A::ENABLED)
    }
}
#[doc = "Channel 1 compare output control"]
pub use super::chctl0_output::CH0COMCTL_A as CH1COMCTL_A;
#[doc = "Field `CH1COMCTL` reader - Channel 1 compare output control"]
pub use super::chctl0_output::CH0COMCTL_R as CH1COMCTL_R;
#[doc = "Field `CH1COMCTL` writer - Channel 1 compare output control"]
pub type CH1COMCTL_W<'a> =
    crate::FieldWriterSafe<'a, u32, CHCTL0_OUTPUT_SPEC, u8, CH1COMCTL_A, 3, 12>;
impl<'a> CH1COMCTL_W<'a> {
    #[doc = "The comparison between the output compare register CHyCV and the counter CNT has no effect on the outputs"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(CH1COMCTL_A::FROZEN)
    }
    #[doc = "Set channel to active level on match. OxCPRE signal is forced high when the counter matches the capture/compare register CHyCV"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(CH1COMCTL_A::ACTIVEONMATCH)
    }
    #[doc = "Set channel to inactive level on match. OxCPRE signal is forced low when the counter matches the capture/compare register CHyCV"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(CH1COMCTL_A::INACTIVEONMATCH)
    }
    #[doc = "OxCPRE toggles when CNT=CHyCV"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CH1COMCTL_A::TOGGLE)
    }
    #[doc = "OxCPRE is forced low"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(CH1COMCTL_A::FORCEINACTIVE)
    }
    #[doc = "OxCPRE is forced high"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(CH1COMCTL_A::FORCEACTIVE)
    }
    #[doc = "In upcounting, channel is active as long as CNT<CHyCV else inactive. In downcounting, channel is inactive as long as CNT>CHyCV else active"]
    #[inline(always)]
    pub fn pwm_mode0(self) -> &'a mut W {
        self.variant(CH1COMCTL_A::PWMMODE0)
    }
    #[doc = "Inversely to PwmMode0"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(CH1COMCTL_A::PWMMODE1)
    }
}
#[doc = "Channel 1 output compare shadow enable"]
pub use super::chctl0_output::CH0COMSEN_A as CH1COMSEN_A;
#[doc = "Field `CH1COMSEN` reader - Channel 1 output compare shadow enable"]
pub use super::chctl0_output::CH0COMSEN_R as CH1COMSEN_R;
#[doc = "Field `CH1COMSEN` writer - Channel 1 output compare shadow enable"]
pub type CH1COMSEN_W<'a> = crate::BitWriter<'a, u32, CHCTL0_OUTPUT_SPEC, CH1COMSEN_A, 11>;
impl<'a> CH1COMSEN_W<'a> {
    #[doc = "Preload register on CHyCV disabled. New values written to CHyCV are taken into account immediately"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH1COMSEN_A::DISABLED)
    }
    #[doc = "Preload register on CHyCV enabled. Preload value is loaded into active register on each update event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH1COMSEN_A::ENABLED)
    }
}
#[doc = "Channel 1 output compare fast enable"]
pub use super::chctl0_output::CH0COMFEN_A as CH1COMFEN_A;
#[doc = "Field `CH1COMFEN` reader - Channel 1 output compare fast enable"]
pub use super::chctl0_output::CH0COMFEN_R as CH1COMFEN_R;
#[doc = "Field `CH1COMFEN` writer - Channel 1 output compare fast enable"]
pub type CH1COMFEN_W<'a> = crate::BitWriter<'a, u32, CHCTL0_OUTPUT_SPEC, CH1COMFEN_A, 10>;
impl<'a> CH1COMFEN_W<'a> {
    #[doc = "The minimum delay from an edge is 5 clock cycles"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(CH1COMFEN_A::SLOW)
    }
    #[doc = "The minimum delay from an edge is 3 clock cycles"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(CH1COMFEN_A::FAST)
    }
}
#[doc = "Channel 1 mode selection"]
pub use super::chctl0_output::CH0MS_A as CH1MS_A;
#[doc = "Field `CH1MS` reader - Channel 1 mode selection"]
pub use super::chctl0_output::CH0MS_R as CH1MS_R;
#[doc = "Field `CH1MS` writer - Channel 1 mode selection"]
pub type CH1MS_W<'a> = crate::FieldWriterSafe<'a, u32, CHCTL0_OUTPUT_SPEC, u8, CH1MS_A, 2, 8>;
impl<'a> CH1MS_W<'a> {
    #[doc = "Channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CH1MS_A::OUTPUT)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI0FE0"]
    #[inline(always)]
    pub fn ci0(self) -> &'a mut W {
        self.variant(CH1MS_A::CI0)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI1FE0"]
    #[inline(always)]
    pub fn ci1(self) -> &'a mut W {
        self.variant(CH1MS_A::CI1)
    }
    #[doc = "Channel is configured as input, ISx is connected to ITS"]
    #[inline(always)]
    pub fn its(self) -> &'a mut W {
        self.variant(CH1MS_A::ITS)
    }
}
#[doc = "Channel 0 output compare clear enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0COMCEN_A {
    #[doc = "0: Output compare clear disabled"]
    DISABLED = 0,
    #[doc = "1: Output compare clear enabled"]
    ENABLED = 1,
}
impl From<CH0COMCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CH0COMCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0COMCEN` reader - Channel 0 output compare clear enable"]
pub type CH0COMCEN_R = crate::BitReader<CH0COMCEN_A>;
impl CH0COMCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0COMCEN_A {
        match self.bits {
            false => CH0COMCEN_A::DISABLED,
            true => CH0COMCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0COMCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0COMCEN_A::ENABLED
    }
}
#[doc = "Field `CH0COMCEN` writer - Channel 0 output compare clear enable"]
pub type CH0COMCEN_W<'a> = crate::BitWriter<'a, u32, CHCTL0_OUTPUT_SPEC, CH0COMCEN_A, 7>;
impl<'a> CH0COMCEN_W<'a> {
    #[doc = "Output compare clear disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH0COMCEN_A::DISABLED)
    }
    #[doc = "Output compare clear enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH0COMCEN_A::ENABLED)
    }
}
#[doc = "Channel 0 compare output control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH0COMCTL_A {
    #[doc = "0: The comparison between the output compare register CHyCV and the counter CNT has no effect on the outputs"]
    FROZEN = 0,
    #[doc = "1: Set channel to active level on match. OxCPRE signal is forced high when the counter matches the capture/compare register CHyCV"]
    ACTIVEONMATCH = 1,
    #[doc = "2: Set channel to inactive level on match. OxCPRE signal is forced low when the counter matches the capture/compare register CHyCV"]
    INACTIVEONMATCH = 2,
    #[doc = "3: OxCPRE toggles when CNT=CHyCV"]
    TOGGLE = 3,
    #[doc = "4: OxCPRE is forced low"]
    FORCEINACTIVE = 4,
    #[doc = "5: OxCPRE is forced high"]
    FORCEACTIVE = 5,
    #[doc = "6: In upcounting, channel is active as long as CNT<CHyCV else inactive. In downcounting, channel is inactive as long as CNT>CHyCV else active"]
    PWMMODE0 = 6,
    #[doc = "7: Inversely to PwmMode0"]
    PWMMODE1 = 7,
}
impl From<CH0COMCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: CH0COMCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH0COMCTL` reader - Channel 0 compare output control"]
pub type CH0COMCTL_R = crate::FieldReader<u8, CH0COMCTL_A>;
impl CH0COMCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0COMCTL_A {
        match self.bits {
            0 => CH0COMCTL_A::FROZEN,
            1 => CH0COMCTL_A::ACTIVEONMATCH,
            2 => CH0COMCTL_A::INACTIVEONMATCH,
            3 => CH0COMCTL_A::TOGGLE,
            4 => CH0COMCTL_A::FORCEINACTIVE,
            5 => CH0COMCTL_A::FORCEACTIVE,
            6 => CH0COMCTL_A::PWMMODE0,
            7 => CH0COMCTL_A::PWMMODE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == CH0COMCTL_A::FROZEN
    }
    #[doc = "Checks if the value of the field is `ACTIVEONMATCH`"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == CH0COMCTL_A::ACTIVEONMATCH
    }
    #[doc = "Checks if the value of the field is `INACTIVEONMATCH`"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == CH0COMCTL_A::INACTIVEONMATCH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CH0COMCTL_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `FORCEINACTIVE`"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == CH0COMCTL_A::FORCEINACTIVE
    }
    #[doc = "Checks if the value of the field is `FORCEACTIVE`"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == CH0COMCTL_A::FORCEACTIVE
    }
    #[doc = "Checks if the value of the field is `PWMMODE0`"]
    #[inline(always)]
    pub fn is_pwm_mode0(&self) -> bool {
        *self == CH0COMCTL_A::PWMMODE0
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == CH0COMCTL_A::PWMMODE1
    }
}
#[doc = "Field `CH0COMCTL` writer - Channel 0 compare output control"]
pub type CH0COMCTL_W<'a> =
    crate::FieldWriterSafe<'a, u32, CHCTL0_OUTPUT_SPEC, u8, CH0COMCTL_A, 3, 4>;
impl<'a> CH0COMCTL_W<'a> {
    #[doc = "The comparison between the output compare register CHyCV and the counter CNT has no effect on the outputs"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(CH0COMCTL_A::FROZEN)
    }
    #[doc = "Set channel to active level on match. OxCPRE signal is forced high when the counter matches the capture/compare register CHyCV"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(CH0COMCTL_A::ACTIVEONMATCH)
    }
    #[doc = "Set channel to inactive level on match. OxCPRE signal is forced low when the counter matches the capture/compare register CHyCV"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(CH0COMCTL_A::INACTIVEONMATCH)
    }
    #[doc = "OxCPRE toggles when CNT=CHyCV"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CH0COMCTL_A::TOGGLE)
    }
    #[doc = "OxCPRE is forced low"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(CH0COMCTL_A::FORCEINACTIVE)
    }
    #[doc = "OxCPRE is forced high"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(CH0COMCTL_A::FORCEACTIVE)
    }
    #[doc = "In upcounting, channel is active as long as CNT<CHyCV else inactive. In downcounting, channel is inactive as long as CNT>CHyCV else active"]
    #[inline(always)]
    pub fn pwm_mode0(self) -> &'a mut W {
        self.variant(CH0COMCTL_A::PWMMODE0)
    }
    #[doc = "Inversely to PwmMode0"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(CH0COMCTL_A::PWMMODE1)
    }
}
#[doc = "Channel 0 compare output shadow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0COMSEN_A {
    #[doc = "0: Preload register on CHyCV disabled. New values written to CHyCV are taken into account immediately"]
    DISABLED = 0,
    #[doc = "1: Preload register on CHyCV enabled. Preload value is loaded into active register on each update event"]
    ENABLED = 1,
}
impl From<CH0COMSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CH0COMSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0COMSEN` reader - Channel 0 compare output shadow enable"]
pub type CH0COMSEN_R = crate::BitReader<CH0COMSEN_A>;
impl CH0COMSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0COMSEN_A {
        match self.bits {
            false => CH0COMSEN_A::DISABLED,
            true => CH0COMSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0COMSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0COMSEN_A::ENABLED
    }
}
#[doc = "Field `CH0COMSEN` writer - Channel 0 compare output shadow enable"]
pub type CH0COMSEN_W<'a> = crate::BitWriter<'a, u32, CHCTL0_OUTPUT_SPEC, CH0COMSEN_A, 3>;
impl<'a> CH0COMSEN_W<'a> {
    #[doc = "Preload register on CHyCV disabled. New values written to CHyCV are taken into account immediately"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH0COMSEN_A::DISABLED)
    }
    #[doc = "Preload register on CHyCV enabled. Preload value is loaded into active register on each update event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH0COMSEN_A::ENABLED)
    }
}
#[doc = "Channel 0 output compare fast enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0COMFEN_A {
    #[doc = "0: The minimum delay from an edge is 5 clock cycles"]
    SLOW = 0,
    #[doc = "1: The minimum delay from an edge is 3 clock cycles"]
    FAST = 1,
}
impl From<CH0COMFEN_A> for bool {
    #[inline(always)]
    fn from(variant: CH0COMFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0COMFEN` reader - Channel 0 output compare fast enable"]
pub type CH0COMFEN_R = crate::BitReader<CH0COMFEN_A>;
impl CH0COMFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0COMFEN_A {
        match self.bits {
            false => CH0COMFEN_A::SLOW,
            true => CH0COMFEN_A::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == CH0COMFEN_A::SLOW
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == CH0COMFEN_A::FAST
    }
}
#[doc = "Field `CH0COMFEN` writer - Channel 0 output compare fast enable"]
pub type CH0COMFEN_W<'a> = crate::BitWriter<'a, u32, CHCTL0_OUTPUT_SPEC, CH0COMFEN_A, 2>;
impl<'a> CH0COMFEN_W<'a> {
    #[doc = "The minimum delay from an edge is 5 clock cycles"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(CH0COMFEN_A::SLOW)
    }
    #[doc = "The minimum delay from an edge is 3 clock cycles"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(CH0COMFEN_A::FAST)
    }
}
#[doc = "Channel 0 I/O mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH0MS_A {
    #[doc = "0: Channel is configured as output"]
    OUTPUT = 0,
    #[doc = "1: Channel is configured as input, ISx is connected to CI0FE0"]
    CI0 = 1,
    #[doc = "2: Channel is configured as input, ISx is connected to CI1FE0"]
    CI1 = 2,
    #[doc = "3: Channel is configured as input, ISx is connected to ITS"]
    ITS = 3,
}
impl From<CH0MS_A> for u8 {
    #[inline(always)]
    fn from(variant: CH0MS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH0MS` reader - Channel 0 I/O mode selection"]
pub type CH0MS_R = crate::FieldReader<u8, CH0MS_A>;
impl CH0MS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0MS_A {
        match self.bits {
            0 => CH0MS_A::OUTPUT,
            1 => CH0MS_A::CI0,
            2 => CH0MS_A::CI1,
            3 => CH0MS_A::ITS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CH0MS_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `CI0`"]
    #[inline(always)]
    pub fn is_ci0(&self) -> bool {
        *self == CH0MS_A::CI0
    }
    #[doc = "Checks if the value of the field is `CI1`"]
    #[inline(always)]
    pub fn is_ci1(&self) -> bool {
        *self == CH0MS_A::CI1
    }
    #[doc = "Checks if the value of the field is `ITS`"]
    #[inline(always)]
    pub fn is_its(&self) -> bool {
        *self == CH0MS_A::ITS
    }
}
#[doc = "Field `CH0MS` writer - Channel 0 I/O mode selection"]
pub type CH0MS_W<'a> = crate::FieldWriterSafe<'a, u32, CHCTL0_OUTPUT_SPEC, u8, CH0MS_A, 2, 0>;
impl<'a> CH0MS_W<'a> {
    #[doc = "Channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CH0MS_A::OUTPUT)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI0FE0"]
    #[inline(always)]
    pub fn ci0(self) -> &'a mut W {
        self.variant(CH0MS_A::CI0)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI1FE0"]
    #[inline(always)]
    pub fn ci1(self) -> &'a mut W {
        self.variant(CH0MS_A::CI1)
    }
    #[doc = "Channel is configured as input, ISx is connected to ITS"]
    #[inline(always)]
    pub fn its(self) -> &'a mut W {
        self.variant(CH0MS_A::ITS)
    }
}
impl R {
    #[doc = "Bit 15 - Channel 1 output compare clear enable"]
    #[inline(always)]
    pub fn ch1comcen(&self) -> CH1COMCEN_R {
        CH1COMCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Channel 1 compare output control"]
    #[inline(always)]
    pub fn ch1comctl(&self) -> CH1COMCTL_R {
        CH1COMCTL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 11 - Channel 1 output compare shadow enable"]
    #[inline(always)]
    pub fn ch1comsen(&self) -> CH1COMSEN_R {
        CH1COMSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 1 output compare fast enable"]
    #[inline(always)]
    pub fn ch1comfen(&self) -> CH1COMFEN_R {
        CH1COMFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    pub fn ch1ms(&self) -> CH1MS_R {
        CH1MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - Channel 0 output compare clear enable"]
    #[inline(always)]
    pub fn ch0comcen(&self) -> CH0COMCEN_R {
        CH0COMCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 0 compare output control"]
    #[inline(always)]
    pub fn ch0comctl(&self) -> CH0COMCTL_R {
        CH0COMCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - Channel 0 compare output shadow enable"]
    #[inline(always)]
    pub fn ch0comsen(&self) -> CH0COMSEN_R {
        CH0COMSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 output compare fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&self) -> CH0COMFEN_R {
        CH0COMFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Channel 0 I/O mode selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> CH0MS_R {
        CH0MS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Channel 1 output compare clear enable"]
    #[inline(always)]
    pub fn ch1comcen(&mut self) -> CH1COMCEN_W {
        CH1COMCEN_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 1 compare output control"]
    #[inline(always)]
    pub fn ch1comctl(&mut self) -> CH1COMCTL_W {
        CH1COMCTL_W::new(self)
    }
    #[doc = "Bit 11 - Channel 1 output compare shadow enable"]
    #[inline(always)]
    pub fn ch1comsen(&mut self) -> CH1COMSEN_W {
        CH1COMSEN_W::new(self)
    }
    #[doc = "Bit 10 - Channel 1 output compare fast enable"]
    #[inline(always)]
    pub fn ch1comfen(&mut self) -> CH1COMFEN_W {
        CH1COMFEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    pub fn ch1ms(&mut self) -> CH1MS_W {
        CH1MS_W::new(self)
    }
    #[doc = "Bit 7 - Channel 0 output compare clear enable"]
    #[inline(always)]
    pub fn ch0comcen(&mut self) -> CH0COMCEN_W {
        CH0COMCEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel 0 compare output control"]
    #[inline(always)]
    pub fn ch0comctl(&mut self) -> CH0COMCTL_W {
        CH0COMCTL_W::new(self)
    }
    #[doc = "Bit 3 - Channel 0 compare output shadow enable"]
    #[inline(always)]
    pub fn ch0comsen(&mut self) -> CH0COMSEN_W {
        CH0COMSEN_W::new(self)
    }
    #[doc = "Bit 2 - Channel 0 output compare fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&mut self) -> CH0COMFEN_W {
        CH0COMFEN_W::new(self)
    }
    #[doc = "Bits 0:1 - Channel 0 I/O mode selection"]
    #[inline(always)]
    pub fn ch0ms(&mut self) -> CH0MS_W {
        CH0MS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel control register 0(output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl0_output](index.html) module"]
pub struct CHCTL0_OUTPUT_SPEC;
impl crate::RegisterSpec for CHCTL0_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctl0_output::R](R) reader structure"]
impl crate::Readable for CHCTL0_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctl0_output::W](W) writer structure"]
impl crate::Writable for CHCTL0_OUTPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCTL0_Output to value 0"]
impl crate::Resettable for CHCTL0_OUTPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
