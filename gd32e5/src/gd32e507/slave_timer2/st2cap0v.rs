#[doc = "Register `ST2CAP0V` reader"]
pub type R = crate::R<ST2CAP0V_SPEC>;
#[doc = "Register `ST2CAP0V` writer"]
pub type W = crate::W<ST2CAP0V_SPEC>;
#[doc = "Field `CAP0VAL` reader - Capture 0 value"]
pub type CAP0VAL_R = crate::FieldReader<u16>;
#[doc = "Field `CAP0VAL` writer - Capture 0 value"]
pub type CAP0VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture 0 value"]
    #[inline(always)]
    pub fn cap0val(&self) -> CAP0VAL_R {
        CAP0VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture 0 value"]
    #[inline(always)]
    #[must_use]
    pub fn cap0val(&mut self) -> CAP0VAL_W<ST2CAP0V_SPEC, 0> {
        CAP0VAL_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx capture 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cap0v::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cap0v::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST2CAP0V_SPEC;
impl crate::RegisterSpec for ST2CAP0V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2cap0v::R`](R) reader structure"]
impl crate::Readable for ST2CAP0V_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st2cap0v::W`](W) writer structure"]
impl crate::Writable for ST2CAP0V_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST2CAP0V to value 0"]
impl crate::Resettable for ST2CAP0V_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
