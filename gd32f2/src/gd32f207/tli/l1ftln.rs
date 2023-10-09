#[doc = "Register `L1FTLN` reader"]
pub type R = crate::R<L1FTLN_SPEC>;
#[doc = "Register `L1FTLN` writer"]
pub type W = crate::W<L1FTLN_SPEC>;
#[doc = "Field `FTLN` reader - Frame Total Line Number"]
pub type FTLN_R = crate::FieldReader<u16>;
#[doc = "Field `FTLN` writer - Frame Total Line Number"]
pub type FTLN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - Frame Total Line Number"]
    #[inline(always)]
    pub fn ftln(&self) -> FTLN_R {
        FTLN_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame Total Line Number"]
    #[inline(always)]
    #[must_use]
    pub fn ftln(&mut self) -> FTLN_W<L1FTLN_SPEC, 0> {
        FTLN_W::new(self)
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
#[doc = "Layer 1 frame total line number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ftln::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ftln::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1FTLN_SPEC;
impl crate::RegisterSpec for L1FTLN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1ftln::R`](R) reader structure"]
impl crate::Readable for L1FTLN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1ftln::W`](W) writer structure"]
impl crate::Writable for L1FTLN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1FTLN to value 0"]
impl crate::Resettable for L1FTLN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
