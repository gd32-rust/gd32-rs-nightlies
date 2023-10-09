#[doc = "Register `DACC_R8DH` reader"]
pub type R = crate::R<DACC_R8DH_SPEC>;
#[doc = "Register `DACC_R8DH` writer"]
pub type W = crate::W<DACC_R8DH_SPEC>;
#[doc = "Field `OUT0_DH` reader - DAC_OUT0 8-bit right-aligned data"]
pub type OUT0_DH_R = crate::FieldReader;
#[doc = "Field `OUT0_DH` writer - DAC_OUT0 8-bit right-aligned data"]
pub type OUT0_DH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `OUT1_DH` reader - DAC_OUT1 8-bit right-aligned data"]
pub type OUT1_DH_R = crate::FieldReader;
#[doc = "Field `OUT1_DH` writer - DAC_OUT1 8-bit right-aligned data"]
pub type OUT1_DH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DAC_OUT0 8-bit right-aligned data"]
    #[inline(always)]
    pub fn out0_dh(&self) -> OUT0_DH_R {
        OUT0_DH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC_OUT1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn out1_dh(&self) -> OUT1_DH_R {
        OUT1_DH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC_OUT0 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn out0_dh(&mut self) -> OUT0_DH_W<DACC_R8DH_SPEC, 0> {
        OUT0_DH_W::new(self)
    }
    #[doc = "Bits 8:15 - DAC_OUT1 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn out1_dh(&mut self) -> OUT1_DH_W<DACC_R8DH_SPEC, 8> {
        OUT1_DH_W::new(self)
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
#[doc = "DAC concurrent mode 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_r8dh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_r8dh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DACC_R8DH_SPEC;
impl crate::RegisterSpec for DACC_R8DH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacc_r8dh::R`](R) reader structure"]
impl crate::Readable for DACC_R8DH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dacc_r8dh::W`](W) writer structure"]
impl crate::Writable for DACC_R8DH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACC_R8DH to value 0"]
impl crate::Resettable for DACC_R8DH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
