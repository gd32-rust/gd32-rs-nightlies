#[doc = "Register `CHCTL0_Input` reader"]
pub struct R(crate::R<CHCTL0_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTL0_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTL0_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTL0_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTL0_Input` writer"]
pub struct W(crate::W<CHCTL0_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTL0_INPUT_SPEC>;
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
impl From<crate::W<CHCTL0_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTL0_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 input capture filter control"]
pub use crate::gd32f130::timer0::chctl0_input::CH0CAPFLT_A;
#[doc = "Field `CH0CAPFLT` reader - Channel 0 input capture filter control"]
pub use crate::gd32f130::timer0::chctl0_input::CH0CAPFLT_R;
#[doc = "Field `CH0CAPFLT` writer - Channel 0 input capture filter control"]
pub type CH0CAPFLT_W<'a> =
    crate::FieldWriterSafe<'a, u16, CHCTL0_INPUT_SPEC, u8, CH0CAPFLT_A, 4, 4>;
impl<'a> CH0CAPFLT_W<'a> {
    #[doc = "Filter disabled. fSAMP=fDTS, N=1"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::NOFILTER)
    }
    #[doc = "fSAMP=fTIMER_CK, N=2"]
    #[inline(always)]
    pub fn timer_ck_n2(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::TIMERCK_N2)
    }
    #[doc = "fSAMP=fTIMER_CK, N=4"]
    #[inline(always)]
    pub fn timer_ck_n4(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::TIMERCK_N4)
    }
    #[doc = "fSAMP=fTIMER_CK, N=8"]
    #[inline(always)]
    pub fn timer_ck_n8(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::TIMERCK_N8)
    }
    #[doc = "fSAMP=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::FDTS_DIV2_N6)
    }
    #[doc = "fSAMP=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::FDTS_DIV2_N8)
    }
    #[doc = "fSAMP=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::FDTS_DIV4_N6)
    }
    #[doc = "fSAMP=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::FDTS_DIV4_N8)
    }
    #[doc = "fSAMP=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::FDTS_DIV8_N6)
    }
    #[doc = "fSAMP=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::FDTS_DIV8_N8)
    }
    #[doc = "fSAMP=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::FDTS_DIV16_N5)
    }
    #[doc = "fSAMP=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::FDTS_DIV16_N6)
    }
    #[doc = "fSAMP=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::FDTS_DIV16_N8)
    }
    #[doc = "fSAMP=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::FDTS_DIV32_N5)
    }
    #[doc = "fSAMP=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::FDTS_DIV32_N6)
    }
    #[doc = "fSAMP=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(CH0CAPFLT_A::FDTS_DIV32_N8)
    }
}
#[doc = "Channel 0 input capture prescaler"]
pub use crate::gd32f130::timer0::chctl0_input::CH0CAPPSC_A;
#[doc = "Field `CH0CAPPSC` reader - Channel 0 input capture prescaler"]
pub use crate::gd32f130::timer0::chctl0_input::CH0CAPPSC_R;
#[doc = "Field `CH0CAPPSC` writer - Channel 0 input capture prescaler"]
pub type CH0CAPPSC_W<'a> =
    crate::FieldWriterSafe<'a, u16, CHCTL0_INPUT_SPEC, u8, CH0CAPPSC_A, 2, 2>;
impl<'a> CH0CAPPSC_W<'a> {
    #[doc = "Prescaler disabled, capture on every edge"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CH0CAPPSC_A::DIV1)
    }
    #[doc = "Capture every 2 edges"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CH0CAPPSC_A::DIV2)
    }
    #[doc = "Capture every 4 edges"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CH0CAPPSC_A::DIV4)
    }
    #[doc = "Capture every 8 edges"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CH0CAPPSC_A::DIV8)
    }
}
#[doc = "Channel 0 mode selection"]
pub use crate::gd32f130::timer0::chctl0_input::CH0MS_A;
#[doc = "Field `CH0MS` reader - Channel 0 mode selection"]
pub use crate::gd32f130::timer0::chctl0_input::CH0MS_R;
#[doc = "Field `CH0MS` writer - Channel 0 mode selection"]
pub type CH0MS_W<'a> = crate::FieldWriterSafe<'a, u16, CHCTL0_INPUT_SPEC, u8, CH0MS_A, 2, 0>;
impl<'a> CH0MS_W<'a> {
    #[doc = "Channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CH0MS_A::OUTPUT)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI0FEx"]
    #[inline(always)]
    pub fn ci0(self) -> &'a mut W {
        self.variant(CH0MS_A::CI0)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI1FEx"]
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
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline(always)]
    pub fn ch0capflt(&self) -> CH0CAPFLT_R {
        CH0CAPFLT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&self) -> CH0CAPPSC_R {
        CH0CAPPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> CH0MS_R {
        CH0MS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline(always)]
    pub fn ch0capflt(&mut self) -> CH0CAPFLT_W {
        CH0CAPFLT_W::new(self)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&mut self) -> CH0CAPPSC_W {
        CH0CAPPSC_W::new(self)
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
#[doc = "Channel control register 0 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl0_input](index.html) module"]
pub struct CHCTL0_INPUT_SPEC;
impl crate::RegisterSpec for CHCTL0_INPUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [chctl0_input::R](R) reader structure"]
impl crate::Readable for CHCTL0_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctl0_input::W](W) writer structure"]
impl crate::Writable for CHCTL0_INPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCTL0_Input to value 0"]
impl crate::Resettable for CHCTL0_INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
