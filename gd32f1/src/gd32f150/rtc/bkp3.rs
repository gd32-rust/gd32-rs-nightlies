#[doc = "Register `BKP3` reader"]
pub type R = crate::R<BKP3_SPEC>;
#[doc = "Register `BKP3` writer"]
pub type W = crate::W<BKP3_SPEC>;
#[doc = "Field `DATA` reader - Backup domain registers"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Backup domain registers"]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Backup domain registers"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Backup domain registers"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<BKP3_SPEC, 0> {
        DATA_W::new(self)
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
#[doc = "backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKP3_SPEC;
impl crate::RegisterSpec for BKP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp3::R`](R) reader structure"]
impl crate::Readable for BKP3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bkp3::W`](W) writer structure"]
impl crate::Writable for BKP3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKP3 to value 0"]
impl crate::Resettable for BKP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
