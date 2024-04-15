#[doc = "Register `ST2CMP2V` reader"]
pub type R = crate::R<St2cmp2vSpec>;
#[doc = "Register `ST2CMP2V` writer"]
pub type W = crate::W<St2cmp2vSpec>;
#[doc = "Field `CMP2VAL` reader - Compare 2 value"]
pub type Cmp2valR = crate::FieldReader<u16>;
#[doc = "Field `CMP2VAL` writer - Compare 2 value"]
pub type Cmp2valW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 2 value"]
    #[inline(always)]
    pub fn cmp2val(&self) -> Cmp2valR {
        Cmp2valR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 2 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2val(&mut self) -> Cmp2valW<St2cmp2vSpec> {
        Cmp2valW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMERx compare 2 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cmp2v::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cmp2v::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St2cmp2vSpec;
impl crate::RegisterSpec for St2cmp2vSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2cmp2v::R`](R) reader structure"]
impl crate::Readable for St2cmp2vSpec {}
#[doc = "`write(|w| ..)` method takes [`st2cmp2v::W`](W) writer structure"]
impl crate::Writable for St2cmp2vSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST2CMP2V to value 0"]
impl crate::Resettable for St2cmp2vSpec {
    const RESET_VALUE: u32 = 0;
}
