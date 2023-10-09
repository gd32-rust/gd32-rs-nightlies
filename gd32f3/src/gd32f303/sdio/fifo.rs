#[doc = "Register `FIFO` reader"]
pub type R = crate::R<FIFO_SPEC>;
#[doc = "Register `FIFO` writer"]
pub type W = crate::W<FIFO_SPEC>;
#[doc = "Field `FIFODT` reader - Receive FIFO data or transmit FIFO data"]
pub type FIFODT_R = crate::FieldReader<u32>;
#[doc = "Field `FIFODT` writer - Receive FIFO data or transmit FIFO data"]
pub type FIFODT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive FIFO data or transmit FIFO data"]
    #[inline(always)]
    pub fn fifodt(&self) -> FIFODT_R {
        FIFODT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive FIFO data or transmit FIFO data"]
    #[inline(always)]
    #[must_use]
    pub fn fifodt(&mut self) -> FIFODT_W<FIFO_SPEC, 0> {
        FIFODT_W::new(self)
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
#[doc = "FIFO data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo::R`](R) reader structure"]
impl crate::Readable for FIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo::W`](W) writer structure"]
impl crate::Writable for FIFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO to value 0"]
impl crate::Resettable for FIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
