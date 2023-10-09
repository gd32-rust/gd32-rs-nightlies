#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `LMIE` reader - Line Mark Interrupt Enable"]
pub type LMIE_R = crate::BitReader;
#[doc = "Field `LMIE` writer - Line Mark Interrupt Enable"]
pub type LMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FEIE` reader - FIFO Error Interrupt Enable"]
pub type FEIE_R = crate::BitReader;
#[doc = "Field `FEIE` writer - FIFO Error Interrupt Enable"]
pub type FEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEIE` reader - Transaction Error Interrupt Enable"]
pub type TEIE_R = crate::BitReader;
#[doc = "Field `TEIE` writer - Transaction Error Interrupt Enable"]
pub type TEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCRIE` reader - Layer Configuration Reloaded Interrupt Enable"]
pub type LCRIE_R = crate::BitReader;
#[doc = "Field `LCRIE` writer - Layer Configuration Reloaded Interrupt Enable"]
pub type LCRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Line Mark Interrupt Enable"]
    #[inline(always)]
    pub fn lmie(&self) -> LMIE_R {
        LMIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Layer Configuration Reloaded Interrupt Enable"]
    #[inline(always)]
    pub fn lcrie(&self) -> LCRIE_R {
        LCRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Line Mark Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lmie(&mut self) -> LMIE_W<INTEN_SPEC, 0> {
        LMIE_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<INTEN_SPEC, 1> {
        FEIE_W::new(self)
    }
    #[doc = "Bit 2 - Transaction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<INTEN_SPEC, 2> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 3 - Layer Configuration Reloaded Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcrie(&mut self) -> LCRIE_W<INTEN_SPEC, 3> {
        LCRIE_W::new(self)
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
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
