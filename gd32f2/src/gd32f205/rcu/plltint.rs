#[doc = "Register `PLLTINT` reader"]
pub type R = crate::R<PLLTINT_SPEC>;
#[doc = "Register `PLLTINT` writer"]
pub type W = crate::W<PLLTINT_SPEC>;
#[doc = "Field `PLLTSTBIF` reader - PLLT stabilization interrupt flag"]
pub type PLLTSTBIF_R = crate::BitReader;
#[doc = "Field `PLLTSTBIE` reader - PLLT stabilization Interrupt Enable"]
pub type PLLTSTBIE_R = crate::BitReader;
#[doc = "Field `PLLTSTBIE` writer - PLLT stabilization Interrupt Enable"]
pub type PLLTSTBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLTSTBIC` reader - PLLT stabilization Interrupt clear"]
pub type PLLTSTBIC_R = crate::BitReader;
#[doc = "Field `PLLTSTBIC` writer - PLLT stabilization Interrupt clear"]
pub type PLLTSTBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 6 - PLLT stabilization interrupt flag"]
    #[inline(always)]
    pub fn plltstbif(&self) -> PLLTSTBIF_R {
        PLLTSTBIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - PLLT stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn plltstbie(&self) -> PLLTSTBIE_R {
        PLLTSTBIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 22 - PLLT stabilization Interrupt clear"]
    #[inline(always)]
    pub fn plltstbic(&self) -> PLLTSTBIC_R {
        PLLTSTBIC_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - PLLT stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn plltstbie(&mut self) -> PLLTSTBIE_W<PLLTINT_SPEC, 14> {
        PLLTSTBIE_W::new(self)
    }
    #[doc = "Bit 22 - PLLT stabilization Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn plltstbic(&mut self) -> PLLTSTBIC_W<PLLTINT_SPEC, 22> {
        PLLTSTBIC_W::new(self)
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
#[doc = "PLLT interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plltint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plltint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLTINT_SPEC;
impl crate::RegisterSpec for PLLTINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plltint::R`](R) reader structure"]
impl crate::Readable for PLLTINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plltint::W`](W) writer structure"]
impl crate::Writable for PLLTINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLTINT to value 0"]
impl crate::Resettable for PLLTINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
