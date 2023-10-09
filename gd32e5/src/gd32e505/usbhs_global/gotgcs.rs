#[doc = "Register `GOTGCS` reader"]
pub type R = crate::R<GOTGCS_SPEC>;
#[doc = "Register `GOTGCS` writer"]
pub type W = crate::W<GOTGCS_SPEC>;
#[doc = "Field `SRPS` reader - SRP success"]
pub type SRPS_R = crate::BitReader;
#[doc = "Field `SRPREQ` reader - SRP request"]
pub type SRPREQ_R = crate::BitReader;
#[doc = "Field `SRPREQ` writer - SRP request"]
pub type SRPREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VOE` reader - Override enable of VBUS valid"]
pub type VOE_R = crate::BitReader;
#[doc = "Field `VOE` writer - Override enable of VBUS valid"]
pub type VOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VOV` reader - Override value of VBUS valid"]
pub type VOV_R = crate::BitReader;
#[doc = "Field `VOV` writer - Override value of VBUS valid"]
pub type VOV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AVOE` reader - Override enable of A-peripheral session valid"]
pub type AVOE_R = crate::BitReader;
#[doc = "Field `AVOE` writer - Override enable of A-peripheral session valid"]
pub type AVOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AVOV` reader - Override value of A-peripheral session valid"]
pub type AVOV_R = crate::BitReader;
#[doc = "Field `AVOV` writer - Override value of A-peripheral session valid"]
pub type AVOV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BVOE` reader - Override enable of B-peripheral session valid"]
pub type BVOE_R = crate::BitReader;
#[doc = "Field `BVOE` writer - Override enable of B-peripheral session valid"]
pub type BVOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BVOV` reader - Override value of B-peripheral session valid"]
pub type BVOV_R = crate::BitReader;
#[doc = "Field `BVOV` writer - Override value of B-peripheral session valid"]
pub type BVOV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNPS` reader - HNP success"]
pub type HNPS_R = crate::BitReader;
#[doc = "Field `HNPREQ` reader - HNP request"]
pub type HNPREQ_R = crate::BitReader;
#[doc = "Field `HNPREQ` writer - HNP request"]
pub type HNPREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HHNPEN` reader - Host HNP enable"]
pub type HHNPEN_R = crate::BitReader;
#[doc = "Field `HHNPEN` writer - Host HNP enable"]
pub type HHNPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DHNPEN` reader - Device HNP enabled"]
pub type DHNPEN_R = crate::BitReader;
#[doc = "Field `DHNPEN` writer - Device HNP enabled"]
pub type DHNPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EHE` reader - Embedded host enable"]
pub type EHE_R = crate::BitReader;
#[doc = "Field `EHE` writer - Embedded host enable"]
pub type EHE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDPS` reader - ID pin status"]
pub type IDPS_R = crate::BitReader;
#[doc = "Field `DI` reader - Debounce interval"]
pub type DI_R = crate::BitReader;
#[doc = "Field `ASV` reader - A-session valid"]
pub type ASV_R = crate::BitReader;
#[doc = "Field `BSV` reader - B-session valid"]
pub type BSV_R = crate::BitReader;
#[doc = "Field `OV` reader - Select OTG version"]
pub type OV_R = crate::BitReader;
#[doc = "Field `OV` writer - Select OTG version"]
pub type OV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SRP success"]
    #[inline(always)]
    pub fn srps(&self) -> SRPS_R {
        SRPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRP request"]
    #[inline(always)]
    pub fn srpreq(&self) -> SRPREQ_R {
        SRPREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Override enable of VBUS valid"]
    #[inline(always)]
    pub fn voe(&self) -> VOE_R {
        VOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Override value of VBUS valid"]
    #[inline(always)]
    pub fn vov(&self) -> VOV_R {
        VOV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Override enable of A-peripheral session valid"]
    #[inline(always)]
    pub fn avoe(&self) -> AVOE_R {
        AVOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Override value of A-peripheral session valid"]
    #[inline(always)]
    pub fn avov(&self) -> AVOV_R {
        AVOV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Override enable of B-peripheral session valid"]
    #[inline(always)]
    pub fn bvoe(&self) -> BVOE_R {
        BVOE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Override value of B-peripheral session valid"]
    #[inline(always)]
    pub fn bvov(&self) -> BVOV_R {
        BVOV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HNP success"]
    #[inline(always)]
    pub fn hnps(&self) -> HNPS_R {
        HNPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HNPREQ_R {
        HNPREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host HNP enable"]
    #[inline(always)]
    pub fn hhnpen(&self) -> HHNPEN_R {
        HHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Embedded host enable"]
    #[inline(always)]
    pub fn ehe(&self) -> EHE_R {
        EHE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - ID pin status"]
    #[inline(always)]
    pub fn idps(&self) -> IDPS_R {
        IDPS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Debounce interval"]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asv(&self) -> ASV_R {
        ASV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsv(&self) -> BSV_R {
        BSV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select OTG version"]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SRP request"]
    #[inline(always)]
    #[must_use]
    pub fn srpreq(&mut self) -> SRPREQ_W<GOTGCS_SPEC, 1> {
        SRPREQ_W::new(self)
    }
    #[doc = "Bit 2 - Override enable of VBUS valid"]
    #[inline(always)]
    #[must_use]
    pub fn voe(&mut self) -> VOE_W<GOTGCS_SPEC, 2> {
        VOE_W::new(self)
    }
    #[doc = "Bit 3 - Override value of VBUS valid"]
    #[inline(always)]
    #[must_use]
    pub fn vov(&mut self) -> VOV_W<GOTGCS_SPEC, 3> {
        VOV_W::new(self)
    }
    #[doc = "Bit 4 - Override enable of A-peripheral session valid"]
    #[inline(always)]
    #[must_use]
    pub fn avoe(&mut self) -> AVOE_W<GOTGCS_SPEC, 4> {
        AVOE_W::new(self)
    }
    #[doc = "Bit 5 - Override value of A-peripheral session valid"]
    #[inline(always)]
    #[must_use]
    pub fn avov(&mut self) -> AVOV_W<GOTGCS_SPEC, 5> {
        AVOV_W::new(self)
    }
    #[doc = "Bit 6 - Override enable of B-peripheral session valid"]
    #[inline(always)]
    #[must_use]
    pub fn bvoe(&mut self) -> BVOE_W<GOTGCS_SPEC, 6> {
        BVOE_W::new(self)
    }
    #[doc = "Bit 7 - Override value of B-peripheral session valid"]
    #[inline(always)]
    #[must_use]
    pub fn bvov(&mut self) -> BVOV_W<GOTGCS_SPEC, 7> {
        BVOV_W::new(self)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    #[must_use]
    pub fn hnpreq(&mut self) -> HNPREQ_W<GOTGCS_SPEC, 9> {
        HNPREQ_W::new(self)
    }
    #[doc = "Bit 10 - Host HNP enable"]
    #[inline(always)]
    #[must_use]
    pub fn hhnpen(&mut self) -> HHNPEN_W<GOTGCS_SPEC, 10> {
        HHNPEN_W::new(self)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dhnpen(&mut self) -> DHNPEN_W<GOTGCS_SPEC, 11> {
        DHNPEN_W::new(self)
    }
    #[doc = "Bit 12 - Embedded host enable"]
    #[inline(always)]
    #[must_use]
    pub fn ehe(&mut self) -> EHE_W<GOTGCS_SPEC, 12> {
        EHE_W::new(self)
    }
    #[doc = "Bit 20 - Select OTG version"]
    #[inline(always)]
    #[must_use]
    pub fn ov(&mut self) -> OV_W<GOTGCS_SPEC, 20> {
        OV_W::new(self)
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
#[doc = "Global OTG control and status register (USBFS_GOTGCS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGCS_SPEC;
impl crate::RegisterSpec for GOTGCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgcs::R`](R) reader structure"]
impl crate::Readable for GOTGCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gotgcs::W`](W) writer structure"]
impl crate::Writable for GOTGCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GOTGCS to value 0x0800"]
impl crate::Resettable for GOTGCS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
