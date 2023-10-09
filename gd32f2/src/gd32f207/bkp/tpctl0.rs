#[doc = "Register `TPCTL0` reader"]
pub type R = crate::R<TPCTL0_SPEC>;
#[doc = "Register `TPCTL0` writer"]
pub type W = crate::W<TPCTL0_SPEC>;
#[doc = "Field `TPEN0` reader - TAMPER0 detection enable"]
pub type TPEN0_R = crate::BitReader;
#[doc = "Field `TPEN0` writer - TAMPER0 detection enable"]
pub type TPEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPAL0` reader - TAMPER0 pin active level"]
pub type TPAL0_R = crate::BitReader;
#[doc = "Field `TPAL0` writer - TAMPER0 pin active level"]
pub type TPAL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TAMPER0 detection enable"]
    #[inline(always)]
    pub fn tpen0(&self) -> TPEN0_R {
        TPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMPER0 pin active level"]
    #[inline(always)]
    pub fn tpal0(&self) -> TPAL0_R {
        TPAL0_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMPER0 detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpen0(&mut self) -> TPEN0_W<TPCTL0_SPEC, 0> {
        TPEN0_W::new(self)
    }
    #[doc = "Bit 1 - TAMPER0 pin active level"]
    #[inline(always)]
    #[must_use]
    pub fn tpal0(&mut self) -> TPAL0_W<TPCTL0_SPEC, 1> {
        TPAL0_W::new(self)
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
#[doc = "Tamper pin control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPCTL0_SPEC;
impl crate::RegisterSpec for TPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpctl0::R`](R) reader structure"]
impl crate::Readable for TPCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpctl0::W`](W) writer structure"]
impl crate::Writable for TPCTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPCTL0 to value 0"]
impl crate::Resettable for TPCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
