#[doc = "Register `L0PPF` reader"]
pub type R = crate::R<L0PPF_SPEC>;
#[doc = "Register `L0PPF` writer"]
pub type W = crate::W<L0PPF_SPEC>;
#[doc = "Field `PPF` reader - Packeted Pixel Format"]
pub type PPF_R = crate::FieldReader;
#[doc = "Field `PPF` writer - Packeted Pixel Format"]
pub type PPF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Packeted Pixel Format"]
    #[inline(always)]
    pub fn ppf(&self) -> PPF_R {
        PPF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Packeted Pixel Format"]
    #[inline(always)]
    #[must_use]
    pub fn ppf(&mut self) -> PPF_W<L0PPF_SPEC, 0> {
        PPF_W::new(self)
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
#[doc = "Layer 0 packeted pixel format register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0ppf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0ppf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0PPF_SPEC;
impl crate::RegisterSpec for L0PPF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0ppf::R`](R) reader structure"]
impl crate::Readable for L0PPF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l0ppf::W`](W) writer structure"]
impl crate::Writable for L0PPF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L0PPF to value 0"]
impl crate::Resettable for L0PPF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
