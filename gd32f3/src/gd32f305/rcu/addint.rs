#[doc = "Register `ADDINT` reader"]
pub type R = crate::R<ADDINT_SPEC>;
#[doc = "Register `ADDINT` writer"]
pub type W = crate::W<ADDINT_SPEC>;
#[doc = "Field `IRC48MSTBIF` reader - IRC48M stabilization interrupt flag"]
pub type IRC48MSTBIF_R = crate::BitReader;
#[doc = "Field `IRC48MSTBIE` reader - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
pub type IRC48MSTBIE_R = crate::BitReader;
#[doc = "Field `IRC48MSTBIE` writer - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
pub type IRC48MSTBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IRC48MSTBIC` writer - Internal 48 MHz RC oscillator Stabilization Interrupt Clear"]
pub type IRC48MSTBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 6 - IRC48M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc48mstbif(&self) -> IRC48MSTBIF_R {
        IRC48MSTBIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc48mstbie(&self) -> IRC48MSTBIE_R {
        IRC48MSTBIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc48mstbie(&mut self) -> IRC48MSTBIE_W<ADDINT_SPEC, 14> {
        IRC48MSTBIE_W::new(self)
    }
    #[doc = "Bit 22 - Internal 48 MHz RC oscillator Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn irc48mstbic(&mut self) -> IRC48MSTBIC_W<ADDINT_SPEC, 22> {
        IRC48MSTBIC_W::new(self)
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
#[doc = "Additional clock interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDINT_SPEC;
impl crate::RegisterSpec for ADDINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addint::R`](R) reader structure"]
impl crate::Readable for ADDINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addint::W`](W) writer structure"]
impl crate::Writable for ADDINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDINT to value 0"]
impl crate::Resettable for ADDINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
