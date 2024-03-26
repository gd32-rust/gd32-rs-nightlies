#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Commutation control shadow register enable"]
pub use crate::gd32f130::timer0::ctl1::Ccse;
#[doc = "Field `CCSE` reader - Commutation control shadow register enable"]
pub use crate::gd32f130::timer0::ctl1::CcseR;
#[doc = "Field `CCSE` writer - Commutation control shadow register enable"]
pub use crate::gd32f130::timer0::ctl1::CcseW;
#[doc = "Commutation control shadow register update control"]
pub use crate::gd32f130::timer0::ctl1::Ccuc;
#[doc = "Field `CCUC` reader - Commutation control shadow register update control"]
pub use crate::gd32f130::timer0::ctl1::CcucR;
#[doc = "Field `CCUC` writer - Commutation control shadow register update control"]
pub use crate::gd32f130::timer0::ctl1::CcucW;
#[doc = "DMA request source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmas {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    OnCompare = 0,
    #[doc = "1: CCx DMA request sent when update event occurs"]
    OnUpdate = 1,
}
impl From<Dmas> for bool {
    #[inline(always)]
    fn from(variant: Dmas) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAS` reader - DMA request source selection"]
pub type DmasR = crate::BitReader<Dmas>;
impl DmasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmas {
        match self.bits {
            false => Dmas::OnCompare,
            true => Dmas::OnUpdate,
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == Dmas::OnCompare
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == Dmas::OnUpdate
    }
}
#[doc = "Field `DMAS` writer - DMA request source selection"]
pub type DmasW<'a, REG> = crate::BitWriter<'a, REG, Dmas>;
impl<'a, REG> DmasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut crate::W<REG> {
        self.variant(Dmas::OnCompare)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn on_update(self) -> &'a mut crate::W<REG> {
        self.variant(Dmas::OnUpdate)
    }
}
#[doc = "Master mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mmc {
    #[doc = "0: Use UPG bit from SWEVG register"]
    Reset = 0,
    #[doc = "1: Use CEN bit from CTL0 register"]
    Enable = 1,
    #[doc = "2: Use the update event"]
    Update = 2,
}
impl From<Mmc> for u8 {
    #[inline(always)]
    fn from(variant: Mmc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mmc {
    type Ux = u8;
}
#[doc = "Field `MMC` reader - Master mode control"]
pub type MmcR = crate::FieldReader<Mmc>;
impl MmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mmc> {
        match self.bits {
            0 => Some(Mmc::Reset),
            1 => Some(Mmc::Enable),
            2 => Some(Mmc::Update),
            _ => None,
        }
    }
    #[doc = "Use UPG bit from SWEVG register"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Mmc::Reset
    }
    #[doc = "Use CEN bit from CTL0 register"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mmc::Enable
    }
    #[doc = "Use the update event"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == Mmc::Update
    }
}
#[doc = "Field `MMC` writer - Master mode control"]
pub type MmcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mmc>;
impl<'a, REG> MmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use UPG bit from SWEVG register"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Reset)
    }
    #[doc = "Use CEN bit from CTL0 register"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Enable)
    }
    #[doc = "Use the update event"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Update)
    }
}
#[doc = "Idle state of channel 0 output"]
pub use crate::gd32f130::timer0::ctl1::Iso0;
#[doc = "Field `ISO0` reader - Idle state of channel 0 output"]
pub use crate::gd32f130::timer0::ctl1::Iso0R;
#[doc = "Field `ISO1` reader - Idle state of channel 1 output"]
pub use crate::gd32f130::timer0::ctl1::Iso0R as Iso1R;
#[doc = "Field `ISO0` writer - Idle state of channel 0 output"]
pub use crate::gd32f130::timer0::ctl1::Iso0W;
#[doc = "Field `ISO1` writer - Idle state of channel 1 output"]
pub use crate::gd32f130::timer0::ctl1::Iso0W as Iso1W;
#[doc = "Idle state of channel 1 output"]
pub use crate::gd32f130::timer0::ctl1::Iso0n;
#[doc = "Field `ISO0N` reader - Idle state of channel 1 output"]
pub use crate::gd32f130::timer0::ctl1::Iso0nR;
#[doc = "Field `ISO0N` writer - Idle state of channel 1 output"]
pub use crate::gd32f130::timer0::ctl1::Iso0nW;
impl R {
    #[doc = "Bit 0 - Commutation control shadow register enable"]
    #[inline(always)]
    pub fn ccse(&self) -> CcseR {
        CcseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    pub fn ccuc(&self) -> CcucR {
        CcucR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&self) -> DmasR {
        DmasR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    pub fn mmc(&self) -> MmcR {
        MmcR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    pub fn iso0(&self) -> Iso0R {
        Iso0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso0n(&self) -> Iso0nR {
        Iso0nR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso1(&self) -> Iso1R {
        Iso1R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Commutation control shadow register enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccse(&mut self) -> CcseW<Ctl1Spec> {
        CcseW::new(self, 0)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    #[must_use]
    pub fn ccuc(&mut self) -> CcucW<Ctl1Spec> {
        CcucW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    #[must_use]
    pub fn dmas(&mut self) -> DmasW<Ctl1Spec> {
        DmasW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    #[must_use]
    pub fn mmc(&mut self) -> MmcW<Ctl1Spec> {
        MmcW::new(self, 4)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso0(&mut self) -> Iso0W<Ctl1Spec> {
        Iso0W::new(self, 8)
    }
    #[doc = "Bit 9 - Idle state of channel 1 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso0n(&mut self) -> Iso0nW<Ctl1Spec> {
        Iso0nW::new(self, 9)
    }
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso1(&mut self) -> Iso1W<Ctl1Spec> {
        Iso1W::new(self, 10)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u16 = 0;
}
