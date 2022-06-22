#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PREDV0 division factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREDV0_A {
    #[doc = "0: Input to PLL not divided"]
    DIV1 = 0,
    #[doc = "1: Input to PLL divided by 2"]
    DIV2 = 1,
    #[doc = "2: Input to PLL divided by 3"]
    DIV3 = 2,
    #[doc = "3: Input to PLL divided by 4"]
    DIV4 = 3,
    #[doc = "4: Input to PLL divided by 5"]
    DIV5 = 4,
    #[doc = "5: Input to PLL divided by 6"]
    DIV6 = 5,
    #[doc = "6: Input to PLL divided by 7"]
    DIV7 = 6,
    #[doc = "7: Input to PLL divided by 8"]
    DIV8 = 7,
    #[doc = "8: Input to PLL divided by 9"]
    DIV9 = 8,
    #[doc = "9: Input to PLL divided by 10"]
    DIV10 = 9,
    #[doc = "10: Input to PLL divided by 11"]
    DIV11 = 10,
    #[doc = "11: Input to PLL divided by 12"]
    DIV12 = 11,
    #[doc = "12: Input to PLL divided by 13"]
    DIV13 = 12,
    #[doc = "13: Input to PLL divided by 14"]
    DIV14 = 13,
    #[doc = "14: Input to PLL divided by 15"]
    DIV15 = 14,
    #[doc = "15: Input to PLL divided by 16"]
    DIV16 = 15,
}
impl From<PREDV0_A> for u8 {
    #[inline(always)]
    fn from(variant: PREDV0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PREDV0` reader - PREDV0 division factor"]
pub type PREDV0_R = crate::FieldReader<u8, PREDV0_A>;
impl PREDV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREDV0_A {
        match self.bits {
            0 => PREDV0_A::DIV1,
            1 => PREDV0_A::DIV2,
            2 => PREDV0_A::DIV3,
            3 => PREDV0_A::DIV4,
            4 => PREDV0_A::DIV5,
            5 => PREDV0_A::DIV6,
            6 => PREDV0_A::DIV7,
            7 => PREDV0_A::DIV8,
            8 => PREDV0_A::DIV9,
            9 => PREDV0_A::DIV10,
            10 => PREDV0_A::DIV11,
            11 => PREDV0_A::DIV12,
            12 => PREDV0_A::DIV13,
            13 => PREDV0_A::DIV14,
            14 => PREDV0_A::DIV15,
            15 => PREDV0_A::DIV16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PREDV0_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PREDV0_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PREDV0_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PREDV0_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PREDV0_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PREDV0_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PREDV0_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PREDV0_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PREDV0_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PREDV0_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PREDV0_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PREDV0_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PREDV0_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PREDV0_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PREDV0_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PREDV0_A::DIV16
    }
}
#[doc = "Field `PREDV0` writer - PREDV0 division factor"]
pub type PREDV0_W<'a> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, PREDV0_A, 4, 0>;
impl<'a> PREDV0_W<'a> {
    #[doc = "Input to PLL not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV1)
    }
    #[doc = "Input to PLL divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV2)
    }
    #[doc = "Input to PLL divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV3)
    }
    #[doc = "Input to PLL divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV4)
    }
    #[doc = "Input to PLL divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV5)
    }
    #[doc = "Input to PLL divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV6)
    }
    #[doc = "Input to PLL divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV7)
    }
    #[doc = "Input to PLL divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV8)
    }
    #[doc = "Input to PLL divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV9)
    }
    #[doc = "Input to PLL divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV10)
    }
    #[doc = "Input to PLL divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV11)
    }
    #[doc = "Input to PLL divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV12)
    }
    #[doc = "Input to PLL divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV13)
    }
    #[doc = "Input to PLL divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV14)
    }
    #[doc = "Input to PLL divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV15)
    }
    #[doc = "Input to PLL divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PREDV0_A::DIV16)
    }
}
#[doc = "PREDV1 division factor"]
pub use PREDV0_A as PREDV1_A;
#[doc = "Field `PREDV1` reader - PREDV1 division factor"]
pub use PREDV0_R as PREDV1_R;
#[doc = "Field `PREDV1` writer - PREDV1 division factor"]
pub use PREDV0_W as PREDV1_W;
#[doc = "Field `PLL1MF` reader - The PLL1 clock multiplication factor"]
pub type PLL1MF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL1MF` writer - The PLL1 clock multiplication factor"]
pub type PLL1MF_W<'a> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 4, 8>;
#[doc = "Field `PLL2MF` reader - The PLL2 clock multiplication factor"]
pub type PLL2MF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL2MF` writer - The PLL2 clock multiplication factor"]
pub type PLL2MF_W<'a> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 4, 12>;
#[doc = "Field `PREDV0SEL` reader - PREDV0 input Clock Source Selection"]
pub type PREDV0SEL_R = crate::BitReader<bool>;
#[doc = "Field `PREDV0SEL` writer - PREDV0 input Clock Source Selection"]
pub type PREDV0SEL_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 16>;
#[doc = "Field `I2S1SEL` reader - I2S1 Clock Source Selection"]
pub type I2S1SEL_R = crate::BitReader<bool>;
#[doc = "Field `I2S1SEL` writer - I2S1 Clock Source Selection"]
pub type I2S1SEL_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 17>;
#[doc = "Field `I2S2SEL` reader - I2S2 Clock Source Selection"]
pub type I2S2SEL_R = crate::BitReader<bool>;
#[doc = "Field `I2S2SEL` writer - I2S2 Clock Source Selection"]
pub type I2S2SEL_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 18>;
impl R {
    #[doc = "Bits 0:3 - PREDV0 division factor"]
    #[inline(always)]
    pub fn predv0(&self) -> PREDV0_R {
        PREDV0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PREDV1 division factor"]
    #[inline(always)]
    pub fn predv1(&self) -> PREDV1_R {
        PREDV1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The PLL1 clock multiplication factor"]
    #[inline(always)]
    pub fn pll1mf(&self) -> PLL1MF_R {
        PLL1MF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - The PLL2 clock multiplication factor"]
    #[inline(always)]
    pub fn pll2mf(&self) -> PLL2MF_R {
        PLL2MF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - PREDV0 input Clock Source Selection"]
    #[inline(always)]
    pub fn predv0sel(&self) -> PREDV0SEL_R {
        PREDV0SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2S1 Clock Source Selection"]
    #[inline(always)]
    pub fn i2s1sel(&self) -> I2S1SEL_R {
        I2S1SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2S2 Clock Source Selection"]
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDV0 division factor"]
    #[inline(always)]
    pub fn predv0(&mut self) -> PREDV0_W {
        PREDV0_W::new(self)
    }
    #[doc = "Bits 4:7 - PREDV1 division factor"]
    #[inline(always)]
    pub fn predv1(&mut self) -> PREDV1_W {
        PREDV1_W::new(self)
    }
    #[doc = "Bits 8:11 - The PLL1 clock multiplication factor"]
    #[inline(always)]
    pub fn pll1mf(&mut self) -> PLL1MF_W {
        PLL1MF_W::new(self)
    }
    #[doc = "Bits 12:15 - The PLL2 clock multiplication factor"]
    #[inline(always)]
    pub fn pll2mf(&mut self) -> PLL2MF_W {
        PLL2MF_W::new(self)
    }
    #[doc = "Bit 16 - PREDV0 input Clock Source Selection"]
    #[inline(always)]
    pub fn predv0sel(&mut self) -> PREDV0SEL_W {
        PREDV0SEL_W::new(self)
    }
    #[doc = "Bit 17 - I2S1 Clock Source Selection"]
    #[inline(always)]
    pub fn i2s1sel(&mut self) -> I2S1SEL_W {
        I2S1SEL_W::new(self)
    }
    #[doc = "Bit 18 - I2S2 Clock Source Selection"]
    #[inline(always)]
    pub fn i2s2sel(&mut self) -> I2S2SEL_W {
        I2S2SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
