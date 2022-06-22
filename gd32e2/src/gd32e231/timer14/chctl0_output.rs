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
#[doc = "Field `CH1COMCTL` reader - Output Compare 1 mode"]
pub type CH1COMCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1COMCTL` writer - Output Compare 1 mode"]
pub type CH1COMCTL_W<'a> = crate::FieldWriter<'a, u32, CHCTL0_OUTPUT_SPEC, u8, u8, 3, 12>;
#[doc = "Field `CH1COMSEN` reader - Output Compare 1 preload enable"]
pub type CH1COMSEN_R = crate::BitReader<bool>;
#[doc = "Field `CH1COMSEN` writer - Output Compare 1 preload enable"]
pub type CH1COMSEN_W<'a> = crate::BitWriter<'a, u32, CHCTL0_OUTPUT_SPEC, bool, 11>;
#[doc = "Field `CH1COMFEN` reader - Output Compare 1 fast enable"]
pub type CH1COMFEN_R = crate::BitReader<bool>;
#[doc = "Field `CH1COMFEN` writer - Output Compare 1 fast enable"]
pub type CH1COMFEN_W<'a> = crate::BitWriter<'a, u32, CHCTL0_OUTPUT_SPEC, bool, 10>;
#[doc = "Field `CH1MS` reader - Capture/Compare 1 selection"]
pub type CH1MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1MS` writer - Capture/Compare 1 selection"]
pub type CH1MS_W<'a> = crate::FieldWriter<'a, u32, CHCTL0_OUTPUT_SPEC, u8, u8, 2, 8>;
#[doc = "Field `CH0COMCTL` reader - Output Compare 0 mode"]
pub type CH0COMCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0COMCTL` writer - Output Compare 0 mode"]
pub type CH0COMCTL_W<'a> = crate::FieldWriter<'a, u32, CHCTL0_OUTPUT_SPEC, u8, u8, 3, 4>;
#[doc = "Field `CH0COMSEN` reader - Output Compare 0 preload enable"]
pub type CH0COMSEN_R = crate::BitReader<bool>;
#[doc = "Field `CH0COMSEN` writer - Output Compare 0 preload enable"]
pub type CH0COMSEN_W<'a> = crate::BitWriter<'a, u32, CHCTL0_OUTPUT_SPEC, bool, 3>;
#[doc = "Field `CH0COMFEN` reader - Output Compare 0 fast enable"]
pub type CH0COMFEN_R = crate::BitReader<bool>;
#[doc = "Field `CH0COMFEN` writer - Output Compare 0 fast enable"]
pub type CH0COMFEN_W<'a> = crate::BitWriter<'a, u32, CHCTL0_OUTPUT_SPEC, bool, 2>;
#[doc = "Field `CH0MS` reader - Capture/Compare 0 selection"]
pub type CH0MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0MS` writer - Capture/Compare 0 selection"]
pub type CH0MS_W<'a> = crate::FieldWriter<'a, u32, CHCTL0_OUTPUT_SPEC, u8, u8, 2, 0>;
impl R {
    #[doc = "Bits 12:14 - Output Compare 1 mode"]
    #[inline(always)]
    pub fn ch1comctl(&self) -> CH1COMCTL_R {
        CH1COMCTL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 11 - Output Compare 1 preload enable"]
    #[inline(always)]
    pub fn ch1comsen(&self) -> CH1COMSEN_R {
        CH1COMSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Output Compare 1 fast enable"]
    #[inline(always)]
    pub fn ch1comfen(&self) -> CH1COMFEN_R {
        CH1COMFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn ch1ms(&self) -> CH1MS_R {
        CH1MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Output Compare 0 mode"]
    #[inline(always)]
    pub fn ch0comctl(&self) -> CH0COMCTL_R {
        CH0COMCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - Output Compare 0 preload enable"]
    #[inline(always)]
    pub fn ch0comsen(&self) -> CH0COMSEN_R {
        CH0COMSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Compare 0 fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&self) -> CH0COMFEN_R {
        CH0COMFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Capture/Compare 0 selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> CH0MS_R {
        CH0MS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - Output Compare 1 mode"]
    #[inline(always)]
    pub fn ch1comctl(&mut self) -> CH1COMCTL_W {
        CH1COMCTL_W::new(self)
    }
    #[doc = "Bit 11 - Output Compare 1 preload enable"]
    #[inline(always)]
    pub fn ch1comsen(&mut self) -> CH1COMSEN_W {
        CH1COMSEN_W::new(self)
    }
    #[doc = "Bit 10 - Output Compare 1 fast enable"]
    #[inline(always)]
    pub fn ch1comfen(&mut self) -> CH1COMFEN_W {
        CH1COMFEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn ch1ms(&mut self) -> CH1MS_W {
        CH1MS_W::new(self)
    }
    #[doc = "Bits 4:6 - Output Compare 0 mode"]
    #[inline(always)]
    pub fn ch0comctl(&mut self) -> CH0COMCTL_W {
        CH0COMCTL_W::new(self)
    }
    #[doc = "Bit 3 - Output Compare 0 preload enable"]
    #[inline(always)]
    pub fn ch0comsen(&mut self) -> CH0COMSEN_W {
        CH0COMSEN_W::new(self)
    }
    #[doc = "Bit 2 - Output Compare 0 fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&mut self) -> CH0COMFEN_W {
        CH0COMFEN_W::new(self)
    }
    #[doc = "Bits 0:1 - Capture/Compare 0 selection"]
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
#[doc = "capture/compare mode register (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl0_output](index.html) module"]
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
