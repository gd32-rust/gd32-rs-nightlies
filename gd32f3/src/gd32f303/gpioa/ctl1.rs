#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port x configuration bits (x = 15)"]
pub use CTL8_A as CTL15_A;
#[doc = "Port x configuration bits (x = 14)"]
pub use CTL8_A as CTL14_A;
#[doc = "Port x configuration bits (x = 13)"]
pub use CTL8_A as CTL13_A;
#[doc = "Port x configuration bits (x = 12)"]
pub use CTL8_A as CTL12_A;
#[doc = "Port x configuration bits (x = 11)"]
pub use CTL8_A as CTL11_A;
#[doc = "Port x configuration bits (x = 10)"]
pub use CTL8_A as CTL10_A;
#[doc = "Port x configuration bits (x = 9)"]
pub use CTL8_A as CTL9_A;
#[doc = "Field `CTL15` reader - Port x configuration bits (x = 15)"]
pub use CTL8_R as CTL15_R;
#[doc = "Field `CTL14` reader - Port x configuration bits (x = 14)"]
pub use CTL8_R as CTL14_R;
#[doc = "Field `CTL13` reader - Port x configuration bits (x = 13)"]
pub use CTL8_R as CTL13_R;
#[doc = "Field `CTL12` reader - Port x configuration bits (x = 12)"]
pub use CTL8_R as CTL12_R;
#[doc = "Field `CTL11` reader - Port x configuration bits (x = 11)"]
pub use CTL8_R as CTL11_R;
#[doc = "Field `CTL10` reader - Port x configuration bits (x = 10)"]
pub use CTL8_R as CTL10_R;
#[doc = "Field `CTL9` reader - Port x configuration bits (x = 9)"]
pub use CTL8_R as CTL9_R;
#[doc = "Field `CTL15` writer - Port x configuration bits (x = 15)"]
pub use CTL8_W as CTL15_W;
#[doc = "Field `CTL14` writer - Port x configuration bits (x = 14)"]
pub use CTL8_W as CTL14_W;
#[doc = "Field `CTL13` writer - Port x configuration bits (x = 13)"]
pub use CTL8_W as CTL13_W;
#[doc = "Field `CTL12` writer - Port x configuration bits (x = 12)"]
pub use CTL8_W as CTL12_W;
#[doc = "Field `CTL11` writer - Port x configuration bits (x = 11)"]
pub use CTL8_W as CTL11_W;
#[doc = "Field `CTL10` writer - Port x configuration bits (x = 10)"]
pub use CTL8_W as CTL10_W;
#[doc = "Field `CTL9` writer - Port x configuration bits (x = 9)"]
pub use CTL8_W as CTL9_W;
#[doc = "Port x mode bits (x = 15)"]
pub use MD8_A as MD15_A;
#[doc = "Port x mode bits (x = 14)"]
pub use MD8_A as MD14_A;
#[doc = "Port x mode bits (x = 13)"]
pub use MD8_A as MD13_A;
#[doc = "Port x mode bits (x = 12)"]
pub use MD8_A as MD12_A;
#[doc = "Port x mode bits (x = 11 )"]
pub use MD8_A as MD11_A;
#[doc = "Port x mode bits (x = 10 )"]
pub use MD8_A as MD10_A;
#[doc = "Port x mode bits (x = 9)"]
pub use MD8_A as MD9_A;
#[doc = "Field `MD15` reader - Port x mode bits (x = 15)"]
pub use MD8_R as MD15_R;
#[doc = "Field `MD14` reader - Port x mode bits (x = 14)"]
pub use MD8_R as MD14_R;
#[doc = "Field `MD13` reader - Port x mode bits (x = 13)"]
pub use MD8_R as MD13_R;
#[doc = "Field `MD12` reader - Port x mode bits (x = 12)"]
pub use MD8_R as MD12_R;
#[doc = "Field `MD11` reader - Port x mode bits (x = 11 )"]
pub use MD8_R as MD11_R;
#[doc = "Field `MD10` reader - Port x mode bits (x = 10 )"]
pub use MD8_R as MD10_R;
#[doc = "Field `MD9` reader - Port x mode bits (x = 9)"]
pub use MD8_R as MD9_R;
#[doc = "Field `MD15` writer - Port x mode bits (x = 15)"]
pub use MD8_W as MD15_W;
#[doc = "Field `MD14` writer - Port x mode bits (x = 14)"]
pub use MD8_W as MD14_W;
#[doc = "Field `MD13` writer - Port x mode bits (x = 13)"]
pub use MD8_W as MD13_W;
#[doc = "Field `MD12` writer - Port x mode bits (x = 12)"]
pub use MD8_W as MD12_W;
#[doc = "Field `MD11` writer - Port x mode bits (x = 11 )"]
pub use MD8_W as MD11_W;
#[doc = "Field `MD10` writer - Port x mode bits (x = 10 )"]
pub use MD8_W as MD10_W;
#[doc = "Field `MD9` writer - Port x mode bits (x = 9)"]
pub use MD8_W as MD9_W;
#[doc = "Port x configuration bits (x = 8)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTL8_A {
    #[doc = "0: Analog mode/GPIO output with push-pull"]
    ANALOGORPUSHPULL = 0,
    #[doc = "1: Floating input/GPIO output with open-drain"]
    FLOATINGOROPENDRAIN = 1,
    #[doc = "2: Input with pull-up pull-down/AFIO output with push-pull"]
    INPUTORAFIOPP = 2,
    #[doc = "3: Reserved/AFIO output with open-drain"]
    RSVDORAFIOOD = 3,
}
impl From<CTL8_A> for u8 {
    #[inline(always)]
    fn from(variant: CTL8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTL8` reader - Port x configuration bits (x = 8)"]
pub type CTL8_R = crate::FieldReader<u8, CTL8_A>;
impl CTL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL8_A {
        match self.bits {
            0 => CTL8_A::ANALOGORPUSHPULL,
            1 => CTL8_A::FLOATINGOROPENDRAIN,
            2 => CTL8_A::INPUTORAFIOPP,
            3 => CTL8_A::RSVDORAFIOOD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ANALOGORPUSHPULL`"]
    #[inline(always)]
    pub fn is_analog_or_push_pull(&self) -> bool {
        *self == CTL8_A::ANALOGORPUSHPULL
    }
    #[doc = "Checks if the value of the field is `FLOATINGOROPENDRAIN`"]
    #[inline(always)]
    pub fn is_floating_or_open_drain(&self) -> bool {
        *self == CTL8_A::FLOATINGOROPENDRAIN
    }
    #[doc = "Checks if the value of the field is `INPUTORAFIOPP`"]
    #[inline(always)]
    pub fn is_input_or_afio_pp(&self) -> bool {
        *self == CTL8_A::INPUTORAFIOPP
    }
    #[doc = "Checks if the value of the field is `RSVDORAFIOOD`"]
    #[inline(always)]
    pub fn is_rsvdor_afio_od(&self) -> bool {
        *self == CTL8_A::RSVDORAFIOOD
    }
}
#[doc = "Field `CTL8` writer - Port x configuration bits (x = 8)"]
pub type CTL8_W<'a> = crate::FieldWriterSafe<'a, u32, CTL1_SPEC, u8, CTL8_A, 2, 2>;
impl<'a> CTL8_W<'a> {
    #[doc = "Analog mode/GPIO output with push-pull"]
    #[inline(always)]
    pub fn analog_or_push_pull(self) -> &'a mut W {
        self.variant(CTL8_A::ANALOGORPUSHPULL)
    }
    #[doc = "Floating input/GPIO output with open-drain"]
    #[inline(always)]
    pub fn floating_or_open_drain(self) -> &'a mut W {
        self.variant(CTL8_A::FLOATINGOROPENDRAIN)
    }
    #[doc = "Input with pull-up pull-down/AFIO output with push-pull"]
    #[inline(always)]
    pub fn input_or_afio_pp(self) -> &'a mut W {
        self.variant(CTL8_A::INPUTORAFIOPP)
    }
    #[doc = "Reserved/AFIO output with open-drain"]
    #[inline(always)]
    pub fn rsvdor_afio_od(self) -> &'a mut W {
        self.variant(CTL8_A::RSVDORAFIOOD)
    }
}
#[doc = "Port x mode bits (x = 8)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MD8_A {
    #[doc = "0: Input mode (reset state)"]
    INPUT = 0,
    #[doc = "1: Output mode ,max speed 10MHz"]
    SPEED10M = 1,
    #[doc = "2: Output mode ,max speed 2MHz"]
    SPEED2M = 2,
    #[doc = "3: Output mode ,max speed 50MHz"]
    SPEED50M = 3,
}
impl From<MD8_A> for u8 {
    #[inline(always)]
    fn from(variant: MD8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MD8` reader - Port x mode bits (x = 8)"]
pub type MD8_R = crate::FieldReader<u8, MD8_A>;
impl MD8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MD8_A {
        match self.bits {
            0 => MD8_A::INPUT,
            1 => MD8_A::SPEED10M,
            2 => MD8_A::SPEED2M,
            3 => MD8_A::SPEED50M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MD8_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SPEED10M`"]
    #[inline(always)]
    pub fn is_speed10m(&self) -> bool {
        *self == MD8_A::SPEED10M
    }
    #[doc = "Checks if the value of the field is `SPEED2M`"]
    #[inline(always)]
    pub fn is_speed2m(&self) -> bool {
        *self == MD8_A::SPEED2M
    }
    #[doc = "Checks if the value of the field is `SPEED50M`"]
    #[inline(always)]
    pub fn is_speed50m(&self) -> bool {
        *self == MD8_A::SPEED50M
    }
}
#[doc = "Field `MD8` writer - Port x mode bits (x = 8)"]
pub type MD8_W<'a> = crate::FieldWriterSafe<'a, u32, CTL1_SPEC, u8, MD8_A, 2, 0>;
impl<'a> MD8_W<'a> {
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MD8_A::INPUT)
    }
    #[doc = "Output mode ,max speed 10MHz"]
    #[inline(always)]
    pub fn speed10m(self) -> &'a mut W {
        self.variant(MD8_A::SPEED10M)
    }
    #[doc = "Output mode ,max speed 2MHz"]
    #[inline(always)]
    pub fn speed2m(self) -> &'a mut W {
        self.variant(MD8_A::SPEED2M)
    }
    #[doc = "Output mode ,max speed 50MHz"]
    #[inline(always)]
    pub fn speed50m(self) -> &'a mut W {
        self.variant(MD8_A::SPEED50M)
    }
}
impl R {
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    pub fn ctl15(&self) -> CTL15_R {
        CTL15_R::new(((self.bits >> 30) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 15)"]
    #[inline(always)]
    pub fn md15(&self) -> MD15_R {
        MD15_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    pub fn ctl14(&self) -> CTL14_R {
        CTL14_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 14)"]
    #[inline(always)]
    pub fn md14(&self) -> MD14_R {
        MD14_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    pub fn ctl13(&self) -> CTL13_R {
        CTL13_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 13)"]
    #[inline(always)]
    pub fn md13(&self) -> MD13_R {
        MD13_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    pub fn ctl12(&self) -> CTL12_R {
        CTL12_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 12)"]
    #[inline(always)]
    pub fn md12(&self) -> MD12_R {
        MD12_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    pub fn ctl11(&self) -> CTL11_R {
        CTL11_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 11 )"]
    #[inline(always)]
    pub fn md11(&self) -> MD11_R {
        MD11_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    pub fn ctl10(&self) -> CTL10_R {
        CTL10_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 10 )"]
    #[inline(always)]
    pub fn md10(&self) -> MD10_R {
        MD10_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    pub fn ctl9(&self) -> CTL9_R {
        CTL9_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 9)"]
    #[inline(always)]
    pub fn md9(&self) -> MD9_R {
        MD9_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    pub fn ctl8(&self) -> CTL8_R {
        CTL8_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Port x mode bits (x = 8)"]
    #[inline(always)]
    pub fn md8(&self) -> MD8_R {
        MD8_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    pub fn ctl15(&mut self) -> CTL15_W {
        CTL15_W::new(self)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 15)"]
    #[inline(always)]
    pub fn md15(&mut self) -> MD15_W {
        MD15_W::new(self)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    pub fn ctl14(&mut self) -> CTL14_W {
        CTL14_W::new(self)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 14)"]
    #[inline(always)]
    pub fn md14(&mut self) -> MD14_W {
        MD14_W::new(self)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    pub fn ctl13(&mut self) -> CTL13_W {
        CTL13_W::new(self)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 13)"]
    #[inline(always)]
    pub fn md13(&mut self) -> MD13_W {
        MD13_W::new(self)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    pub fn ctl12(&mut self) -> CTL12_W {
        CTL12_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 12)"]
    #[inline(always)]
    pub fn md12(&mut self) -> MD12_W {
        MD12_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    pub fn ctl11(&mut self) -> CTL11_W {
        CTL11_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 11 )"]
    #[inline(always)]
    pub fn md11(&mut self) -> MD11_W {
        MD11_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    pub fn ctl10(&mut self) -> CTL10_W {
        CTL10_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 10 )"]
    #[inline(always)]
    pub fn md10(&mut self) -> MD10_W {
        MD10_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    pub fn ctl9(&mut self) -> CTL9_W {
        CTL9_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 9)"]
    #[inline(always)]
    pub fn md9(&mut self) -> MD9_W {
        MD9_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    pub fn ctl8(&mut self) -> CTL8_W {
        CTL8_W::new(self)
    }
    #[doc = "Bits 0:1 - Port x mode bits (x = 8)"]
    #[inline(always)]
    pub fn md8(&mut self) -> MD8_W {
        MD8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "port control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0x4444_4444"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4444_4444
    }
}
