#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `START` writer - Start message digest calculation"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAE` reader - DMA enable"]
pub type DMAE_R = crate::BitReader;
#[doc = "Field `DMAE` writer - DMA enable"]
pub type DMAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATAM` reader - Data type mode"]
pub type DATAM_R = crate::FieldReader;
#[doc = "Field `DATAM` writer - Data type mode"]
pub type DATAM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `HMS` reader - HAU mode selection"]
pub type HMS_R = crate::BitReader;
#[doc = "Field `HMS` writer - HAU mode selection"]
pub type HMS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALGM0` reader - Algorithm selection bit 0"]
pub type ALGM0_R = crate::BitReader;
#[doc = "Field `ALGM0` writer - Algorithm selection bit 0"]
pub type ALGM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NWIF` reader - Number of words in IN FIFO"]
pub type NWIF_R = crate::FieldReader;
#[doc = "Field `DINE` reader - DI register is not empty"]
pub type DINE_R = crate::BitReader;
#[doc = "Field `MDS` reader - Multiple DMA selection"]
pub type MDS_R = crate::BitReader;
#[doc = "Field `MDS` writer - Multiple DMA selection"]
pub type MDS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `KLM` reader - Key length mode"]
pub type KLM_R = crate::BitReader;
#[doc = "Field `KLM` writer - Key length mode"]
pub type KLM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALGM1` reader - Algorithm selection bit 1"]
pub type ALGM1_R = crate::BitReader;
#[doc = "Field `ALGM1` writer - Algorithm selection bit 1"]
pub type ALGM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data type mode"]
    #[inline(always)]
    pub fn datam(&self) -> DATAM_R {
        DATAM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - HAU mode selection"]
    #[inline(always)]
    pub fn hms(&self) -> HMS_R {
        HMS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Algorithm selection bit 0"]
    #[inline(always)]
    pub fn algm0(&self) -> ALGM0_R {
        ALGM0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of words in IN FIFO"]
    #[inline(always)]
    pub fn nwif(&self) -> NWIF_R {
        NWIF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DI register is not empty"]
    #[inline(always)]
    pub fn dine(&self) -> DINE_R {
        DINE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Multiple DMA selection"]
    #[inline(always)]
    pub fn mds(&self) -> MDS_R {
        MDS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Key length mode"]
    #[inline(always)]
    pub fn klm(&self) -> KLM_R {
        KLM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Algorithm selection bit 1"]
    #[inline(always)]
    pub fn algm1(&self) -> ALGM1_R {
        ALGM1_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Start message digest calculation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTL_SPEC, 2> {
        START_W::new(self)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmae(&mut self) -> DMAE_W<CTL_SPEC, 3> {
        DMAE_W::new(self)
    }
    #[doc = "Bits 4:5 - Data type mode"]
    #[inline(always)]
    #[must_use]
    pub fn datam(&mut self) -> DATAM_W<CTL_SPEC, 4> {
        DATAM_W::new(self)
    }
    #[doc = "Bit 6 - HAU mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn hms(&mut self) -> HMS_W<CTL_SPEC, 6> {
        HMS_W::new(self)
    }
    #[doc = "Bit 7 - Algorithm selection bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn algm0(&mut self) -> ALGM0_W<CTL_SPEC, 7> {
        ALGM0_W::new(self)
    }
    #[doc = "Bit 13 - Multiple DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn mds(&mut self) -> MDS_W<CTL_SPEC, 13> {
        MDS_W::new(self)
    }
    #[doc = "Bit 16 - Key length mode"]
    #[inline(always)]
    #[must_use]
    pub fn klm(&mut self) -> KLM_W<CTL_SPEC, 16> {
        KLM_W::new(self)
    }
    #[doc = "Bit 18 - Algorithm selection bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn algm1(&mut self) -> ALGM1_W<CTL_SPEC, 18> {
        ALGM1_W::new(self)
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
#[doc = "HAU control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
