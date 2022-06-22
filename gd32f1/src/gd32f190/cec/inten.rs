#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAERRIE` reader - TAERR Interrupt Enable"]
pub type TAERRIE_R = crate::BitReader<bool>;
#[doc = "Field `TAERRIE` writer - TAERR Interrupt Enable"]
pub type TAERRIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 12>;
#[doc = "Field `TERRIE` reader - TERR Interrupt Enable"]
pub type TERRIE_R = crate::BitReader<bool>;
#[doc = "Field `TERRIE` writer - TERR Interrupt Enable"]
pub type TERRIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 11>;
#[doc = "Field `TUIE` reader - TU Interrupt Enable"]
pub type TUIE_R = crate::BitReader<bool>;
#[doc = "Field `TUIE` writer - TU Interrupt Enable"]
pub type TUIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 10>;
#[doc = "Field `TXENDIE` reader - TEND Interrupt Enable"]
pub type TXENDIE_R = crate::BitReader<bool>;
#[doc = "Field `TXENDIE` writer - TEND Interrupt Enable"]
pub type TXENDIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 9>;
#[doc = "Field `TBRIE` reader - TBR Interrupt Enable"]
pub type TBRIE_R = crate::BitReader<bool>;
#[doc = "Field `TBRIE` writer - TBR Interrupt Enable"]
pub type TBRIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 8>;
#[doc = "Field `LSTARBIE` reader - ALRLST Interrupt Enable"]
pub type LSTARBIE_R = crate::BitReader<bool>;
#[doc = "Field `LSTARBIE` writer - ALRLST Interrupt Enable"]
pub type LSTARBIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 7>;
#[doc = "Field `RAEIE` reader - RAE Interrupt Enable"]
pub type RAEIE_R = crate::BitReader<bool>;
#[doc = "Field `RAEIE` writer - RAE Interrupt Enable"]
pub type RAEIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 6>;
#[doc = "Field `RLBPEIE` reader - RLBPE Interrupt Enable"]
pub type RLBPEIE_R = crate::BitReader<bool>;
#[doc = "Field `RLBPEIE` writer - RLBPE Interrupt Enable"]
pub type RLBPEIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 5>;
#[doc = "Field `RSBPEIE` reader - RSBPE Interrupt Enable"]
pub type RSBPEIE_R = crate::BitReader<bool>;
#[doc = "Field `RSBPEIE` writer - RSBPE Interrupt Enable"]
pub type RSBPEIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 4>;
#[doc = "Field `RBREIE` reader - RBRE Interrupt Enable"]
pub type RBREIE_R = crate::BitReader<bool>;
#[doc = "Field `RBREIE` writer - RBRE Interrupt Enable"]
pub type RBREIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 3>;
#[doc = "Field `ROIE` reader - RO Interrupt Enable"]
pub type ROIE_R = crate::BitReader<bool>;
#[doc = "Field `ROIE` writer - RO Interrupt Enable"]
pub type ROIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 2>;
#[doc = "Field `RENDIE` reader - REND Interrupt Enable"]
pub type RENDIE_R = crate::BitReader<bool>;
#[doc = "Field `RENDIE` writer - REND Interrupt Enable"]
pub type RENDIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 1>;
#[doc = "Field `RBRIE` reader - RBR Interrupt Enable"]
pub type RBRIE_R = crate::BitReader<bool>;
#[doc = "Field `RBRIE` writer - RBR Interrupt Enable"]
pub type RBRIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 12 - TAERR Interrupt Enable"]
    #[inline(always)]
    pub fn taerrie(&self) -> TAERRIE_R {
        TAERRIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - TERR Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - TU Interrupt Enable"]
    #[inline(always)]
    pub fn tuie(&self) -> TUIE_R {
        TUIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - TEND Interrupt Enable"]
    #[inline(always)]
    pub fn txendie(&self) -> TXENDIE_R {
        TXENDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - TBR Interrupt Enable"]
    #[inline(always)]
    pub fn tbrie(&self) -> TBRIE_R {
        TBRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - ALRLST Interrupt Enable"]
    #[inline(always)]
    pub fn lstarbie(&self) -> LSTARBIE_R {
        LSTARBIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - RAE Interrupt Enable"]
    #[inline(always)]
    pub fn raeie(&self) -> RAEIE_R {
        RAEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - RLBPE Interrupt Enable"]
    #[inline(always)]
    pub fn rlbpeie(&self) -> RLBPEIE_R {
        RLBPEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - RSBPE Interrupt Enable"]
    #[inline(always)]
    pub fn rsbpeie(&self) -> RSBPEIE_R {
        RSBPEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - RBRE Interrupt Enable"]
    #[inline(always)]
    pub fn rbreie(&self) -> RBREIE_R {
        RBREIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - RO Interrupt Enable"]
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - REND Interrupt Enable"]
    #[inline(always)]
    pub fn rendie(&self) -> RENDIE_R {
        RENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - RBR Interrupt Enable"]
    #[inline(always)]
    pub fn rbrie(&self) -> RBRIE_R {
        RBRIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - TAERR Interrupt Enable"]
    #[inline(always)]
    pub fn taerrie(&mut self) -> TAERRIE_W {
        TAERRIE_W::new(self)
    }
    #[doc = "Bit 11 - TERR Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&mut self) -> TERRIE_W {
        TERRIE_W::new(self)
    }
    #[doc = "Bit 10 - TU Interrupt Enable"]
    #[inline(always)]
    pub fn tuie(&mut self) -> TUIE_W {
        TUIE_W::new(self)
    }
    #[doc = "Bit 9 - TEND Interrupt Enable"]
    #[inline(always)]
    pub fn txendie(&mut self) -> TXENDIE_W {
        TXENDIE_W::new(self)
    }
    #[doc = "Bit 8 - TBR Interrupt Enable"]
    #[inline(always)]
    pub fn tbrie(&mut self) -> TBRIE_W {
        TBRIE_W::new(self)
    }
    #[doc = "Bit 7 - ALRLST Interrupt Enable"]
    #[inline(always)]
    pub fn lstarbie(&mut self) -> LSTARBIE_W {
        LSTARBIE_W::new(self)
    }
    #[doc = "Bit 6 - RAE Interrupt Enable"]
    #[inline(always)]
    pub fn raeie(&mut self) -> RAEIE_W {
        RAEIE_W::new(self)
    }
    #[doc = "Bit 5 - RLBPE Interrupt Enable"]
    #[inline(always)]
    pub fn rlbpeie(&mut self) -> RLBPEIE_W {
        RLBPEIE_W::new(self)
    }
    #[doc = "Bit 4 - RSBPE Interrupt Enable"]
    #[inline(always)]
    pub fn rsbpeie(&mut self) -> RSBPEIE_W {
        RSBPEIE_W::new(self)
    }
    #[doc = "Bit 3 - RBRE Interrupt Enable"]
    #[inline(always)]
    pub fn rbreie(&mut self) -> RBREIE_W {
        RBREIE_W::new(self)
    }
    #[doc = "Bit 2 - RO Interrupt Enable"]
    #[inline(always)]
    pub fn roie(&mut self) -> ROIE_W {
        ROIE_W::new(self)
    }
    #[doc = "Bit 1 - REND Interrupt Enable"]
    #[inline(always)]
    pub fn rendie(&mut self) -> RENDIE_W {
        RENDIE_W::new(self)
    }
    #[doc = "Bit 0 - RBR Interrupt Enable"]
    #[inline(always)]
    pub fn rbrie(&mut self) -> RBRIE_W {
        RBRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
