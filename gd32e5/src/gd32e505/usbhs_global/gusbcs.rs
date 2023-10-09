#[doc = "Register `GUSBCS` reader"]
pub type R = crate::R<GUSBCS_SPEC>;
#[doc = "Register `GUSBCS` writer"]
pub type W = crate::W<GUSBCS_SPEC>;
#[doc = "Field `TOC` reader - Timeout calibration"]
pub type TOC_R = crate::FieldReader;
#[doc = "Field `TOC` writer - Timeout calibration"]
pub type TOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `HS_CUR_FE` reader - HS current software enable"]
pub type HS_CUR_FE_R = crate::BitReader;
#[doc = "Field `HS_CUR_FE` writer - HS current software enable"]
pub type HS_CUR_FE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMBPHY_HS` reader - Embedded HS PHY selected"]
pub type EMBPHY_HS_R = crate::BitReader;
#[doc = "Field `EMBPHY_HS` writer - Embedded HS PHY selected"]
pub type EMBPHY_HS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMBPHY_FS` reader - Embedded FS PHY selected"]
pub type EMBPHY_FS_R = crate::BitReader;
#[doc = "Field `EMBPHY_FS` writer - Embedded FS PHY selected"]
pub type EMBPHY_FS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRPCEN` reader - SRP capability enable"]
pub type SRPCEN_R = crate::BitReader;
#[doc = "Field `SRPCEN` writer - SRP capability enable"]
pub type SRPCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNPCEN` reader - HNP capability enable"]
pub type HNPCEN_R = crate::BitReader;
#[doc = "Field `HNPCEN` writer - HNP capability enable"]
pub type HNPCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UTT` reader - USB turnaround time"]
pub type UTT_R = crate::FieldReader;
#[doc = "Field `UTT` writer - USB turnaround time"]
pub type UTT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ULPIEVD` reader - ULPI external VBUS driver"]
pub type ULPIEVD_R = crate::BitReader;
#[doc = "Field `ULPIEVD` writer - ULPI external VBUS driver"]
pub type ULPIEVD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ULPIEOI` reader - ULPI external over-current indicator"]
pub type ULPIEOI_R = crate::BitReader;
#[doc = "Field `ULPIEOI` writer - ULPI external over-current indicator"]
pub type ULPIEOI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FHM` reader - Force host mode"]
pub type FHM_R = crate::BitReader;
#[doc = "Field `FHM` writer - Force host mode"]
pub type FHM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FDM` reader - Force device mode"]
pub type FDM_R = crate::BitReader;
#[doc = "Field `FDM` writer - Force device mode"]
pub type FDM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Timeout calibration"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - HS current software enable"]
    #[inline(always)]
    pub fn hs_cur_fe(&self) -> HS_CUR_FE_R {
        HS_CUR_FE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Embedded HS PHY selected"]
    #[inline(always)]
    pub fn embphy_hs(&self) -> EMBPHY_HS_R {
        EMBPHY_HS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Embedded FS PHY selected"]
    #[inline(always)]
    pub fn embphy_fs(&self) -> EMBPHY_FS_R {
        EMBPHY_FS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SRP capability enable"]
    #[inline(always)]
    pub fn srpcen(&self) -> SRPCEN_R {
        SRPCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP capability enable"]
    #[inline(always)]
    pub fn hnpcen(&self) -> HNPCEN_R {
        HNPCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn utt(&self) -> UTT_R {
        UTT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - ULPI external VBUS driver"]
    #[inline(always)]
    pub fn ulpievd(&self) -> ULPIEVD_R {
        ULPIEVD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ULPI external over-current indicator"]
    #[inline(always)]
    pub fn ulpieoi(&self) -> ULPIEOI_R {
        ULPIEOI_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    pub fn fhm(&self) -> FHM_R {
        FHM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    pub fn fdm(&self) -> FDM_R {
        FDM_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timeout calibration"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<GUSBCS_SPEC, 0> {
        TOC_W::new(self)
    }
    #[doc = "Bit 4 - HS current software enable"]
    #[inline(always)]
    #[must_use]
    pub fn hs_cur_fe(&mut self) -> HS_CUR_FE_W<GUSBCS_SPEC, 4> {
        HS_CUR_FE_W::new(self)
    }
    #[doc = "Bit 5 - Embedded HS PHY selected"]
    #[inline(always)]
    #[must_use]
    pub fn embphy_hs(&mut self) -> EMBPHY_HS_W<GUSBCS_SPEC, 5> {
        EMBPHY_HS_W::new(self)
    }
    #[doc = "Bit 6 - Embedded FS PHY selected"]
    #[inline(always)]
    #[must_use]
    pub fn embphy_fs(&mut self) -> EMBPHY_FS_W<GUSBCS_SPEC, 6> {
        EMBPHY_FS_W::new(self)
    }
    #[doc = "Bit 8 - SRP capability enable"]
    #[inline(always)]
    #[must_use]
    pub fn srpcen(&mut self) -> SRPCEN_W<GUSBCS_SPEC, 8> {
        SRPCEN_W::new(self)
    }
    #[doc = "Bit 9 - HNP capability enable"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcen(&mut self) -> HNPCEN_W<GUSBCS_SPEC, 9> {
        HNPCEN_W::new(self)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    #[must_use]
    pub fn utt(&mut self) -> UTT_W<GUSBCS_SPEC, 10> {
        UTT_W::new(self)
    }
    #[doc = "Bit 20 - ULPI external VBUS driver"]
    #[inline(always)]
    #[must_use]
    pub fn ulpievd(&mut self) -> ULPIEVD_W<GUSBCS_SPEC, 20> {
        ULPIEVD_W::new(self)
    }
    #[doc = "Bit 21 - ULPI external over-current indicator"]
    #[inline(always)]
    #[must_use]
    pub fn ulpieoi(&mut self) -> ULPIEOI_W<GUSBCS_SPEC, 21> {
        ULPIEOI_W::new(self)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    #[must_use]
    pub fn fhm(&mut self) -> FHM_W<GUSBCS_SPEC, 29> {
        FHM_W::new(self)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    #[must_use]
    pub fn fdm(&mut self) -> FDM_W<GUSBCS_SPEC, 30> {
        FDM_W::new(self)
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
#[doc = "Global USB control and status register (USBHS_GUSBCSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GUSBCS_SPEC;
impl crate::RegisterSpec for GUSBCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcs::R`](R) reader structure"]
impl crate::Readable for GUSBCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gusbcs::W`](W) writer structure"]
impl crate::Writable for GUSBCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GUSBCS to value 0x0a00"]
impl crate::Resettable for GUSBCS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a00;
}
