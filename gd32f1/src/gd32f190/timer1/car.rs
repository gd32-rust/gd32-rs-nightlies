#[doc = "Register `CAR` reader"]
pub type R = crate::R<CAR_SPEC>;
#[doc = "Register `CAR` writer"]
pub type W = crate::W<CAR_SPEC>;
#[doc = "Field `CAR` reader - Low Auto-reload value"]
pub type CAR_R = crate::FieldReader<u32>;
#[doc = "Field `CAR` writer - Low Auto-reload value"]
pub type CAR_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Low Auto-reload value"]
    #[inline(always)]
    pub fn car(&self) -> CAR_R {
        CAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low Auto-reload value"]
    #[inline(always)]
    #[must_use]
    pub fn car(&mut self) -> CAR_W<CAR_SPEC, 0> {
        CAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`car::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`car::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAR_SPEC;
impl crate::RegisterSpec for CAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`car::R`](R) reader structure"]
impl crate::Readable for CAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`car::W`](W) writer structure"]
impl crate::Writable for CAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAR to value 0"]
impl crate::Resettable for CAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
