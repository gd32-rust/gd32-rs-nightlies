#[doc = "Register `ST1CMP0CP` reader"]
pub type R = crate::R<ST1CMP0CP_SPEC>;
#[doc = "Register `ST1CMP0CP` writer"]
pub type W = crate::W<ST1CMP0CP_SPEC>;
#[doc = "Field `CMP0VAL` reader - Compare 0 value"]
pub type CMP0VAL_R = crate::FieldReader<u16>;
#[doc = "Field `CMP0VAL` writer - Compare 0 value"]
pub type CMP0VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `CREP` reader - Counter repetition value"]
pub type CREP_R = crate::FieldReader;
#[doc = "Field `CREP` writer - Counter repetition value"]
pub type CREP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:15 - Compare 0 value"]
    #[inline(always)]
    pub fn cmp0val(&self) -> CMP0VAL_R {
        CMP0VAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Counter repetition value"]
    #[inline(always)]
    pub fn crep(&self) -> CREP_R {
        CREP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 0 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0val(&mut self) -> CMP0VAL_W<ST1CMP0CP_SPEC, 0> {
        CMP0VAL_W::new(self)
    }
    #[doc = "Bits 16:23 - Counter repetition value"]
    #[inline(always)]
    #[must_use]
    pub fn crep(&mut self) -> CREP_W<ST1CMP0CP_SPEC, 16> {
        CREP_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMER1 compare 0 composite register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cmp0cp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cmp0cp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST1CMP0CP_SPEC;
impl crate::RegisterSpec for ST1CMP0CP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1cmp0cp::R`](R) reader structure"]
impl crate::Readable for ST1CMP0CP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st1cmp0cp::W`](W) writer structure"]
impl crate::Writable for ST1CMP0CP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1CMP0CP to value 0"]
impl crate::Resettable for ST1CMP0CP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
