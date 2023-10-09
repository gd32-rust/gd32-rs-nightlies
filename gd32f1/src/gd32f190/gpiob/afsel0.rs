#[doc = "Register `AFSEL0` reader"]
pub type R = crate::R<AFSEL0_SPEC>;
#[doc = "Register `AFSEL0` writer"]
pub type W = crate::W<AFSEL0_SPEC>;
#[doc = "Field `SEL0` reader - Pin 0 alternate function selected"]
pub type SEL0_R = crate::FieldReader<SEL0_A>;
#[doc = "Pin 0 alternate function selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SEL0_A {
    type Ux = u8;
}
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
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == SEL0_A::AF0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == SEL0_A::AF1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == SEL0_A::AF2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == SEL0_A::AF3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == SEL0_A::AF4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == SEL0_A::AF5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == SEL0_A::AF6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == SEL0_A::AF7
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == SEL0_A::AF9
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == SEL0_A::AF11
    }
}
#[doc = "Field `SEL0` writer - Pin 0 alternate function selected"]
pub type SEL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, SEL0_A>;
impl<'a, REG, const O: u8> SEL0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::AF0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::AF1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::AF2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::AF3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::AF4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::AF5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::AF6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::AF7)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::AF9)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::AF11)
    }
}
#[doc = "Field `SEL1` reader - Pin 1 alternate function selected"]
pub use SEL0_R as SEL1_R;
#[doc = "Field `SEL2` reader - Pin 2 alternate function selected"]
pub use SEL0_R as SEL2_R;
#[doc = "Field `SEL3` reader - Pin 3 alternate function selected"]
pub use SEL0_R as SEL3_R;
#[doc = "Field `SEL4` reader - Pin 4 alternate function selected"]
pub use SEL0_R as SEL4_R;
#[doc = "Field `SEL5` reader - Pin 5 alternate function selected"]
pub use SEL0_R as SEL5_R;
#[doc = "Field `SEL6` reader - Pin 6 alternate function selected"]
pub use SEL0_R as SEL6_R;
#[doc = "Field `SEL7` reader - Pin 7 alternate function selected"]
pub use SEL0_R as SEL7_R;
#[doc = "Field `SEL1` writer - Pin 1 alternate function selected"]
pub use SEL0_W as SEL1_W;
#[doc = "Field `SEL2` writer - Pin 2 alternate function selected"]
pub use SEL0_W as SEL2_W;
#[doc = "Field `SEL3` writer - Pin 3 alternate function selected"]
pub use SEL0_W as SEL3_W;
#[doc = "Field `SEL4` writer - Pin 4 alternate function selected"]
pub use SEL0_W as SEL4_W;
#[doc = "Field `SEL5` writer - Pin 5 alternate function selected"]
pub use SEL0_W as SEL5_W;
#[doc = "Field `SEL6` writer - Pin 6 alternate function selected"]
pub use SEL0_W as SEL6_W;
#[doc = "Field `SEL7` writer - Pin 7 alternate function selected"]
pub use SEL0_W as SEL7_W;
impl R {
    #[doc = "Bits 0:3 - Pin 0 alternate function selected"]
    #[inline(always)]
    pub fn sel0(&self) -> SEL0_R {
        SEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pin 1 alternate function selected"]
    #[inline(always)]
    pub fn sel1(&self) -> SEL1_R {
        SEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pin 2 alternate function selected"]
    #[inline(always)]
    pub fn sel2(&self) -> SEL2_R {
        SEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pin 3 alternate function selected"]
    #[inline(always)]
    pub fn sel3(&self) -> SEL3_R {
        SEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Pin 4 alternate function selected"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL4_R {
        SEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Pin 5 alternate function selected"]
    #[inline(always)]
    pub fn sel5(&self) -> SEL5_R {
        SEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Pin 6 alternate function selected"]
    #[inline(always)]
    pub fn sel6(&self) -> SEL6_R {
        SEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Pin 7 alternate function selected"]
    #[inline(always)]
    pub fn sel7(&self) -> SEL7_R {
        SEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin 0 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> SEL0_W<AFSEL0_SPEC, 0> {
        SEL0_W::new(self)
    }
    #[doc = "Bits 4:7 - Pin 1 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> SEL1_W<AFSEL0_SPEC, 4> {
        SEL1_W::new(self)
    }
    #[doc = "Bits 8:11 - Pin 2 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> SEL2_W<AFSEL0_SPEC, 8> {
        SEL2_W::new(self)
    }
    #[doc = "Bits 12:15 - Pin 3 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel3(&mut self) -> SEL3_W<AFSEL0_SPEC, 12> {
        SEL3_W::new(self)
    }
    #[doc = "Bits 16:19 - Pin 4 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel4(&mut self) -> SEL4_W<AFSEL0_SPEC, 16> {
        SEL4_W::new(self)
    }
    #[doc = "Bits 20:23 - Pin 5 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel5(&mut self) -> SEL5_W<AFSEL0_SPEC, 20> {
        SEL5_W::new(self)
    }
    #[doc = "Bits 24:27 - Pin 6 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel6(&mut self) -> SEL6_W<AFSEL0_SPEC, 24> {
        SEL6_W::new(self)
    }
    #[doc = "Bits 28:31 - Pin 7 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel7(&mut self) -> SEL7_W<AFSEL0_SPEC, 28> {
        SEL7_W::new(self)
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
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFSEL0_SPEC;
impl crate::RegisterSpec for AFSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afsel0::R`](R) reader structure"]
impl crate::Readable for AFSEL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afsel0::W`](W) writer structure"]
impl crate::Writable for AFSEL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFSEL0 to value 0"]
impl crate::Resettable for AFSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
