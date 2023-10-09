#[doc = "Register `AFSEL1` reader"]
pub type R = crate::R<AFSEL1_SPEC>;
#[doc = "Register `AFSEL1` writer"]
pub type W = crate::W<AFSEL1_SPEC>;
#[doc = "Field `SEL8` reader - Pin 8 alternate function selected"]
pub type SEL8_R = crate::FieldReader<SEL8_A>;
#[doc = "Pin 8 alternate function selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SEL8_A {
    type Ux = u8;
}
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
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == SEL8_A::AF0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == SEL8_A::AF1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == SEL8_A::AF2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == SEL8_A::AF3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == SEL8_A::AF4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == SEL8_A::AF5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == SEL8_A::AF6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == SEL8_A::AF7
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == SEL8_A::AF9
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == SEL8_A::AF11
    }
}
#[doc = "Field `SEL8` writer - Pin 8 alternate function selected"]
pub type SEL8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, SEL8_A>;
impl<'a, REG, const O: u8> SEL8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(SEL8_A::AF0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(SEL8_A::AF1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(SEL8_A::AF2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(SEL8_A::AF3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(SEL8_A::AF4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(SEL8_A::AF5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(SEL8_A::AF6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(SEL8_A::AF7)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut crate::W<REG> {
        self.variant(SEL8_A::AF9)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut crate::W<REG> {
        self.variant(SEL8_A::AF11)
    }
}
#[doc = "Field `SEL9` reader - Pin 9 alternate function selected"]
pub use SEL8_R as SEL9_R;
#[doc = "Field `SEL10` reader - Pin 10 alternate function selected"]
pub use SEL8_R as SEL10_R;
#[doc = "Field `SEL11` reader - Pin 11 alternate function selected"]
pub use SEL8_R as SEL11_R;
#[doc = "Field `SEL12` reader - Pin 12 alternate function selected"]
pub use SEL8_R as SEL12_R;
#[doc = "Field `SEL13` reader - Pin 13 alternate function selected"]
pub use SEL8_R as SEL13_R;
#[doc = "Field `SEL14` reader - Pin 14 alternate function selected"]
pub use SEL8_R as SEL14_R;
#[doc = "Field `SEL15` reader - Pin 15 alternate function selected"]
pub use SEL8_R as SEL15_R;
#[doc = "Field `SEL9` writer - Pin 9 alternate function selected"]
pub use SEL8_W as SEL9_W;
#[doc = "Field `SEL10` writer - Pin 10 alternate function selected"]
pub use SEL8_W as SEL10_W;
#[doc = "Field `SEL11` writer - Pin 11 alternate function selected"]
pub use SEL8_W as SEL11_W;
#[doc = "Field `SEL12` writer - Pin 12 alternate function selected"]
pub use SEL8_W as SEL12_W;
#[doc = "Field `SEL13` writer - Pin 13 alternate function selected"]
pub use SEL8_W as SEL13_W;
#[doc = "Field `SEL14` writer - Pin 14 alternate function selected"]
pub use SEL8_W as SEL14_W;
#[doc = "Field `SEL15` writer - Pin 15 alternate function selected"]
pub use SEL8_W as SEL15_W;
impl R {
    #[doc = "Bits 0:3 - Pin 8 alternate function selected"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL8_R {
        SEL8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pin 9 alternate function selected"]
    #[inline(always)]
    pub fn sel9(&self) -> SEL9_R {
        SEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pin 10 alternate function selected"]
    #[inline(always)]
    pub fn sel10(&self) -> SEL10_R {
        SEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pin 11 alternate function selected"]
    #[inline(always)]
    pub fn sel11(&self) -> SEL11_R {
        SEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Pin 12 alternate function selected"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL12_R {
        SEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Pin 13 alternate function selected"]
    #[inline(always)]
    pub fn sel13(&self) -> SEL13_R {
        SEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Pin 14 alternate function selected"]
    #[inline(always)]
    pub fn sel14(&self) -> SEL14_R {
        SEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Pin 15 alternate function selected"]
    #[inline(always)]
    pub fn sel15(&self) -> SEL15_R {
        SEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin 8 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel8(&mut self) -> SEL8_W<AFSEL1_SPEC, 0> {
        SEL8_W::new(self)
    }
    #[doc = "Bits 4:7 - Pin 9 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel9(&mut self) -> SEL9_W<AFSEL1_SPEC, 4> {
        SEL9_W::new(self)
    }
    #[doc = "Bits 8:11 - Pin 10 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel10(&mut self) -> SEL10_W<AFSEL1_SPEC, 8> {
        SEL10_W::new(self)
    }
    #[doc = "Bits 12:15 - Pin 11 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel11(&mut self) -> SEL11_W<AFSEL1_SPEC, 12> {
        SEL11_W::new(self)
    }
    #[doc = "Bits 16:19 - Pin 12 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel12(&mut self) -> SEL12_W<AFSEL1_SPEC, 16> {
        SEL12_W::new(self)
    }
    #[doc = "Bits 20:23 - Pin 13 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel13(&mut self) -> SEL13_W<AFSEL1_SPEC, 20> {
        SEL13_W::new(self)
    }
    #[doc = "Bits 24:27 - Pin 14 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel14(&mut self) -> SEL14_W<AFSEL1_SPEC, 24> {
        SEL14_W::new(self)
    }
    #[doc = "Bits 28:31 - Pin 15 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel15(&mut self) -> SEL15_W<AFSEL1_SPEC, 28> {
        SEL15_W::new(self)
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
#[doc = "GPIO alternate function register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFSEL1_SPEC;
impl crate::RegisterSpec for AFSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afsel1::R`](R) reader structure"]
impl crate::Readable for AFSEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afsel1::W`](W) writer structure"]
impl crate::Writable for AFSEL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFSEL1 to value 0"]
impl crate::Resettable for AFSEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
