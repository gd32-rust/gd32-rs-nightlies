#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALGM1` reader - Algorithm selection bit 1"]
pub type ALGM1_R = crate::BitReader<bool>;
#[doc = "Field `ALGM1` writer - Algorithm selection bit 1"]
pub type ALGM1_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 18>;
#[doc = "Field `KLM` reader - Key length mode"]
pub type KLM_R = crate::BitReader<bool>;
#[doc = "Field `KLM` writer - Key length mode"]
pub type KLM_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 16>;
#[doc = "Field `MDS` reader - Multiple DMA selection"]
pub type MDS_R = crate::BitReader<bool>;
#[doc = "Field `MDS` writer - Multiple DMA selection"]
pub type MDS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 13>;
#[doc = "Field `DINE` reader - DI register is not empty"]
pub type DINE_R = crate::BitReader<bool>;
#[doc = "Field `NWIF` reader - Number of words in IN FIFO"]
pub type NWIF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALGM0` reader - Algorithm selection bit 0"]
pub type ALGM0_R = crate::BitReader<bool>;
#[doc = "Field `ALGM0` writer - Algorithm selection bit 0"]
pub type ALGM0_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 7>;
#[doc = "Field `HMS` reader - HAU mode selection"]
pub type HMS_R = crate::BitReader<bool>;
#[doc = "Field `HMS` writer - HAU mode selection"]
pub type HMS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 6>;
#[doc = "Field `DATAM` reader - Data type mode"]
pub type DATAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAM` writer - Data type mode"]
pub type DATAM_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, 4>;
#[doc = "Field `DMAE` reader - DMA enable"]
pub type DMAE_R = crate::BitReader<bool>;
#[doc = "Field `DMAE` writer - DMA enable"]
pub type DMAE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 3>;
#[doc = "Field `START` writer - Start message digest calculation"]
pub type START_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 18 - Algorithm selection bit 1"]
    #[inline(always)]
    pub fn algm1(&self) -> ALGM1_R {
        ALGM1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 16 - Key length mode"]
    #[inline(always)]
    pub fn klm(&self) -> KLM_R {
        KLM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 13 - Multiple DMA selection"]
    #[inline(always)]
    pub fn mds(&self) -> MDS_R {
        MDS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - DI register is not empty"]
    #[inline(always)]
    pub fn dine(&self) -> DINE_R {
        DINE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of words in IN FIFO"]
    #[inline(always)]
    pub fn nwif(&self) -> NWIF_R {
        NWIF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Algorithm selection bit 0"]
    #[inline(always)]
    pub fn algm0(&self) -> ALGM0_R {
        ALGM0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - HAU mode selection"]
    #[inline(always)]
    pub fn hms(&self) -> HMS_R {
        HMS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data type mode"]
    #[inline(always)]
    pub fn datam(&self) -> DATAM_R {
        DATAM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Algorithm selection bit 1"]
    #[inline(always)]
    pub fn algm1(&mut self) -> ALGM1_W {
        ALGM1_W::new(self)
    }
    #[doc = "Bit 16 - Key length mode"]
    #[inline(always)]
    pub fn klm(&mut self) -> KLM_W {
        KLM_W::new(self)
    }
    #[doc = "Bit 13 - Multiple DMA selection"]
    #[inline(always)]
    pub fn mds(&mut self) -> MDS_W {
        MDS_W::new(self)
    }
    #[doc = "Bit 7 - Algorithm selection bit 0"]
    #[inline(always)]
    pub fn algm0(&mut self) -> ALGM0_W {
        ALGM0_W::new(self)
    }
    #[doc = "Bit 6 - HAU mode selection"]
    #[inline(always)]
    pub fn hms(&mut self) -> HMS_W {
        HMS_W::new(self)
    }
    #[doc = "Bits 4:5 - Data type mode"]
    #[inline(always)]
    pub fn datam(&mut self) -> DATAM_W {
        DATAM_W::new(self)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmae(&mut self) -> DMAE_W {
        DMAE_W::new(self)
    }
    #[doc = "Bit 2 - Start message digest calculation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HAU control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
