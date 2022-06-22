#[doc = "Register `AFSEL0` reader"]
pub struct R(crate::R<AFSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFSEL0` writer"]
pub struct W(crate::W<AFSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFSEL0_SPEC>;
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
impl From<crate::W<AFSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_A as SEL7_A;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_A as SEL6_A;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_A as SEL5_A;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_A as SEL4_A;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_A as SEL3_A;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_A as SEL2_A;
#[doc = "Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_A as SEL1_A;
#[doc = "Field `SEL7` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_R as SEL7_R;
#[doc = "Field `SEL6` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_R as SEL6_R;
#[doc = "Field `SEL5` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_R as SEL5_R;
#[doc = "Field `SEL4` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_R as SEL4_R;
#[doc = "Field `SEL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_R as SEL3_R;
#[doc = "Field `SEL2` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_R as SEL2_R;
#[doc = "Field `SEL1` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_R as SEL1_R;
#[doc = "Field `SEL7` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_W as SEL7_W;
#[doc = "Field `SEL6` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_W as SEL6_W;
#[doc = "Field `SEL5` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_W as SEL5_W;
#[doc = "Field `SEL4` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_W as SEL4_W;
#[doc = "Field `SEL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_W as SEL3_W;
#[doc = "Field `SEL2` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_W as SEL2_W;
#[doc = "Field `SEL1` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use SEL0_W as SEL1_W;
#[doc = "Alternate function selection for port x bit y (y = 0..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL0_A {
    #[doc = "0: AF0"]
    AF0 = 0,
    #[doc = "1: AF1"]
    AF1 = 1,
    #[doc = "2: AF2"]
    AF2 = 2,
    #[doc = "3: AF3"]
    AF3 = 3,
    #[doc = "4: AF4"]
    AF4 = 4,
    #[doc = "5: AF5"]
    AF5 = 5,
    #[doc = "6: AF6"]
    AF6 = 6,
    #[doc = "7: AF7"]
    AF7 = 7,
    #[doc = "9: AF9"]
    AF9 = 9,
    #[doc = "11: AF11"]
    AF11 = 11,
}
impl From<SEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL0` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type SEL0_R = crate::FieldReader<u8, SEL0_A>;
impl SEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL0_A> {
        match self.bits {
            0 => Some(SEL0_A::AF0),
            1 => Some(SEL0_A::AF1),
            2 => Some(SEL0_A::AF2),
            3 => Some(SEL0_A::AF3),
            4 => Some(SEL0_A::AF4),
            5 => Some(SEL0_A::AF5),
            6 => Some(SEL0_A::AF6),
            7 => Some(SEL0_A::AF7),
            9 => Some(SEL0_A::AF9),
            11 => Some(SEL0_A::AF11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AF0`"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == SEL0_A::AF0
    }
    #[doc = "Checks if the value of the field is `AF1`"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == SEL0_A::AF1
    }
    #[doc = "Checks if the value of the field is `AF2`"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == SEL0_A::AF2
    }
    #[doc = "Checks if the value of the field is `AF3`"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == SEL0_A::AF3
    }
    #[doc = "Checks if the value of the field is `AF4`"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == SEL0_A::AF4
    }
    #[doc = "Checks if the value of the field is `AF5`"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == SEL0_A::AF5
    }
    #[doc = "Checks if the value of the field is `AF6`"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == SEL0_A::AF6
    }
    #[doc = "Checks if the value of the field is `AF7`"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == SEL0_A::AF7
    }
    #[doc = "Checks if the value of the field is `AF9`"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == SEL0_A::AF9
    }
    #[doc = "Checks if the value of the field is `AF11`"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == SEL0_A::AF11
    }
}
#[doc = "Field `SEL0` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type SEL0_W<'a> = crate::FieldWriter<'a, u32, AFSEL0_SPEC, u8, SEL0_A, 4, 0>;
impl<'a> SEL0_W<'a> {
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(SEL0_A::AF0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(SEL0_A::AF1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(SEL0_A::AF2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(SEL0_A::AF3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(SEL0_A::AF4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(SEL0_A::AF5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(SEL0_A::AF6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(SEL0_A::AF7)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(SEL0_A::AF9)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(SEL0_A::AF11)
    }
}
impl R {
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel7(&self) -> SEL7_R {
        SEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel6(&self) -> SEL6_R {
        SEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel5(&self) -> SEL5_R {
        SEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL4_R {
        SEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel3(&self) -> SEL3_R {
        SEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel2(&self) -> SEL2_R {
        SEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel1(&self) -> SEL1_R {
        SEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel0(&self) -> SEL0_R {
        SEL0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel7(&mut self) -> SEL7_W {
        SEL7_W::new(self)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel6(&mut self) -> SEL6_W {
        SEL6_W::new(self)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel5(&mut self) -> SEL5_W {
        SEL5_W::new(self)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel4(&mut self) -> SEL4_W {
        SEL4_W::new(self)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel3(&mut self) -> SEL3_W {
        SEL3_W::new(self)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel2(&mut self) -> SEL2_W {
        SEL2_W::new(self)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel1(&mut self) -> SEL1_W {
        SEL1_W::new(self)
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel0(&mut self) -> SEL0_W {
        SEL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afsel0](index.html) module"]
pub struct AFSEL0_SPEC;
impl crate::RegisterSpec for AFSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afsel0::R](R) reader structure"]
impl crate::Readable for AFSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afsel0::W](W) writer structure"]
impl crate::Writable for AFSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFSEL0 to value 0"]
impl crate::Resettable for AFSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
