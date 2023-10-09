#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
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
#[doc = "Field `TI0S` reader - Channel 0 trigger input selection"]
pub type TI0S_R = crate::BitReader;
#[doc = "Field `TI0S` writer - Channel 0 trigger input selection"]
pub type TI0S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
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
    #[doc = "Bit 7 - Channel 0 trigger input selection"]
    #[inline(always)]
    pub fn ti0s(&self) -> TI0S_R {
        TI0S_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
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
    #[doc = "Bit 7 - Channel 0 trigger input selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti0s(&mut self) -> TI0S_W<CTL1_SPEC, 7> {
        TI0S_W::new(self)
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
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
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
