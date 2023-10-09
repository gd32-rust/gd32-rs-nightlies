#[doc = "Register `ADPCTL` reader"]
pub type R = crate::R<ADPCTL_SPEC>;
#[doc = "Register `ADPCTL` writer"]
pub type W = crate::W<ADPCTL_SPEC>;
#[doc = "Field `DSCHGPR` reader - Time of probe discharge"]
pub type DSCHGPR_R = crate::FieldReader;
#[doc = "Field `DSCHGPR` writer - Time of probe discharge"]
pub type DSCHGPR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RESOPR` reader - The resolution of CHGT value"]
pub type RESOPR_R = crate::FieldReader;
#[doc = "Field `RESOPR` writer - The resolution of CHGT value"]
pub type RESOPR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PERPR` reader - Period of probe"]
pub type PERPR_R = crate::FieldReader;
#[doc = "Field `PERPR` writer - Period of probe"]
pub type PERPR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CHGT` reader - The latest time that VBUS ramps from VADPSINK to VADPPRB"]
pub type CHGT_R = crate::FieldReader<u16>;
#[doc = "Field `PREN` reader - ADP probe enable"]
pub type PREN_R = crate::BitReader;
#[doc = "Field `PREN` writer - ADP probe enable"]
pub type PREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SNEN` reader - ADP sense enable"]
pub type SNEN_R = crate::BitReader;
#[doc = "Field `SNEN` writer - ADP sense enable"]
pub type SNEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADPRST` reader - ADP reset"]
pub type ADPRST_R = crate::BitReader;
#[doc = "Field `ADPEN` reader - ADP enable"]
pub type ADPEN_R = crate::BitReader;
#[doc = "Field `ADPEN` writer - ADP enable"]
pub type ADPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADPPRF` reader - ADP probe interrupt flag"]
pub type ADPPRF_R = crate::BitReader;
#[doc = "Field `ADPPRF` writer - ADP probe interrupt flag"]
pub type ADPPRF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADPSNF` reader - ADP sense interrupt flag"]
pub type ADPSNF_R = crate::BitReader;
#[doc = "Field `ADPSNF` writer - ADP sense interrupt flag"]
pub type ADPSNF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADPTF` reader - ADP timeout interrupt flag"]
pub type ADPTF_R = crate::BitReader;
#[doc = "Field `ADPTF` writer - ADP timeout interrupt flag"]
pub type ADPTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADPPRFM` reader - The mask of ADP probe interrupt flag"]
pub type ADPPRFM_R = crate::BitReader;
#[doc = "Field `ADPPRFM` writer - The mask of ADP probe interrupt flag"]
pub type ADPPRFM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADPSNFM` reader - The mask of ADP sense interrupt flag"]
pub type ADPSNFM_R = crate::BitReader;
#[doc = "Field `ADPSNFM` writer - The mask of ADP sense interrupt flag"]
pub type ADPSNFM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADPTFM` reader - The mask of ADP timeout interrupt flag"]
pub type ADPTFM_R = crate::BitReader;
#[doc = "Field `ADPTFM` writer - The mask of ADP timeout interrupt flag"]
pub type ADPTFM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWR` reader - Read and write request"]
pub type RWR_R = crate::FieldReader;
#[doc = "Field `RWR` writer - Read and write request"]
pub type RWR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Time of probe discharge"]
    #[inline(always)]
    pub fn dschgpr(&self) -> DSCHGPR_R {
        DSCHGPR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The resolution of CHGT value"]
    #[inline(always)]
    pub fn resopr(&self) -> RESOPR_R {
        RESOPR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Period of probe"]
    #[inline(always)]
    pub fn perpr(&self) -> PERPR_R {
        PERPR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:16 - The latest time that VBUS ramps from VADPSINK to VADPPRB"]
    #[inline(always)]
    pub fn chgt(&self) -> CHGT_R {
        CHGT_R::new(((self.bits >> 6) & 0x07ff) as u16)
    }
    #[doc = "Bit 17 - ADP probe enable"]
    #[inline(always)]
    pub fn pren(&self) -> PREN_R {
        PREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADP sense enable"]
    #[inline(always)]
    pub fn snen(&self) -> SNEN_R {
        SNEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADP reset"]
    #[inline(always)]
    pub fn adprst(&self) -> ADPRST_R {
        ADPRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADP enable"]
    #[inline(always)]
    pub fn adpen(&self) -> ADPEN_R {
        ADPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADP probe interrupt flag"]
    #[inline(always)]
    pub fn adpprf(&self) -> ADPPRF_R {
        ADPPRF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADP sense interrupt flag"]
    #[inline(always)]
    pub fn adpsnf(&self) -> ADPSNF_R {
        ADPSNF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADP timeout interrupt flag"]
    #[inline(always)]
    pub fn adptf(&self) -> ADPTF_R {
        ADPTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The mask of ADP probe interrupt flag"]
    #[inline(always)]
    pub fn adpprfm(&self) -> ADPPRFM_R {
        ADPPRFM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The mask of ADP sense interrupt flag"]
    #[inline(always)]
    pub fn adpsnfm(&self) -> ADPSNFM_R {
        ADPSNFM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The mask of ADP timeout interrupt flag"]
    #[inline(always)]
    pub fn adptfm(&self) -> ADPTFM_R {
        ADPTFM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Read and write request"]
    #[inline(always)]
    pub fn rwr(&self) -> RWR_R {
        RWR_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Time of probe discharge"]
    #[inline(always)]
    #[must_use]
    pub fn dschgpr(&mut self) -> DSCHGPR_W<ADPCTL_SPEC, 0> {
        DSCHGPR_W::new(self)
    }
    #[doc = "Bits 2:3 - The resolution of CHGT value"]
    #[inline(always)]
    #[must_use]
    pub fn resopr(&mut self) -> RESOPR_W<ADPCTL_SPEC, 2> {
        RESOPR_W::new(self)
    }
    #[doc = "Bits 4:5 - Period of probe"]
    #[inline(always)]
    #[must_use]
    pub fn perpr(&mut self) -> PERPR_W<ADPCTL_SPEC, 4> {
        PERPR_W::new(self)
    }
    #[doc = "Bit 17 - ADP probe enable"]
    #[inline(always)]
    #[must_use]
    pub fn pren(&mut self) -> PREN_W<ADPCTL_SPEC, 17> {
        PREN_W::new(self)
    }
    #[doc = "Bit 18 - ADP sense enable"]
    #[inline(always)]
    #[must_use]
    pub fn snen(&mut self) -> SNEN_W<ADPCTL_SPEC, 18> {
        SNEN_W::new(self)
    }
    #[doc = "Bit 20 - ADP enable"]
    #[inline(always)]
    #[must_use]
    pub fn adpen(&mut self) -> ADPEN_W<ADPCTL_SPEC, 20> {
        ADPEN_W::new(self)
    }
    #[doc = "Bit 21 - ADP probe interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adpprf(&mut self) -> ADPPRF_W<ADPCTL_SPEC, 21> {
        ADPPRF_W::new(self)
    }
    #[doc = "Bit 22 - ADP sense interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adpsnf(&mut self) -> ADPSNF_W<ADPCTL_SPEC, 22> {
        ADPSNF_W::new(self)
    }
    #[doc = "Bit 23 - ADP timeout interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adptf(&mut self) -> ADPTF_W<ADPCTL_SPEC, 23> {
        ADPTF_W::new(self)
    }
    #[doc = "Bit 24 - The mask of ADP probe interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adpprfm(&mut self) -> ADPPRFM_W<ADPCTL_SPEC, 24> {
        ADPPRFM_W::new(self)
    }
    #[doc = "Bit 25 - The mask of ADP sense interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adpsnfm(&mut self) -> ADPSNFM_W<ADPCTL_SPEC, 25> {
        ADPSNFM_W::new(self)
    }
    #[doc = "Bit 26 - The mask of ADP timeout interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adptfm(&mut self) -> ADPTFM_W<ADPCTL_SPEC, 26> {
        ADPTFM_W::new(self)
    }
    #[doc = "Bits 27:28 - Read and write request"]
    #[inline(always)]
    #[must_use]
    pub fn rwr(&mut self) -> RWR_W<ADPCTL_SPEC, 27> {
        RWR_W::new(self)
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
#[doc = "ADP control andstatus register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADPCTL_SPEC;
impl crate::RegisterSpec for ADPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adpctl::R`](R) reader structure"]
impl crate::Readable for ADPCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adpctl::W`](W) writer structure"]
impl crate::Writable for ADPCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADPCTL to value 0x1000"]
impl crate::Resettable for ADPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
