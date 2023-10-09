#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Counter enable"]
pub use crate::gd32e508::timer0::ctl0::CEN_A;
#[doc = "Field `CEN` reader - Counter enable"]
pub use crate::gd32e508::timer0::ctl0::CEN_R;
#[doc = "Field `CEN` writer - Counter enable"]
pub use crate::gd32e508::timer0::ctl0::CEN_W;
#[doc = "Update disable"]
pub use crate::gd32e508::timer0::ctl0::UPDIS_A;
#[doc = "Field `UPDIS` reader - Update disable"]
pub use crate::gd32e508::timer0::ctl0::UPDIS_R;
#[doc = "Field `UPDIS` writer - Update disable"]
pub use crate::gd32e508::timer0::ctl0::UPDIS_W;
#[doc = "Update source"]
pub use crate::gd32e508::timer0::ctl0::UPS_A;
#[doc = "Field `UPS` reader - Update source"]
pub use crate::gd32e508::timer0::ctl0::UPS_R;
#[doc = "Field `UPS` writer - Update source"]
pub use crate::gd32e508::timer0::ctl0::UPS_W;
#[doc = "Field `SPM` reader - Single pulse mode"]
pub type SPM_R = crate::BitReader<SPM_A>;
#[doc = "Single pulse mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPM_A {
    #[doc = "0: Counter is not stopped at update event"]
    DISABLED = 0,
    #[doc = "1: Counter stops counting at the next update event (clearing the CEN bit)"]
    ENABLED = 1,
}
impl From<SPM_A> for bool {
    #[inline(always)]
    fn from(variant: SPM_A) -> Self {
        variant as u8 != 0
    }
}
impl SPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPM_A {
        match self.bits {
            false => SPM_A::DISABLED,
            true => SPM_A::ENABLED,
        }
    }
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPM_A::DISABLED
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPM_A::ENABLED
    }
}
#[doc = "Field `SPM` writer - Single pulse mode"]
pub type SPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPM_A>;
impl<'a, REG, const O: u8> SPM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPM_A::DISABLED)
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPM_A::ENABLED)
    }
}
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAM` reader - Counter aligns mode selection"]
pub type CAM_R = crate::FieldReader;
#[doc = "Field `CAM` writer - Counter aligns mode selection"]
pub type CAM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Auto-reload shadow enable"]
pub use crate::gd32e508::timer0::ctl0::ARSE_A;
#[doc = "Field `ARSE` reader - Auto-reload shadow enable"]
pub use crate::gd32e508::timer0::ctl0::ARSE_R;
#[doc = "Field `ARSE` writer - Auto-reload shadow enable"]
pub use crate::gd32e508::timer0::ctl0::ARSE_W;
#[doc = "Field `CKDIV` reader - Clock division"]
pub type CKDIV_R = crate::FieldReader;
#[doc = "Field `CKDIV` writer - Clock division"]
pub type CKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&self) -> UPDIS_R {
        UPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update source"]
    #[inline(always)]
    pub fn ups(&self) -> UPS_R {
        UPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    pub fn spm(&self) -> SPM_R {
        SPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Counter aligns mode selection"]
    #[inline(always)]
    pub fn cam(&self) -> CAM_R {
        CAM_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    pub fn arse(&self) -> ARSE_R {
        ARSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<CTL0_SPEC, 0> {
        CEN_W::new(self)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    #[must_use]
    pub fn updis(&mut self) -> UPDIS_W<CTL0_SPEC, 1> {
        UPDIS_W::new(self)
    }
    #[doc = "Bit 2 - Update source"]
    #[inline(always)]
    #[must_use]
    pub fn ups(&mut self) -> UPS_W<CTL0_SPEC, 2> {
        UPS_W::new(self)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    #[must_use]
    pub fn spm(&mut self) -> SPM_W<CTL0_SPEC, 3> {
        SPM_W::new(self)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CTL0_SPEC, 4> {
        DIR_W::new(self)
    }
    #[doc = "Bits 5:6 - Counter aligns mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn cam(&mut self) -> CAM_W<CTL0_SPEC, 5> {
        CAM_W::new(self)
    }
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    #[must_use]
    pub fn arse(&mut self) -> ARSE_W<CTL0_SPEC, 7> {
        ARSE_W::new(self)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv(&mut self) -> CKDIV_W<CTL0_SPEC, 8> {
        CKDIV_W::new(self)
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
#[doc = "control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
