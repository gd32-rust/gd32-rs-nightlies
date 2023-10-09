#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `RBRIE` reader - RBR Interrupt Enable"]
pub type RBRIE_R = crate::BitReader;
#[doc = "Field `RBRIE` writer - RBR Interrupt Enable"]
pub type RBRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RENDIE` reader - REND Interrupt Enable"]
pub type RENDIE_R = crate::BitReader;
#[doc = "Field `RENDIE` writer - REND Interrupt Enable"]
pub type RENDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROIE` reader - RO Interrupt Enable"]
pub type ROIE_R = crate::BitReader;
#[doc = "Field `ROIE` writer - RO Interrupt Enable"]
pub type ROIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBREIE` reader - RBRE Interrupt Enable"]
pub type RBREIE_R = crate::BitReader;
#[doc = "Field `RBREIE` writer - RBRE Interrupt Enable"]
pub type RBREIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSBPEIE` reader - RSBPE Interrupt Enable"]
pub type RSBPEIE_R = crate::BitReader;
#[doc = "Field `RSBPEIE` writer - RSBPE Interrupt Enable"]
pub type RSBPEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RLBPEIE` reader - RLBPE Interrupt Enable"]
pub type RLBPEIE_R = crate::BitReader;
#[doc = "Field `RLBPEIE` writer - RLBPE Interrupt Enable"]
pub type RLBPEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAEIE` reader - RAE Interrupt Enable"]
pub type RAEIE_R = crate::BitReader;
#[doc = "Field `RAEIE` writer - RAE Interrupt Enable"]
pub type RAEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSTARBIE` reader - ALRLST Interrupt Enable"]
pub type LSTARBIE_R = crate::BitReader;
#[doc = "Field `LSTARBIE` writer - ALRLST Interrupt Enable"]
pub type LSTARBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TBRIE` reader - TBR Interrupt Enable"]
pub type TBRIE_R = crate::BitReader;
#[doc = "Field `TBRIE` writer - TBR Interrupt Enable"]
pub type TBRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXENDIE` reader - TEND Interrupt Enable"]
pub type TXENDIE_R = crate::BitReader;
#[doc = "Field `TXENDIE` writer - TEND Interrupt Enable"]
pub type TXENDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TUIE` reader - TU Interrupt Enable"]
pub type TUIE_R = crate::BitReader;
#[doc = "Field `TUIE` writer - TU Interrupt Enable"]
pub type TUIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TERRIE` reader - TERR Interrupt Enable"]
pub type TERRIE_R = crate::BitReader;
#[doc = "Field `TERRIE` writer - TERR Interrupt Enable"]
pub type TERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAERRIE` reader - TAERR Interrupt Enable"]
pub type TAERRIE_R = crate::BitReader;
#[doc = "Field `TAERRIE` writer - TAERR Interrupt Enable"]
pub type TAERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - RBR Interrupt Enable"]
    #[inline(always)]
    pub fn rbrie(&self) -> RBRIE_R {
        RBRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REND Interrupt Enable"]
    #[inline(always)]
    pub fn rendie(&self) -> RENDIE_R {
        RENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RO Interrupt Enable"]
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RBRE Interrupt Enable"]
    #[inline(always)]
    pub fn rbreie(&self) -> RBREIE_R {
        RBREIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RSBPE Interrupt Enable"]
    #[inline(always)]
    pub fn rsbpeie(&self) -> RSBPEIE_R {
        RSBPEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RLBPE Interrupt Enable"]
    #[inline(always)]
    pub fn rlbpeie(&self) -> RLBPEIE_R {
        RLBPEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RAE Interrupt Enable"]
    #[inline(always)]
    pub fn raeie(&self) -> RAEIE_R {
        RAEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ALRLST Interrupt Enable"]
    #[inline(always)]
    pub fn lstarbie(&self) -> LSTARBIE_R {
        LSTARBIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TBR Interrupt Enable"]
    #[inline(always)]
    pub fn tbrie(&self) -> TBRIE_R {
        TBRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TEND Interrupt Enable"]
    #[inline(always)]
    pub fn txendie(&self) -> TXENDIE_R {
        TXENDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TU Interrupt Enable"]
    #[inline(always)]
    pub fn tuie(&self) -> TUIE_R {
        TUIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TERR Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TAERR Interrupt Enable"]
    #[inline(always)]
    pub fn taerrie(&self) -> TAERRIE_R {
        TAERRIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RBR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbrie(&mut self) -> RBRIE_W<INTEN_SPEC, 0> {
        RBRIE_W::new(self)
    }
    #[doc = "Bit 1 - REND Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rendie(&mut self) -> RENDIE_W<INTEN_SPEC, 1> {
        RENDIE_W::new(self)
    }
    #[doc = "Bit 2 - RO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn roie(&mut self) -> ROIE_W<INTEN_SPEC, 2> {
        ROIE_W::new(self)
    }
    #[doc = "Bit 3 - RBRE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbreie(&mut self) -> RBREIE_W<INTEN_SPEC, 3> {
        RBREIE_W::new(self)
    }
    #[doc = "Bit 4 - RSBPE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsbpeie(&mut self) -> RSBPEIE_W<INTEN_SPEC, 4> {
        RSBPEIE_W::new(self)
    }
    #[doc = "Bit 5 - RLBPE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rlbpeie(&mut self) -> RLBPEIE_W<INTEN_SPEC, 5> {
        RLBPEIE_W::new(self)
    }
    #[doc = "Bit 6 - RAE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn raeie(&mut self) -> RAEIE_W<INTEN_SPEC, 6> {
        RAEIE_W::new(self)
    }
    #[doc = "Bit 7 - ALRLST Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lstarbie(&mut self) -> LSTARBIE_W<INTEN_SPEC, 7> {
        LSTARBIE_W::new(self)
    }
    #[doc = "Bit 8 - TBR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbrie(&mut self) -> TBRIE_W<INTEN_SPEC, 8> {
        TBRIE_W::new(self)
    }
    #[doc = "Bit 9 - TEND Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txendie(&mut self) -> TXENDIE_W<INTEN_SPEC, 9> {
        TXENDIE_W::new(self)
    }
    #[doc = "Bit 10 - TU Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuie(&mut self) -> TUIE_W<INTEN_SPEC, 10> {
        TUIE_W::new(self)
    }
    #[doc = "Bit 11 - TERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn terrie(&mut self) -> TERRIE_W<INTEN_SPEC, 11> {
        TERRIE_W::new(self)
    }
    #[doc = "Bit 12 - TAERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn taerrie(&mut self) -> TAERRIE_W<INTEN_SPEC, 12> {
        TAERRIE_W::new(self)
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
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
