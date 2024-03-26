#[doc = "Register `DIEP2LEN` reader"]
pub type R = crate::R<Diep2lenSpec>;
#[doc = "Register `DIEP2LEN` writer"]
pub type W = crate::W<Diep2lenSpec>;
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TlenR = crate::FieldReader<u32>;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TlenW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PCNT` reader - Packet count"]
pub type PcntR = crate::FieldReader<u16>;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&self) -> TlenR {
        TlenR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PcntR {
        PcntR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    #[must_use]
    pub fn tlen(&mut self) -> TlenW<Diep2lenSpec> {
        TlenW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt(&mut self) -> PcntW<Diep2lenSpec> {
        PcntW::new(self, 19)
    }
}
#[doc = "device IN endpoint-2 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep2lenSpec;
impl crate::RegisterSpec for Diep2lenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep2len::R`](R) reader structure"]
impl crate::Readable for Diep2lenSpec {}
#[doc = "`write(|w| ..)` method takes [`diep2len::W`](W) writer structure"]
impl crate::Writable for Diep2lenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEP2LEN to value 0"]
impl crate::Resettable for Diep2lenSpec {
    const RESET_VALUE: u32 = 0;
}
