#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `CNT` reader - 7-bit counter"]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - 7-bit counter"]
pub type CNT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 7, O>;
#[doc = "Field `WDGTEN` reader - Activation bit"]
pub type WDGTEN_R = crate::BitReader<WDGTEN_A>;
#[doc = "Activation bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDGTEN_A {
    #[doc = "0: Watchdog disabled"]
    DISABLED = 0,
    #[doc = "1: Watchdog enabled"]
    ENABLED = 1,
}
impl From<WDGTEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDGTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WDGTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDGTEN_A {
        match self.bits {
            false => WDGTEN_A::DISABLED,
            true => WDGTEN_A::ENABLED,
        }
    }
    #[doc = "Watchdog disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDGTEN_A::DISABLED
    }
    #[doc = "Watchdog enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDGTEN_A::ENABLED
    }
}
#[doc = "Field `WDGTEN` writer - Activation bit"]
pub type WDGTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDGTEN_A>;
impl<'a, REG, const O: u8> WDGTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Watchdog disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTEN_A::DISABLED)
    }
    #[doc = "Watchdog enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:6 - 7-bit counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    pub fn wdgten(&self) -> WDGTEN_R {
        WDGTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit counter"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CTL_SPEC, 0> {
        CNT_W::new(self)
    }
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    #[must_use]
    pub fn wdgten(&mut self) -> WDGTEN_W<CTL_SPEC, 7> {
        WDGTEN_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x7f"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
