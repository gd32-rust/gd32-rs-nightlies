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
#[doc = "Field `CH3COMCEN` reader - Output compare 3 clear enable"]
pub type CH3COMCEN_R = crate::BitReader<bool>;
#[doc = "Field `CH3COMCEN` writer - Output compare 3 clear enable"]
pub type CH3COMCEN_W<'a> = crate::BitWriter<'a, u32, CHCTL1_OUTPUT_SPEC, bool, 15>;
#[doc = "Field `CH3COMCTL` reader - Output compare 3 mode"]
pub type CH3COMCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3COMCTL` writer - Output compare 3 mode"]
pub type CH3COMCTL_W<'a> = crate::FieldWriter<'a, u32, CHCTL1_OUTPUT_SPEC, u8, u8, 3, 12>;
#[doc = "Field `CH3COMSEN` reader - Output compare 3 preload enable"]
pub type CH3COMSEN_R = crate::BitReader<bool>;
#[doc = "Field `CH3COMSEN` writer - Output compare 3 preload enable"]
pub type CH3COMSEN_W<'a> = crate::BitWriter<'a, u32, CHCTL1_OUTPUT_SPEC, bool, 11>;
#[doc = "Field `CH3COMFEN` reader - Output compare 3 fast enable"]
pub type CH3COMFEN_R = crate::BitReader<bool>;
#[doc = "Field `CH3COMFEN` writer - Output compare 3 fast enable"]
pub type CH3COMFEN_W<'a> = crate::BitWriter<'a, u32, CHCTL1_OUTPUT_SPEC, bool, 10>;
#[doc = "Field `CH3MS` reader - Capture/Compare 3 selection"]
pub type CH3MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3MS` writer - Capture/Compare 3 selection"]
pub type CH3MS_W<'a> = crate::FieldWriter<'a, u32, CHCTL1_OUTPUT_SPEC, u8, u8, 2, 8>;
#[doc = "Field `CH2COMCEN` reader - Output compare 2 clear enable"]
pub type CH2COMCEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2COMCEN` writer - Output compare 2 clear enable"]
pub type CH2COMCEN_W<'a> = crate::BitWriter<'a, u32, CHCTL1_OUTPUT_SPEC, bool, 7>;
#[doc = "Field `CH2COMCTL` reader - Output compare 2 mode"]
pub type CH2COMCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2COMCTL` writer - Output compare 2 mode"]
pub type CH2COMCTL_W<'a> = crate::FieldWriter<'a, u32, CHCTL1_OUTPUT_SPEC, u8, u8, 3, 4>;
#[doc = "Field `CH2COMSEN` reader - Output compare 2 preload enable"]
pub type CH2COMSEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2COMSEN` writer - Output compare 2 preload enable"]
pub type CH2COMSEN_W<'a> = crate::BitWriter<'a, u32, CHCTL1_OUTPUT_SPEC, bool, 3>;
#[doc = "Field `CH2COMFEN` reader - Output compare 2 fast enable"]
pub type CH2COMFEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2COMFEN` writer - Output compare 2 fast enable"]
pub type CH2COMFEN_W<'a> = crate::BitWriter<'a, u32, CHCTL1_OUTPUT_SPEC, bool, 2>;
#[doc = "Field `CH2MS` reader - Capture/Compare 2 selection"]
pub type CH2MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2MS` writer - Capture/Compare 2 selection"]
pub type CH2MS_W<'a> = crate::FieldWriter<'a, u32, CHCTL1_OUTPUT_SPEC, u8, u8, 2, 0>;
impl R {
    #[doc = "Bit 15 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn ch3comcen(&self) -> CH3COMCEN_R {
        CH3COMCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 3 mode"]
    #[inline(always)]
    pub fn ch3comctl(&self) -> CH3COMCTL_R {
        CH3COMCTL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 11 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn ch3comsen(&self) -> CH3COMSEN_R {
        CH3COMSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn ch3comfen(&self) -> CH3COMFEN_R {
        CH3COMFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn ch3ms(&self) -> CH3MS_R {
        CH3MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - Output compare 2 clear enable"]
    #[inline(always)]
    pub fn ch2comcen(&self) -> CH2COMCEN_R {
        CH2COMCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 2 mode"]
    #[inline(always)]
    pub fn ch2comctl(&self) -> CH2COMCTL_R {
        CH2COMCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - Output compare 2 preload enable"]
    #[inline(always)]
    pub fn ch2comsen(&self) -> CH2COMSEN_R {
        CH2COMSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Output compare 2 fast enable"]
    #[inline(always)]
    pub fn ch2comfen(&self) -> CH2COMFEN_R {
        CH2COMFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn ch2ms(&self) -> CH2MS_R {
        CH2MS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn ch3comcen(&mut self) -> CH3COMCEN_W {
        CH3COMCEN_W::new(self)
    }
    #[doc = "Bits 12:14 - Output compare 3 mode"]
    #[inline(always)]
    pub fn ch3comctl(&mut self) -> CH3COMCTL_W {
        CH3COMCTL_W::new(self)
    }
    #[doc = "Bit 11 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn ch3comsen(&mut self) -> CH3COMSEN_W {
        CH3COMSEN_W::new(self)
    }
    #[doc = "Bit 10 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn ch3comfen(&mut self) -> CH3COMFEN_W {
        CH3COMFEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn ch3ms(&mut self) -> CH3MS_W {
        CH3MS_W::new(self)
    }
    #[doc = "Bit 7 - Output compare 2 clear enable"]
    #[inline(always)]
    pub fn ch2comcen(&mut self) -> CH2COMCEN_W {
        CH2COMCEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Output compare 2 mode"]
    #[inline(always)]
    pub fn ch2comctl(&mut self) -> CH2COMCTL_W {
        CH2COMCTL_W::new(self)
    }
    #[doc = "Bit 3 - Output compare 2 preload enable"]
    #[inline(always)]
    pub fn ch2comsen(&mut self) -> CH2COMSEN_W {
        CH2COMSEN_W::new(self)
    }
    #[doc = "Bit 2 - Output compare 2 fast enable"]
    #[inline(always)]
    pub fn ch2comfen(&mut self) -> CH2COMFEN_W {
        CH2COMFEN_W::new(self)
    }
    #[doc = "Bits 0:1 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn ch2ms(&mut self) -> CH2MS_W {
        CH2MS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 1 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl1_output](index.html) module"]
pub struct CHCTL1_OUTPUT_SPEC;
impl crate::RegisterSpec for CHCTL1_OUTPUT_SPEC {
    type Ux = u32;
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
