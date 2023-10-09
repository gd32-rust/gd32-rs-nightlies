#[doc = "Register `ST1CMP3V` reader"]
pub type R = crate::R<ST1CMP3V_SPEC>;
#[doc = "Register `ST1CMP3V` writer"]
pub type W = crate::W<ST1CMP3V_SPEC>;
#[doc = "Field `CMP3VAL` reader - Compare 3 value"]
pub type CMP3VAL_R = crate::FieldReader<u16>;
#[doc = "Field `CMP3VAL` writer - Compare 3 value"]
pub type CMP3VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 3 value"]
    #[inline(always)]
    pub fn cmp3val(&self) -> CMP3VAL_R {
        CMP3VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 3 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3val(&mut self) -> CMP3VAL_W<ST1CMP3V_SPEC, 0> {
        CMP3VAL_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMER1 compare 3 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cmp3v::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cmp3v::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST1CMP3V_SPEC;
impl crate::RegisterSpec for ST1CMP3V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1cmp3v::R`](R) reader structure"]
impl crate::Readable for ST1CMP3V_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st1cmp3v::W`](W) writer structure"]
impl crate::Writable for ST1CMP3V_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1CMP3V to value 0"]
impl crate::Resettable for ST1CMP3V_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
