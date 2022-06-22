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
#[doc = "Field `CH0CAPFLT` reader - Channel 0 input capture filter control"]
pub type CH0CAPFLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0CAPFLT` writer - Channel 0 input capture filter control"]
pub type CH0CAPFLT_W<'a> = crate::FieldWriter<'a, u32, CHCTL0_INPUT_SPEC, u8, u8, 4, 4>;
#[doc = "Field `CH0CAPPSC` reader - Channel 0 input capture prescaler"]
pub type CH0CAPPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0CAPPSC` writer - Channel 0 input capture prescaler"]
pub type CH0CAPPSC_W<'a> = crate::FieldWriter<'a, u32, CHCTL0_INPUT_SPEC, u8, u8, 2, 2>;
#[doc = "Field `CH0MS` reader - Channel 0 mode selection"]
pub type CH0MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0MS` writer - Channel 0 mode selection"]
pub type CH0MS_W<'a> = crate::FieldWriter<'a, u32, CHCTL0_INPUT_SPEC, u8, u8, 2, 0>;
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel control register 0 ( (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl0_input](index.html) module"]
pub struct CHCTL0_INPUT_SPEC;
impl crate::RegisterSpec for CHCTL0_INPUT_SPEC {
    type Ux = u32;
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
