#[doc = "Register `ADDAPB1RST` reader"]
pub type R = crate::R<ADDAPB1RST_SPEC>;
#[doc = "Register `ADDAPB1RST` writer"]
pub type W = crate::W<ADDAPB1RST_SPEC>;
#[doc = "Field `CTCRST` reader - CTC reset"]
pub type CTCRST_R = crate::BitReader<CTCRST_A>;
#[doc = "CTC reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCRST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<CTCRST_A> for bool {
    #[inline(always)]
    fn from(variant: CTCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl CTCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTCRST_A> {
        match self.bits {
            true => Some(CTCRST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CTCRST_A::RESET
    }
}
#[doc = "Field `CTCRST` writer - CTC reset"]
pub type CTCRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTCRST_A>;
impl<'a, REG, const O: u8> CTCRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CTCRST_A::RESET)
    }
}
impl R {
    #[doc = "Bit 27 - CTC reset"]
    #[inline(always)]
    pub fn ctcrst(&self) -> CTCRST_R {
        CTCRST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - CTC reset"]
    #[inline(always)]
    #[must_use]
    pub fn ctcrst(&mut self) -> CTCRST_W<ADDAPB1RST_SPEC, 27> {
        CTCRST_W::new(self)
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
#[doc = "APB1 additional reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDAPB1RST_SPEC;
impl crate::RegisterSpec for ADDAPB1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb1rst::R`](R) reader structure"]
impl crate::Readable for ADDAPB1RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addapb1rst::W`](W) writer structure"]
impl crate::Writable for ADDAPB1RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDAPB1RST to value 0"]
impl crate::Resettable for ADDAPB1RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
