#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pin 15 configuration bits"]
pub use CTL0_A as CTL15_A;
#[doc = "Pin 14 configuration bits"]
pub use CTL0_A as CTL14_A;
#[doc = "Pin 13 configuration bits"]
pub use CTL0_A as CTL13_A;
#[doc = "Pin 12 configuration bits"]
pub use CTL0_A as CTL12_A;
#[doc = "Pin 11 configuration bits"]
pub use CTL0_A as CTL11_A;
#[doc = "Pin 10 configuration bits"]
pub use CTL0_A as CTL10_A;
#[doc = "Pin 9 configuration bits"]
pub use CTL0_A as CTL9_A;
#[doc = "Pin 8 configuration bits"]
pub use CTL0_A as CTL8_A;
#[doc = "Pin 7 configuration bits"]
pub use CTL0_A as CTL7_A;
#[doc = "Pin 6 configuration bits"]
pub use CTL0_A as CTL6_A;
#[doc = "Pin 5 configuration bits"]
pub use CTL0_A as CTL5_A;
#[doc = "Pin 4 configuration bits"]
pub use CTL0_A as CTL4_A;
#[doc = "Pin 3 configuration bits"]
pub use CTL0_A as CTL3_A;
#[doc = "Pin 2 configuration bits"]
pub use CTL0_A as CTL2_A;
#[doc = "Pin 1 configuration bits"]
pub use CTL0_A as CTL1_A;
#[doc = "Field `CTL15` reader - Pin 15 configuration bits"]
pub use CTL0_R as CTL15_R;
#[doc = "Field `CTL14` reader - Pin 14 configuration bits"]
pub use CTL0_R as CTL14_R;
#[doc = "Field `CTL13` reader - Pin 13 configuration bits"]
pub use CTL0_R as CTL13_R;
#[doc = "Field `CTL12` reader - Pin 12 configuration bits"]
pub use CTL0_R as CTL12_R;
#[doc = "Field `CTL11` reader - Pin 11 configuration bits"]
pub use CTL0_R as CTL11_R;
#[doc = "Field `CTL10` reader - Pin 10 configuration bits"]
pub use CTL0_R as CTL10_R;
#[doc = "Field `CTL9` reader - Pin 9 configuration bits"]
pub use CTL0_R as CTL9_R;
#[doc = "Field `CTL8` reader - Pin 8 configuration bits"]
pub use CTL0_R as CTL8_R;
#[doc = "Field `CTL7` reader - Pin 7 configuration bits"]
pub use CTL0_R as CTL7_R;
#[doc = "Field `CTL6` reader - Pin 6 configuration bits"]
pub use CTL0_R as CTL6_R;
#[doc = "Field `CTL5` reader - Pin 5 configuration bits"]
pub use CTL0_R as CTL5_R;
#[doc = "Field `CTL4` reader - Pin 4 configuration bits"]
pub use CTL0_R as CTL4_R;
#[doc = "Field `CTL3` reader - Pin 3 configuration bits"]
pub use CTL0_R as CTL3_R;
#[doc = "Field `CTL2` reader - Pin 2 configuration bits"]
pub use CTL0_R as CTL2_R;
#[doc = "Field `CTL1` reader - Pin 1 configuration bits"]
pub use CTL0_R as CTL1_R;
#[doc = "Field `CTL15` writer - Pin 15 configuration bits"]
pub use CTL0_W as CTL15_W;
#[doc = "Field `CTL14` writer - Pin 14 configuration bits"]
pub use CTL0_W as CTL14_W;
#[doc = "Field `CTL13` writer - Pin 13 configuration bits"]
pub use CTL0_W as CTL13_W;
#[doc = "Field `CTL12` writer - Pin 12 configuration bits"]
pub use CTL0_W as CTL12_W;
#[doc = "Field `CTL11` writer - Pin 11 configuration bits"]
pub use CTL0_W as CTL11_W;
#[doc = "Field `CTL10` writer - Pin 10 configuration bits"]
pub use CTL0_W as CTL10_W;
#[doc = "Field `CTL9` writer - Pin 9 configuration bits"]
pub use CTL0_W as CTL9_W;
#[doc = "Field `CTL8` writer - Pin 8 configuration bits"]
pub use CTL0_W as CTL8_W;
#[doc = "Field `CTL7` writer - Pin 7 configuration bits"]
pub use CTL0_W as CTL7_W;
#[doc = "Field `CTL6` writer - Pin 6 configuration bits"]
pub use CTL0_W as CTL6_W;
#[doc = "Field `CTL5` writer - Pin 5 configuration bits"]
pub use CTL0_W as CTL5_W;
#[doc = "Field `CTL4` writer - Pin 4 configuration bits"]
pub use CTL0_W as CTL4_W;
#[doc = "Field `CTL3` writer - Pin 3 configuration bits"]
pub use CTL0_W as CTL3_W;
#[doc = "Field `CTL2` writer - Pin 2 configuration bits"]
pub use CTL0_W as CTL2_W;
#[doc = "Field `CTL1` writer - Pin 1 configuration bits"]
pub use CTL0_W as CTL1_W;
#[doc = "Pin 0 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTL0_A {
    #[doc = "0: Input mode (reset state)"]
    INPUT = 0,
    #[doc = "1: General purpose output mode"]
    OUTPUT = 1,
    #[doc = "2: Alternate function mode"]
    ALTERNATE = 2,
    #[doc = "3: Analog mode"]
    ANALOG = 3,
}
impl From<CTL0_A> for u8 {
    #[inline(always)]
    fn from(variant: CTL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTL0` reader - Pin 0 configuration bits"]
pub type CTL0_R = crate::FieldReader<u8, CTL0_A>;
impl CTL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL0_A {
        match self.bits {
            0 => CTL0_A::INPUT,
            1 => CTL0_A::OUTPUT,
            2 => CTL0_A::ALTERNATE,
            3 => CTL0_A::ANALOG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == CTL0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CTL0_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == CTL0_A::ALTERNATE
    }
    #[doc = "Checks if the value of the field is `ANALOG`"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == CTL0_A::ANALOG
    }
}
#[doc = "Field `CTL0` writer - Pin 0 configuration bits"]
pub type CTL0_W<'a> = crate::FieldWriterSafe<'a, u32, CTL_SPEC, u8, CTL0_A, 2, 0>;
impl<'a> CTL0_W<'a> {
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(CTL0_A::INPUT)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CTL0_A::OUTPUT)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(CTL0_A::ALTERNATE)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(CTL0_A::ANALOG)
    }
}
impl R {
    #[doc = "Bits 30:31 - Pin 15 configuration bits"]
    #[inline(always)]
    pub fn ctl15(&self) -> CTL15_R {
        CTL15_R::new(((self.bits >> 30) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Pin 14 configuration bits"]
    #[inline(always)]
    pub fn ctl14(&self) -> CTL14_R {
        CTL14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Pin 13 configuration bits"]
    #[inline(always)]
    pub fn ctl13(&self) -> CTL13_R {
        CTL13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Pin 12 configuration bits"]
    #[inline(always)]
    pub fn ctl12(&self) -> CTL12_R {
        CTL12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin 11 configuration bits"]
    #[inline(always)]
    pub fn ctl11(&self) -> CTL11_R {
        CTL11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin 10 configuration bits"]
    #[inline(always)]
    pub fn ctl10(&self) -> CTL10_R {
        CTL10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin 9 configuration bits"]
    #[inline(always)]
    pub fn ctl9(&self) -> CTL9_R {
        CTL9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin 8 configuration bits"]
    #[inline(always)]
    pub fn ctl8(&self) -> CTL8_R {
        CTL8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Pin 7 configuration bits"]
    #[inline(always)]
    pub fn ctl7(&self) -> CTL7_R {
        CTL7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pin 6 configuration bits"]
    #[inline(always)]
    pub fn ctl6(&self) -> CTL6_R {
        CTL6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin 5 configuration bits"]
    #[inline(always)]
    pub fn ctl5(&self) -> CTL5_R {
        CTL5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin 4 configuration bits"]
    #[inline(always)]
    pub fn ctl4(&self) -> CTL4_R {
        CTL4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin 3 configuration bits"]
    #[inline(always)]
    pub fn ctl3(&self) -> CTL3_R {
        CTL3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin 2 configuration bits"]
    #[inline(always)]
    pub fn ctl2(&self) -> CTL2_R {
        CTL2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin 1 configuration bits"]
    #[inline(always)]
    pub fn ctl1(&self) -> CTL1_R {
        CTL1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Pin 0 configuration bits"]
    #[inline(always)]
    pub fn ctl0(&self) -> CTL0_R {
        CTL0_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Pin 15 configuration bits"]
    #[inline(always)]
    pub fn ctl15(&mut self) -> CTL15_W {
        CTL15_W::new(self)
    }
    #[doc = "Bits 28:29 - Pin 14 configuration bits"]
    #[inline(always)]
    pub fn ctl14(&mut self) -> CTL14_W {
        CTL14_W::new(self)
    }
    #[doc = "Bits 26:27 - Pin 13 configuration bits"]
    #[inline(always)]
    pub fn ctl13(&mut self) -> CTL13_W {
        CTL13_W::new(self)
    }
    #[doc = "Bits 24:25 - Pin 12 configuration bits"]
    #[inline(always)]
    pub fn ctl12(&mut self) -> CTL12_W {
        CTL12_W::new(self)
    }
    #[doc = "Bits 22:23 - Pin 11 configuration bits"]
    #[inline(always)]
    pub fn ctl11(&mut self) -> CTL11_W {
        CTL11_W::new(self)
    }
    #[doc = "Bits 20:21 - Pin 10 configuration bits"]
    #[inline(always)]
    pub fn ctl10(&mut self) -> CTL10_W {
        CTL10_W::new(self)
    }
    #[doc = "Bits 18:19 - Pin 9 configuration bits"]
    #[inline(always)]
    pub fn ctl9(&mut self) -> CTL9_W {
        CTL9_W::new(self)
    }
    #[doc = "Bits 16:17 - Pin 8 configuration bits"]
    #[inline(always)]
    pub fn ctl8(&mut self) -> CTL8_W {
        CTL8_W::new(self)
    }
    #[doc = "Bits 14:15 - Pin 7 configuration bits"]
    #[inline(always)]
    pub fn ctl7(&mut self) -> CTL7_W {
        CTL7_W::new(self)
    }
    #[doc = "Bits 12:13 - Pin 6 configuration bits"]
    #[inline(always)]
    pub fn ctl6(&mut self) -> CTL6_W {
        CTL6_W::new(self)
    }
    #[doc = "Bits 10:11 - Pin 5 configuration bits"]
    #[inline(always)]
    pub fn ctl5(&mut self) -> CTL5_W {
        CTL5_W::new(self)
    }
    #[doc = "Bits 8:9 - Pin 4 configuration bits"]
    #[inline(always)]
    pub fn ctl4(&mut self) -> CTL4_W {
        CTL4_W::new(self)
    }
    #[doc = "Bits 6:7 - Pin 3 configuration bits"]
    #[inline(always)]
    pub fn ctl3(&mut self) -> CTL3_W {
        CTL3_W::new(self)
    }
    #[doc = "Bits 4:5 - Pin 2 configuration bits"]
    #[inline(always)]
    pub fn ctl2(&mut self) -> CTL2_W {
        CTL2_W::new(self)
    }
    #[doc = "Bits 2:3 - Pin 1 configuration bits"]
    #[inline(always)]
    pub fn ctl1(&mut self) -> CTL1_W {
        CTL1_W::new(self)
    }
    #[doc = "Bits 0:1 - Pin 0 configuration bits"]
    #[inline(always)]
    pub fn ctl0(&mut self) -> CTL0_W {
        CTL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
