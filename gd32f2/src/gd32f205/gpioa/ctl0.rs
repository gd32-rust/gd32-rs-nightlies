#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port x configuration bits (x = 7)"]
pub use CTL0_A as CTL7_A;
#[doc = "Port x configuration bits (x = 6)"]
pub use CTL0_A as CTL6_A;
#[doc = "Port x configuration bits (x = 5)"]
pub use CTL0_A as CTL5_A;
#[doc = "Port x configuration bits (x = 4)"]
pub use CTL0_A as CTL4_A;
#[doc = "Port x configuration bits (x = 3)"]
pub use CTL0_A as CTL3_A;
#[doc = "Port x configuration bits (x = 2)"]
pub use CTL0_A as CTL2_A;
#[doc = "Port x configuration bits (x = 1)"]
pub use CTL0_A as CTL1_A;
#[doc = "Field `CTL7` reader - Port x configuration bits (x = 7)"]
pub use CTL0_R as CTL7_R;
#[doc = "Field `CTL6` reader - Port x configuration bits (x = 6)"]
pub use CTL0_R as CTL6_R;
#[doc = "Field `CTL5` reader - Port x configuration bits (x = 5)"]
pub use CTL0_R as CTL5_R;
#[doc = "Field `CTL4` reader - Port x configuration bits (x = 4)"]
pub use CTL0_R as CTL4_R;
#[doc = "Field `CTL3` reader - Port x configuration bits (x = 3)"]
pub use CTL0_R as CTL3_R;
#[doc = "Field `CTL2` reader - Port x configuration bits (x = 2)"]
pub use CTL0_R as CTL2_R;
#[doc = "Field `CTL1` reader - Port x configuration bits (x = 1)"]
pub use CTL0_R as CTL1_R;
#[doc = "Field `CTL7` writer - Port x configuration bits (x = 7)"]
pub use CTL0_W as CTL7_W;
#[doc = "Field `CTL6` writer - Port x configuration bits (x = 6)"]
pub use CTL0_W as CTL6_W;
#[doc = "Field `CTL5` writer - Port x configuration bits (x = 5)"]
pub use CTL0_W as CTL5_W;
#[doc = "Field `CTL4` writer - Port x configuration bits (x = 4)"]
pub use CTL0_W as CTL4_W;
#[doc = "Field `CTL3` writer - Port x configuration bits (x = 3)"]
pub use CTL0_W as CTL3_W;
#[doc = "Field `CTL2` writer - Port x configuration bits (x = 2)"]
pub use CTL0_W as CTL2_W;
#[doc = "Field `CTL1` writer - Port x configuration bits (x = 1)"]
pub use CTL0_W as CTL1_W;
#[doc = "Port x mode bits (x = 7)"]
pub use MD0_A as MD7_A;
#[doc = "Port x mode bits (x = 6)"]
pub use MD0_A as MD6_A;
#[doc = "Port x mode bits (x = 5)"]
pub use MD0_A as MD5_A;
#[doc = "Port x mode bits (x = 4)"]
pub use MD0_A as MD4_A;
#[doc = "Port x mode bits (x = 3 )"]
pub use MD0_A as MD3_A;
#[doc = "Port x mode bits (x = 2 )"]
pub use MD0_A as MD2_A;
#[doc = "Port x mode bits (x = 1)"]
pub use MD0_A as MD1_A;
#[doc = "Field `MD7` reader - Port x mode bits (x = 7)"]
pub use MD0_R as MD7_R;
#[doc = "Field `MD6` reader - Port x mode bits (x = 6)"]
pub use MD0_R as MD6_R;
#[doc = "Field `MD5` reader - Port x mode bits (x = 5)"]
pub use MD0_R as MD5_R;
#[doc = "Field `MD4` reader - Port x mode bits (x = 4)"]
pub use MD0_R as MD4_R;
#[doc = "Field `MD3` reader - Port x mode bits (x = 3 )"]
pub use MD0_R as MD3_R;
#[doc = "Field `MD2` reader - Port x mode bits (x = 2 )"]
pub use MD0_R as MD2_R;
#[doc = "Field `MD1` reader - Port x mode bits (x = 1)"]
pub use MD0_R as MD1_R;
#[doc = "Field `MD7` writer - Port x mode bits (x = 7)"]
pub use MD0_W as MD7_W;
#[doc = "Field `MD6` writer - Port x mode bits (x = 6)"]
pub use MD0_W as MD6_W;
#[doc = "Field `MD5` writer - Port x mode bits (x = 5)"]
pub use MD0_W as MD5_W;
#[doc = "Field `MD4` writer - Port x mode bits (x = 4)"]
pub use MD0_W as MD4_W;
#[doc = "Field `MD3` writer - Port x mode bits (x = 3 )"]
pub use MD0_W as MD3_W;
#[doc = "Field `MD2` writer - Port x mode bits (x = 2 )"]
pub use MD0_W as MD2_W;
#[doc = "Field `MD1` writer - Port x mode bits (x = 1)"]
pub use MD0_W as MD1_W;
#[doc = "Port x configuration bits (x = 0)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTL0_A {
    #[doc = "0: Analog mode/GPIO output with push-pull"]
    ANALOGORPUSHPULL = 0,
    #[doc = "1: Floating input/GPIO output with open-drain"]
    FLOATINGOROPENDRAIN = 1,
    #[doc = "2: Input with pull-up pull-down/AFIO output with push-pull"]
    INPUTORAFIOPP = 2,
    #[doc = "3: Reserved/AFIO output with open-drain"]
    RSVDORAFIOOD = 3,
}
impl From<CTL0_A> for u8 {
    #[inline(always)]
    fn from(variant: CTL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTL0` reader - Port x configuration bits (x = 0)"]
pub type CTL0_R = crate::FieldReader<u8, CTL0_A>;
impl CTL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL0_A {
        match self.bits {
            0 => CTL0_A::ANALOGORPUSHPULL,
            1 => CTL0_A::FLOATINGOROPENDRAIN,
            2 => CTL0_A::INPUTORAFIOPP,
            3 => CTL0_A::RSVDORAFIOOD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ANALOGORPUSHPULL`"]
    #[inline(always)]
    pub fn is_analog_or_push_pull(&self) -> bool {
        *self == CTL0_A::ANALOGORPUSHPULL
    }
    #[doc = "Checks if the value of the field is `FLOATINGOROPENDRAIN`"]
    #[inline(always)]
    pub fn is_floating_or_open_drain(&self) -> bool {
        *self == CTL0_A::FLOATINGOROPENDRAIN
    }
    #[doc = "Checks if the value of the field is `INPUTORAFIOPP`"]
    #[inline(always)]
    pub fn is_input_or_afio_pp(&self) -> bool {
        *self == CTL0_A::INPUTORAFIOPP
    }
    #[doc = "Checks if the value of the field is `RSVDORAFIOOD`"]
    #[inline(always)]
    pub fn is_rsvdor_afio_od(&self) -> bool {
        *self == CTL0_A::RSVDORAFIOOD
    }
}
#[doc = "Field `CTL0` writer - Port x configuration bits (x = 0)"]
pub type CTL0_W<'a> = crate::FieldWriterSafe<'a, u32, CTL0_SPEC, u8, CTL0_A, 2, 2>;
impl<'a> CTL0_W<'a> {
    #[doc = "Analog mode/GPIO output with push-pull"]
    #[inline(always)]
    pub fn analog_or_push_pull(self) -> &'a mut W {
        self.variant(CTL0_A::ANALOGORPUSHPULL)
    }
    #[doc = "Floating input/GPIO output with open-drain"]
    #[inline(always)]
    pub fn floating_or_open_drain(self) -> &'a mut W {
        self.variant(CTL0_A::FLOATINGOROPENDRAIN)
    }
    #[doc = "Input with pull-up pull-down/AFIO output with push-pull"]
    #[inline(always)]
    pub fn input_or_afio_pp(self) -> &'a mut W {
        self.variant(CTL0_A::INPUTORAFIOPP)
    }
    #[doc = "Reserved/AFIO output with open-drain"]
    #[inline(always)]
    pub fn rsvdor_afio_od(self) -> &'a mut W {
        self.variant(CTL0_A::RSVDORAFIOOD)
    }
}
#[doc = "Port x mode bits (x = 0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MD0_A {
    #[doc = "0: Input mode (reset state)"]
    INPUT = 0,
    #[doc = "1: Output mode ,max speed 10MHz"]
    SPEED10M = 1,
    #[doc = "2: Output mode ,max speed 2MHz"]
    SPEED2M = 2,
    #[doc = "3: Output mode ,max speed 50MHz"]
    SPEED50M = 3,
}
impl From<MD0_A> for u8 {
    #[inline(always)]
    fn from(variant: MD0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MD0` reader - Port x mode bits (x = 0)"]
pub type MD0_R = crate::FieldReader<u8, MD0_A>;
impl MD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MD0_A {
        match self.bits {
            0 => MD0_A::INPUT,
            1 => MD0_A::SPEED10M,
            2 => MD0_A::SPEED2M,
            3 => MD0_A::SPEED50M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MD0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SPEED10M`"]
    #[inline(always)]
    pub fn is_speed10m(&self) -> bool {
        *self == MD0_A::SPEED10M
    }
    #[doc = "Checks if the value of the field is `SPEED2M`"]
    #[inline(always)]
    pub fn is_speed2m(&self) -> bool {
        *self == MD0_A::SPEED2M
    }
    #[doc = "Checks if the value of the field is `SPEED50M`"]
    #[inline(always)]
    pub fn is_speed50m(&self) -> bool {
        *self == MD0_A::SPEED50M
    }
}
#[doc = "Field `MD0` writer - Port x mode bits (x = 0)"]
pub type MD0_W<'a> = crate::FieldWriterSafe<'a, u32, CTL0_SPEC, u8, MD0_A, 2, 0>;
impl<'a> MD0_W<'a> {
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MD0_A::INPUT)
    }
    #[doc = "Output mode ,max speed 10MHz"]
    #[inline(always)]
    pub fn speed10m(self) -> &'a mut W {
        self.variant(MD0_A::SPEED10M)
    }
    #[doc = "Output mode ,max speed 2MHz"]
    #[inline(always)]
    pub fn speed2m(self) -> &'a mut W {
        self.variant(MD0_A::SPEED2M)
    }
    #[doc = "Output mode ,max speed 50MHz"]
    #[inline(always)]
    pub fn speed50m(self) -> &'a mut W {
        self.variant(MD0_A::SPEED50M)
    }
}
impl R {
    #[doc = "Bits 30:31 - Port x configuration bits (x = 7)"]
    #[inline(always)]
    pub fn ctl7(&self) -> CTL7_R {
        CTL7_R::new(((self.bits >> 30) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 7)"]
    #[inline(always)]
    pub fn md7(&self) -> MD7_R {
        MD7_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 6)"]
    #[inline(always)]
    pub fn ctl6(&self) -> CTL6_R {
        CTL6_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 6)"]
    #[inline(always)]
    pub fn md6(&self) -> MD6_R {
        MD6_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 5)"]
    #[inline(always)]
    pub fn ctl5(&self) -> CTL5_R {
        CTL5_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 5)"]
    #[inline(always)]
    pub fn md5(&self) -> MD5_R {
        MD5_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 4)"]
    #[inline(always)]
    pub fn ctl4(&self) -> CTL4_R {
        CTL4_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 4)"]
    #[inline(always)]
    pub fn md4(&self) -> MD4_R {
        MD4_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 3)"]
    #[inline(always)]
    pub fn ctl3(&self) -> CTL3_R {
        CTL3_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 3 )"]
    #[inline(always)]
    pub fn md3(&self) -> MD3_R {
        MD3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 2)"]
    #[inline(always)]
    pub fn ctl2(&self) -> CTL2_R {
        CTL2_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 2 )"]
    #[inline(always)]
    pub fn md2(&self) -> MD2_R {
        MD2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 1)"]
    #[inline(always)]
    pub fn ctl1(&self) -> CTL1_R {
        CTL1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 1)"]
    #[inline(always)]
    pub fn md1(&self) -> MD1_R {
        MD1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 0)"]
    #[inline(always)]
    pub fn ctl0(&self) -> CTL0_R {
        CTL0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Port x mode bits (x = 0)"]
    #[inline(always)]
    pub fn md0(&self) -> MD0_R {
        MD0_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (x = 7)"]
    #[inline(always)]
    pub fn ctl7(&mut self) -> CTL7_W {
        CTL7_W::new(self)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 7)"]
    #[inline(always)]
    pub fn md7(&mut self) -> MD7_W {
        MD7_W::new(self)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 6)"]
    #[inline(always)]
    pub fn ctl6(&mut self) -> CTL6_W {
        CTL6_W::new(self)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 6)"]
    #[inline(always)]
    pub fn md6(&mut self) -> MD6_W {
        MD6_W::new(self)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 5)"]
    #[inline(always)]
    pub fn ctl5(&mut self) -> CTL5_W {
        CTL5_W::new(self)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 5)"]
    #[inline(always)]
    pub fn md5(&mut self) -> MD5_W {
        MD5_W::new(self)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 4)"]
    #[inline(always)]
    pub fn ctl4(&mut self) -> CTL4_W {
        CTL4_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 4)"]
    #[inline(always)]
    pub fn md4(&mut self) -> MD4_W {
        MD4_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 3)"]
    #[inline(always)]
    pub fn ctl3(&mut self) -> CTL3_W {
        CTL3_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 3 )"]
    #[inline(always)]
    pub fn md3(&mut self) -> MD3_W {
        MD3_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 2)"]
    #[inline(always)]
    pub fn ctl2(&mut self) -> CTL2_W {
        CTL2_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 2 )"]
    #[inline(always)]
    pub fn md2(&mut self) -> MD2_W {
        MD2_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 1)"]
    #[inline(always)]
    pub fn ctl1(&mut self) -> CTL1_W {
        CTL1_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 1)"]
    #[inline(always)]
    pub fn md1(&mut self) -> MD1_W {
        MD1_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 0)"]
    #[inline(always)]
    pub fn ctl0(&mut self) -> CTL0_W {
        CTL0_W::new(self)
    }
    #[doc = "Bits 0:1 - Port x mode bits (x = 0)"]
    #[inline(always)]
    pub fn md0(&mut self) -> MD0_W {
        MD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "port control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL0 to value 0x4444_4444"]
impl crate::Resettable for CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4444_4444
    }
}
