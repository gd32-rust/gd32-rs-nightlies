#[doc = "Register `CHCTL1_Input` reader"]
pub struct R(crate::R<CHCTL1_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTL1_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTL1_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTL1_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTL1_Input` writer"]
pub struct W(crate::W<CHCTL1_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTL1_INPUT_SPEC>;
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
impl From<crate::W<CHCTL1_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTL1_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 3 input capture filter control"]
pub use crate::gd32f190::timer0::chctl0_input::CH0CAPFLT_A as CH3CAPFLT_A;
#[doc = "Field `CH3CAPFLT` reader - Channel 3 input capture filter control"]
pub use crate::gd32f190::timer0::chctl0_input::CH0CAPFLT_R as CH3CAPFLT_R;
#[doc = "Field `CH3CAPFLT` writer - Channel 3 input capture filter control"]
pub type CH3CAPFLT_W<'a> =
    crate::FieldWriterSafe<'a, u16, CHCTL1_INPUT_SPEC, u8, CH3CAPFLT_A, 4, 12>;
impl<'a> CH3CAPFLT_W<'a> {
    #[doc = "Filter disabled. fSAMP=fDTS, N=1"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::NOFILTER)
    }
    #[doc = "fSAMP=fTIMER_CK, N=2"]
    #[inline(always)]
    pub fn timer_ck_n2(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::TIMERCK_N2)
    }
    #[doc = "fSAMP=fTIMER_CK, N=4"]
    #[inline(always)]
    pub fn timer_ck_n4(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::TIMERCK_N4)
    }
    #[doc = "fSAMP=fTIMER_CK, N=8"]
    #[inline(always)]
    pub fn timer_ck_n8(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::TIMERCK_N8)
    }
    #[doc = "fSAMP=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::FDTS_DIV2_N6)
    }
    #[doc = "fSAMP=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::FDTS_DIV2_N8)
    }
    #[doc = "fSAMP=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::FDTS_DIV4_N6)
    }
    #[doc = "fSAMP=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::FDTS_DIV4_N8)
    }
    #[doc = "fSAMP=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::FDTS_DIV8_N6)
    }
    #[doc = "fSAMP=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::FDTS_DIV8_N8)
    }
    #[doc = "fSAMP=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::FDTS_DIV16_N5)
    }
    #[doc = "fSAMP=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::FDTS_DIV16_N6)
    }
    #[doc = "fSAMP=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::FDTS_DIV16_N8)
    }
    #[doc = "fSAMP=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::FDTS_DIV32_N5)
    }
    #[doc = "fSAMP=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::FDTS_DIV32_N6)
    }
    #[doc = "fSAMP=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(CH3CAPFLT_A::FDTS_DIV32_N8)
    }
}
#[doc = "Channel 3 input capture prescaler"]
pub use crate::gd32f190::timer0::chctl0_input::CH0CAPPSC_A as CH3CAPPSC_A;
#[doc = "Field `CH3CAPPSC` reader - Channel 3 input capture prescaler"]
pub use crate::gd32f190::timer0::chctl0_input::CH0CAPPSC_R as CH3CAPPSC_R;
#[doc = "Field `CH3CAPPSC` writer - Channel 3 input capture prescaler"]
pub type CH3CAPPSC_W<'a> =
    crate::FieldWriterSafe<'a, u16, CHCTL1_INPUT_SPEC, u8, CH3CAPPSC_A, 2, 10>;
impl<'a> CH3CAPPSC_W<'a> {
    #[doc = "Prescaler disabled, capture on every edge"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CH3CAPPSC_A::DIV1)
    }
    #[doc = "Capture every 2 edges"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CH3CAPPSC_A::DIV2)
    }
    #[doc = "Capture every 4 edges"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CH3CAPPSC_A::DIV4)
    }
    #[doc = "Capture every 8 edges"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CH3CAPPSC_A::DIV8)
    }
}
#[doc = "Channel 3 mode selection"]
pub use crate::gd32f190::timer0::chctl0_input::CH0MS_A as CH3MS_A;
#[doc = "Field `CH3MS` reader - Channel 3 mode selection"]
pub use crate::gd32f190::timer0::chctl0_input::CH0MS_R as CH3MS_R;
#[doc = "Field `CH3MS` writer - Channel 3 mode selection"]
pub type CH3MS_W<'a> = crate::FieldWriterSafe<'a, u16, CHCTL1_INPUT_SPEC, u8, CH3MS_A, 2, 8>;
impl<'a> CH3MS_W<'a> {
    #[doc = "Channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CH3MS_A::OUTPUT)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI0FEx"]
    #[inline(always)]
    pub fn ci0(self) -> &'a mut W {
        self.variant(CH3MS_A::CI0)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI1FEx"]
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
#[doc = "Channel 2 input capture filter control"]
pub use crate::gd32f190::timer0::chctl0_input::CH0CAPFLT_A as CH2CAPFLT_A;
#[doc = "Field `CH2CAPFLT` reader - Channel 2 input capture filter control"]
pub use crate::gd32f190::timer0::chctl0_input::CH0CAPFLT_R as CH2CAPFLT_R;
#[doc = "Field `CH2CAPFLT` writer - Channel 2 input capture filter control"]
pub type CH2CAPFLT_W<'a> =
    crate::FieldWriterSafe<'a, u16, CHCTL1_INPUT_SPEC, u8, CH2CAPFLT_A, 4, 4>;
impl<'a> CH2CAPFLT_W<'a> {
    #[doc = "Filter disabled. fSAMP=fDTS, N=1"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::NOFILTER)
    }
    #[doc = "fSAMP=fTIMER_CK, N=2"]
    #[inline(always)]
    pub fn timer_ck_n2(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::TIMERCK_N2)
    }
    #[doc = "fSAMP=fTIMER_CK, N=4"]
    #[inline(always)]
    pub fn timer_ck_n4(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::TIMERCK_N4)
    }
    #[doc = "fSAMP=fTIMER_CK, N=8"]
    #[inline(always)]
    pub fn timer_ck_n8(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::TIMERCK_N8)
    }
    #[doc = "fSAMP=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::FDTS_DIV2_N6)
    }
    #[doc = "fSAMP=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::FDTS_DIV2_N8)
    }
    #[doc = "fSAMP=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::FDTS_DIV4_N6)
    }
    #[doc = "fSAMP=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::FDTS_DIV4_N8)
    }
    #[doc = "fSAMP=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::FDTS_DIV8_N6)
    }
    #[doc = "fSAMP=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::FDTS_DIV8_N8)
    }
    #[doc = "fSAMP=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::FDTS_DIV16_N5)
    }
    #[doc = "fSAMP=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::FDTS_DIV16_N6)
    }
    #[doc = "fSAMP=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::FDTS_DIV16_N8)
    }
    #[doc = "fSAMP=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::FDTS_DIV32_N5)
    }
    #[doc = "fSAMP=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::FDTS_DIV32_N6)
    }
    #[doc = "fSAMP=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(CH2CAPFLT_A::FDTS_DIV32_N8)
    }
}
#[doc = "Channel 2 input capture prescaler"]
pub use crate::gd32f190::timer0::chctl0_input::CH0CAPPSC_A as CH2CAPPSC_A;
#[doc = "Field `CH2CAPPSC` reader - Channel 2 input capture prescaler"]
pub use crate::gd32f190::timer0::chctl0_input::CH0CAPPSC_R as CH2CAPPSC_R;
#[doc = "Field `CH2CAPPSC` writer - Channel 2 input capture prescaler"]
pub type CH2CAPPSC_W<'a> =
    crate::FieldWriterSafe<'a, u16, CHCTL1_INPUT_SPEC, u8, CH2CAPPSC_A, 2, 2>;
impl<'a> CH2CAPPSC_W<'a> {
    #[doc = "Prescaler disabled, capture on every edge"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CH2CAPPSC_A::DIV1)
    }
    #[doc = "Capture every 2 edges"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CH2CAPPSC_A::DIV2)
    }
    #[doc = "Capture every 4 edges"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CH2CAPPSC_A::DIV4)
    }
    #[doc = "Capture every 8 edges"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CH2CAPPSC_A::DIV8)
    }
}
#[doc = "Channel 2 mode selection"]
pub use crate::gd32f190::timer0::chctl0_input::CH0MS_A as CH2MS_A;
#[doc = "Field `CH2MS` reader - Channel 2 mode selection"]
pub use crate::gd32f190::timer0::chctl0_input::CH0MS_R as CH2MS_R;
#[doc = "Field `CH2MS` writer - Channel 2 mode selection"]
pub type CH2MS_W<'a> = crate::FieldWriterSafe<'a, u16, CHCTL1_INPUT_SPEC, u8, CH2MS_A, 2, 0>;
impl<'a> CH2MS_W<'a> {
    #[doc = "Channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CH2MS_A::OUTPUT)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI0FEx"]
    #[inline(always)]
    pub fn ci0(self) -> &'a mut W {
        self.variant(CH2MS_A::CI0)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI1FEx"]
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
    #[doc = "Bits 12:15 - Channel 3 input capture filter control"]
    #[inline(always)]
    pub fn ch3capflt(&self) -> CH3CAPFLT_R {
        CH3CAPFLT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Channel 3 input capture prescaler"]
    #[inline(always)]
    pub fn ch3cappsc(&self) -> CH3CAPPSC_R {
        CH3CAPPSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&self) -> CH3MS_R {
        CH3MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Channel 2 input capture filter control"]
    #[inline(always)]
    pub fn ch2capflt(&self) -> CH2CAPFLT_R {
        CH2CAPFLT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - Channel 2 input capture prescaler"]
    #[inline(always)]
    pub fn ch2cappsc(&self) -> CH2CAPPSC_R {
        CH2CAPPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline(always)]
    pub fn ch2ms(&self) -> CH2MS_R {
        CH2MS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Channel 3 input capture filter control"]
    #[inline(always)]
    pub fn ch3capflt(&mut self) -> CH3CAPFLT_W {
        CH3CAPFLT_W::new(self)
    }
    #[doc = "Bits 10:11 - Channel 3 input capture prescaler"]
    #[inline(always)]
    pub fn ch3cappsc(&mut self) -> CH3CAPPSC_W {
        CH3CAPPSC_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&mut self) -> CH3MS_W {
        CH3MS_W::new(self)
    }
    #[doc = "Bits 4:7 - Channel 2 input capture filter control"]
    #[inline(always)]
    pub fn ch2capflt(&mut self) -> CH2CAPFLT_W {
        CH2CAPFLT_W::new(self)
    }
    #[doc = "Bits 2:3 - Channel 2 input capture prescaler"]
    #[inline(always)]
    pub fn ch2cappsc(&mut self) -> CH2CAPPSC_W {
        CH2CAPPSC_W::new(self)
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
#[doc = "Channel control register 1 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl1_input](index.html) module"]
pub struct CHCTL1_INPUT_SPEC;
impl crate::RegisterSpec for CHCTL1_INPUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [chctl1_input::R](R) reader structure"]
impl crate::Readable for CHCTL1_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctl1_input::W](W) writer structure"]
impl crate::Writable for CHCTL1_INPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCTL1_Input to value 0"]
impl crate::Resettable for CHCTL1_INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
