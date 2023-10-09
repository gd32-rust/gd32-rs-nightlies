#[doc = "Register `DMAINTEN` reader"]
pub type R = crate::R<DMAINTEN_SPEC>;
#[doc = "Register `DMAINTEN` writer"]
pub type W = crate::W<DMAINTEN_SPEC>;
#[doc = "Break interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::BRKIE_A;
#[doc = "Field `BRKIE` reader - Break interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::BRKIE_R;
#[doc = "Field `BRKIE` writer - Break interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::BRKIE_W;
#[doc = "Capture/Compare 0 interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::CH0IE_A;
#[doc = "Field `CH0IE` reader - Capture/Compare 0 interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::CH0IE_R;
#[doc = "Field `CH0IE` writer - Capture/Compare 0 interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::CH0IE_W;
#[doc = "COM interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::CMTIE_A;
#[doc = "Field `CMTIE` reader - COM interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::CMTIE_R;
#[doc = "Field `CMTIE` writer - COM interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::CMTIE_W;
#[doc = "Update interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::UPIE_A;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::UPIE_R;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::UPIE_W;
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UPDEN_R = crate::BitReader<UPDEN_A>;
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDEN_A {
    #[doc = "0: Update DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Update DMA request enabled"]
    ENABLED = 1,
}
impl From<UPDEN_A> for bool {
    #[inline(always)]
    fn from(variant: UPDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDEN_A {
        match self.bits {
            false => UPDEN_A::DISABLED,
            true => UPDEN_A::ENABLED,
        }
    }
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDEN_A::DISABLED
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDEN_A::ENABLED
    }
}
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UPDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UPDEN_A>;
impl<'a, REG, const O: u8> UPDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDEN_A::DISABLED)
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDEN_A::ENABLED)
    }
}
#[doc = "Field `CH0DEN` reader - Capture/Compare 0 DMA request enable"]
pub type CH0DEN_R = crate::BitReader<CH0DEN_A>;
#[doc = "Capture/Compare 0 DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0DEN_A {
    #[doc = "0: Capture/compare DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Capture/compare DMA request enabled"]
    ENABLED = 1,
}
impl From<CH0DEN_A> for bool {
    #[inline(always)]
    fn from(variant: CH0DEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0DEN_A {
        match self.bits {
            false => CH0DEN_A::DISABLED,
            true => CH0DEN_A::ENABLED,
        }
    }
    #[doc = "Capture/compare DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0DEN_A::DISABLED
    }
    #[doc = "Capture/compare DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0DEN_A::ENABLED
    }
}
#[doc = "Field `CH0DEN` writer - Capture/Compare 0 DMA request enable"]
pub type CH0DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0DEN_A>;
impl<'a, REG, const O: u8> CH0DEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CH0DEN_A::DISABLED)
    }
    #[doc = "Capture/compare DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CH0DEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 0 interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&self) -> CH0IE_R {
        CH0IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&self) -> CMTIE_R {
        CMTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn brkie(&self) -> BRKIE_R {
        BRKIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UPDEN_R {
        UPDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 0 DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&self) -> CH0DEN_R {
        CH0DEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<DMAINTEN_SPEC, 0> {
        UPIE_W::new(self)
    }
    #[doc = "Bit 1 - Capture/Compare 0 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ie(&mut self) -> CH0IE_W<DMAINTEN_SPEC, 1> {
        CH0IE_W::new(self)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtie(&mut self) -> CMTIE_W<DMAINTEN_SPEC, 5> {
        CMTIE_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn brkie(&mut self) -> BRKIE_W<DMAINTEN_SPEC, 7> {
        BRKIE_W::new(self)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn upden(&mut self) -> UPDEN_W<DMAINTEN_SPEC, 8> {
        UPDEN_W::new(self)
    }
    #[doc = "Bit 9 - Capture/Compare 0 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0den(&mut self) -> CH0DEN_W<DMAINTEN_SPEC, 9> {
        CH0DEN_W::new(self)
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
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmainten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmainten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAINTEN_SPEC;
impl crate::RegisterSpec for DMAINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmainten::R`](R) reader structure"]
impl crate::Readable for DMAINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmainten::W`](W) writer structure"]
impl crate::Writable for DMAINTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DMAINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
