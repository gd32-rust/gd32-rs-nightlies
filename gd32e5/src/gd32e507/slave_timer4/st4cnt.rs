#[doc = "Register `ST4CNT` reader"]
pub type R = crate::R<St4cntSpec>;
#[doc = "Register `ST4CNT` writer"]
pub type W = crate::W<St4cntSpec>;
#[doc = "Field `CNT` reader - The current counter value"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - The current counter value"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The current counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The current counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<St4cntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMERx counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St4cntSpec;
impl crate::RegisterSpec for St4cntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st4cnt::R`](R) reader structure"]
impl crate::Readable for St4cntSpec {}
#[doc = "`write(|w| ..)` method takes [`st4cnt::W`](W) writer structure"]
impl crate::Writable for St4cntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST4CNT to value 0"]
impl crate::Resettable for St4cntSpec {
    const RESET_VALUE: u32 = 0;
}
