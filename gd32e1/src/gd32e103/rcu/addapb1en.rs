#[doc = "Register `ADDAPB1EN` reader"]
pub type R = crate::R<ADDAPB1EN_SPEC>;
#[doc = "Register `ADDAPB1EN` writer"]
pub type W = crate::W<ADDAPB1EN_SPEC>;
#[doc = "Field `CTCEN` reader - CTC clock enable"]
pub type CTCEN_R = crate::BitReader<CTCEN_A>;
#[doc = "CTC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCEN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<CTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCEN_A {
        match self.bits {
            false => CTCEN_A::DISABLED,
            true => CTCEN_A::ENABLED,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTCEN_A::DISABLED
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTCEN_A::ENABLED
    }
}
#[doc = "Field `CTCEN` writer - CTC clock enable"]
pub type CTCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTCEN_A>;
impl<'a, REG, const O: u8> CTCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTCEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTCEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 27 - CTC clock enable"]
    #[inline(always)]
    pub fn ctcen(&self) -> CTCEN_R {
        CTCEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - CTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctcen(&mut self) -> CTCEN_W<ADDAPB1EN_SPEC, 27> {
        CTCEN_W::new(self)
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
#[doc = "APB1 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDAPB1EN_SPEC;
impl crate::RegisterSpec for ADDAPB1EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb1en::R`](R) reader structure"]
impl crate::Readable for ADDAPB1EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addapb1en::W`](W) writer structure"]
impl crate::Writable for ADDAPB1EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDAPB1EN to value 0"]
impl crate::Resettable for ADDAPB1EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
