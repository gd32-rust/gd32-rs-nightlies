#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Commutation control shadow register enable"]
pub use crate::gd32f150::timer0::ctl1::CCSE_A;
#[doc = "Field `CCSE` reader - Commutation control shadow register enable"]
pub use crate::gd32f150::timer0::ctl1::CCSE_R;
#[doc = "Field `CCSE` writer - Commutation control shadow register enable"]
pub use crate::gd32f150::timer0::ctl1::CCSE_W;
#[doc = "Commutation control shadow register update control"]
pub use crate::gd32f150::timer0::ctl1::CCUC_A;
#[doc = "Field `CCUC` reader - Commutation control shadow register update control"]
pub use crate::gd32f150::timer0::ctl1::CCUC_R;
#[doc = "Field `CCUC` writer - Commutation control shadow register update control"]
pub use crate::gd32f150::timer0::ctl1::CCUC_W;
#[doc = "Field `DMAS` reader - DMA request source selection"]
pub type DMAS_R = crate::BitReader<DMAS_A>;
#[doc = "DMA request source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAS_A {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    ON_COMPARE = 0,
    #[doc = "1: CCx DMA request sent when update event occurs"]
    ON_UPDATE = 1,
}
impl From<DMAS_A> for bool {
    #[inline(always)]
    fn from(variant: DMAS_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAS_A {
        match self.bits {
            false => DMAS_A::ON_COMPARE,
            true => DMAS_A::ON_UPDATE,
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == DMAS_A::ON_COMPARE
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == DMAS_A::ON_UPDATE
    }
}
#[doc = "Field `DMAS` writer - DMA request source selection"]
pub type DMAS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMAS_A>;
impl<'a, REG, const O: u8> DMAS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut crate::W<REG> {
        self.variant(DMAS_A::ON_COMPARE)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn on_update(self) -> &'a mut crate::W<REG> {
        self.variant(DMAS_A::ON_UPDATE)
    }
}
#[doc = "Field `MMC` reader - Master mode control"]
pub type MMC_R = crate::FieldReader<MMC_A>;
#[doc = "Master mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMC_A {
    #[doc = "0: Use UPG bit from SWEVG register"]
    RESET = 0,
    #[doc = "1: Use CEN bit from CTL0 register"]
    ENABLE = 1,
    #[doc = "2: Use the update event"]
    UPDATE = 2,
}
impl From<MMC_A> for u8 {
    #[inline(always)]
    fn from(variant: MMC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMC_A {
    type Ux = u8;
}
impl MMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MMC_A> {
        match self.bits {
            0 => Some(MMC_A::RESET),
            1 => Some(MMC_A::ENABLE),
            2 => Some(MMC_A::UPDATE),
            _ => None,
        }
    }
    #[doc = "Use UPG bit from SWEVG register"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMC_A::RESET
    }
    #[doc = "Use CEN bit from CTL0 register"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMC_A::ENABLE
    }
    #[doc = "Use the update event"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMC_A::UPDATE
    }
}
#[doc = "Field `MMC` writer - Master mode control"]
pub type MMC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, MMC_A>;
impl<'a, REG, const O: u8> MMC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use UPG bit from SWEVG register"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(MMC_A::RESET)
    }
    #[doc = "Use CEN bit from CTL0 register"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MMC_A::ENABLE)
    }
    #[doc = "Use the update event"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(MMC_A::UPDATE)
    }
}
#[doc = "Idle state of channel 1 output"]
pub use crate::gd32f150::timer0::ctl1::ISO0N_A;
#[doc = "Field `ISO0N` reader - Idle state of channel 1 output"]
pub use crate::gd32f150::timer0::ctl1::ISO0N_R;
#[doc = "Field `ISO0N` writer - Idle state of channel 1 output"]
pub use crate::gd32f150::timer0::ctl1::ISO0N_W;
#[doc = "Idle state of channel 0 output"]
pub use crate::gd32f150::timer0::ctl1::ISO0_A;
#[doc = "Field `ISO0` reader - Idle state of channel 0 output"]
pub use crate::gd32f150::timer0::ctl1::ISO0_R;
#[doc = "Field `ISO1` reader - Idle state of channel 1 output"]
pub use crate::gd32f150::timer0::ctl1::ISO0_R as ISO1_R;
#[doc = "Field `ISO0` writer - Idle state of channel 0 output"]
pub use crate::gd32f150::timer0::ctl1::ISO0_W;
#[doc = "Field `ISO1` writer - Idle state of channel 1 output"]
pub use crate::gd32f150::timer0::ctl1::ISO0_W as ISO1_W;
impl R {
    #[doc = "Bit 0 - Commutation control shadow register enable"]
    #[inline(always)]
    pub fn ccse(&self) -> CCSE_R {
        CCSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    pub fn ccuc(&self) -> CCUC_R {
        CCUC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    pub fn iso0(&self) -> ISO0_R {
        ISO0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso0n(&self) -> ISO0N_R {
        ISO0N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso1(&self) -> ISO1_R {
        ISO1_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Commutation control shadow register enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccse(&mut self) -> CCSE_W<CTL1_SPEC, 0> {
        CCSE_W::new(self)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    #[must_use]
    pub fn ccuc(&mut self) -> CCUC_W<CTL1_SPEC, 2> {
        CCUC_W::new(self)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    #[must_use]
    pub fn dmas(&mut self) -> DMAS_W<CTL1_SPEC, 3> {
        DMAS_W::new(self)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    #[must_use]
    pub fn mmc(&mut self) -> MMC_W<CTL1_SPEC, 4> {
        MMC_W::new(self)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso0(&mut self) -> ISO0_W<CTL1_SPEC, 8> {
        ISO0_W::new(self)
    }
    #[doc = "Bit 9 - Idle state of channel 1 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso0n(&mut self) -> ISO0N_W<CTL1_SPEC, 9> {
        ISO0N_W::new(self)
    }
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    #[must_use]
    pub fn iso1(&mut self) -> ISO1_W<CTL1_SPEC, 10> {
        ISO1_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
