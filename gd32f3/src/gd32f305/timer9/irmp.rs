#[doc = "Register `IRMP` reader"]
pub type R = crate::R<IRMP_SPEC>;
#[doc = "Register `IRMP` writer"]
pub type W = crate::W<IRMP_SPEC>;
#[doc = "Field `ITI1_RMP` reader - Internal trigger input1 remap"]
pub type ITI1_RMP_R = crate::FieldReader;
#[doc = "Field `ITI1_RMP` writer - Internal trigger input1 remap"]
pub type ITI1_RMP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 10:11 - Internal trigger input1 remap"]
    #[inline(always)]
    pub fn iti1_rmp(&self) -> ITI1_RMP_R {
        ITI1_RMP_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 10:11 - Internal trigger input1 remap"]
    #[inline(always)]
    #[must_use]
    pub fn iti1_rmp(&mut self) -> ITI1_RMP_W<IRMP_SPEC, 10> {
        ITI1_RMP_W::new(self)
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
#[doc = "channel input remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRMP_SPEC;
impl crate::RegisterSpec for IRMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irmp::R`](R) reader structure"]
impl crate::Readable for IRMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irmp::W`](W) writer structure"]
impl crate::Writable for IRMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRMP to value 0"]
impl crate::Resettable for IRMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
