#[doc = "Register `CHCTL1_Output` reader"]
pub type R = crate::R<CHCTL1_OUTPUT_SPEC>;
#[doc = "Register `CHCTL1_Output` writer"]
pub type W = crate::W<CHCTL1_OUTPUT_SPEC>;
#[doc = "Output compare 2 clear enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMCEN_A;
#[doc = "Field `CH2COMCEN` reader - Output compare 2 clear enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMCEN_R as CH2COMCEN_R;
#[doc = "Field `CH3COMCEN` reader - Output compare 3 clear enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMCEN_R as CH3COMCEN_R;
#[doc = "Field `CH2COMCEN` writer - Output compare 2 clear enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMCEN_W as CH2COMCEN_W;
#[doc = "Field `CH3COMCEN` writer - Output compare 3 clear enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMCEN_W as CH3COMCEN_W;
#[doc = "Output compare 2 mode"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMCTL_A;
#[doc = "Field `CH2COMCTL` reader - Output compare 2 mode"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMCTL_R as CH2COMCTL_R;
#[doc = "Field `CH3COMCTL` reader - Output compare 3 mode"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMCTL_R as CH3COMCTL_R;
#[doc = "Field `CH2COMCTL` writer - Output compare 2 mode"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMCTL_W as CH2COMCTL_W;
#[doc = "Field `CH3COMCTL` writer - Output compare 3 mode"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMCTL_W as CH3COMCTL_W;
#[doc = "Output compare 2 fast enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMFEN_A;
#[doc = "Field `CH2COMFEN` reader - Output compare 2 fast enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMFEN_R as CH2COMFEN_R;
#[doc = "Field `CH3COMFEN` reader - Output compare 3 fast enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMFEN_R as CH3COMFEN_R;
#[doc = "Field `CH2COMFEN` writer - Output compare 2 fast enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMFEN_W as CH2COMFEN_W;
#[doc = "Field `CH3COMFEN` writer - Output compare 3 fast enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMFEN_W as CH3COMFEN_W;
#[doc = "Output compare 2 preload enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMSEN_A;
#[doc = "Field `CH2COMSEN` reader - Output compare 2 preload enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMSEN_R as CH2COMSEN_R;
#[doc = "Field `CH3COMSEN` reader - Output compare 3 preload enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMSEN_R as CH3COMSEN_R;
#[doc = "Field `CH2COMSEN` writer - Output compare 2 preload enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMSEN_W as CH2COMSEN_W;
#[doc = "Field `CH3COMSEN` writer - Output compare 3 preload enable"]
pub use crate::gd32e230::timer0::chctl0_output::CH0COMSEN_W as CH3COMSEN_W;
#[doc = "Capture/Compare 2 selection"]
pub use crate::gd32e230::timer0::chctl0_output::CH0MS_A;
#[doc = "Field `CH2MS` reader - Capture/Compare 2 selection"]
pub use crate::gd32e230::timer0::chctl0_output::CH0MS_R as CH2MS_R;
#[doc = "Field `CH3MS` reader - Capture/Compare 3 selection"]
pub use crate::gd32e230::timer0::chctl0_output::CH0MS_R as CH3MS_R;
#[doc = "Field `CH2MS` writer - Capture/Compare 2 selection"]
pub use crate::gd32e230::timer0::chctl0_output::CH0MS_W as CH2MS_W;
#[doc = "Field `CH3MS` writer - Capture/Compare 3 selection"]
pub use crate::gd32e230::timer0::chctl0_output::CH0MS_W as CH3MS_W;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn ch2ms(&self) -> CH2MS_R {
        CH2MS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 2 fast enable"]
    #[inline(always)]
    pub fn ch2comfen(&self) -> CH2COMFEN_R {
        CH2COMFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 2 preload enable"]
    #[inline(always)]
    pub fn ch2comsen(&self) -> CH2COMSEN_R {
        CH2COMSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 2 mode"]
    #[inline(always)]
    pub fn ch2comctl(&self) -> CH2COMCTL_R {
        CH2COMCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 2 clear enable"]
    #[inline(always)]
    pub fn ch2comcen(&self) -> CH2COMCEN_R {
        CH2COMCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn ch3ms(&self) -> CH3MS_R {
        CH3MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn ch3comfen(&self) -> CH3COMFEN_R {
        CH3COMFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn ch3comsen(&self) -> CH3COMSEN_R {
        CH3COMSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 3 mode"]
    #[inline(always)]
    pub fn ch3comctl(&self) -> CH3COMCTL_R {
        CH3COMCTL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn ch3comcen(&self) -> CH3COMCEN_R {
        CH3COMCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 2 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ms(&mut self) -> CH2MS_W<CHCTL1_OUTPUT_SPEC, 0> {
        CH2MS_W::new(self)
    }
    #[doc = "Bit 2 - Output compare 2 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2comfen(&mut self) -> CH2COMFEN_W<CHCTL1_OUTPUT_SPEC, 2> {
        CH2COMFEN_W::new(self)
    }
    #[doc = "Bit 3 - Output compare 2 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2comsen(&mut self) -> CH2COMSEN_W<CHCTL1_OUTPUT_SPEC, 3> {
        CH2COMSEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Output compare 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch2comctl(&mut self) -> CH2COMCTL_W<CHCTL1_OUTPUT_SPEC, 4> {
        CH2COMCTL_W::new(self)
    }
    #[doc = "Bit 7 - Output compare 2 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2comcen(&mut self) -> CH2COMCEN_W<CHCTL1_OUTPUT_SPEC, 7> {
        CH2COMCEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/Compare 3 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ms(&mut self) -> CH3MS_W<CHCTL1_OUTPUT_SPEC, 8> {
        CH3MS_W::new(self)
    }
    #[doc = "Bit 10 - Output compare 3 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3comfen(&mut self) -> CH3COMFEN_W<CHCTL1_OUTPUT_SPEC, 10> {
        CH3COMFEN_W::new(self)
    }
    #[doc = "Bit 11 - Output compare 3 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3comsen(&mut self) -> CH3COMSEN_W<CHCTL1_OUTPUT_SPEC, 11> {
        CH3COMSEN_W::new(self)
    }
    #[doc = "Bits 12:14 - Output compare 3 mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch3comctl(&mut self) -> CH3COMCTL_W<CHCTL1_OUTPUT_SPEC, 12> {
        CH3COMCTL_W::new(self)
    }
    #[doc = "Bit 15 - Output compare 3 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3comcen(&mut self) -> CH3COMCEN_W<CHCTL1_OUTPUT_SPEC, 15> {
        CH3COMCEN_W::new(self)
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
#[doc = "capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl1_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl1_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTL1_OUTPUT_SPEC;
impl crate::RegisterSpec for CHCTL1_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl1_output::R`](R) reader structure"]
impl crate::Readable for CHCTL1_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctl1_output::W`](W) writer structure"]
impl crate::Writable for CHCTL1_OUTPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTL1_Output to value 0"]
impl crate::Resettable for CHCTL1_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
