#[doc = "Register `IV0L` reader"]
pub type R = crate::R<IV0L_SPEC>;
#[doc = "Register `IV0L` writer"]
pub type W = crate::W<IV0L_SPEC>;
#[doc = "Field `IV0L` reader - The initialization vector for DES,TDES,AES"]
pub type IV0L_R = crate::FieldReader<u32>;
#[doc = "Field `IV0L` writer - The initialization vector for DES,TDES,AES"]
pub type IV0L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv0l(&self) -> IV0L_R {
        IV0L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    #[must_use]
    pub fn iv0l(&mut self) -> IV0L_W<IV0L_SPEC, 0> {
        IV0L_W::new(self)
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
#[doc = "CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv0l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv0l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IV0L_SPEC;
impl crate::RegisterSpec for IV0L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv0l::R`](R) reader structure"]
impl crate::Readable for IV0L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iv0l::W`](W) writer structure"]
impl crate::Writable for IV0L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IV0L to value 0"]
impl crate::Resettable for IV0L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
