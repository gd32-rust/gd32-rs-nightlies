#[doc = "Register `DAC1_R8DH` reader"]
pub type R = crate::R<Dac1R8dhSpec>;
#[doc = "Register `DAC1_R8DH` writer"]
pub type W = crate::W<Dac1R8dhSpec>;
#[doc = "Field `DAC1_DH` reader - DAC1 8-bit right-aligned data"]
pub type Dac1DhR = crate::FieldReader;
#[doc = "Field `DAC1_DH` writer - DAC1 8-bit right-aligned data"]
pub type Dac1DhW<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&self) -> Dac1DhR {
        Dac1DhR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac1_dh(&mut self) -> Dac1DhW<Dac1R8dhSpec> {
        Dac1DhW::new(self, 0)
    }
}
#[doc = "DAC1 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1_r8dh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1_r8dh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac1R8dhSpec;
impl crate::RegisterSpec for Dac1R8dhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1_r8dh::R`](R) reader structure"]
impl crate::Readable for Dac1R8dhSpec {}
#[doc = "`write(|w| ..)` method takes [`dac1_r8dh::W`](W) writer structure"]
impl crate::Writable for Dac1R8dhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC1_R8DH to value 0"]
impl crate::Resettable for Dac1R8dhSpec {
    const RESET_VALUE: u32 = 0;
}
