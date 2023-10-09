#[doc = "Register `ST4CMP2V` reader"]
pub type R = crate::R<ST4CMP2V_SPEC>;
#[doc = "Register `ST4CMP2V` writer"]
pub type W = crate::W<ST4CMP2V_SPEC>;
#[doc = "Field `CMP2VAL` reader - Compare 2 value"]
pub type CMP2VAL_R = crate::FieldReader<u16>;
#[doc = "Field `CMP2VAL` writer - Compare 2 value"]
pub type CMP2VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 2 value"]
    #[inline(always)]
    pub fn cmp2val(&self) -> CMP2VAL_R {
        CMP2VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 2 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2val(&mut self) -> CMP2VAL_W<ST4CMP2V_SPEC, 0> {
        CMP2VAL_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx compare 2 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cmp2v::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cmp2v::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST4CMP2V_SPEC;
impl crate::RegisterSpec for ST4CMP2V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st4cmp2v::R`](R) reader structure"]
impl crate::Readable for ST4CMP2V_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st4cmp2v::W`](W) writer structure"]
impl crate::Writable for ST4CMP2V_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST4CMP2V to value 0"]
impl crate::Resettable for ST4CMP2V_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
