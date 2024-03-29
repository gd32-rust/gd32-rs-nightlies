#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `IINTEN` reader - In FIFO interrupt enable"]
pub type IintenR = crate::BitReader;
#[doc = "Field `IINTEN` writer - In FIFO interrupt enable"]
pub type IintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OINTEN` reader - Out FIFO interrupt enable"]
pub type OintenR = crate::BitReader;
#[doc = "Field `OINTEN` writer - Out FIFO interrupt enable"]
pub type OintenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - In FIFO interrupt enable"]
    #[inline(always)]
    pub fn iinten(&self) -> IintenR {
        IintenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Out FIFO interrupt enable"]
    #[inline(always)]
    pub fn ointen(&self) -> OintenR {
        OintenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - In FIFO interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn iinten(&mut self) -> IintenW<IntenSpec> {
        IintenW::new(self, 0)
    }
    #[doc = "Bit 1 - Out FIFO interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ointen(&mut self) -> OintenW<IntenSpec> {
        OintenW::new(self, 1)
    }
}
#[doc = "CAU interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
