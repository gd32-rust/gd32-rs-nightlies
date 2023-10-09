#[doc = "Register `LPMINTF` reader"]
pub type R = crate::R<LPMINTF_SPEC>;
#[doc = "Register `LPMINTF` writer"]
pub type W = crate::W<LPMINTF_SPEC>;
#[doc = "Field `LPMSTIF` reader - LPM token Correct transfer interrupt flag"]
pub type LPMSTIF_R = crate::BitReader;
#[doc = "Field `LPMSTIF` writer - LPM token Correct transfer interrupt flag"]
pub type LPMSTIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 15 - LPM token Correct transfer interrupt flag"]
    #[inline(always)]
    pub fn lpmstif(&self) -> LPMSTIF_R {
        LPMSTIF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - LPM token Correct transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn lpmstif(&mut self) -> LPMSTIF_W<LPMINTF_SPEC, 15> {
        LPMSTIF_W::new(self)
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
#[doc = "USB LPM interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmintf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmintf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPMINTF_SPEC;
impl crate::RegisterSpec for LPMINTF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lpmintf::R`](R) reader structure"]
impl crate::Readable for LPMINTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpmintf::W`](W) writer structure"]
impl crate::Writable for LPMINTF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMINTF to value 0"]
impl crate::Resettable for LPMINTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
