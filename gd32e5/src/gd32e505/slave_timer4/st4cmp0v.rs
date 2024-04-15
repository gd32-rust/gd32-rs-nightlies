#[doc = "Register `ST4CMP0V` reader"]
pub type R = crate::R<St4cmp0vSpec>;
#[doc = "Register `ST4CMP0V` writer"]
pub type W = crate::W<St4cmp0vSpec>;
#[doc = "Field `CMP0VAL` reader - Compare 0 value"]
pub type Cmp0valR = crate::FieldReader<u16>;
#[doc = "Field `CMP0VAL` writer - Compare 0 value"]
pub type Cmp0valW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 0 value"]
    #[inline(always)]
    pub fn cmp0val(&self) -> Cmp0valR {
        Cmp0valR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 0 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0val(&mut self) -> Cmp0valW<St4cmp0vSpec> {
        Cmp0valW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMER4 compare 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cmp0v::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cmp0v::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St4cmp0vSpec;
impl crate::RegisterSpec for St4cmp0vSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st4cmp0v::R`](R) reader structure"]
impl crate::Readable for St4cmp0vSpec {}
#[doc = "`write(|w| ..)` method takes [`st4cmp0v::W`](W) writer structure"]
impl crate::Writable for St4cmp0vSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST4CMP0V to value 0"]
impl crate::Resettable for St4cmp0vSpec {
    const RESET_VALUE: u32 = 0;
}
