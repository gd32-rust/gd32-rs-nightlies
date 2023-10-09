#[doc = "Register `SAMPT1` reader"]
pub type R = crate::R<SAMPT1_SPEC>;
#[doc = "Register `SAMPT1` writer"]
pub type W = crate::W<SAMPT1_SPEC>;
#[doc = "Field `SPT0` reader - Channel 0 sample time selection"]
pub type SPT0_R = crate::FieldReader<SPT0_A>;
#[doc = "Channel 0 sample time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SPT0_A {
    type Ux = u8;
}
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
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SPT0_A::CYCLES1_5
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SPT0_A::CYCLES7_5
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SPT0_A::CYCLES13_5
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SPT0_A::CYCLES28_5
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SPT0_A::CYCLES41_5
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SPT0_A::CYCLES55_5
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SPT0_A::CYCLES71_5
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SPT0_A::CYCLES239_5
    }
}
#[doc = "Field `SPT0` writer - Channel 0 sample time selection"]
pub type SPT0_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, SPT0_A>;
impl<'a, REG, const O: u8> SPT0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT0_A::CYCLES1_5)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT0_A::CYCLES7_5)
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT0_A::CYCLES13_5)
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT0_A::CYCLES28_5)
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT0_A::CYCLES41_5)
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT0_A::CYCLES55_5)
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT0_A::CYCLES71_5)
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT0_A::CYCLES239_5)
    }
}
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
    #[must_use]
    pub fn spt0(&mut self) -> SPT0_W<SAMPT1_SPEC, 0> {
        SPT0_W::new(self)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt1(&mut self) -> SPT1_W<SAMPT1_SPEC, 3> {
        SPT1_W::new(self)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt2(&mut self) -> SPT2_W<SAMPT1_SPEC, 6> {
        SPT2_W::new(self)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt3(&mut self) -> SPT3_W<SAMPT1_SPEC, 9> {
        SPT3_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt4(&mut self) -> SPT4_W<SAMPT1_SPEC, 12> {
        SPT4_W::new(self)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt5(&mut self) -> SPT5_W<SAMPT1_SPEC, 15> {
        SPT5_W::new(self)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt6(&mut self) -> SPT6_W<SAMPT1_SPEC, 18> {
        SPT6_W::new(self)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt7(&mut self) -> SPT7_W<SAMPT1_SPEC, 21> {
        SPT7_W::new(self)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt8(&mut self) -> SPT8_W<SAMPT1_SPEC, 24> {
        SPT8_W::new(self)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt9(&mut self) -> SPT9_W<SAMPT1_SPEC, 27> {
        SPT9_W::new(self)
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
#[doc = "Sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMPT1_SPEC;
impl crate::RegisterSpec for SAMPT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sampt1::R`](R) reader structure"]
impl crate::Readable for SAMPT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sampt1::W`](W) writer structure"]
impl crate::Writable for SAMPT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPT1 to value 0"]
impl crate::Resettable for SAMPT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
