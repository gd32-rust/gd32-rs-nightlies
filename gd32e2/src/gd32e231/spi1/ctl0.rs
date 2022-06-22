#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDEN` reader - Bidirectional enable"]
pub type BDEN_R = crate::BitReader<bool>;
#[doc = "Field `BDEN` writer - Bidirectional enable"]
pub type BDEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 15>;
#[doc = "Field `BDOEN` reader - Bidirectional Transmit output enable"]
pub type BDOEN_R = crate::BitReader<bool>;
#[doc = "Field `BDOEN` writer - Bidirectional Transmit output enable"]
pub type BDOEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 14>;
#[doc = "Field `CRCEN` reader - Hardware CRC calculation enable"]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - Hardware CRC calculation enable"]
pub type CRCEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 13>;
#[doc = "Field `CRCNT` reader - CRC transfer next"]
pub type CRCNT_R = crate::BitReader<bool>;
#[doc = "Field `CRCNT` writer - CRC transfer next"]
pub type CRCNT_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 12>;
#[doc = "Field `CRCL` reader - CRC length"]
pub type CRCL_R = crate::BitReader<bool>;
#[doc = "Field `CRCL` writer - CRC length"]
pub type CRCL_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 11>;
#[doc = "Field `RO` reader - Receive only"]
pub type RO_R = crate::BitReader<bool>;
#[doc = "Field `RO` writer - Receive only"]
pub type RO_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 10>;
#[doc = "Field `SWNSSEN` reader - NSS Software Mode Selection"]
pub type SWNSSEN_R = crate::BitReader<bool>;
#[doc = "Field `SWNSSEN` writer - NSS Software Mode Selection"]
pub type SWNSSEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 9>;
#[doc = "Field `SWNSS` reader - NSS Pin Selection In NSS Software Mode"]
pub type SWNSS_R = crate::BitReader<bool>;
#[doc = "Field `SWNSS` writer - NSS Pin Selection In NSS Software Mode"]
pub type SWNSS_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 8>;
#[doc = "Field `LF` reader - LSB First Mode"]
pub type LF_R = crate::BitReader<bool>;
#[doc = "Field `LF` writer - LSB First Mode"]
pub type LF_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 7>;
#[doc = "Field `SPIEN` reader - SPI enable"]
pub type SPIEN_R = crate::BitReader<bool>;
#[doc = "Field `SPIEN` writer - SPI enable"]
pub type SPIEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 6>;
#[doc = "Field `PSC` reader - Master Clock Prescaler Selection"]
pub type PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSC` writer - Master Clock Prescaler Selection"]
pub type PSC_W<'a> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 3, 3>;
#[doc = "Field `MSTMOD` reader - Master Mode Enable"]
pub type MSTMOD_R = crate::BitReader<bool>;
#[doc = "Field `MSTMOD` writer - Master Mode Enable"]
pub type MSTMOD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 2>;
#[doc = "Field `CKPL` reader - Clock Polarity Selection"]
pub type CKPL_R = crate::BitReader<bool>;
#[doc = "Field `CKPL` writer - Clock Polarity Selection"]
pub type CKPL_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 1>;
#[doc = "Field `CKPH` reader - Clock Phase Selection"]
pub type CKPH_R = crate::BitReader<bool>;
#[doc = "Field `CKPH` writer - Clock Phase Selection"]
pub type CKPH_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 15 - Bidirectional enable"]
    #[inline(always)]
    pub fn bden(&self) -> BDEN_R {
        BDEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Bidirectional Transmit output enable"]
    #[inline(always)]
    pub fn bdoen(&self) -> BDOEN_R {
        BDOEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnt(&self) -> CRCNT_R {
        CRCNT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - CRC length"]
    #[inline(always)]
    pub fn crcl(&self) -> CRCL_R {
        CRCL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn ro(&self) -> RO_R {
        RO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - NSS Software Mode Selection"]
    #[inline(always)]
    pub fn swnssen(&self) -> SWNSSEN_R {
        SWNSSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Pin Selection In NSS Software Mode"]
    #[inline(always)]
    pub fn swnss(&self) -> SWNSS_R {
        SWNSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - LSB First Mode"]
    #[inline(always)]
    pub fn lf(&self) -> LF_R {
        LF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Master Clock Prescaler Selection"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 2 - Master Mode Enable"]
    #[inline(always)]
    pub fn mstmod(&self) -> MSTMOD_R {
        MSTMOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity Selection"]
    #[inline(always)]
    pub fn ckpl(&self) -> CKPL_R {
        CKPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Clock Phase Selection"]
    #[inline(always)]
    pub fn ckph(&self) -> CKPH_R {
        CKPH_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Bidirectional enable"]
    #[inline(always)]
    pub fn bden(&mut self) -> BDEN_W {
        BDEN_W::new(self)
    }
    #[doc = "Bit 14 - Bidirectional Transmit output enable"]
    #[inline(always)]
    pub fn bdoen(&mut self) -> BDOEN_W {
        BDOEN_W::new(self)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnt(&mut self) -> CRCNT_W {
        CRCNT_W::new(self)
    }
    #[doc = "Bit 11 - CRC length"]
    #[inline(always)]
    pub fn crcl(&mut self) -> CRCL_W {
        CRCL_W::new(self)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn ro(&mut self) -> RO_W {
        RO_W::new(self)
    }
    #[doc = "Bit 9 - NSS Software Mode Selection"]
    #[inline(always)]
    pub fn swnssen(&mut self) -> SWNSSEN_W {
        SWNSSEN_W::new(self)
    }
    #[doc = "Bit 8 - NSS Pin Selection In NSS Software Mode"]
    #[inline(always)]
    pub fn swnss(&mut self) -> SWNSS_W {
        SWNSS_W::new(self)
    }
    #[doc = "Bit 7 - LSB First Mode"]
    #[inline(always)]
    pub fn lf(&mut self) -> LF_W {
        LF_W::new(self)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spien(&mut self) -> SPIEN_W {
        SPIEN_W::new(self)
    }
    #[doc = "Bits 3:5 - Master Clock Prescaler Selection"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W::new(self)
    }
    #[doc = "Bit 2 - Master Mode Enable"]
    #[inline(always)]
    pub fn mstmod(&mut self) -> MSTMOD_W {
        MSTMOD_W::new(self)
    }
    #[doc = "Bit 1 - Clock Polarity Selection"]
    #[inline(always)]
    pub fn ckpl(&mut self) -> CKPL_W {
        CKPL_W::new(self)
    }
    #[doc = "Bit 0 - Clock Phase Selection"]
    #[inline(always)]
    pub fn ckph(&mut self) -> CKPH_W {
        CKPH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
