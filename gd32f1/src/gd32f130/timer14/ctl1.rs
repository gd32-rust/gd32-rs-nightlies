#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Idle state of channel 1 output"]
pub use crate::gd32f130::timer0::ctl1::ISO0_A as ISO1_A;
#[doc = "Field `ISO1` reader - Idle state of channel 1 output"]
pub use crate::gd32f130::timer0::ctl1::ISO0_R as ISO1_R;
#[doc = "Field `ISO1` writer - Idle state of channel 1 output"]
pub type ISO1_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, ISO1_A, 10>;
impl<'a> ISO1_W<'a> {
    #[doc = "CHn_O=0 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ISO1_A::LOW)
    }
    #[doc = "CHn_O=1 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ISO1_A::HIGH)
    }
}
#[doc = "Idle state of channel 1 output"]
pub use crate::gd32f130::timer0::ctl1::ISO0N_A;
#[doc = "Field `ISO0N` reader - Idle state of channel 1 output"]
pub use crate::gd32f130::timer0::ctl1::ISO0N_R;
#[doc = "Field `ISO0N` writer - Idle state of channel 1 output"]
pub type ISO0N_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, ISO0N_A, 9>;
impl<'a> ISO0N_W<'a> {
    #[doc = "CHn_ON=0 when POEN=0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ISO0N_A::LOW)
    }
    #[doc = "CHn_ON=1 when POEN=0"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ISO0N_A::HIGH)
    }
}
#[doc = "Idle state of channel 0 output"]
pub use crate::gd32f130::timer0::ctl1::ISO0_A;
#[doc = "Field `ISO0` reader - Idle state of channel 0 output"]
pub use crate::gd32f130::timer0::ctl1::ISO0_R;
#[doc = "Field `ISO0` writer - Idle state of channel 0 output"]
pub type ISO0_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, ISO0_A, 8>;
impl<'a> ISO0_W<'a> {
    #[doc = "CHn_O=0 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ISO0_A::LOW)
    }
    #[doc = "CHn_O=1 (after a dead-time if CHn_ON is implemented) when POEN=0"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ISO0_A::HIGH)
    }
}
#[doc = "Master mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `MMC` reader - Master mode control"]
pub type MMC_R = crate::FieldReader<u8, MMC_A>;
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
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMC_A::RESET
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMC_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMC_A::UPDATE
    }
}
#[doc = "Field `MMC` writer - Master mode control"]
pub type MMC_W<'a> = crate::FieldWriter<'a, u16, CTL1_SPEC, u8, MMC_A, 3, 4>;
impl<'a> MMC_W<'a> {
    #[doc = "Use UPG bit from SWEVG register"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMC_A::RESET)
    }
    #[doc = "Use CEN bit from CTL0 register"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMC_A::ENABLE)
    }
    #[doc = "Use the update event"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMC_A::UPDATE)
    }
}
#[doc = "DMA request source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAS_A {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    ONCOMPARE = 0,
    #[doc = "1: CCx DMA request sent when update event occurs"]
    ONUPDATE = 1,
}
impl From<DMAS_A> for bool {
    #[inline(always)]
    fn from(variant: DMAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAS` reader - DMA request source selection"]
pub type DMAS_R = crate::BitReader<DMAS_A>;
impl DMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAS_A {
        match self.bits {
            false => DMAS_A::ONCOMPARE,
            true => DMAS_A::ONUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `ONCOMPARE`"]
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == DMAS_A::ONCOMPARE
    }
    #[doc = "Checks if the value of the field is `ONUPDATE`"]
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == DMAS_A::ONUPDATE
    }
}
#[doc = "Field `DMAS` writer - DMA request source selection"]
pub type DMAS_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, DMAS_A, 3>;
impl<'a> DMAS_W<'a> {
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut W {
        self.variant(DMAS_A::ONCOMPARE)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn on_update(self) -> &'a mut W {
        self.variant(DMAS_A::ONUPDATE)
    }
}
#[doc = "Commutation control shadow register update control"]
pub use crate::gd32f130::timer0::ctl1::CCUC_A;
#[doc = "Field `CCUC` reader - Commutation control shadow register update control"]
pub use crate::gd32f130::timer0::ctl1::CCUC_R;
#[doc = "Field `CCUC` writer - Commutation control shadow register update control"]
pub type CCUC_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, CCUC_A, 2>;
impl<'a> CCUC_W<'a> {
    #[doc = "Capture/compare are updated only by setting the CMTG bit"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(CCUC_A::DEFAULT)
    }
    #[doc = "Capture/compare are updated by setting the CMTG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn with_rising_edge(self) -> &'a mut W {
        self.variant(CCUC_A::WITHRISINGEDGE)
    }
}
#[doc = "Commutation control shadow register enable"]
pub use crate::gd32f130::timer0::ctl1::CCSE_A;
#[doc = "Field `CCSE` reader - Commutation control shadow register enable"]
pub use crate::gd32f130::timer0::ctl1::CCSE_R;
#[doc = "Field `CCSE` writer - Commutation control shadow register enable"]
pub type CCSE_W<'a> = crate::BitWriter<'a, u16, CTL1_SPEC, CCSE_A, 0>;
impl<'a> CCSE_W<'a> {
    #[doc = "The shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits are disabled"]
    #[inline(always)]
    pub fn not_preloaded(self) -> &'a mut W {
        self.variant(CCSE_A::NOTPRELOADED)
    }
    #[doc = "The shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits are enabled"]
    #[inline(always)]
    pub fn preloaded(self) -> &'a mut W {
        self.variant(CCSE_A::PRELOADED)
    }
}
impl R {
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso1(&self) -> ISO1_R {
        ISO1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso0n(&self) -> ISO0N_R {
        ISO0N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    pub fn iso0(&self) -> ISO0_R {
        ISO0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    pub fn ccuc(&self) -> CCUC_R {
        CCUC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - Commutation control shadow register enable"]
    #[inline(always)]
    pub fn ccse(&self) -> CCSE_R {
        CCSE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso1(&mut self) -> ISO1_W {
        ISO1_W::new(self)
    }
    #[doc = "Bit 9 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso0n(&mut self) -> ISO0N_W {
        ISO0N_W::new(self)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    pub fn iso0(&mut self) -> ISO0_W {
        ISO0_W::new(self)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    pub fn mmc(&mut self) -> MMC_W {
        MMC_W::new(self)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&mut self) -> DMAS_W {
        DMAS_W::new(self)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    pub fn ccuc(&mut self) -> CCUC_W {
        CCUC_W::new(self)
    }
    #[doc = "Bit 0 - Commutation control shadow register enable"]
    #[inline(always)]
    pub fn ccse(&mut self) -> CCSE_W {
        CCSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
