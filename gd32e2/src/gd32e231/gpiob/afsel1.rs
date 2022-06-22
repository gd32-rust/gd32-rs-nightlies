#[doc = "Register `AFSEL1` reader"]
pub struct R(crate::R<AFSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFSEL1` writer"]
pub struct W(crate::W<AFSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFSEL1_SPEC>;
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
impl From<crate::W<AFSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_A as SEL15_A;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_A as SEL14_A;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_A as SEL13_A;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_A as SEL12_A;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_A as SEL11_A;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_A as SEL10_A;
#[doc = "Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_A as SEL9_A;
#[doc = "Field `SEL15` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_R as SEL15_R;
#[doc = "Field `SEL14` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_R as SEL14_R;
#[doc = "Field `SEL13` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_R as SEL13_R;
#[doc = "Field `SEL12` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_R as SEL12_R;
#[doc = "Field `SEL11` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_R as SEL11_R;
#[doc = "Field `SEL10` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_R as SEL10_R;
#[doc = "Field `SEL9` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_R as SEL9_R;
#[doc = "Field `SEL15` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_W as SEL15_W;
#[doc = "Field `SEL14` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_W as SEL14_W;
#[doc = "Field `SEL13` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_W as SEL13_W;
#[doc = "Field `SEL12` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_W as SEL12_W;
#[doc = "Field `SEL11` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_W as SEL11_W;
#[doc = "Field `SEL10` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_W as SEL10_W;
#[doc = "Field `SEL9` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use SEL8_W as SEL9_W;
#[doc = "Alternate function selection for port x bit y (y = 8..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL8_A {
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
impl From<SEL8_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL8` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type SEL8_R = crate::FieldReader<u8, SEL8_A>;
impl SEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL8_A> {
        match self.bits {
            0 => Some(SEL8_A::AF0),
            1 => Some(SEL8_A::AF1),
            2 => Some(SEL8_A::AF2),
            3 => Some(SEL8_A::AF3),
            4 => Some(SEL8_A::AF4),
            5 => Some(SEL8_A::AF5),
            6 => Some(SEL8_A::AF6),
            7 => Some(SEL8_A::AF7),
            9 => Some(SEL8_A::AF9),
            11 => Some(SEL8_A::AF11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AF0`"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == SEL8_A::AF0
    }
    #[doc = "Checks if the value of the field is `AF1`"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == SEL8_A::AF1
    }
    #[doc = "Checks if the value of the field is `AF2`"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == SEL8_A::AF2
    }
    #[doc = "Checks if the value of the field is `AF3`"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == SEL8_A::AF3
    }
    #[doc = "Checks if the value of the field is `AF4`"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == SEL8_A::AF4
    }
    #[doc = "Checks if the value of the field is `AF5`"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == SEL8_A::AF5
    }
    #[doc = "Checks if the value of the field is `AF6`"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == SEL8_A::AF6
    }
    #[doc = "Checks if the value of the field is `AF7`"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == SEL8_A::AF7
    }
    #[doc = "Checks if the value of the field is `AF9`"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == SEL8_A::AF9
    }
    #[doc = "Checks if the value of the field is `AF11`"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == SEL8_A::AF11
    }
}
#[doc = "Field `SEL8` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type SEL8_W<'a> = crate::FieldWriter<'a, u32, AFSEL1_SPEC, u8, SEL8_A, 4, 0>;
impl<'a> SEL8_W<'a> {
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(SEL8_A::AF0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(SEL8_A::AF1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(SEL8_A::AF2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(SEL8_A::AF3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(SEL8_A::AF4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(SEL8_A::AF5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(SEL8_A::AF6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(SEL8_A::AF7)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(SEL8_A::AF9)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(SEL8_A::AF11)
    }
}
impl R {
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel15(&self) -> SEL15_R {
        SEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel14(&self) -> SEL14_R {
        SEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel13(&self) -> SEL13_R {
        SEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL12_R {
        SEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel11(&self) -> SEL11_R {
        SEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel10(&self) -> SEL10_R {
        SEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel9(&self) -> SEL9_R {
        SEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL8_R {
        SEL8_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel15(&mut self) -> SEL15_W {
        SEL15_W::new(self)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel14(&mut self) -> SEL14_W {
        SEL14_W::new(self)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel13(&mut self) -> SEL13_W {
        SEL13_W::new(self)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel12(&mut self) -> SEL12_W {
        SEL12_W::new(self)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel11(&mut self) -> SEL11_W {
        SEL11_W::new(self)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel10(&mut self) -> SEL10_W {
        SEL10_W::new(self)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel9(&mut self) -> SEL9_W {
        SEL9_W::new(self)
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel8(&mut self) -> SEL8_W {
        SEL8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO alternate function register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afsel1](index.html) module"]
pub struct AFSEL1_SPEC;
impl crate::RegisterSpec for AFSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afsel1::R](R) reader structure"]
impl crate::Readable for AFSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afsel1::W](W) writer structure"]
impl crate::Writable for AFSEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFSEL1 to value 0"]
impl crate::Resettable for AFSEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
