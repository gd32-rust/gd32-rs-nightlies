#[doc = "Register `L1FLLEN` reader"]
pub type R = crate::R<L1FLLEN_SPEC>;
#[doc = "Register `L1FLLEN` writer"]
pub type W = crate::W<L1FLLEN_SPEC>;
#[doc = "Field `FLL` reader - Frame Line Length"]
pub type FLL_R = crate::FieldReader<u16>;
#[doc = "Field `FLL` writer - Frame Line Length"]
pub type FLL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `STDOFF` reader - Frame Buffer Stride Offset"]
pub type STDOFF_R = crate::FieldReader<u16>;
#[doc = "Field `STDOFF` writer - Frame Buffer Stride Offset"]
pub type STDOFF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - Frame Line Length"]
    #[inline(always)]
    pub fn fll(&self) -> FLL_R {
        FLL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Frame Buffer Stride Offset"]
    #[inline(always)]
    pub fn stdoff(&self) -> STDOFF_R {
        STDOFF_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Line Length"]
    #[inline(always)]
    #[must_use]
    pub fn fll(&mut self) -> FLL_W<L1FLLEN_SPEC, 0> {
        FLL_W::new(self)
    }
    #[doc = "Bits 16:29 - Frame Buffer Stride Offset"]
    #[inline(always)]
    #[must_use]
    pub fn stdoff(&mut self) -> STDOFF_W<L1FLLEN_SPEC, 16> {
        STDOFF_W::new(self)
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
#[doc = "Layer 1 frame line length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1fllen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1fllen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1FLLEN_SPEC;
impl crate::RegisterSpec for L1FLLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1fllen::R`](R) reader structure"]
impl crate::Readable for L1FLLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1fllen::W`](W) writer structure"]
impl crate::Writable for L1FLLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1FLLEN to value 0"]
impl crate::Resettable for L1FLLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
