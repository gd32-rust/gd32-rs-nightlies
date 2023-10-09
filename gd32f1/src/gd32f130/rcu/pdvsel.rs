#[doc = "Register `PDVSEL` reader"]
pub type R = crate::R<PDVSEL_SPEC>;
#[doc = "Register `PDVSEL` writer"]
pub type W = crate::W<PDVSEL_SPEC>;
#[doc = "Field `PDRVS` reader - Power down voltage select"]
pub type PDRVS_R = crate::BitReader<PDRVS_A>;
#[doc = "Power down voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDRVS_A {
    #[doc = "0: The power down voltage is 2.6 V"]
    V2_6 = 0,
    #[doc = "1: The power down voltage is 1.8 V"]
    V1_8 = 1,
}
impl From<PDRVS_A> for bool {
    #[inline(always)]
    fn from(variant: PDRVS_A) -> Self {
        variant as u8 != 0
    }
}
impl PDRVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDRVS_A {
        match self.bits {
            false => PDRVS_A::V2_6,
            true => PDRVS_A::V1_8,
        }
    }
    #[doc = "The power down voltage is 2.6 V"]
    #[inline(always)]
    pub fn is_v2_6(&self) -> bool {
        *self == PDRVS_A::V2_6
    }
    #[doc = "The power down voltage is 1.8 V"]
    #[inline(always)]
    pub fn is_v1_8(&self) -> bool {
        *self == PDRVS_A::V1_8
    }
}
#[doc = "Field `PDRVS` writer - Power down voltage select"]
pub type PDRVS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PDRVS_A>;
impl<'a, REG, const O: u8> PDRVS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The power down voltage is 2.6 V"]
    #[inline(always)]
    pub fn v2_6(self) -> &'a mut crate::W<REG> {
        self.variant(PDRVS_A::V2_6)
    }
    #[doc = "The power down voltage is 1.8 V"]
    #[inline(always)]
    pub fn v1_8(self) -> &'a mut crate::W<REG> {
        self.variant(PDRVS_A::V1_8)
    }
}
impl R {
    #[doc = "Bit 0 - Power down voltage select"]
    #[inline(always)]
    pub fn pdrvs(&self) -> PDRVS_R {
        PDRVS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn pdrvs(&mut self) -> PDRVS_W<PDVSEL_SPEC, 0> {
        PDRVS_W::new(self)
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
#[doc = "Power down voltage select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdvsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdvsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDVSEL_SPEC;
impl crate::RegisterSpec for PDVSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdvsel::R`](R) reader structure"]
impl crate::Readable for PDVSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdvsel::W`](W) writer structure"]
impl crate::Writable for PDVSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDVSEL to value 0"]
impl crate::Resettable for PDVSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
