#[doc = "Register `MAC_FRMF` reader"]
pub struct R(crate::R<MAC_FRMF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_FRMF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_FRMF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_FRMF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_FRMF` writer"]
pub struct W(crate::W<MAC_FRMF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_FRMF_SPEC>;
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
impl From<crate::W<MAC_FRMF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_FRMF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PM` reader - Promiscuous mode"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - Promiscuous mode"]
pub type PM_W<'a> = crate::BitWriter<'a, u32, MAC_FRMF_SPEC, bool, 0>;
#[doc = "Field `HUF` reader - Hash unicast filter"]
pub type HUF_R = crate::BitReader<bool>;
#[doc = "Field `HUF` writer - Hash unicast filter"]
pub type HUF_W<'a> = crate::BitWriter<'a, u32, MAC_FRMF_SPEC, bool, 1>;
#[doc = "Field `HMF` reader - Hash multicast filter"]
pub type HMF_R = crate::BitReader<bool>;
#[doc = "Field `HMF` writer - Hash multicast filter"]
pub type HMF_W<'a> = crate::BitWriter<'a, u32, MAC_FRMF_SPEC, bool, 2>;
#[doc = "Field `DAIFLT` reader - Destination address inverse filtering"]
pub type DAIFLT_R = crate::BitReader<bool>;
#[doc = "Field `DAIFLT` writer - Destination address inverse filtering"]
pub type DAIFLT_W<'a> = crate::BitWriter<'a, u32, MAC_FRMF_SPEC, bool, 3>;
#[doc = "Field `MFD` reader - multicast filter disable"]
pub type MFD_R = crate::BitReader<bool>;
#[doc = "Field `MFD` writer - multicast filter disable"]
pub type MFD_W<'a> = crate::BitWriter<'a, u32, MAC_FRMF_SPEC, bool, 4>;
#[doc = "Field `BFRMD` reader - Broadcast frames disable"]
pub type BFRMD_R = crate::BitReader<bool>;
#[doc = "Field `BFRMD` writer - Broadcast frames disable"]
pub type BFRMD_W<'a> = crate::BitWriter<'a, u32, MAC_FRMF_SPEC, bool, 5>;
#[doc = "Field `PCFRM` reader - Pass control frames"]
pub type PCFRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFRM` writer - Pass control frames"]
pub type PCFRM_W<'a> = crate::FieldWriter<'a, u32, MAC_FRMF_SPEC, u8, u8, 2, 6>;
#[doc = "Field `SAIFLT` reader - Source address inverse filtering"]
pub type SAIFLT_R = crate::BitReader<bool>;
#[doc = "Field `SAIFLT` writer - Source address inverse filtering"]
pub type SAIFLT_W<'a> = crate::BitWriter<'a, u32, MAC_FRMF_SPEC, bool, 8>;
#[doc = "Field `SAFLT` reader - Source address filter"]
pub type SAFLT_R = crate::BitReader<bool>;
#[doc = "Field `SAFLT` writer - Source address filter"]
pub type SAFLT_W<'a> = crate::BitWriter<'a, u32, MAC_FRMF_SPEC, bool, 9>;
#[doc = "Field `HPFLT` reader - Hash or perfect filter"]
pub type HPFLT_R = crate::BitReader<bool>;
#[doc = "Field `HPFLT` writer - Hash or perfect filter"]
pub type HPFLT_W<'a> = crate::BitWriter<'a, u32, MAC_FRMF_SPEC, bool, 10>;
#[doc = "Field `FAR` reader - Frames all receive"]
pub type FAR_R = crate::BitReader<bool>;
#[doc = "Field `FAR` writer - Frames all receive"]
pub type FAR_W<'a> = crate::BitWriter<'a, u32, MAC_FRMF_SPEC, bool, 31>;
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
    pub fn pm(&mut self) -> PM_W {
        PM_W::new(self)
    }
    #[doc = "Bit 1 - Hash unicast filter"]
    #[inline(always)]
    pub fn huf(&mut self) -> HUF_W {
        HUF_W::new(self)
    }
    #[doc = "Bit 2 - Hash multicast filter"]
    #[inline(always)]
    pub fn hmf(&mut self) -> HMF_W {
        HMF_W::new(self)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    pub fn daiflt(&mut self) -> DAIFLT_W {
        DAIFLT_W::new(self)
    }
    #[doc = "Bit 4 - multicast filter disable"]
    #[inline(always)]
    pub fn mfd(&mut self) -> MFD_W {
        MFD_W::new(self)
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    pub fn bfrmd(&mut self) -> BFRMD_W {
        BFRMD_W::new(self)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcfrm(&mut self) -> PCFRM_W {
        PCFRM_W::new(self)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saiflt(&mut self) -> SAIFLT_W {
        SAIFLT_W::new(self)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    pub fn saflt(&mut self) -> SAFLT_W {
        SAFLT_W::new(self)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpflt(&mut self) -> HPFLT_W {
        HPFLT_W::new(self)
    }
    #[doc = "Bit 31 - Frames all receive"]
    #[inline(always)]
    pub fn far(&mut self) -> FAR_W {
        FAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC frame filter register (MAC_FRMF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_frmf](index.html) module"]
pub struct MAC_FRMF_SPEC;
impl crate::RegisterSpec for MAC_FRMF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_frmf::R](R) reader structure"]
impl crate::Readable for MAC_FRMF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_frmf::W](W) writer structure"]
impl crate::Writable for MAC_FRMF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_FRMF to value 0"]
impl crate::Resettable for MAC_FRMF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
