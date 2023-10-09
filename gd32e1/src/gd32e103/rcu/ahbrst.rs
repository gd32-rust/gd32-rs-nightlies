#[doc = "Register `AHBRST` reader"]
pub type R = crate::R<AHBRST_SPEC>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::W<AHBRST_SPEC>;
#[doc = "Field `USBFSRST` reader - USBFS reset"]
pub type USBFSRST_R = crate::BitReader<USBFSRST_A>;
#[doc = "USBFS reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBFSRST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<USBFSRST_A> for bool {
    #[inline(always)]
    fn from(variant: USBFSRST_A) -> Self {
        variant as u8 != 0
    }
}
impl USBFSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USBFSRST_A> {
        match self.bits {
            true => Some(USBFSRST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == USBFSRST_A::RESET
    }
}
#[doc = "Field `USBFSRST` writer - USBFS reset"]
pub type USBFSRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USBFSRST_A>;
impl<'a, REG, const O: u8> USBFSRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSRST_A::RESET)
    }
}
impl R {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    pub fn usbfsrst(&self) -> USBFSRST_R {
        USBFSRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsrst(&mut self) -> USBFSRST_W<AHBRST_SPEC, 12> {
        USBFSRST_W::new(self)
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
