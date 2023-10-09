#[doc = "Register `GCCFG` reader"]
pub type R = crate::R<GCCFG_SPEC>;
#[doc = "Register `GCCFG` writer"]
pub type W = crate::W<GCCFG_SPEC>;
#[doc = "Field `DCDF` reader - Data connect detection status"]
pub type DCDF_R = crate::BitReader;
#[doc = "Field `DCDF` writer - Data connect detection status"]
pub type DCDF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDF` reader - Primary detection status"]
pub type PDF_R = crate::BitReader;
#[doc = "Field `PDF` writer - Primary detection status"]
pub type PDF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDF` reader - Secondary detection status"]
pub type SDF_R = crate::BitReader;
#[doc = "Field `SDF` writer - Secondary detection status"]
pub type SDF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PS2F` reader - PS2 detection status"]
pub type PS2F_R = crate::BitReader;
#[doc = "Field `PS2F` writer - PS2 detection status"]
pub type PS2F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BCDEN` reader - Battery charging detection enable"]
pub type BCDEN_R = crate::BitReader;
#[doc = "Field `BCDEN` writer - Battery charging detection enable"]
pub type BCDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCDMEN` reader - Data connect detection mode enable"]
pub type DCDMEN_R = crate::BitReader;
#[doc = "Field `DCDMEN` writer - Data connect detection mode enable"]
pub type DCDMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDMEN` reader - Primary detection mode enable"]
pub type PDMEN_R = crate::BitReader;
#[doc = "Field `PDMEN` writer - Primary detection mode enable"]
pub type PDMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDMEN` reader - Secondary detection mode enable"]
pub type SDMEN_R = crate::BitReader;
#[doc = "Field `SDMEN` writer - Secondary detection mode enable"]
pub type SDMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWRON` reader - Power on"]
pub type PWRON_R = crate::BitReader;
#[doc = "Field `PWRON` writer - Power on"]
pub type PWRON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFOEN` reader - SOF output enable"]
pub type SOFOEN_R = crate::BitReader;
#[doc = "Field `SOFOEN` writer - SOF output enable"]
pub type SOFOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VDEN` reader - Enable of VBUS sensing comparator to detect VBUS valid"]
pub type VDEN_R = crate::BitReader;
#[doc = "Field `VDEN` writer - Enable of VBUS sensing comparator to detect VBUS valid"]
pub type VDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Data connect detection status"]
    #[inline(always)]
    pub fn dcdf(&self) -> DCDF_R {
        DCDF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Primary detection status"]
    #[inline(always)]
    pub fn pdf(&self) -> PDF_R {
        PDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secondary detection status"]
    #[inline(always)]
    pub fn sdf(&self) -> SDF_R {
        SDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PS2 detection status"]
    #[inline(always)]
    pub fn ps2f(&self) -> PS2F_R {
        PS2F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12 - Battery charging detection enable"]
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data connect detection mode enable"]
    #[inline(always)]
    pub fn dcdmen(&self) -> DCDMEN_R {
        DCDMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Primary detection mode enable"]
    #[inline(always)]
    pub fn pdmen(&self) -> PDMEN_R {
        PDMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Secondary detection mode enable"]
    #[inline(always)]
    pub fn sdmen(&self) -> SDMEN_R {
        SDMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Power on"]
    #[inline(always)]
    pub fn pwron(&self) -> PWRON_R {
        PWRON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofoen(&self) -> SOFOEN_R {
        SOFOEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable of VBUS sensing comparator to detect VBUS valid"]
    #[inline(always)]
    pub fn vden(&self) -> VDEN_R {
        VDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data connect detection status"]
    #[inline(always)]
    #[must_use]
    pub fn dcdf(&mut self) -> DCDF_W<GCCFG_SPEC, 0> {
        DCDF_W::new(self)
    }
    #[doc = "Bit 1 - Primary detection status"]
    #[inline(always)]
    #[must_use]
    pub fn pdf(&mut self) -> PDF_W<GCCFG_SPEC, 1> {
        PDF_W::new(self)
    }
    #[doc = "Bit 2 - Secondary detection status"]
    #[inline(always)]
    #[must_use]
    pub fn sdf(&mut self) -> SDF_W<GCCFG_SPEC, 2> {
        SDF_W::new(self)
    }
    #[doc = "Bit 3 - PS2 detection status"]
    #[inline(always)]
    #[must_use]
    pub fn ps2f(&mut self) -> PS2F_W<GCCFG_SPEC, 3> {
        PS2F_W::new(self)
    }
    #[doc = "Bit 12 - Battery charging detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn bcden(&mut self) -> BCDEN_W<GCCFG_SPEC, 12> {
        BCDEN_W::new(self)
    }
    #[doc = "Bit 13 - Data connect detection mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdmen(&mut self) -> DCDMEN_W<GCCFG_SPEC, 13> {
        DCDMEN_W::new(self)
    }
    #[doc = "Bit 14 - Primary detection mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdmen(&mut self) -> PDMEN_W<GCCFG_SPEC, 14> {
        PDMEN_W::new(self)
    }
    #[doc = "Bit 15 - Secondary detection mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmen(&mut self) -> SDMEN_W<GCCFG_SPEC, 15> {
        SDMEN_W::new(self)
    }
    #[doc = "Bit 16 - Power on"]
    #[inline(always)]
    #[must_use]
    pub fn pwron(&mut self) -> PWRON_W<GCCFG_SPEC, 16> {
        PWRON_W::new(self)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofoen(&mut self) -> SOFOEN_W<GCCFG_SPEC, 20> {
        SOFOEN_W::new(self)
    }
    #[doc = "Bit 21 - Enable of VBUS sensing comparator to detect VBUS valid"]
    #[inline(always)]
    #[must_use]
    pub fn vden(&mut self) -> VDEN_W<GCCFG_SPEC, 21> {
        VDEN_W::new(self)
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
#[doc = "Global core configuration register (USBFS_GCCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCCFG_SPEC;
impl crate::RegisterSpec for GCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gccfg::R`](R) reader structure"]
impl crate::Readable for GCCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gccfg::W`](W) writer structure"]
impl crate::Writable for GCCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCCFG to value 0"]
impl crate::Resettable for GCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
