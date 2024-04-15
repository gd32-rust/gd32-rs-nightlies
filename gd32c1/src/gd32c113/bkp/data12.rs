#[doc = "Register `DATA12` reader"]
pub type R = crate::R<Data12Spec>;
#[doc = "Register `DATA12` writer"]
pub type W = crate::W<Data12Spec>;
#[doc = "Field `DATA` reader - Backup data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Backup data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Data12Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Backup data register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data12Spec;
impl crate::RegisterSpec for Data12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data12::R`](R) reader structure"]
impl crate::Readable for Data12Spec {}
#[doc = "`write(|w| ..)` method takes [`data12::W`](W) writer structure"]
impl crate::Writable for Data12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA12 to value 0"]
impl crate::Resettable for Data12Spec {
    const RESET_VALUE: u32 = 0;
}
