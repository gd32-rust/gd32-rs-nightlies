#[doc = "Register `CHCTL0_Output` reader"]
pub type R = crate::R<CHCTL0_OUTPUT_SPEC>;
#[doc = "Register `CHCTL0_Output` writer"]
pub type W = crate::W<CHCTL0_OUTPUT_SPEC>;
#[doc = "Field `CH0MS` reader - Channel 0 I/O mode selection"]
pub type CH0MS_R = crate::FieldReader<CH0MS_A>;
#[doc = "Channel 0 I/O mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CH0MS_A {
    type Ux = u8;
}
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
    #[doc = "Channel is configured as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CH0MS_A::OUTPUT
    }
    #[doc = "Channel is configured as input, ISx is connected to CI0FE0"]
    #[inline(always)]
    pub fn is_ci0(&self) -> bool {
        *self == CH0MS_A::CI0
    }
    #[doc = "Channel is configured as input, ISx is connected to CI1FE0"]
    #[inline(always)]
    pub fn is_ci1(&self) -> bool {
        *self == CH0MS_A::CI1
    }
    #[doc = "Channel is configured as input, ISx is connected to ITS"]
    #[inline(always)]
    pub fn is_its(&self) -> bool {
        *self == CH0MS_A::ITS
    }
}
#[doc = "Field `CH0MS` writer - Channel 0 I/O mode selection"]
pub type CH0MS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CH0MS_A>;
impl<'a, REG, const O: u8> CH0MS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(CH0MS_A::OUTPUT)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI0FE0"]
    #[inline(always)]
    pub fn ci0(self) -> &'a mut crate::W<REG> {
        self.variant(CH0MS_A::CI0)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI1FE0"]
    #[inline(always)]
    pub fn ci1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0MS_A::CI1)
    }
    #[doc = "Channel is configured as input, ISx is connected to ITS"]
    #[inline(always)]
    pub fn its(self) -> &'a mut crate::W<REG> {
        self.variant(CH0MS_A::ITS)
    }
}
#[doc = "Field `CH0COMFEN` reader - Channel 0 output compare fast enable"]
pub type CH0COMFEN_R = crate::BitReader<CH0COMFEN_A>;
#[doc = "Channel 0 output compare fast enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CH0COMFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0COMFEN_A {
        match self.bits {
            false => CH0COMFEN_A::SLOW,
            true => CH0COMFEN_A::FAST,
        }
    }
    #[doc = "The minimum delay from an edge is 5 clock cycles"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == CH0COMFEN_A::SLOW
    }
    #[doc = "The minimum delay from an edge is 3 clock cycles"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == CH0COMFEN_A::FAST
    }
}
#[doc = "Field `CH0COMFEN` writer - Channel 0 output compare fast enable"]
pub type CH0COMFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0COMFEN_A>;
impl<'a, REG, const O: u8> CH0COMFEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The minimum delay from an edge is 5 clock cycles"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMFEN_A::SLOW)
    }
    #[doc = "The minimum delay from an edge is 3 clock cycles"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMFEN_A::FAST)
    }
}
#[doc = "Field `CH0COMSEN` reader - Channel 0 compare output shadow enable"]
pub type CH0COMSEN_R = crate::BitReader<CH0COMSEN_A>;
#[doc = "Channel 0 compare output shadow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CH0COMSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0COMSEN_A {
        match self.bits {
            false => CH0COMSEN_A::DISABLED,
            true => CH0COMSEN_A::ENABLED,
        }
    }
    #[doc = "Preload register on CHyCV disabled. New values written to CHyCV are taken into account immediately"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0COMSEN_A::DISABLED
    }
    #[doc = "Preload register on CHyCV enabled. Preload value is loaded into active register on each update event"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0COMSEN_A::ENABLED
    }
}
#[doc = "Field `CH0COMSEN` writer - Channel 0 compare output shadow enable"]
pub type CH0COMSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0COMSEN_A>;
impl<'a, REG, const O: u8> CH0COMSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preload register on CHyCV disabled. New values written to CHyCV are taken into account immediately"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMSEN_A::DISABLED)
    }
    #[doc = "Preload register on CHyCV enabled. Preload value is loaded into active register on each update event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMSEN_A::ENABLED)
    }
}
#[doc = "Field `CH0COMCTL` reader - Channel 0 compare output control"]
pub type CH0COMCTL_R = crate::FieldReader<CH0COMCTL_A>;
#[doc = "Channel 0 compare output control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH0COMCTL_A {
    #[doc = "0: The comparison between the output compare register CHyCV and the counter CNT has no effect on the outputs"]
    FROZEN = 0,
    #[doc = "1: Set channel to active level on match. OxCPRE signal is forced high when the counter matches the capture/compare register CHyCV"]
    ACTIVE_ON_MATCH = 1,
    #[doc = "2: Set channel to inactive level on match. OxCPRE signal is forced low when the counter matches the capture/compare register CHyCV"]
    INACTIVE_ON_MATCH = 2,
    #[doc = "3: OxCPRE toggles when CNT=CHyCV"]
    TOGGLE = 3,
    #[doc = "4: OxCPRE is forced low"]
    FORCE_INACTIVE = 4,
    #[doc = "5: OxCPRE is forced high"]
    FORCE_ACTIVE = 5,
    #[doc = "6: In upcounting, channel is active as long as CNT&lt;CHyCV else inactive. In downcounting, channel is inactive as long as CNT>CHyCV else active"]
    PWM_MODE0 = 6,
    #[doc = "7: Inversely to PwmMode0"]
    PWM_MODE1 = 7,
}
impl From<CH0COMCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: CH0COMCTL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH0COMCTL_A {
    type Ux = u8;
}
impl CH0COMCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0COMCTL_A {
        match self.bits {
            0 => CH0COMCTL_A::FROZEN,
            1 => CH0COMCTL_A::ACTIVE_ON_MATCH,
            2 => CH0COMCTL_A::INACTIVE_ON_MATCH,
            3 => CH0COMCTL_A::TOGGLE,
            4 => CH0COMCTL_A::FORCE_INACTIVE,
            5 => CH0COMCTL_A::FORCE_ACTIVE,
            6 => CH0COMCTL_A::PWM_MODE0,
            7 => CH0COMCTL_A::PWM_MODE1,
            _ => unreachable!(),
        }
    }
    #[doc = "The comparison between the output compare register CHyCV and the counter CNT has no effect on the outputs"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == CH0COMCTL_A::FROZEN
    }
    #[doc = "Set channel to active level on match. OxCPRE signal is forced high when the counter matches the capture/compare register CHyCV"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == CH0COMCTL_A::ACTIVE_ON_MATCH
    }
    #[doc = "Set channel to inactive level on match. OxCPRE signal is forced low when the counter matches the capture/compare register CHyCV"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == CH0COMCTL_A::INACTIVE_ON_MATCH
    }
    #[doc = "OxCPRE toggles when CNT=CHyCV"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CH0COMCTL_A::TOGGLE
    }
    #[doc = "OxCPRE is forced low"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == CH0COMCTL_A::FORCE_INACTIVE
    }
    #[doc = "OxCPRE is forced high"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == CH0COMCTL_A::FORCE_ACTIVE
    }
    #[doc = "In upcounting, channel is active as long as CNT&lt;CHyCV else inactive. In downcounting, channel is inactive as long as CNT>CHyCV else active"]
    #[inline(always)]
    pub fn is_pwm_mode0(&self) -> bool {
        *self == CH0COMCTL_A::PWM_MODE0
    }
    #[doc = "Inversely to PwmMode0"]
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == CH0COMCTL_A::PWM_MODE1
    }
}
#[doc = "Field `CH0COMCTL` writer - Channel 0 compare output control"]
pub type CH0COMCTL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, CH0COMCTL_A>;
impl<'a, REG, const O: u8> CH0COMCTL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The comparison between the output compare register CHyCV and the counter CNT has no effect on the outputs"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMCTL_A::FROZEN)
    }
    #[doc = "Set channel to active level on match. OxCPRE signal is forced high when the counter matches the capture/compare register CHyCV"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMCTL_A::ACTIVE_ON_MATCH)
    }
    #[doc = "Set channel to inactive level on match. OxCPRE signal is forced low when the counter matches the capture/compare register CHyCV"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMCTL_A::INACTIVE_ON_MATCH)
    }
    #[doc = "OxCPRE toggles when CNT=CHyCV"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMCTL_A::TOGGLE)
    }
    #[doc = "OxCPRE is forced low"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMCTL_A::FORCE_INACTIVE)
    }
    #[doc = "OxCPRE is forced high"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMCTL_A::FORCE_ACTIVE)
    }
    #[doc = "In upcounting, channel is active as long as CNT&lt;CHyCV else inactive. In downcounting, channel is inactive as long as CNT>CHyCV else active"]
    #[inline(always)]
    pub fn pwm_mode0(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMCTL_A::PWM_MODE0)
    }
    #[doc = "Inversely to PwmMode0"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMCTL_A::PWM_MODE1)
    }
}
#[doc = "Field `CH0COMCEN` reader - Channel 0 output compare clear enable"]
pub type CH0COMCEN_R = crate::BitReader<CH0COMCEN_A>;
#[doc = "Channel 0 output compare clear enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CH0COMCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0COMCEN_A {
        match self.bits {
            false => CH0COMCEN_A::DISABLED,
            true => CH0COMCEN_A::ENABLED,
        }
    }
    #[doc = "Output compare clear disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0COMCEN_A::DISABLED
    }
    #[doc = "Output compare clear enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0COMCEN_A::ENABLED
    }
}
#[doc = "Field `CH0COMCEN` writer - Channel 0 output compare clear enable"]
pub type CH0COMCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0COMCEN_A>;
impl<'a, REG, const O: u8> CH0COMCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output compare clear disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMCEN_A::DISABLED)
    }
    #[doc = "Output compare clear enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CH0COMCEN_A::ENABLED)
    }
}
#[doc = "Field `CH1COMCEN` reader - Channel 1 output compare clear enable"]
pub use CH0COMCEN_R as CH1COMCEN_R;
#[doc = "Field `CH1COMCEN` writer - Channel 1 output compare clear enable"]
pub use CH0COMCEN_W as CH1COMCEN_W;
#[doc = "Field `CH1COMCTL` reader - Channel 1 compare output control"]
pub use CH0COMCTL_R as CH1COMCTL_R;
#[doc = "Field `CH1COMCTL` writer - Channel 1 compare output control"]
pub use CH0COMCTL_W as CH1COMCTL_W;
#[doc = "Field `CH1COMFEN` reader - Channel 1 output compare fast enable"]
pub use CH0COMFEN_R as CH1COMFEN_R;
#[doc = "Field `CH1COMFEN` writer - Channel 1 output compare fast enable"]
pub use CH0COMFEN_W as CH1COMFEN_W;
#[doc = "Field `CH1COMSEN` reader - Channel 1 output compare shadow enable"]
pub use CH0COMSEN_R as CH1COMSEN_R;
#[doc = "Field `CH1COMSEN` writer - Channel 1 output compare shadow enable"]
pub use CH0COMSEN_W as CH1COMSEN_W;
#[doc = "Field `CH1MS` reader - Channel 1 mode selection"]
pub use CH0MS_R as CH1MS_R;
#[doc = "Field `CH1MS` writer - Channel 1 mode selection"]
pub use CH0MS_W as CH1MS_W;
impl R {
    #[doc = "Bits 0:1 - Channel 0 I/O mode selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> CH0MS_R {
        CH0MS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Channel 0 output compare fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&self) -> CH0COMFEN_R {
        CH0COMFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 compare output shadow enable"]
    #[inline(always)]
    pub fn ch0comsen(&self) -> CH0COMSEN_R {
        CH0COMSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 0 compare output control"]
    #[inline(always)]
    pub fn ch0comctl(&self) -> CH0COMCTL_R {
        CH0COMCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 0 output compare clear enable"]
    #[inline(always)]
    pub fn ch0comcen(&self) -> CH0COMCEN_R {
        CH0COMCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    pub fn ch1ms(&self) -> CH1MS_R {
        CH1MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Channel 1 output compare fast enable"]
    #[inline(always)]
    pub fn ch1comfen(&self) -> CH1COMFEN_R {
        CH1COMFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 1 output compare shadow enable"]
    #[inline(always)]
    pub fn ch1comsen(&self) -> CH1COMSEN_R {
        CH1COMSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Channel 1 compare output control"]
    #[inline(always)]
    pub fn ch1comctl(&self) -> CH1COMCTL_R {
        CH1COMCTL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Channel 1 output compare clear enable"]
    #[inline(always)]
    pub fn ch1comcen(&self) -> CH1COMCEN_R {
        CH1COMCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 I/O mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ms(&mut self) -> CH0MS_W<CHCTL0_OUTPUT_SPEC, 0> {
        CH0MS_W::new(self)
    }
    #[doc = "Bit 2 - Channel 0 output compare fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0comfen(&mut self) -> CH0COMFEN_W<CHCTL0_OUTPUT_SPEC, 2> {
        CH0COMFEN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 0 compare output shadow enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0comsen(&mut self) -> CH0COMSEN_W<CHCTL0_OUTPUT_SPEC, 3> {
        CH0COMSEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel 0 compare output control"]
    #[inline(always)]
    #[must_use]
    pub fn ch0comctl(&mut self) -> CH0COMCTL_W<CHCTL0_OUTPUT_SPEC, 4> {
        CH0COMCTL_W::new(self)
    }
    #[doc = "Bit 7 - Channel 0 output compare clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0comcen(&mut self) -> CH0COMCEN_W<CHCTL0_OUTPUT_SPEC, 7> {
        CH0COMCEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ms(&mut self) -> CH1MS_W<CHCTL0_OUTPUT_SPEC, 8> {
        CH1MS_W::new(self)
    }
    #[doc = "Bit 10 - Channel 1 output compare fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1comfen(&mut self) -> CH1COMFEN_W<CHCTL0_OUTPUT_SPEC, 10> {
        CH1COMFEN_W::new(self)
    }
    #[doc = "Bit 11 - Channel 1 output compare shadow enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1comsen(&mut self) -> CH1COMSEN_W<CHCTL0_OUTPUT_SPEC, 11> {
        CH1COMSEN_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 1 compare output control"]
    #[inline(always)]
    #[must_use]
    pub fn ch1comctl(&mut self) -> CH1COMCTL_W<CHCTL0_OUTPUT_SPEC, 12> {
        CH1COMCTL_W::new(self)
    }
    #[doc = "Bit 15 - Channel 1 output compare clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1comcen(&mut self) -> CH1COMCEN_W<CHCTL0_OUTPUT_SPEC, 15> {
        CH1COMCEN_W::new(self)
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
#[doc = "Channel control register 0(output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl0_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl0_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTL0_OUTPUT_SPEC;
impl crate::RegisterSpec for CHCTL0_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl0_output::R`](R) reader structure"]
impl crate::Readable for CHCTL0_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctl0_output::W`](W) writer structure"]
impl crate::Writable for CHCTL0_OUTPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTL0_Output to value 0"]
impl crate::Resettable for CHCTL0_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
