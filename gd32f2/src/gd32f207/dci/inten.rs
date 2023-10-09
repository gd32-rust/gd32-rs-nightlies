#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `EFIE` reader - End of Frame Interrupt Enable"]
pub type EFIE_R = crate::BitReader;
#[doc = "Field `EFIE` writer - End of Frame Interrupt Enable"]
pub type EFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVRIE` reader - FIFO Overrun interrupt Enable"]
pub type OVRIE_R = crate::BitReader;
#[doc = "Field `OVRIE` writer - FIFO Overrun interrupt Enable"]
pub type OVRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESEIE` reader - Embedded Synchronous Error Interrupt Enable"]
pub type ESEIE_R = crate::BitReader;
#[doc = "Field `ESEIE` writer - Embedded Synchronous Error Interrupt Enable"]
pub type ESEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VSIE` reader - Vsync Interrupt Enable"]
pub type VSIE_R = crate::BitReader;
#[doc = "Field `VSIE` writer - Vsync Interrupt Enable"]
pub type VSIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ELIE` reader - End of line interrupt Enable"]
pub type ELIE_R = crate::BitReader;
#[doc = "Field `ELIE` writer - End of line interrupt Enable"]
pub type ELIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - End of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn efie(&self) -> EFIE_R {
        EFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Overrun interrupt Enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded Synchronous Error Interrupt Enable"]
    #[inline(always)]
    pub fn eseie(&self) -> ESEIE_R {
        ESEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vsync Interrupt Enable"]
    #[inline(always)]
    pub fn vsie(&self) -> VSIE_R {
        VSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of line interrupt Enable"]
    #[inline(always)]
    pub fn elie(&self) -> ELIE_R {
        ELIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn efie(&mut self) -> EFIE_W<INTEN_SPEC, 0> {
        EFIE_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Overrun interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<INTEN_SPEC, 1> {
        OVRIE_W::new(self)
    }
    #[doc = "Bit 2 - Embedded Synchronous Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eseie(&mut self) -> ESEIE_W<INTEN_SPEC, 2> {
        ESEIE_W::new(self)
    }
    #[doc = "Bit 3 - Vsync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vsie(&mut self) -> VSIE_W<INTEN_SPEC, 3> {
        VSIE_W::new(self)
    }
    #[doc = "Bit 4 - End of line interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn elie(&mut self) -> ELIE_W<INTEN_SPEC, 4> {
        ELIE_W::new(self)
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
#[doc = "DCI inrerrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
