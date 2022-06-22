#[doc = "Register `CHCTL1_Output` reader"]
pub struct R(crate::R<CHCTL1_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTL1_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTL1_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTL1_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTL1_Output` writer"]
pub struct W(crate::W<CHCTL1_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTL1_OUTPUT_SPEC>;
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
impl From<crate::W<CHCTL1_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTL1_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 3 output compare clear enable"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMCEN_A as CH3COMCEN_A;
#[doc = "Field `CH3COMCEN` reader - Channel 3 output compare clear enable"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMCEN_R as CH3COMCEN_R;
#[doc = "Field `CH3COMCEN` writer - Channel 3 output compare clear enable"]
pub type CH3COMCEN_W<'a> = crate::BitWriter<'a, u16, CHCTL1_OUTPUT_SPEC, CH3COMCEN_A, 15>;
impl<'a> CH3COMCEN_W<'a> {
    #[doc = "Output compare clear disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH3COMCEN_A::DISABLED)
    }
    #[doc = "Output compare clear enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH3COMCEN_A::ENABLED)
    }
}
#[doc = "Channel 3 compare output control"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMCTL_A as CH3COMCTL_A;
#[doc = "Field `CH3COMCTL` reader - Channel 3 compare output control"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMCTL_R as CH3COMCTL_R;
#[doc = "Field `CH3COMCTL` writer - Channel 3 compare output control"]
pub type CH3COMCTL_W<'a> =
    crate::FieldWriterSafe<'a, u16, CHCTL1_OUTPUT_SPEC, u8, CH3COMCTL_A, 3, 12>;
impl<'a> CH3COMCTL_W<'a> {
    #[doc = "The comparison between the output compare register CHyCV and the counter CNT has no effect on the outputs"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(CH3COMCTL_A::FROZEN)
    }
    #[doc = "Set channel to active level on match. OxCPRE signal is forced high when the counter matches the capture/compare register CHyCV"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(CH3COMCTL_A::ACTIVEONMATCH)
    }
    #[doc = "Set channel to inactive level on match. OxCPRE signal is forced low when the counter matches the capture/compare register CHyCV"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(CH3COMCTL_A::INACTIVEONMATCH)
    }
    #[doc = "OxCPRE toggles when CNT=CHyCV"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CH3COMCTL_A::TOGGLE)
    }
    #[doc = "OxCPRE is forced low"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(CH3COMCTL_A::FORCEINACTIVE)
    }
    #[doc = "OxCPRE is forced high"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(CH3COMCTL_A::FORCEACTIVE)
    }
    #[doc = "In upcounting, channel is active as long as CNT<CHyCV else inactive. In downcounting, channel is inactive as long as CNT>CHyCV else active"]
    #[inline(always)]
    pub fn pwm_mode0(self) -> &'a mut W {
        self.variant(CH3COMCTL_A::PWMMODE0)
    }
    #[doc = "Inversely to PwmMode0"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(CH3COMCTL_A::PWMMODE1)
    }
}
#[doc = "Channel 3 compare output control"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMSEN_A as CH3COMSEN_A;
#[doc = "Field `CH3COMSEN` reader - Channel 3 compare output control"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMSEN_R as CH3COMSEN_R;
#[doc = "Field `CH3COMSEN` writer - Channel 3 compare output control"]
pub type CH3COMSEN_W<'a> = crate::BitWriter<'a, u16, CHCTL1_OUTPUT_SPEC, CH3COMSEN_A, 11>;
impl<'a> CH3COMSEN_W<'a> {
    #[doc = "Preload register on CHyCV disabled. New values written to CHyCV are taken into account immediately"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH3COMSEN_A::DISABLED)
    }
    #[doc = "Preload register on CHyCV enabled. Preload value is loaded into active register on each update event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH3COMSEN_A::ENABLED)
    }
}
#[doc = "Channel 3 output compare fast enable"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMFEN_A as CH3COMFEN_A;
#[doc = "Field `CH3COMFEN` reader - Channel 3 output compare fast enable"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMFEN_R as CH3COMFEN_R;
#[doc = "Field `CH3COMFEN` writer - Channel 3 output compare fast enable"]
pub type CH3COMFEN_W<'a> = crate::BitWriter<'a, u16, CHCTL1_OUTPUT_SPEC, CH3COMFEN_A, 10>;
impl<'a> CH3COMFEN_W<'a> {
    #[doc = "The minimum delay from an edge is 5 clock cycles"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(CH3COMFEN_A::SLOW)
    }
    #[doc = "The minimum delay from an edge is 3 clock cycles"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(CH3COMFEN_A::FAST)
    }
}
#[doc = "Channel 3 mode selection"]
pub use crate::gd32f190::timer0::chctl0_output::CH0MS_A as CH3MS_A;
#[doc = "Field `CH3MS` reader - Channel 3 mode selection"]
pub use crate::gd32f190::timer0::chctl0_output::CH0MS_R as CH3MS_R;
#[doc = "Field `CH3MS` writer - Channel 3 mode selection"]
pub type CH3MS_W<'a> = crate::FieldWriterSafe<'a, u16, CHCTL1_OUTPUT_SPEC, u8, CH3MS_A, 2, 8>;
impl<'a> CH3MS_W<'a> {
    #[doc = "Channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CH3MS_A::OUTPUT)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI0FE0"]
    #[inline(always)]
    pub fn ci0(self) -> &'a mut W {
        self.variant(CH3MS_A::CI0)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI1FE0"]
    #[inline(always)]
    pub fn ci1(self) -> &'a mut W {
        self.variant(CH3MS_A::CI1)
    }
    #[doc = "Channel is configured as input, ISx is connected to ITS"]
    #[inline(always)]
    pub fn its(self) -> &'a mut W {
        self.variant(CH3MS_A::ITS)
    }
}
#[doc = "Channel 2 output compare clear enable"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMCEN_A as CH2COMCEN_A;
#[doc = "Field `CH2COMCEN` reader - Channel 2 output compare clear enable"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMCEN_R as CH2COMCEN_R;
#[doc = "Field `CH2COMCEN` writer - Channel 2 output compare clear enable"]
pub type CH2COMCEN_W<'a> = crate::BitWriter<'a, u16, CHCTL1_OUTPUT_SPEC, CH2COMCEN_A, 7>;
impl<'a> CH2COMCEN_W<'a> {
    #[doc = "Output compare clear disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH2COMCEN_A::DISABLED)
    }
    #[doc = "Output compare clear enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH2COMCEN_A::ENABLED)
    }
}
#[doc = "Channel 2 compare output control"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMCTL_A as CH2COMCTL_A;
#[doc = "Field `CH2COMCTL` reader - Channel 2 compare output control"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMCTL_R as CH2COMCTL_R;
#[doc = "Field `CH2COMCTL` writer - Channel 2 compare output control"]
pub type CH2COMCTL_W<'a> =
    crate::FieldWriterSafe<'a, u16, CHCTL1_OUTPUT_SPEC, u8, CH2COMCTL_A, 3, 4>;
impl<'a> CH2COMCTL_W<'a> {
    #[doc = "The comparison between the output compare register CHyCV and the counter CNT has no effect on the outputs"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(CH2COMCTL_A::FROZEN)
    }
    #[doc = "Set channel to active level on match. OxCPRE signal is forced high when the counter matches the capture/compare register CHyCV"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(CH2COMCTL_A::ACTIVEONMATCH)
    }
    #[doc = "Set channel to inactive level on match. OxCPRE signal is forced low when the counter matches the capture/compare register CHyCV"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(CH2COMCTL_A::INACTIVEONMATCH)
    }
    #[doc = "OxCPRE toggles when CNT=CHyCV"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CH2COMCTL_A::TOGGLE)
    }
    #[doc = "OxCPRE is forced low"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(CH2COMCTL_A::FORCEINACTIVE)
    }
    #[doc = "OxCPRE is forced high"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(CH2COMCTL_A::FORCEACTIVE)
    }
    #[doc = "In upcounting, channel is active as long as CNT<CHyCV else inactive. In downcounting, channel is inactive as long as CNT>CHyCV else active"]
    #[inline(always)]
    pub fn pwm_mode0(self) -> &'a mut W {
        self.variant(CH2COMCTL_A::PWMMODE0)
    }
    #[doc = "Inversely to PwmMode0"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(CH2COMCTL_A::PWMMODE1)
    }
}
#[doc = "Channel 2 output compare shadow enable"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMSEN_A as CH2COMSEN_A;
#[doc = "Field `CH2COMSEN` reader - Channel 2 output compare shadow enable"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMSEN_R as CH2COMSEN_R;
#[doc = "Field `CH2COMSEN` writer - Channel 2 output compare shadow enable"]
pub type CH2COMSEN_W<'a> = crate::BitWriter<'a, u16, CHCTL1_OUTPUT_SPEC, CH2COMSEN_A, 3>;
impl<'a> CH2COMSEN_W<'a> {
    #[doc = "Preload register on CHyCV disabled. New values written to CHyCV are taken into account immediately"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH2COMSEN_A::DISABLED)
    }
    #[doc = "Preload register on CHyCV enabled. Preload value is loaded into active register on each update event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH2COMSEN_A::ENABLED)
    }
}
#[doc = "Channel 2 output compare fast enable"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMFEN_A as CH2COMFEN_A;
#[doc = "Field `CH2COMFEN` reader - Channel 2 output compare fast enable"]
pub use crate::gd32f190::timer0::chctl0_output::CH0COMFEN_R as CH2COMFEN_R;
#[doc = "Field `CH2COMFEN` writer - Channel 2 output compare fast enable"]
pub type CH2COMFEN_W<'a> = crate::BitWriter<'a, u16, CHCTL1_OUTPUT_SPEC, CH2COMFEN_A, 2>;
impl<'a> CH2COMFEN_W<'a> {
    #[doc = "The minimum delay from an edge is 5 clock cycles"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(CH2COMFEN_A::SLOW)
    }
    #[doc = "The minimum delay from an edge is 3 clock cycles"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(CH2COMFEN_A::FAST)
    }
}
#[doc = "Channel 2 mode selection"]
pub use crate::gd32f190::timer0::chctl0_output::CH0MS_A as CH2MS_A;
#[doc = "Field `CH2MS` reader - Channel 2 mode selection"]
pub use crate::gd32f190::timer0::chctl0_output::CH0MS_R as CH2MS_R;
#[doc = "Field `CH2MS` writer - Channel 2 mode selection"]
pub type CH2MS_W<'a> = crate::FieldWriterSafe<'a, u16, CHCTL1_OUTPUT_SPEC, u8, CH2MS_A, 2, 0>;
impl<'a> CH2MS_W<'a> {
    #[doc = "Channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CH2MS_A::OUTPUT)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI0FE0"]
    #[inline(always)]
    pub fn ci0(self) -> &'a mut W {
        self.variant(CH2MS_A::CI0)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI1FE0"]
    #[inline(always)]
    pub fn ci1(self) -> &'a mut W {
        self.variant(CH2MS_A::CI1)
    }
    #[doc = "Channel is configured as input, ISx is connected to ITS"]
    #[inline(always)]
    pub fn its(self) -> &'a mut W {
        self.variant(CH2MS_A::ITS)
    }
}
impl R {
    #[doc = "Bit 15 - Channel 3 output compare clear enable"]
    #[inline(always)]
    pub fn ch3comcen(&self) -> CH3COMCEN_R {
        CH3COMCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Channel 3 compare output control"]
    #[inline(always)]
    pub fn ch3comctl(&self) -> CH3COMCTL_R {
        CH3COMCTL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 11 - Channel 3 compare output control"]
    #[inline(always)]
    pub fn ch3comsen(&self) -> CH3COMSEN_R {
        CH3COMSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 3 output compare fast enable"]
    #[inline(always)]
    pub fn ch3comfen(&self) -> CH3COMFEN_R {
        CH3COMFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&self) -> CH3MS_R {
        CH3MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - Channel 2 output compare clear enable"]
    #[inline(always)]
    pub fn ch2comcen(&self) -> CH2COMCEN_R {
        CH2COMCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 2 compare output control"]
    #[inline(always)]
    pub fn ch2comctl(&self) -> CH2COMCTL_R {
        CH2COMCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - Channel 2 output compare shadow enable"]
    #[inline(always)]
    pub fn ch2comsen(&self) -> CH2COMSEN_R {
        CH2COMSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 output compare fast enable"]
    #[inline(always)]
    pub fn ch2comfen(&self) -> CH2COMFEN_R {
        CH2COMFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline(always)]
    pub fn ch2ms(&self) -> CH2MS_R {
        CH2MS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Channel 3 output compare clear enable"]
    #[inline(always)]
    pub fn ch3comcen(&mut self) -> CH3COMCEN_W {
        CH3COMCEN_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 3 compare output control"]
    #[inline(always)]
    pub fn ch3comctl(&mut self) -> CH3COMCTL_W {
        CH3COMCTL_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 compare output control"]
    #[inline(always)]
    pub fn ch3comsen(&mut self) -> CH3COMSEN_W {
        CH3COMSEN_W::new(self)
    }
    #[doc = "Bit 10 - Channel 3 output compare fast enable"]
    #[inline(always)]
    pub fn ch3comfen(&mut self) -> CH3COMFEN_W {
        CH3COMFEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&mut self) -> CH3MS_W {
        CH3MS_W::new(self)
    }
    #[doc = "Bit 7 - Channel 2 output compare clear enable"]
    #[inline(always)]
    pub fn ch2comcen(&mut self) -> CH2COMCEN_W {
        CH2COMCEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel 2 compare output control"]
    #[inline(always)]
    pub fn ch2comctl(&mut self) -> CH2COMCTL_W {
        CH2COMCTL_W::new(self)
    }
    #[doc = "Bit 3 - Channel 2 output compare shadow enable"]
    #[inline(always)]
    pub fn ch2comsen(&mut self) -> CH2COMSEN_W {
        CH2COMSEN_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 output compare fast enable"]
    #[inline(always)]
    pub fn ch2comfen(&mut self) -> CH2COMFEN_W {
        CH2COMFEN_W::new(self)
    }
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline(always)]
    pub fn ch2ms(&mut self) -> CH2MS_W {
        CH2MS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel control register 1 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl1_output](index.html) module"]
pub struct CHCTL1_OUTPUT_SPEC;
impl crate::RegisterSpec for CHCTL1_OUTPUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [chctl1_output::R](R) reader structure"]
impl crate::Readable for CHCTL1_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctl1_output::W](W) writer structure"]
impl crate::Writable for CHCTL1_OUTPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCTL1_Output to value 0"]
impl crate::Resettable for CHCTL1_OUTPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
