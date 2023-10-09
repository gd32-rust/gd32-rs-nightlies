#[doc = "Register `FMPCFG` reader"]
pub type R = crate::R<FMPCFG_SPEC>;
#[doc = "Register `FMPCFG` writer"]
pub type W = crate::W<FMPCFG_SPEC>;
#[doc = "Field `FMPEN` reader - Fast-mode-plus enable"]
pub type FMPEN_R = crate::BitReader<FMPEN_A>;
#[doc = "Fast-mode-plus enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPEN_A {
    #[doc = "0: Fast mode plus disabled"]
    DISABLED = 0,
    #[doc = "1: Fast mode plus (1MHz max) enabled"]
    ENABLED = 1,
}
impl From<FMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FMPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMPEN_A {
        match self.bits {
            false => FMPEN_A::DISABLED,
            true => FMPEN_A::ENABLED,
        }
    }
    #[doc = "Fast mode plus disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMPEN_A::DISABLED
    }
    #[doc = "Fast mode plus (1MHz max) enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMPEN_A::ENABLED
    }
}
#[doc = "Field `FMPEN` writer - Fast-mode-plus enable"]
pub type FMPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FMPEN_A>;
impl<'a, REG, const O: u8> FMPEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast mode plus disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMPEN_A::DISABLED)
    }
    #[doc = "Fast mode plus (1MHz max) enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMPEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Fast-mode-plus enable"]
    #[inline(always)]
    pub fn fmpen(&self) -> FMPEN_R {
        FMPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast-mode-plus enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmpen(&mut self) -> FMPEN_W<FMPCFG_SPEC, 0> {
        FMPEN_W::new(self)
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
#[doc = "Fast mode plus configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmpcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmpcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMPCFG_SPEC;
impl crate::RegisterSpec for FMPCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmpcfg::R`](R) reader structure"]
impl crate::Readable for FMPCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmpcfg::W`](W) writer structure"]
impl crate::Writable for FMPCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMPCFG to value 0"]
impl crate::Resettable for FMPCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
