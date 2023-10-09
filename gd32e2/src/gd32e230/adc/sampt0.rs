#[doc = "Register `SAMPT0` reader"]
pub type R = crate::R<SAMPT0_SPEC>;
#[doc = "Register `SAMPT0` writer"]
pub type W = crate::W<SAMPT0_SPEC>;
#[doc = "Field `SPT16` reader - Channel 16 sample time selection"]
pub type SPT16_R = crate::FieldReader<SPT16_A>;
#[doc = "Channel 16 sample time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPT16_A {
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
impl From<SPT16_A> for u8 {
    #[inline(always)]
    fn from(variant: SPT16_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPT16_A {
    type Ux = u8;
}
impl SPT16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPT16_A {
        match self.bits {
            0 => SPT16_A::CYCLES1_5,
            1 => SPT16_A::CYCLES7_5,
            2 => SPT16_A::CYCLES13_5,
            3 => SPT16_A::CYCLES28_5,
            4 => SPT16_A::CYCLES41_5,
            5 => SPT16_A::CYCLES55_5,
            6 => SPT16_A::CYCLES71_5,
            7 => SPT16_A::CYCLES239_5,
            _ => unreachable!(),
        }
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SPT16_A::CYCLES1_5
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SPT16_A::CYCLES7_5
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SPT16_A::CYCLES13_5
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SPT16_A::CYCLES28_5
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SPT16_A::CYCLES41_5
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SPT16_A::CYCLES55_5
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SPT16_A::CYCLES71_5
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SPT16_A::CYCLES239_5
    }
}
#[doc = "Field `SPT16` writer - Channel 16 sample time selection"]
pub type SPT16_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, SPT16_A>;
impl<'a, REG, const O: u8> SPT16_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT16_A::CYCLES1_5)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT16_A::CYCLES7_5)
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT16_A::CYCLES13_5)
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT16_A::CYCLES28_5)
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT16_A::CYCLES41_5)
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT16_A::CYCLES55_5)
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT16_A::CYCLES71_5)
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut crate::W<REG> {
        self.variant(SPT16_A::CYCLES239_5)
    }
}
#[doc = "Field `SPT17` reader - Channel 17 sample time selection"]
pub use SPT16_R as SPT17_R;
#[doc = "Field `SPT17` writer - Channel 17 sample time selection"]
pub use SPT16_W as SPT17_W;
impl R {
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
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt16(&mut self) -> SPT16_W<SAMPT0_SPEC, 18> {
        SPT16_W::new(self)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt17(&mut self) -> SPT17_W<SAMPT0_SPEC, 21> {
        SPT17_W::new(self)
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
#[doc = "Sampling time register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMPT0_SPEC;
impl crate::RegisterSpec for SAMPT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sampt0::R`](R) reader structure"]
impl crate::Readable for SAMPT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sampt0::W`](W) writer structure"]
impl crate::Writable for SAMPT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPT0 to value 0"]
impl crate::Resettable for SAMPT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
