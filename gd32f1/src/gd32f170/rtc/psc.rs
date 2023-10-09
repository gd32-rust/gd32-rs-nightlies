#[doc = "Register `PSC` reader"]
pub type R = crate::R<PSC_SPEC>;
#[doc = "Register `PSC` writer"]
pub type W = crate::W<PSC_SPEC>;
#[doc = "Field `FACTOR_S` reader - Synchronous prescaler factor"]
pub type FACTOR_S_R = crate::FieldReader<u16>;
#[doc = "Field `FACTOR_S` writer - Synchronous prescaler factor"]
pub type FACTOR_S_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `FACTOR_A` reader - Asynchronous prescaler factor"]
pub type FACTOR_A_R = crate::FieldReader;
#[doc = "Field `FACTOR_A` writer - Asynchronous prescaler factor"]
pub type FACTOR_A_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn factor_s(&self) -> FACTOR_S_R {
        FACTOR_S_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn factor_a(&self) -> FACTOR_A_R {
        FACTOR_A_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    #[must_use]
    pub fn factor_s(&mut self) -> FACTOR_S_W<PSC_SPEC, 0> {
        FACTOR_S_W::new(self)
    }
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    #[must_use]
    pub fn factor_a(&mut self) -> FACTOR_A_W<PSC_SPEC, 16> {
        FACTOR_A_W::new(self)
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
#[doc = "Time prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSC_SPEC;
impl crate::RegisterSpec for PSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psc::R`](R) reader structure"]
impl crate::Readable for PSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psc::W`](W) writer structure"]
impl crate::Writable for PSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSC to value 0x007f_00ff"]
impl crate::Resettable for PSC_SPEC {
    const RESET_VALUE: Self::Ux = 0x007f_00ff;
}
