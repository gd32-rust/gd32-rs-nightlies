#[doc = "Register `SAMPT1` reader"]
pub struct R(crate::R<SAMPT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPT1` writer"]
pub struct W(crate::W<SAMPT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPT1_SPEC>;
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
impl From<crate::W<SAMPT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 sample time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPT0_A {
    #[doc = "0: 1.5 ADC clock cycles"]
    CYCLES1_5 = 0,
    #[doc = "1: 7.5 ADC clock cycles"]
    CYCLES7_5 = 1,
    #[doc = "2: 13.5 ADC clock cycles"]
    CYCLES13_5 = 2,
    #[doc = "3: 28.5 ADC clock cycles"]
    CYCLES28_5 = 3,
    #[doc = "4: 41.5 ADC clock cycles"]
    CYCLES41_5 = 4,
    #[doc = "5: 55.5 ADC clock cycles"]
    CYCLES55_5 = 5,
    #[doc = "6: 71.5 ADC clock cycles"]
    CYCLES71_5 = 6,
    #[doc = "7: 239.5 ADC clock cycles"]
    CYCLES239_5 = 7,
}
impl From<SPT0_A> for u8 {
    #[inline(always)]
    fn from(variant: SPT0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPT0` reader - Channel 0 sample time selection"]
pub type SPT0_R = crate::FieldReader<u8, SPT0_A>;
impl SPT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPT0_A {
        match self.bits {
            0 => SPT0_A::CYCLES1_5,
            1 => SPT0_A::CYCLES7_5,
            2 => SPT0_A::CYCLES13_5,
            3 => SPT0_A::CYCLES28_5,
            4 => SPT0_A::CYCLES41_5,
            5 => SPT0_A::CYCLES55_5,
            6 => SPT0_A::CYCLES71_5,
            7 => SPT0_A::CYCLES239_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES1_5`"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SPT0_A::CYCLES1_5
    }
    #[doc = "Checks if the value of the field is `CYCLES7_5`"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SPT0_A::CYCLES7_5
    }
    #[doc = "Checks if the value of the field is `CYCLES13_5`"]
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SPT0_A::CYCLES13_5
    }
    #[doc = "Checks if the value of the field is `CYCLES28_5`"]
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SPT0_A::CYCLES28_5
    }
    #[doc = "Checks if the value of the field is `CYCLES41_5`"]
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SPT0_A::CYCLES41_5
    }
    #[doc = "Checks if the value of the field is `CYCLES55_5`"]
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SPT0_A::CYCLES55_5
    }
    #[doc = "Checks if the value of the field is `CYCLES71_5`"]
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SPT0_A::CYCLES71_5
    }
    #[doc = "Checks if the value of the field is `CYCLES239_5`"]
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SPT0_A::CYCLES239_5
    }
}
#[doc = "Field `SPT0` writer - Channel 0 sample time selection"]
pub type SPT0_W<'a> = crate::FieldWriterSafe<'a, u32, SAMPT1_SPEC, u8, SPT0_A, 3, 0>;
impl<'a> SPT0_W<'a> {
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SPT0_A::CYCLES1_5)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SPT0_A::CYCLES7_5)
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut W {
        self.variant(SPT0_A::CYCLES13_5)
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut W {
        self.variant(SPT0_A::CYCLES28_5)
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut W {
        self.variant(SPT0_A::CYCLES41_5)
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut W {
        self.variant(SPT0_A::CYCLES55_5)
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut W {
        self.variant(SPT0_A::CYCLES71_5)
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut W {
        self.variant(SPT0_A::CYCLES239_5)
    }
}
#[doc = "Channel 1 sample time selection"]
pub use SPT0_A as SPT1_A;
#[doc = "Channel 2 sample time selection"]
pub use SPT0_A as SPT2_A;
#[doc = "Channel 3 sample time selection"]
pub use SPT0_A as SPT3_A;
#[doc = "Channel 4 sample time selection"]
pub use SPT0_A as SPT4_A;
#[doc = "Channel 5 sample time selection"]
pub use SPT0_A as SPT5_A;
#[doc = "Channel 6 sample time selection"]
pub use SPT0_A as SPT6_A;
#[doc = "Channel 7 sample time selection"]
pub use SPT0_A as SPT7_A;
#[doc = "Channel 8 sample time selection"]
pub use SPT0_A as SPT8_A;
#[doc = "Channel 9 sample time selection"]
pub use SPT0_A as SPT9_A;
#[doc = "Field `SPT1` reader - Channel 1 sample time selection"]
pub use SPT0_R as SPT1_R;
#[doc = "Field `SPT2` reader - Channel 2 sample time selection"]
pub use SPT0_R as SPT2_R;
#[doc = "Field `SPT3` reader - Channel 3 sample time selection"]
pub use SPT0_R as SPT3_R;
#[doc = "Field `SPT4` reader - Channel 4 sample time selection"]
pub use SPT0_R as SPT4_R;
#[doc = "Field `SPT5` reader - Channel 5 sample time selection"]
pub use SPT0_R as SPT5_R;
#[doc = "Field `SPT6` reader - Channel 6 sample time selection"]
pub use SPT0_R as SPT6_R;
#[doc = "Field `SPT7` reader - Channel 7 sample time selection"]
pub use SPT0_R as SPT7_R;
#[doc = "Field `SPT8` reader - Channel 8 sample time selection"]
pub use SPT0_R as SPT8_R;
#[doc = "Field `SPT9` reader - Channel 9 sample time selection"]
pub use SPT0_R as SPT9_R;
#[doc = "Field `SPT1` writer - Channel 1 sample time selection"]
pub use SPT0_W as SPT1_W;
#[doc = "Field `SPT2` writer - Channel 2 sample time selection"]
pub use SPT0_W as SPT2_W;
#[doc = "Field `SPT3` writer - Channel 3 sample time selection"]
pub use SPT0_W as SPT3_W;
#[doc = "Field `SPT4` writer - Channel 4 sample time selection"]
pub use SPT0_W as SPT4_W;
#[doc = "Field `SPT5` writer - Channel 5 sample time selection"]
pub use SPT0_W as SPT5_W;
#[doc = "Field `SPT6` writer - Channel 6 sample time selection"]
pub use SPT0_W as SPT6_W;
#[doc = "Field `SPT7` writer - Channel 7 sample time selection"]
pub use SPT0_W as SPT7_W;
#[doc = "Field `SPT8` writer - Channel 8 sample time selection"]
pub use SPT0_W as SPT8_W;
#[doc = "Field `SPT9` writer - Channel 9 sample time selection"]
pub use SPT0_W as SPT9_W;
impl R {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    pub fn spt0(&self) -> SPT0_R {
        SPT0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    pub fn spt1(&self) -> SPT1_R {
        SPT1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    pub fn spt2(&self) -> SPT2_R {
        SPT2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    pub fn spt3(&self) -> SPT3_R {
        SPT3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    pub fn spt4(&self) -> SPT4_R {
        SPT4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    pub fn spt5(&self) -> SPT5_R {
        SPT5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    pub fn spt6(&self) -> SPT6_R {
        SPT6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    pub fn spt7(&self) -> SPT7_R {
        SPT7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    pub fn spt8(&self) -> SPT8_R {
        SPT8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    pub fn spt9(&self) -> SPT9_R {
        SPT9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    pub fn spt0(&mut self) -> SPT0_W {
        SPT0_W::new(self)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    pub fn spt1(&mut self) -> SPT1_W {
        SPT1_W::new(self)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    pub fn spt2(&mut self) -> SPT2_W {
        SPT2_W::new(self)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    pub fn spt3(&mut self) -> SPT3_W {
        SPT3_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    pub fn spt4(&mut self) -> SPT4_W {
        SPT4_W::new(self)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    pub fn spt5(&mut self) -> SPT5_W {
        SPT5_W::new(self)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    pub fn spt6(&mut self) -> SPT6_W {
        SPT6_W::new(self)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    pub fn spt7(&mut self) -> SPT7_W {
        SPT7_W::new(self)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    pub fn spt8(&mut self) -> SPT8_W {
        SPT8_W::new(self)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    pub fn spt9(&mut self) -> SPT9_W {
        SPT9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sampling time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampt1](index.html) module"]
pub struct SAMPT1_SPEC;
impl crate::RegisterSpec for SAMPT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sampt1::R](R) reader structure"]
impl crate::Readable for SAMPT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sampt1::W](W) writer structure"]
impl crate::Writable for SAMPT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMPT1 to value 0"]
impl crate::Resettable for SAMPT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
