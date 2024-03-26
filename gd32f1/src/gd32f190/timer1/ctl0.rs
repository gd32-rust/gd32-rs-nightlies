#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Counter enable"]
pub use crate::gd32f190::timer0::ctl0::Cen;
#[doc = "Field `CEN` reader - Counter enable"]
pub use crate::gd32f190::timer0::ctl0::CenR;
#[doc = "Field `CEN` writer - Counter enable"]
pub use crate::gd32f190::timer0::ctl0::CenW;
#[doc = "Update disable"]
pub use crate::gd32f190::timer0::ctl0::Updis;
#[doc = "Field `UPDIS` reader - Update disable"]
pub use crate::gd32f190::timer0::ctl0::UpdisR;
#[doc = "Field `UPDIS` writer - Update disable"]
pub use crate::gd32f190::timer0::ctl0::UpdisW;
#[doc = "Update request source"]
pub use crate::gd32f190::timer0::ctl0::Ups;
#[doc = "Field `UPS` reader - Update request source"]
pub use crate::gd32f190::timer0::ctl0::UpsR;
#[doc = "Field `UPS` writer - Update request source"]
pub use crate::gd32f190::timer0::ctl0::UpsW;
#[doc = "Single pulse mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spm {
    #[doc = "0: Counter is not stopped at update event"]
    Disabled = 0,
    #[doc = "1: Counter stops counting at the next update event (clearing the CEN bit)"]
    Enabled = 1,
}
impl From<Spm> for bool {
    #[inline(always)]
    fn from(variant: Spm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPM` reader - Single pulse mode"]
pub type SpmR = crate::BitReader<Spm>;
impl SpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spm {
        match self.bits {
            false => Spm::Disabled,
            true => Spm::Enabled,
        }
    }
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Spm::Disabled
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Spm::Enabled
    }
}
#[doc = "Field `SPM` writer - Single pulse mode"]
pub type SpmW<'a, REG> = crate::BitWriter<'a, REG, Spm>;
impl<'a, REG> SpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Spm::Disabled)
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Spm::Enabled)
    }
}
#[doc = "Auto-reload shadow enable"]
pub use crate::gd32f190::timer0::ctl0::Arse;
#[doc = "Field `ARSE` reader - Auto-reload shadow enable"]
pub use crate::gd32f190::timer0::ctl0::ArseR;
#[doc = "Field `ARSE` writer - Auto-reload shadow enable"]
pub use crate::gd32f190::timer0::ctl0::ArseW;
#[doc = "Counter aligns mode selection"]
pub use crate::gd32f190::timer0::ctl0::Cam;
#[doc = "Field `CAM` reader - Counter aligns mode selection"]
pub use crate::gd32f190::timer0::ctl0::CamR;
#[doc = "Field `CAM` writer - Counter aligns mode selection"]
pub use crate::gd32f190::timer0::ctl0::CamW;
#[doc = "Clock division"]
pub use crate::gd32f190::timer0::ctl0::Ckdiv;
#[doc = "Field `CKDIV` reader - Clock division"]
pub use crate::gd32f190::timer0::ctl0::CkdivR;
#[doc = "Field `CKDIV` writer - Clock division"]
pub use crate::gd32f190::timer0::ctl0::CkdivW;
#[doc = "Direction"]
pub use crate::gd32f190::timer0::ctl0::Dir;
#[doc = "Field `DIR` reader - Direction"]
pub use crate::gd32f190::timer0::ctl0::DirR;
#[doc = "Field `DIR` writer - Direction"]
pub use crate::gd32f190::timer0::ctl0::DirW;
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&self) -> UpdisR {
        UpdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn ups(&self) -> UpsR {
        UpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    pub fn spm(&self) -> SpmR {
        SpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Counter aligns mode selection"]
    #[inline(always)]
    pub fn cam(&self) -> CamR {
        CamR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    pub fn arse(&self) -> ArseR {
        ArseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CkdivR {
        CkdivR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CenW<Ctl0Spec> {
        CenW::new(self, 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    #[must_use]
    pub fn updis(&mut self) -> UpdisW<Ctl0Spec> {
        UpdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    #[must_use]
    pub fn ups(&mut self) -> UpsW<Ctl0Spec> {
        UpsW::new(self, 2)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    #[must_use]
    pub fn spm(&mut self) -> SpmW<Ctl0Spec> {
        SpmW::new(self, 3)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<Ctl0Spec> {
        DirW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Counter aligns mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn cam(&mut self) -> CamW<Ctl0Spec> {
        CamW::new(self, 5)
    }
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    #[must_use]
    pub fn arse(&mut self) -> ArseW<Ctl0Spec> {
        ArseW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv(&mut self) -> CkdivW<Ctl0Spec> {
        CkdivW::new(self, 8)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u16 = 0;
}
