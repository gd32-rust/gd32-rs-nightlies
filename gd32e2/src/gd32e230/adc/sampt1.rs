#[doc = "Register `SAMPT1` reader"]
pub type R = crate::R<Sampt1Spec>;
#[doc = "Register `SAMPT1` writer"]
pub type W = crate::W<Sampt1Spec>;
#[doc = "Channel 0 sample time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spt0 {
    #[doc = "0: 1.5 ADC clock cycles"]
    Cycles1_5 = 0,
    #[doc = "1: 7.5 ADC clock cycles"]
    Cycles7_5 = 1,
    #[doc = "2: 13.5 ADC clock cycles"]
    Cycles13_5 = 2,
    #[doc = "3: 28.5 ADC clock cycles"]
    Cycles28_5 = 3,
    #[doc = "4: 41.5 ADC clock cycles"]
    Cycles41_5 = 4,
    #[doc = "5: 55.5 ADC clock cycles"]
    Cycles55_5 = 5,
    #[doc = "6: 71.5 ADC clock cycles"]
    Cycles71_5 = 6,
    #[doc = "7: 239.5 ADC clock cycles"]
    Cycles239_5 = 7,
}
impl From<Spt0> for u8 {
    #[inline(always)]
    fn from(variant: Spt0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spt0 {
    type Ux = u8;
}
#[doc = "Field `SPT0` reader - Channel 0 sample time selection"]
pub type Spt0R = crate::FieldReader<Spt0>;
impl Spt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spt0 {
        match self.bits {
            0 => Spt0::Cycles1_5,
            1 => Spt0::Cycles7_5,
            2 => Spt0::Cycles13_5,
            3 => Spt0::Cycles28_5,
            4 => Spt0::Cycles41_5,
            5 => Spt0::Cycles55_5,
            6 => Spt0::Cycles71_5,
            7 => Spt0::Cycles239_5,
            _ => unreachable!(),
        }
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == Spt0::Cycles1_5
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == Spt0::Cycles7_5
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == Spt0::Cycles13_5
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == Spt0::Cycles28_5
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == Spt0::Cycles41_5
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == Spt0::Cycles55_5
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == Spt0::Cycles71_5
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == Spt0::Cycles239_5
    }
}
#[doc = "Field `SPT0` writer - Channel 0 sample time selection"]
pub type Spt0W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Spt0>;
impl<'a, REG> Spt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles1_5)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles7_5)
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles13_5)
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles28_5)
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles41_5)
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles55_5)
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles71_5)
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt0::Cycles239_5)
    }
}
#[doc = "Field `SPT1` reader - Channel 1 sample time selection"]
pub use Spt0R as Spt1R;
#[doc = "Field `SPT2` reader - Channel 2 sample time selection"]
pub use Spt0R as Spt2R;
#[doc = "Field `SPT3` reader - Channel 3 sample time selection"]
pub use Spt0R as Spt3R;
#[doc = "Field `SPT4` reader - Channel 4 sample time selection"]
pub use Spt0R as Spt4R;
#[doc = "Field `SPT5` reader - Channel 5 sample time selection"]
pub use Spt0R as Spt5R;
#[doc = "Field `SPT6` reader - Channel 6 sample time selection"]
pub use Spt0R as Spt6R;
#[doc = "Field `SPT7` reader - Channel 7 sample time selection"]
pub use Spt0R as Spt7R;
#[doc = "Field `SPT8` reader - Channel 8 sample time selection"]
pub use Spt0R as Spt8R;
#[doc = "Field `SPT9` reader - Channel 9 sample time selection"]
pub use Spt0R as Spt9R;
#[doc = "Field `SPT1` writer - Channel 1 sample time selection"]
pub use Spt0W as Spt1W;
#[doc = "Field `SPT2` writer - Channel 2 sample time selection"]
pub use Spt0W as Spt2W;
#[doc = "Field `SPT3` writer - Channel 3 sample time selection"]
pub use Spt0W as Spt3W;
#[doc = "Field `SPT4` writer - Channel 4 sample time selection"]
pub use Spt0W as Spt4W;
#[doc = "Field `SPT5` writer - Channel 5 sample time selection"]
pub use Spt0W as Spt5W;
#[doc = "Field `SPT6` writer - Channel 6 sample time selection"]
pub use Spt0W as Spt6W;
#[doc = "Field `SPT7` writer - Channel 7 sample time selection"]
pub use Spt0W as Spt7W;
#[doc = "Field `SPT8` writer - Channel 8 sample time selection"]
pub use Spt0W as Spt8W;
#[doc = "Field `SPT9` writer - Channel 9 sample time selection"]
pub use Spt0W as Spt9W;
impl R {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    pub fn spt0(&self) -> Spt0R {
        Spt0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    pub fn spt1(&self) -> Spt1R {
        Spt1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    pub fn spt2(&self) -> Spt2R {
        Spt2R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    pub fn spt3(&self) -> Spt3R {
        Spt3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    pub fn spt4(&self) -> Spt4R {
        Spt4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    pub fn spt5(&self) -> Spt5R {
        Spt5R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    pub fn spt6(&self) -> Spt6R {
        Spt6R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    pub fn spt7(&self) -> Spt7R {
        Spt7R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    pub fn spt8(&self) -> Spt8R {
        Spt8R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    pub fn spt9(&self) -> Spt9R {
        Spt9R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt0(&mut self) -> Spt0W<Sampt1Spec> {
        Spt0W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt1(&mut self) -> Spt1W<Sampt1Spec> {
        Spt1W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt2(&mut self) -> Spt2W<Sampt1Spec> {
        Spt2W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt3(&mut self) -> Spt3W<Sampt1Spec> {
        Spt3W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt4(&mut self) -> Spt4W<Sampt1Spec> {
        Spt4W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt5(&mut self) -> Spt5W<Sampt1Spec> {
        Spt5W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt6(&mut self) -> Spt6W<Sampt1Spec> {
        Spt6W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt7(&mut self) -> Spt7W<Sampt1Spec> {
        Spt7W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt8(&mut self) -> Spt8W<Sampt1Spec> {
        Spt8W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt9(&mut self) -> Spt9W<Sampt1Spec> {
        Spt9W::new(self, 27)
    }
}
#[doc = "Sampling time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sampt1Spec;
impl crate::RegisterSpec for Sampt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sampt1::R`](R) reader structure"]
impl crate::Readable for Sampt1Spec {}
#[doc = "`write(|w| ..)` method takes [`sampt1::W`](W) writer structure"]
impl crate::Writable for Sampt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAMPT1 to value 0"]
impl crate::Resettable for Sampt1Spec {
    const RESET_VALUE: u32 = 0;
}
