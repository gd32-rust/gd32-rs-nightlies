#[doc = "Register `CHCTL0_Input` reader"]
pub type R = crate::R<CHCTL0_INPUT_SPEC>;
#[doc = "Register `CHCTL0_Input` writer"]
pub type W = crate::W<CHCTL0_INPUT_SPEC>;
#[doc = "Input capture 0 filter"]
pub use crate::gd32e230::timer0::chctl0_input::CH0CAPFLT_A;
#[doc = "Field `CH0CAPFLT` reader - Input capture 0 filter"]
pub use crate::gd32e230::timer0::chctl0_input::CH0CAPFLT_R;
#[doc = "Field `CH1CAPFLT` reader - Input capture 1 filter"]
pub use crate::gd32e230::timer0::chctl0_input::CH0CAPFLT_R as CH1CAPFLT_R;
#[doc = "Field `CH0CAPFLT` writer - Input capture 0 filter"]
pub use crate::gd32e230::timer0::chctl0_input::CH0CAPFLT_W;
#[doc = "Field `CH1CAPFLT` writer - Input capture 1 filter"]
pub use crate::gd32e230::timer0::chctl0_input::CH0CAPFLT_W as CH1CAPFLT_W;
#[doc = "Input capture 0 prescaler"]
pub use crate::gd32e230::timer0::chctl0_input::CH0CAPPSC_A;
#[doc = "Field `CH0CAPPSC` reader - Input capture 0 prescaler"]
pub use crate::gd32e230::timer0::chctl0_input::CH0CAPPSC_R;
#[doc = "Field `CH1CAPPSC` reader - Input capture 1 prescaler"]
pub use crate::gd32e230::timer0::chctl0_input::CH0CAPPSC_R as CH1CAPPSC_R;
#[doc = "Field `CH0CAPPSC` writer - Input capture 0 prescaler"]
pub use crate::gd32e230::timer0::chctl0_input::CH0CAPPSC_W;
#[doc = "Field `CH1CAPPSC` writer - Input capture 1 prescaler"]
pub use crate::gd32e230::timer0::chctl0_input::CH0CAPPSC_W as CH1CAPPSC_W;
#[doc = "Capture/Compare 0 selection"]
pub use crate::gd32e230::timer0::chctl0_input::CH0MS_A;
#[doc = "Field `CH0MS` reader - Capture/Compare 0 selection"]
pub use crate::gd32e230::timer0::chctl0_input::CH0MS_R;
#[doc = "Field `CH1MS` reader - Capture/Compare 1 selection"]
pub use crate::gd32e230::timer0::chctl0_input::CH0MS_R as CH1MS_R;
#[doc = "Field `CH0MS` writer - Capture/Compare 0 selection"]
pub use crate::gd32e230::timer0::chctl0_input::CH0MS_W;
#[doc = "Field `CH1MS` writer - Capture/Compare 1 selection"]
pub use crate::gd32e230::timer0::chctl0_input::CH0MS_W as CH1MS_W;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 0 selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> CH0MS_R {
        CH0MS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 0 prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&self) -> CH0CAPPSC_R {
        CH0CAPPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 0 filter"]
    #[inline(always)]
    pub fn ch0capflt(&self) -> CH0CAPFLT_R {
        CH0CAPFLT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn ch1ms(&self) -> CH1MS_R {
        CH1MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ch1cappsc(&self) -> CH1CAPPSC_R {
        CH1CAPPSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ch1capflt(&self) -> CH1CAPFLT_R {
        CH1CAPFLT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 0 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ms(&mut self) -> CH0MS_W<CHCTL0_INPUT_SPEC, 0> {
        CH0MS_W::new(self)
    }
    #[doc = "Bits 2:3 - Input capture 0 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cappsc(&mut self) -> CH0CAPPSC_W<CHCTL0_INPUT_SPEC, 2> {
        CH0CAPPSC_W::new(self)
    }
    #[doc = "Bits 4:7 - Input capture 0 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ch0capflt(&mut self) -> CH0CAPFLT_W<CHCTL0_INPUT_SPEC, 4> {
        CH0CAPFLT_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/Compare 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ms(&mut self) -> CH1MS_W<CHCTL0_INPUT_SPEC, 8> {
        CH1MS_W::new(self)
    }
    #[doc = "Bits 10:11 - Input capture 1 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cappsc(&mut self) -> CH1CAPPSC_W<CHCTL0_INPUT_SPEC, 10> {
        CH1CAPPSC_W::new(self)
    }
    #[doc = "Bits 12:15 - Input capture 1 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ch1capflt(&mut self) -> CH1CAPFLT_W<CHCTL0_INPUT_SPEC, 12> {
        CH1CAPFLT_W::new(self)
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
#[doc = "capture/compare mode register 0 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl0_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl0_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTL0_INPUT_SPEC;
impl crate::RegisterSpec for CHCTL0_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl0_input::R`](R) reader structure"]
impl crate::Readable for CHCTL0_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctl0_input::W`](W) writer structure"]
impl crate::Writable for CHCTL0_INPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTL0_Input to value 0"]
impl crate::Resettable for CHCTL0_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
