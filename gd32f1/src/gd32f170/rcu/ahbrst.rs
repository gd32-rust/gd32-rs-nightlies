#[doc = "Register `AHBRST` reader"]
pub type R = crate::R<AHBRST_SPEC>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::W<AHBRST_SPEC>;
#[doc = "Field `PARST` reader - GPIO port A reset"]
pub type PARST_R = crate::BitReader<PARST_A>;
#[doc = "GPIO port A reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<PARST_A> for bool {
    #[inline(always)]
    fn from(variant: PARST_A) -> Self {
        variant as u8 != 0
    }
}
impl PARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PARST_A> {
        match self.bits {
            true => Some(PARST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PARST_A::RESET
    }
}
#[doc = "Field `PARST` writer - GPIO port A reset"]
pub type PARST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PARST_A>;
impl<'a, REG, const O: u8> PARST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PARST_A::RESET)
    }
}
#[doc = "Field `PBRST` reader - GPIO port B reset"]
pub use PARST_R as PBRST_R;
#[doc = "Field `PCRST` reader - GPIO port C reset"]
pub use PARST_R as PCRST_R;
#[doc = "Field `PDRST` reader - GPIO port D reset"]
pub use PARST_R as PDRST_R;
#[doc = "Field `PFRST` reader - GPIO port F reset"]
pub use PARST_R as PFRST_R;
#[doc = "Field `TSIRST` reader - TSI unit reset"]
pub use PARST_R as TSIRST_R;
#[doc = "Field `PBRST` writer - GPIO port B reset"]
pub use PARST_W as PBRST_W;
#[doc = "Field `PCRST` writer - GPIO port C reset"]
pub use PARST_W as PCRST_W;
#[doc = "Field `PDRST` writer - GPIO port D reset"]
pub use PARST_W as PDRST_W;
#[doc = "Field `PFRST` writer - GPIO port F reset"]
pub use PARST_W as PFRST_W;
#[doc = "Field `TSIRST` writer - TSI unit reset"]
pub use PARST_W as TSIRST_W;
impl R {
    #[doc = "Bit 17 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&self) -> PARST_R {
        PARST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&self) -> PBRST_R {
        PBRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&self) -> PCRST_R {
        PCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO port D reset"]
    #[inline(always)]
    pub fn pdrst(&self) -> PDRST_R {
        PDRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO port F reset"]
    #[inline(always)]
    pub fn pfrst(&self) -> PFRST_R {
        PFRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - TSI unit reset"]
    #[inline(always)]
    pub fn tsirst(&self) -> TSIRST_R {
        TSIRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - GPIO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn parst(&mut self) -> PARST_W<AHBRST_SPEC, 17> {
        PARST_W::new(self)
    }
    #[doc = "Bit 18 - GPIO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst(&mut self) -> PBRST_W<AHBRST_SPEC, 18> {
        PBRST_W::new(self)
    }
    #[doc = "Bit 19 - GPIO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst(&mut self) -> PCRST_W<AHBRST_SPEC, 19> {
        PCRST_W::new(self)
    }
    #[doc = "Bit 20 - GPIO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst(&mut self) -> PDRST_W<AHBRST_SPEC, 20> {
        PDRST_W::new(self)
    }
    #[doc = "Bit 22 - GPIO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn pfrst(&mut self) -> PFRST_W<AHBRST_SPEC, 22> {
        PFRST_W::new(self)
    }
    #[doc = "Bit 24 - TSI unit reset"]
    #[inline(always)]
    #[must_use]
    pub fn tsirst(&mut self) -> TSIRST_W<AHBRST_SPEC, 24> {
        TSIRST_W::new(self)
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
#[doc = "AHB reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST_SPEC;
impl crate::RegisterSpec for AHBRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst::R`](R) reader structure"]
impl crate::Readable for AHBRST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst::W`](W) writer structure"]
impl crate::Writable for AHBRST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::Resettable for AHBRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
