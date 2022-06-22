#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Auto-reload shadow enable"]
pub use crate::gd32f150::timer0::ctl0::ARSE_A;
#[doc = "Field `ARSE` reader - Auto-reload shadow enable"]
pub use crate::gd32f150::timer0::ctl0::ARSE_R;
#[doc = "Field `ARSE` writer - Auto-reload shadow enable"]
pub type ARSE_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, ARSE_A, 7>;
impl<'a> ARSE_W<'a> {
    #[doc = "The shadow register for CAR is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARSE_A::DISABLED)
    }
    #[doc = "The shadow register for CAR is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARSE_A::ENABLED)
    }
}
#[doc = "Single pulse mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SPM` reader - Single pulse mode"]
pub type SPM_R = crate::BitReader<SPM_A>;
impl SPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPM_A {
        match self.bits {
            false => SPM_A::DISABLED,
            true => SPM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPM_A::ENABLED
    }
}
#[doc = "Field `SPM` writer - Single pulse mode"]
pub type SPM_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, SPM_A, 3>;
impl<'a> SPM_W<'a> {
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPM_A::DISABLED)
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPM_A::ENABLED)
    }
}
#[doc = "Update source"]
pub use crate::gd32f150::timer0::ctl0::UPS_A;
#[doc = "Field `UPS` reader - Update source"]
pub use crate::gd32f150::timer0::ctl0::UPS_R;
#[doc = "Field `UPS` writer - Update source"]
pub type UPS_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, UPS_A, 2>;
impl<'a> UPS_W<'a> {
    #[doc = "Any of counter overflow/underflow, setting UPG, or update through slave mode, generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn any_event(self) -> &'a mut W {
        self.variant(UPS_A::ANYEVENT)
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn counter_only(self) -> &'a mut W {
        self.variant(UPS_A::COUNTERONLY)
    }
}
#[doc = "Update disable"]
pub use crate::gd32f150::timer0::ctl0::UPDIS_A;
#[doc = "Field `UPDIS` reader - Update disable"]
pub use crate::gd32f150::timer0::ctl0::UPDIS_R;
#[doc = "Field `UPDIS` writer - Update disable"]
pub type UPDIS_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, UPDIS_A, 1>;
impl<'a> UPDIS_W<'a> {
    #[doc = "Update event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPDIS_A::ENABLED)
    }
    #[doc = "Update event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPDIS_A::DISABLED)
    }
}
#[doc = "Counter enable"]
pub use crate::gd32f150::timer0::ctl0::CEN_A;
#[doc = "Field `CEN` reader - Counter enable"]
pub use crate::gd32f150::timer0::ctl0::CEN_R;
#[doc = "Field `CEN` writer - Counter enable"]
pub type CEN_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, CEN_A, 0>;
impl<'a> CEN_W<'a> {
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEN_A::DISABLED)
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    pub fn arse(&self) -> ARSE_R {
        ARSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    pub fn spm(&self) -> SPM_R {
        SPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Update source"]
    #[inline(always)]
    pub fn ups(&self) -> UPS_R {
        UPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&self) -> UPDIS_R {
        UPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    pub fn arse(&mut self) -> ARSE_W {
        ARSE_W::new(self)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    pub fn spm(&mut self) -> SPM_W {
        SPM_W::new(self)
    }
    #[doc = "Bit 2 - Update source"]
    #[inline(always)]
    pub fn ups(&mut self) -> UPS_W {
        UPS_W::new(self)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&mut self) -> UPDIS_W {
        UPDIS_W::new(self)
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
