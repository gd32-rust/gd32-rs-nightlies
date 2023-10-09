#[doc = "Register `CHCTL0_Output` reader"]
pub type R = crate::R<CHCTL0_OUTPUT_SPEC>;
#[doc = "Register `CHCTL0_Output` writer"]
pub type W = crate::W<CHCTL0_OUTPUT_SPEC>;
#[doc = "Channel 0 output compare clear enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMCEN_A;
#[doc = "Field `CH0COMCEN` reader - Channel 0 output compare clear enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMCEN_R;
#[doc = "Field `CH1COMCEN` reader - Channel 1 output compare clear enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMCEN_R as CH1COMCEN_R;
#[doc = "Field `CH0COMCEN` writer - Channel 0 output compare clear enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMCEN_W;
#[doc = "Field `CH1COMCEN` writer - Channel 1 output compare clear enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMCEN_W as CH1COMCEN_W;
#[doc = "Channel 0 compare output control"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMCTL_A;
#[doc = "Field `CH0COMCTL` reader - Channel 0 compare output control"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMCTL_R;
#[doc = "Field `CH1COMCTL` reader - Channel 1 compare output control"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMCTL_R as CH1COMCTL_R;
#[doc = "Field `CH0COMCTL` writer - Channel 0 compare output control"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMCTL_W;
#[doc = "Field `CH1COMCTL` writer - Channel 1 compare output control"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMCTL_W as CH1COMCTL_W;
#[doc = "Channel 0 output compare fast enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMFEN_A;
#[doc = "Field `CH0COMFEN` reader - Channel 0 output compare fast enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMFEN_R;
#[doc = "Field `CH1COMFEN` reader - Channel 1 output compare fast enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMFEN_R as CH1COMFEN_R;
#[doc = "Field `CH0COMFEN` writer - Channel 0 output compare fast enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMFEN_W;
#[doc = "Field `CH1COMFEN` writer - Channel 1 output compare fast enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMFEN_W as CH1COMFEN_W;
#[doc = "Channel 0 compare output shadow enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMSEN_A;
#[doc = "Field `CH0COMSEN` reader - Channel 0 compare output shadow enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMSEN_R;
#[doc = "Field `CH1COMSEN` reader - Channel 1 output compare shadow enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMSEN_R as CH1COMSEN_R;
#[doc = "Field `CH0COMSEN` writer - Channel 0 compare output shadow enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMSEN_W;
#[doc = "Field `CH1COMSEN` writer - Channel 1 output compare shadow enable"]
pub use crate::gd32e503::timer0::chctl0_output::CH0COMSEN_W as CH1COMSEN_W;
#[doc = "Channel 0 I/O mode selection"]
pub use crate::gd32e503::timer0::chctl0_output::CH0MS_A;
#[doc = "Field `CH0MS` reader - Channel 0 I/O mode selection"]
pub use crate::gd32e503::timer0::chctl0_output::CH0MS_R;
#[doc = "Field `CH1MS` reader - Channel 1 mode selection"]
pub use crate::gd32e503::timer0::chctl0_output::CH0MS_R as CH1MS_R;
#[doc = "Field `CH0MS` writer - Channel 0 I/O mode selection"]
pub use crate::gd32e503::timer0::chctl0_output::CH0MS_W;
#[doc = "Field `CH1MS` writer - Channel 1 mode selection"]
pub use crate::gd32e503::timer0::chctl0_output::CH0MS_W as CH1MS_W;
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
#[doc = "Channel control register 0 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl0_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl0_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
