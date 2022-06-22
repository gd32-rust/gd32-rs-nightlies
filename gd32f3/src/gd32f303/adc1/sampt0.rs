#[doc = "Register `SAMPT0` reader"]
pub struct R(crate::R<SAMPT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPT0` writer"]
pub struct W(crate::W<SAMPT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPT0_SPEC>;
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
impl From<crate::W<SAMPT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 10 sample time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPT10_A {
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
impl From<SPT10_A> for u8 {
    #[inline(always)]
    fn from(variant: SPT10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPT10` reader - Channel 10 sample time selection"]
pub type SPT10_R = crate::FieldReader<u8, SPT10_A>;
impl SPT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPT10_A {
        match self.bits {
            0 => SPT10_A::CYCLES1_5,
            1 => SPT10_A::CYCLES7_5,
            2 => SPT10_A::CYCLES13_5,
            3 => SPT10_A::CYCLES28_5,
            4 => SPT10_A::CYCLES41_5,
            5 => SPT10_A::CYCLES55_5,
            6 => SPT10_A::CYCLES71_5,
            7 => SPT10_A::CYCLES239_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES1_5`"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SPT10_A::CYCLES1_5
    }
    #[doc = "Checks if the value of the field is `CYCLES7_5`"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SPT10_A::CYCLES7_5
    }
    #[doc = "Checks if the value of the field is `CYCLES13_5`"]
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SPT10_A::CYCLES13_5
    }
    #[doc = "Checks if the value of the field is `CYCLES28_5`"]
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SPT10_A::CYCLES28_5
    }
    #[doc = "Checks if the value of the field is `CYCLES41_5`"]
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SPT10_A::CYCLES41_5
    }
    #[doc = "Checks if the value of the field is `CYCLES55_5`"]
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SPT10_A::CYCLES55_5
    }
    #[doc = "Checks if the value of the field is `CYCLES71_5`"]
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SPT10_A::CYCLES71_5
    }
    #[doc = "Checks if the value of the field is `CYCLES239_5`"]
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SPT10_A::CYCLES239_5
    }
}
#[doc = "Field `SPT10` writer - Channel 10 sample time selection"]
pub type SPT10_W<'a> = crate::FieldWriterSafe<'a, u32, SAMPT0_SPEC, u8, SPT10_A, 3, 0>;
impl<'a> SPT10_W<'a> {
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SPT10_A::CYCLES1_5)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SPT10_A::CYCLES7_5)
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut W {
        self.variant(SPT10_A::CYCLES13_5)
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut W {
        self.variant(SPT10_A::CYCLES28_5)
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut W {
        self.variant(SPT10_A::CYCLES41_5)
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut W {
        self.variant(SPT10_A::CYCLES55_5)
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut W {
        self.variant(SPT10_A::CYCLES71_5)
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut W {
        self.variant(SPT10_A::CYCLES239_5)
    }
}
#[doc = "Channel 11 sample time selection"]
pub use SPT10_A as SPT11_A;
#[doc = "Channel 12 sample time selection"]
pub use SPT10_A as SPT12_A;
#[doc = "Channel 13 sample time selection"]
pub use SPT10_A as SPT13_A;
#[doc = "Channel 14 sample time selection"]
pub use SPT10_A as SPT14_A;
#[doc = "Channel 15 sample time selection"]
pub use SPT10_A as SPT15_A;
#[doc = "Channel 16 sample time selection"]
pub use SPT10_A as SPT16_A;
#[doc = "Channel 17 sample time selection"]
pub use SPT10_A as SPT17_A;
#[doc = "Field `SPT11` reader - Channel 11 sample time selection"]
pub use SPT10_R as SPT11_R;
#[doc = "Field `SPT12` reader - Channel 12 sample time selection"]
pub use SPT10_R as SPT12_R;
#[doc = "Field `SPT13` reader - Channel 13 sample time selection"]
pub use SPT10_R as SPT13_R;
#[doc = "Field `SPT14` reader - Channel 14 sample time selection"]
pub use SPT10_R as SPT14_R;
#[doc = "Field `SPT15` reader - Channel 15 sample time selection"]
pub use SPT10_R as SPT15_R;
#[doc = "Field `SPT16` reader - Channel 16 sample time selection"]
pub use SPT10_R as SPT16_R;
#[doc = "Field `SPT17` reader - Channel 17 sample time selection"]
pub use SPT10_R as SPT17_R;
#[doc = "Field `SPT11` writer - Channel 11 sample time selection"]
pub use SPT10_W as SPT11_W;
#[doc = "Field `SPT12` writer - Channel 12 sample time selection"]
pub use SPT10_W as SPT12_W;
#[doc = "Field `SPT13` writer - Channel 13 sample time selection"]
pub use SPT10_W as SPT13_W;
#[doc = "Field `SPT14` writer - Channel 14 sample time selection"]
pub use SPT10_W as SPT14_W;
#[doc = "Field `SPT15` writer - Channel 15 sample time selection"]
pub use SPT10_W as SPT15_W;
#[doc = "Field `SPT16` writer - Channel 16 sample time selection"]
pub use SPT10_W as SPT16_W;
#[doc = "Field `SPT17` writer - Channel 17 sample time selection"]
pub use SPT10_W as SPT17_W;
impl R {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    pub fn spt10(&self) -> SPT10_R {
        SPT10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    pub fn spt11(&self) -> SPT11_R {
        SPT11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    pub fn spt12(&self) -> SPT12_R {
        SPT12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    pub fn spt13(&self) -> SPT13_R {
        SPT13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    pub fn spt14(&self) -> SPT14_R {
        SPT14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    pub fn spt15(&self) -> SPT15_R {
        SPT15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn spt16(&self) -> SPT16_R {
        SPT16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn spt17(&self) -> SPT17_R {
        SPT17_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    pub fn spt10(&mut self) -> SPT10_W {
        SPT10_W::new(self)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    pub fn spt11(&mut self) -> SPT11_W {
        SPT11_W::new(self)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    pub fn spt12(&mut self) -> SPT12_W {
        SPT12_W::new(self)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    pub fn spt13(&mut self) -> SPT13_W {
        SPT13_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    pub fn spt14(&mut self) -> SPT14_W {
        SPT14_W::new(self)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    pub fn spt15(&mut self) -> SPT15_W {
        SPT15_W::new(self)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn spt16(&mut self) -> SPT16_W {
        SPT16_W::new(self)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn spt17(&mut self) -> SPT17_W {
        SPT17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample time register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampt0](index.html) module"]
pub struct SAMPT0_SPEC;
impl crate::RegisterSpec for SAMPT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sampt0::R](R) reader structure"]
impl crate::Readable for SAMPT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sampt0::W](W) writer structure"]
impl crate::Writable for SAMPT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMPT0 to value 0"]
impl crate::Resettable for SAMPT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
