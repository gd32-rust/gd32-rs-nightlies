#[doc = "Register `OUT1_DO` reader"]
pub type R = crate::R<OUT1_DO_SPEC>;
#[doc = "Field `OUT1_DO` reader - DAC_OUT1 data output"]
pub type OUT1_DO_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC_OUT1 data output"]
    #[inline(always)]
    pub fn out1_do(&self) -> OUT1_DO_R {
        OUT1_DO_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC_OUT1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1_do::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT1_DO_SPEC;
impl crate::RegisterSpec for OUT1_DO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out1_do::R`](R) reader structure"]
impl crate::Readable for OUT1_DO_SPEC {}
#[doc = "`reset()` method sets OUT1_DO to value 0"]
impl crate::Resettable for OUT1_DO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
