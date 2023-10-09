#[doc = "Register `ST0CAP1V` reader"]
pub type R = crate::R<ST0CAP1V_SPEC>;
#[doc = "Register `ST0CAP1V` writer"]
pub type W = crate::W<ST0CAP1V_SPEC>;
#[doc = "Field `CAP1VAL` reader - Capture 1 value"]
pub type CAP1VAL_R = crate::FieldReader<u16>;
#[doc = "Field `CAP1VAL` writer - Capture 1 value"]
pub type CAP1VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture 1 value"]
    #[inline(always)]
    pub fn cap1val(&self) -> CAP1VAL_R {
        CAP1VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture 1 value"]
    #[inline(always)]
    #[must_use]
    pub fn cap1val(&mut self) -> CAP1VAL_W<ST0CAP1V_SPEC, 0> {
        CAP1VAL_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMER0 capture 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cap1v::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cap1v::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST0CAP1V_SPEC;
impl crate::RegisterSpec for ST0CAP1V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0cap1v::R`](R) reader structure"]
impl crate::Readable for ST0CAP1V_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st0cap1v::W`](W) writer structure"]
impl crate::Writable for ST0CAP1V_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST0CAP1V to value 0"]
impl crate::Resettable for ST0CAP1V_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
