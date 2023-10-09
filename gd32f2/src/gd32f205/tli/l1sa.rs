#[doc = "Register `L1SA` reader"]
pub type R = crate::R<L1SA_SPEC>;
#[doc = "Register `L1SA` writer"]
pub type W = crate::W<L1SA_SPEC>;
#[doc = "Field `SA` reader - Specified alpha"]
pub type SA_R = crate::FieldReader;
#[doc = "Field `SA` writer - Specified alpha"]
pub type SA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Specified alpha"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specified alpha"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<L1SA_SPEC, 0> {
        SA_W::new(self)
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
#[doc = "Layer 1 specified alpha register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1sa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1sa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1SA_SPEC;
impl crate::RegisterSpec for L1SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1sa::R`](R) reader structure"]
impl crate::Readable for L1SA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1sa::W`](W) writer structure"]
impl crate::Writable for L1SA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1SA to value 0xff"]
impl crate::Resettable for L1SA_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
