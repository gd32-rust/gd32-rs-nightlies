#[doc = "Register `DATA20` reader"]
pub type R = crate::R<DATA20_SPEC>;
#[doc = "Register `DATA20` writer"]
pub type W = crate::W<DATA20_SPEC>;
#[doc = "Field `DATA` reader - Backup data"]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Backup data"]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DATA20_SPEC, 0> {
        DATA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Backup data register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA20_SPEC;
impl crate::RegisterSpec for DATA20_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`data20::R`](R) reader structure"]
impl crate::Readable for DATA20_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data20::W`](W) writer structure"]
impl crate::Writable for DATA20_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA20 to value 0"]
impl crate::Resettable for DATA20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
