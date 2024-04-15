#[doc = "Register `CH2MADDR` reader"]
pub type R = crate::R<Ch2maddrSpec>;
#[doc = "Register `CH2MADDR` writer"]
pub type W = crate::W<Ch2maddrSpec>;
#[doc = "Field `MADDR` reader - Memory base address"]
pub type MaddrR = crate::FieldReader<u32>;
#[doc = "Field `MADDR` writer - Memory base address"]
pub type MaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory base address"]
    #[inline(always)]
    pub fn maddr(&self) -> MaddrR {
        MaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory base address"]
    #[inline(always)]
    #[must_use]
    pub fn maddr(&mut self) -> MaddrW<Ch2maddrSpec> {
        MaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 2 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2maddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2maddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2maddrSpec;
impl crate::RegisterSpec for Ch2maddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2maddr::R`](R) reader structure"]
impl crate::Readable for Ch2maddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2maddr::W`](W) writer structure"]
impl crate::Writable for Ch2maddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2MADDR to value 0"]
impl crate::Resettable for Ch2maddrSpec {
    const RESET_VALUE: u32 = 0;
}
