#[doc = "Register `NPCTL1` reader"]
pub struct R(crate::R<NPCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NPCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NPCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NPCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NPCTL1` writer"]
pub struct W(crate::W<NPCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NPCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<NPCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NPCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCSZ` reader - ECC size"]
pub type ECCSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECCSZ` writer - ECC size"]
pub type ECCSZ_W<'a> = crate::FieldWriter<'a, u32, NPCTL1_SPEC, u8, u8, 3, 17>;
#[doc = "Field `ATR` reader - ALE to RE delay"]
pub type ATR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATR` writer - ALE to RE delay"]
pub type ATR_W<'a> = crate::FieldWriter<'a, u32, NPCTL1_SPEC, u8, u8, 4, 13>;
#[doc = "Field `CTR` reader - CLE to RE delay"]
pub type CTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTR` writer - CLE to RE delay"]
pub type CTR_W<'a> = crate::FieldWriter<'a, u32, NPCTL1_SPEC, u8, u8, 4, 9>;
#[doc = "Field `ECCEN` reader - ECC enable"]
pub type ECCEN_R = crate::BitReader<bool>;
#[doc = "Field `ECCEN` writer - ECC enable"]
pub type ECCEN_W<'a> = crate::BitWriter<'a, u32, NPCTL1_SPEC, bool, 6>;
#[doc = "Field `NDW` reader - NAND bank memory data bus width"]
pub type NDW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NDW` writer - NAND bank memory data bus width"]
pub type NDW_W<'a> = crate::FieldWriter<'a, u32, NPCTL1_SPEC, u8, u8, 2, 4>;
#[doc = "Field `NDTP` reader - NAND bank memory type"]
pub type NDTP_R = crate::BitReader<bool>;
#[doc = "Field `NDTP` writer - NAND bank memory type"]
pub type NDTP_W<'a> = crate::BitWriter<'a, u32, NPCTL1_SPEC, bool, 3>;
#[doc = "Field `NDBKEN` reader - NAND bank enable"]
pub type NDBKEN_R = crate::BitReader<bool>;
#[doc = "Field `NDBKEN` writer - NAND bank enable"]
pub type NDBKEN_W<'a> = crate::BitWriter<'a, u32, NPCTL1_SPEC, bool, 2>;
#[doc = "Field `NDWTEN` reader - Wait feature enable"]
pub type NDWTEN_R = crate::BitReader<bool>;
#[doc = "Field `NDWTEN` writer - Wait feature enable"]
pub type NDWTEN_W<'a> = crate::BitWriter<'a, u32, NPCTL1_SPEC, bool, 1>;
impl R {
    #[doc = "Bits 17:19 - ECC size"]
    #[inline(always)]
    pub fn eccsz(&self) -> ECCSZ_R {
        ECCSZ_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    pub fn atr(&self) -> ATR_R {
        ATR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - ECC enable"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 4:5 - NAND bank memory data bus width"]
    #[inline(always)]
    pub fn ndw(&self) -> NDW_R {
        NDW_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 3 - NAND bank memory type"]
    #[inline(always)]
    pub fn ndtp(&self) -> NDTP_R {
        NDTP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - NAND bank enable"]
    #[inline(always)]
    pub fn ndbken(&self) -> NDBKEN_R {
        NDBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    pub fn ndwten(&self) -> NDWTEN_R {
        NDWTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 17:19 - ECC size"]
    #[inline(always)]
    pub fn eccsz(&mut self) -> ECCSZ_W {
        ECCSZ_W::new(self)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    pub fn atr(&mut self) -> ATR_W {
        ATR_W::new(self)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CTR_W {
        CTR_W::new(self)
    }
    #[doc = "Bit 6 - ECC enable"]
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W {
        ECCEN_W::new(self)
    }
    #[doc = "Bits 4:5 - NAND bank memory data bus width"]
    #[inline(always)]
    pub fn ndw(&mut self) -> NDW_W {
        NDW_W::new(self)
    }
    #[doc = "Bit 3 - NAND bank memory type"]
    #[inline(always)]
    pub fn ndtp(&mut self) -> NDTP_W {
        NDTP_W::new(self)
    }
    #[doc = "Bit 2 - NAND bank enable"]
    #[inline(always)]
    pub fn ndbken(&mut self) -> NDBKEN_W {
        NDBKEN_W::new(self)
    }
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    pub fn ndwten(&mut self) -> NDWTEN_W {
        NDWTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NAND flash/PC card control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npctl1](index.html) module"]
pub struct NPCTL1_SPEC;
impl crate::RegisterSpec for NPCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [npctl1::R](R) reader structure"]
impl crate::Readable for NPCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [npctl1::W](W) writer structure"]
impl crate::Writable for NPCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NPCTL1 to value 0x18"]
impl crate::Resettable for NPCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x18
    }
}
