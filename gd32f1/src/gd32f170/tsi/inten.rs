#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `CTCFIE` reader - Charge-transfer complete flag Interrupt Enable"]
pub type CTCFIE_R = crate::BitReader;
#[doc = "Field `CTCFIE` writer - Charge-transfer complete flag Interrupt Enable"]
pub type CTCFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MNERRIE` reader - Max Cycle Number Error Interrupt Enable"]
pub type MNERRIE_R = crate::BitReader;
#[doc = "Field `MNERRIE` writer - Max Cycle Number Error Interrupt Enable"]
pub type MNERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Charge-transfer complete flag Interrupt Enable"]
    #[inline(always)]
    pub fn ctcfie(&self) -> CTCFIE_R {
        CTCFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max Cycle Number Error Interrupt Enable"]
    #[inline(always)]
    pub fn mnerrie(&self) -> MNERRIE_R {
        MNERRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Charge-transfer complete flag Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctcfie(&mut self) -> CTCFIE_W<INTEN_SPEC, 0> {
        CTCFIE_W::new(self)
    }
    #[doc = "Bit 1 - Max Cycle Number Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mnerrie(&mut self) -> MNERRIE_W<INTEN_SPEC, 1> {
        MNERRIE_W::new(self)
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
