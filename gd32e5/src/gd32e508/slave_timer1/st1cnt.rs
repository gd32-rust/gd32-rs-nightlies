#[doc = "Register `ST1CNT` reader"]
pub type R = crate::R<ST1CNT_SPEC>;
#[doc = "Register `ST1CNT` writer"]
pub type W = crate::W<ST1CNT_SPEC>;
#[doc = "Field `CNT` reader - The current counter value"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - The current counter value"]
pub type CNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - The current counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The current counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<ST1CNT_SPEC, 0> {
        CNT_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMER1 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST1CNT_SPEC;
impl crate::RegisterSpec for ST1CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1cnt::R`](R) reader structure"]
impl crate::Readable for ST1CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st1cnt::W`](W) writer structure"]
impl crate::Writable for ST1CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1CNT to value 0"]
impl crate::Resettable for ST1CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}