#[doc = "Register `CMDCTL` reader"]
pub struct R(crate::R<CMDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDCTL` writer"]
pub struct W(crate::W<CMDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDCTL_SPEC>;
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
impl From<crate::W<CMDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDIDX` reader - Command index"]
pub type CMDIDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDIDX` writer - Command index"]
pub type CMDIDX_W<'a> = crate::FieldWriter<'a, u32, CMDCTL_SPEC, u8, u8, 6, 0>;
#[doc = "Field `CMDRESP` reader - Command response type bits"]
pub type CMDRESP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDRESP` writer - Command response type bits"]
pub type CMDRESP_W<'a> = crate::FieldWriter<'a, u32, CMDCTL_SPEC, u8, u8, 2, 6>;
#[doc = "Field `INTWAIT` reader - Interrupt wait instead of timeout"]
pub type INTWAIT_R = crate::BitReader<bool>;
#[doc = "Field `INTWAIT` writer - Interrupt wait instead of timeout"]
pub type INTWAIT_W<'a> = crate::BitWriter<'a, u32, CMDCTL_SPEC, bool, 8>;
#[doc = "Field `WAITDEND` reader - Waits for ends of data transfer"]
pub type WAITDEND_R = crate::BitReader<bool>;
#[doc = "Field `WAITDEND` writer - Waits for ends of data transfer"]
pub type WAITDEND_W<'a> = crate::BitWriter<'a, u32, CMDCTL_SPEC, bool, 9>;
#[doc = "Field `CSMEN` reader - Command state machine (CSM) enable bit"]
pub type CSMEN_R = crate::BitReader<bool>;
#[doc = "Field `CSMEN` writer - Command state machine (CSM) enable bit"]
pub type CSMEN_W<'a> = crate::BitWriter<'a, u32, CMDCTL_SPEC, bool, 10>;
#[doc = "Field `SUSPEND` reader - SD I/O suspend command(SD I/O only)"]
pub type SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `SUSPEND` writer - SD I/O suspend command(SD I/O only)"]
pub type SUSPEND_W<'a> = crate::BitWriter<'a, u32, CMDCTL_SPEC, bool, 11>;
#[doc = "Field `ENCMDC` reader - CMD completion signal enabled (CE-ATA only)"]
pub type ENCMDC_R = crate::BitReader<bool>;
#[doc = "Field `ENCMDC` writer - CMD completion signal enabled (CE-ATA only)"]
pub type ENCMDC_W<'a> = crate::BitWriter<'a, u32, CMDCTL_SPEC, bool, 12>;
#[doc = "Field `NINTEN` reader - No CE-ATA Interrupt (CE-ATA only)"]
pub type NINTEN_R = crate::BitReader<bool>;
#[doc = "Field `NINTEN` writer - No CE-ATA Interrupt (CE-ATA only)"]
pub type NINTEN_W<'a> = crate::BitWriter<'a, u32, CMDCTL_SPEC, bool, 13>;
#[doc = "Field `ATAEN` reader - CE-ATA command enable(CE-ATA only)"]
pub type ATAEN_R = crate::BitReader<bool>;
#[doc = "Field `ATAEN` writer - CE-ATA command enable(CE-ATA only)"]
pub type ATAEN_W<'a> = crate::BitWriter<'a, u32, CMDCTL_SPEC, bool, 14>;
impl R {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CMDIDX_R {
        CMDIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Command response type bits"]
    #[inline(always)]
    pub fn cmdresp(&self) -> CMDRESP_R {
        CMDRESP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Interrupt wait instead of timeout"]
    #[inline(always)]
    pub fn intwait(&self) -> INTWAIT_R {
        INTWAIT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Waits for ends of data transfer"]
    #[inline(always)]
    pub fn waitdend(&self) -> WAITDEND_R {
        WAITDEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Command state machine (CSM) enable bit"]
    #[inline(always)]
    pub fn csmen(&self) -> CSMEN_R {
        CSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O suspend command(SD I/O only)"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CMD completion signal enabled (CE-ATA only)"]
    #[inline(always)]
    pub fn encmdc(&self) -> ENCMDC_R {
        ENCMDC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - No CE-ATA Interrupt (CE-ATA only)"]
    #[inline(always)]
    pub fn ninten(&self) -> NINTEN_R {
        NINTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CE-ATA command enable(CE-ATA only)"]
    #[inline(always)]
    pub fn ataen(&self) -> ATAEN_R {
        ATAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdidx(&mut self) -> CMDIDX_W {
        CMDIDX_W::new(self)
    }
    #[doc = "Bits 6:7 - Command response type bits"]
    #[inline(always)]
    pub fn cmdresp(&mut self) -> CMDRESP_W {
        CMDRESP_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt wait instead of timeout"]
    #[inline(always)]
    pub fn intwait(&mut self) -> INTWAIT_W {
        INTWAIT_W::new(self)
    }
    #[doc = "Bit 9 - Waits for ends of data transfer"]
    #[inline(always)]
    pub fn waitdend(&mut self) -> WAITDEND_W {
        WAITDEND_W::new(self)
    }
    #[doc = "Bit 10 - Command state machine (CSM) enable bit"]
    #[inline(always)]
    pub fn csmen(&mut self) -> CSMEN_W {
        CSMEN_W::new(self)
    }
    #[doc = "Bit 11 - SD I/O suspend command(SD I/O only)"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W::new(self)
    }
    #[doc = "Bit 12 - CMD completion signal enabled (CE-ATA only)"]
    #[inline(always)]
    pub fn encmdc(&mut self) -> ENCMDC_W {
        ENCMDC_W::new(self)
    }
    #[doc = "Bit 13 - No CE-ATA Interrupt (CE-ATA only)"]
    #[inline(always)]
    pub fn ninten(&mut self) -> NINTEN_W {
        NINTEN_W::new(self)
    }
    #[doc = "Bit 14 - CE-ATA command enable(CE-ATA only)"]
    #[inline(always)]
    pub fn ataen(&mut self) -> ATAEN_W {
        ATAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdctl](index.html) module"]
pub struct CMDCTL_SPEC;
impl crate::RegisterSpec for CMDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdctl::R](R) reader structure"]
impl crate::Readable for CMDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdctl::W](W) writer structure"]
impl crate::Writable for CMDCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDCTL to value 0"]
impl crate::Resettable for CMDCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
