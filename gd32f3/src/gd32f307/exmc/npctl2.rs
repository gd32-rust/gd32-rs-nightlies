#[doc = "Register `NPCTL2` reader"]
pub type R = crate::R<NPCTL2_SPEC>;
#[doc = "Register `NPCTL2` writer"]
pub type W = crate::W<NPCTL2_SPEC>;
#[doc = "Field `NDWTEN` reader - Wait feature enable"]
pub type NDWTEN_R = crate::BitReader;
#[doc = "Field `NDWTEN` writer - Wait feature enable"]
pub type NDWTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NDBKEN` reader - NAND bank enable"]
pub type NDBKEN_R = crate::BitReader;
#[doc = "Field `NDBKEN` writer - NAND bank enable"]
pub type NDBKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NDTP` reader - NAND bank memory type"]
pub type NDTP_R = crate::BitReader;
#[doc = "Field `NDTP` writer - NAND bank memory type"]
pub type NDTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NDW` reader - NAND bank memory data bus width"]
pub type NDW_R = crate::FieldReader;
#[doc = "Field `NDW` writer - NAND bank memory data bus width"]
pub type NDW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ECCEN` reader - ECC enable"]
pub type ECCEN_R = crate::BitReader;
#[doc = "Field `ECCEN` writer - ECC enable"]
pub type ECCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTR` reader - CLE to RE delay"]
pub type CTR_R = crate::FieldReader;
#[doc = "Field `CTR` writer - CLE to RE delay"]
pub type CTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ATR` reader - ALE to RE delay"]
pub type ATR_R = crate::FieldReader;
#[doc = "Field `ATR` writer - ALE to RE delay"]
pub type ATR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ECCSZ` reader - ECC size"]
pub type ECCSZ_R = crate::FieldReader;
#[doc = "Field `ECCSZ` writer - ECC size"]
pub type ECCSZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    pub fn ndwten(&self) -> NDWTEN_R {
        NDWTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NAND bank enable"]
    #[inline(always)]
    pub fn ndbken(&self) -> NDBKEN_R {
        NDBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NAND bank memory type"]
    #[inline(always)]
    pub fn ndtp(&self) -> NDTP_R {
        NDTP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - NAND bank memory data bus width"]
    #[inline(always)]
    pub fn ndw(&self) -> NDW_R {
        NDW_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ECC enable"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    pub fn atr(&self) -> ATR_R {
        ATR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - ECC size"]
    #[inline(always)]
    pub fn eccsz(&self) -> ECCSZ_R {
        ECCSZ_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn ndwten(&mut self) -> NDWTEN_W<NPCTL2_SPEC, 1> {
        NDWTEN_W::new(self)
    }
    #[doc = "Bit 2 - NAND bank enable"]
    #[inline(always)]
    #[must_use]
    pub fn ndbken(&mut self) -> NDBKEN_W<NPCTL2_SPEC, 2> {
        NDBKEN_W::new(self)
    }
    #[doc = "Bit 3 - NAND bank memory type"]
    #[inline(always)]
    #[must_use]
    pub fn ndtp(&mut self) -> NDTP_W<NPCTL2_SPEC, 3> {
        NDTP_W::new(self)
    }
    #[doc = "Bits 4:5 - NAND bank memory data bus width"]
    #[inline(always)]
    #[must_use]
    pub fn ndw(&mut self) -> NDW_W<NPCTL2_SPEC, 4> {
        NDW_W::new(self)
    }
    #[doc = "Bit 6 - ECC enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<NPCTL2_SPEC, 6> {
        ECCEN_W::new(self)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<NPCTL2_SPEC, 9> {
        CTR_W::new(self)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    #[must_use]
    pub fn atr(&mut self) -> ATR_W<NPCTL2_SPEC, 13> {
        ATR_W::new(self)
    }
    #[doc = "Bits 17:19 - ECC size"]
    #[inline(always)]
    #[must_use]
    pub fn eccsz(&mut self) -> ECCSZ_W<NPCTL2_SPEC, 17> {
        ECCSZ_W::new(self)
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
#[doc = "NAND flash/PC card control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NPCTL2_SPEC;
impl crate::RegisterSpec for NPCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`npctl2::R`](R) reader structure"]
impl crate::Readable for NPCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`npctl2::W`](W) writer structure"]
impl crate::Writable for NPCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NPCTL2 to value 0x18"]
impl crate::Resettable for NPCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}
