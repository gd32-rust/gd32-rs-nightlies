#[doc = "Register `LPMCTL` reader"]
pub type R = crate::R<LPMCTL_SPEC>;
#[doc = "Register `LPMCTL` writer"]
pub type W = crate::W<LPMCTL_SPEC>;
#[doc = "Field `LPMSTIE` reader - LPM token successful transfer interrupt enable"]
pub type LPMSTIE_R = crate::BitReader;
#[doc = "Field `LPMSTIE` writer - LPM token successful transfer interrupt enable"]
pub type LPMSTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 15 - LPM token successful transfer interrupt enable"]
    #[inline(always)]
    pub fn lpmstie(&self) -> LPMSTIE_R {
        LPMSTIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - LPM token successful transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmstie(&mut self) -> LPMSTIE_W<LPMCTL_SPEC, 15> {
        LPMSTIE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB LPM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPMCTL_SPEC;
impl crate::RegisterSpec for LPMCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lpmctl::R`](R) reader structure"]
impl crate::Readable for LPMCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpmctl::W`](W) writer structure"]
impl crate::Writable for LPMCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMCTL to value 0"]
impl crate::Resettable for LPMCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
