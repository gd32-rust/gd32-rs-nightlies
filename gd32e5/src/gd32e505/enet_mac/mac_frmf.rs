#[doc = "Register `MAC_FRMF` reader"]
pub type R = crate::R<MAC_FRMF_SPEC>;
#[doc = "Register `MAC_FRMF` writer"]
pub type W = crate::W<MAC_FRMF_SPEC>;
#[doc = "Field `PM` reader - Promiscuous mode"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - Promiscuous mode"]
pub type PM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HUF` reader - Hash unicast filter"]
pub type HUF_R = crate::BitReader;
#[doc = "Field `HUF` writer - Hash unicast filter"]
pub type HUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HMF` reader - Hash multicast filter"]
pub type HMF_R = crate::BitReader;
#[doc = "Field `HMF` writer - Hash multicast filter"]
pub type HMF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAIFLT` reader - Destination address inverse filtering"]
pub type DAIFLT_R = crate::BitReader;
#[doc = "Field `DAIFLT` writer - Destination address inverse filtering"]
pub type DAIFLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MFD` reader - multicast filter disable"]
pub type MFD_R = crate::BitReader;
#[doc = "Field `MFD` writer - multicast filter disable"]
pub type MFD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BFRMD` reader - Broadcast frames disable"]
pub type BFRMD_R = crate::BitReader;
#[doc = "Field `BFRMD` writer - Broadcast frames disable"]
pub type BFRMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCFRM` reader - Pass control frames"]
pub type PCFRM_R = crate::FieldReader;
#[doc = "Field `PCFRM` writer - Pass control frames"]
pub type PCFRM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SAIFLT` reader - Source address inverse filtering"]
pub type SAIFLT_R = crate::BitReader;
#[doc = "Field `SAIFLT` writer - Source address inverse filtering"]
pub type SAIFLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAFLT` reader - Source address filter"]
pub type SAFLT_R = crate::BitReader;
#[doc = "Field `SAFLT` writer - Source address filter"]
pub type SAFLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPFLT` reader - Hash or perfect filter"]
pub type HPFLT_R = crate::BitReader;
#[doc = "Field `HPFLT` writer - Hash or perfect filter"]
pub type HPFLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAR` reader - Frames all receive"]
pub type FAR_R = crate::BitReader;
#[doc = "Field `FAR` writer - Frames all receive"]
pub type FAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash unicast filter"]
    #[inline(always)]
    pub fn huf(&self) -> HUF_R {
        HUF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash multicast filter"]
    #[inline(always)]
    pub fn hmf(&self) -> HMF_R {
        HMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    pub fn daiflt(&self) -> DAIFLT_R {
        DAIFLT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - multicast filter disable"]
    #[inline(always)]
    pub fn mfd(&self) -> MFD_R {
        MFD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    pub fn bfrmd(&self) -> BFRMD_R {
        BFRMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcfrm(&self) -> PCFRM_R {
        PCFRM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saiflt(&self) -> SAIFLT_R {
        SAIFLT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    pub fn saflt(&self) -> SAFLT_R {
        SAFLT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpflt(&self) -> HPFLT_R {
        HPFLT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 31 - Frames all receive"]
    #[inline(always)]
    pub fn far(&self) -> FAR_R {
        FAR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<MAC_FRMF_SPEC, 0> {
        PM_W::new(self)
    }
    #[doc = "Bit 1 - Hash unicast filter"]
    #[inline(always)]
    #[must_use]
    pub fn huf(&mut self) -> HUF_W<MAC_FRMF_SPEC, 1> {
        HUF_W::new(self)
    }
    #[doc = "Bit 2 - Hash multicast filter"]
    #[inline(always)]
    #[must_use]
    pub fn hmf(&mut self) -> HMF_W<MAC_FRMF_SPEC, 2> {
        HMF_W::new(self)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    #[must_use]
    pub fn daiflt(&mut self) -> DAIFLT_W<MAC_FRMF_SPEC, 3> {
        DAIFLT_W::new(self)
    }
    #[doc = "Bit 4 - multicast filter disable"]
    #[inline(always)]
    #[must_use]
    pub fn mfd(&mut self) -> MFD_W<MAC_FRMF_SPEC, 4> {
        MFD_W::new(self)
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    #[must_use]
    pub fn bfrmd(&mut self) -> BFRMD_W<MAC_FRMF_SPEC, 5> {
        BFRMD_W::new(self)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    #[must_use]
    pub fn pcfrm(&mut self) -> PCFRM_W<MAC_FRMF_SPEC, 6> {
        PCFRM_W::new(self)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    #[must_use]
    pub fn saiflt(&mut self) -> SAIFLT_W<MAC_FRMF_SPEC, 8> {
        SAIFLT_W::new(self)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    #[must_use]
    pub fn saflt(&mut self) -> SAFLT_W<MAC_FRMF_SPEC, 9> {
        SAFLT_W::new(self)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    #[must_use]
    pub fn hpflt(&mut self) -> HPFLT_W<MAC_FRMF_SPEC, 10> {
        HPFLT_W::new(self)
    }
    #[doc = "Bit 31 - Frames all receive"]
    #[inline(always)]
    #[must_use]
    pub fn far(&mut self) -> FAR_W<MAC_FRMF_SPEC, 31> {
        FAR_W::new(self)
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
#[doc = "Ethernet MAC frame filter register (MAC_FRMF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_frmf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_frmf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_FRMF_SPEC;
impl crate::RegisterSpec for MAC_FRMF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_frmf::R`](R) reader structure"]
impl crate::Readable for MAC_FRMF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_frmf::W`](W) writer structure"]
impl crate::Writable for MAC_FRMF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_FRMF to value 0"]
impl crate::Resettable for MAC_FRMF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
