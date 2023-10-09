#[doc = "Register `STAT1` reader"]
pub type R = crate::R<STAT1_SPEC>;
#[doc = "Register `STAT1` writer"]
pub type W = crate::W<STAT1_SPEC>;
#[doc = "Field `RTF` writer - Receiver timeout flag"]
pub type RTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EBF` writer - End of block flag"]
pub type EBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BSY` reader - Busy flag"]
pub type BSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 16 - Busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Receiver timeout flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtf(&mut self) -> RTF_W<STAT1_SPEC, 11> {
        RTF_W::new(self)
    }
    #[doc = "Bit 12 - End of block flag"]
    #[inline(always)]
    #[must_use]
    pub fn ebf(&mut self) -> EBF_W<STAT1_SPEC, 12> {
        EBF_W::new(self)
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
#[doc = "Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT1_SPEC;
impl crate::RegisterSpec for STAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat1::R`](R) reader structure"]
impl crate::Readable for STAT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat1::W`](W) writer structure"]
impl crate::Writable for STAT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT1 to value 0xc0"]
impl crate::Resettable for STAT1_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
