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
#[doc = "Channel 0 output compare clear enable"]
pub use crate::gd32f150::timer0::chctl0_output::CH0COMCEN_A;
#[doc = "Field `CH0COMCEN` reader - Channel 0 output compare clear enable"]
pub use crate::gd32f150::timer0::chctl0_output::CH0COMCEN_R;
#[doc = "Field `CH0COMCEN` writer - Channel 0 output compare clear enable"]
pub type CH0COMCEN_W<'a> = crate::BitWriter<'a, u16, CHCTL0_OUTPUT_SPEC, CH0COMCEN_A, 7>;
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
#[doc = "Channel 0 compare output control"]
pub use crate::gd32f150::timer0::chctl0_output::CH0COMCTL_A;
#[doc = "Field `CH0COMCTL` reader - Channel 0 compare output control"]
pub use crate::gd32f150::timer0::chctl0_output::CH0COMCTL_R;
#[doc = "Field `CH0COMCTL` writer - Channel 0 compare output control"]
pub type CH0COMCTL_W<'a> =
    crate::FieldWriterSafe<'a, u16, CHCTL0_OUTPUT_SPEC, u8, CH0COMCTL_A, 3, 4>;
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
#[doc = "Channel 0 output compare shadow enable"]
pub use crate::gd32f150::timer0::chctl0_output::CH0COMSEN_A;
#[doc = "Field `CH0COMSEN` reader - Channel 0 output compare shadow enable"]
pub use crate::gd32f150::timer0::chctl0_output::CH0COMSEN_R;
#[doc = "Field `CH0COMSEN` writer - Channel 0 output compare shadow enable"]
pub type CH0COMSEN_W<'a> = crate::BitWriter<'a, u16, CHCTL0_OUTPUT_SPEC, CH0COMSEN_A, 3>;
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
#[doc = "Channel 0 output compare fast enable"]
pub use crate::gd32f150::timer0::chctl0_output::CH0COMFEN_A;
#[doc = "Field `CH0COMFEN` reader - Channel 0 output compare fast enable"]
pub use crate::gd32f150::timer0::chctl0_output::CH0COMFEN_R;
#[doc = "Field `CH0COMFEN` writer - Channel 0 output compare fast enable"]
pub type CH0COMFEN_W<'a> = crate::BitWriter<'a, u16, CHCTL0_OUTPUT_SPEC, CH0COMFEN_A, 2>;
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
#[doc = "Channel 0 mode selection"]
pub use crate::gd32f150::timer0::chctl0_output::CH0MS_A;
#[doc = "Field `CH0MS` reader - Channel 0 mode selection"]
pub use crate::gd32f150::timer0::chctl0_output::CH0MS_R;
#[doc = "Field `CH0MS` writer - Channel 0 mode selection"]
pub type CH0MS_W<'a> = crate::FieldWriterSafe<'a, u16, CHCTL0_OUTPUT_SPEC, u8, CH0MS_A, 2, 0>;
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
    #[doc = "Bit 3 - Channel 0 output compare shadow enable"]
    #[inline(always)]
    pub fn ch0comsen(&self) -> CH0COMSEN_R {
        CH0COMSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 output compare fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&self) -> CH0COMFEN_R {
        CH0COMFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> CH0MS_R {
        CH0MS_R::new((self.bits & 3) as u8)
    }
}
impl W {
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
    #[doc = "Bit 3 - Channel 0 output compare shadow enable"]
    #[inline(always)]
    pub fn ch0comsen(&mut self) -> CH0COMSEN_W {
        CH0COMSEN_W::new(self)
    }
    #[doc = "Bit 2 - Channel 0 output compare fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&mut self) -> CH0COMFEN_W {
        CH0COMFEN_W::new(self)
    }
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    pub fn ch0ms(&mut self) -> CH0MS_W {
        CH0MS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel control register 0 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl0_output](index.html) module"]
pub struct CHCTL0_OUTPUT_SPEC;
impl crate::RegisterSpec for CHCTL0_OUTPUT_SPEC {
    type Ux = u16;
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
