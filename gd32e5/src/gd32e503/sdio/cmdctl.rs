#[doc = "Register `CMDCTL` reader"]
pub type R = crate::R<CMDCTL_SPEC>;
#[doc = "Register `CMDCTL` writer"]
pub type W = crate::W<CMDCTL_SPEC>;
#[doc = "Field `CMDIDX` reader - Command index"]
pub type CMDIDX_R = crate::FieldReader;
#[doc = "Field `CMDIDX` writer - Command index"]
pub type CMDIDX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CMDRESP` reader - Command response type bits"]
pub type CMDRESP_R = crate::FieldReader;
#[doc = "Field `CMDRESP` writer - Command response type bits"]
pub type CMDRESP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `INTWAIT` reader - Interrupt wait instead of timeout"]
pub type INTWAIT_R = crate::BitReader;
#[doc = "Field `INTWAIT` writer - Interrupt wait instead of timeout"]
pub type INTWAIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAITDEND` reader - Waits for ends of data transfer"]
pub type WAITDEND_R = crate::BitReader;
#[doc = "Field `WAITDEND` writer - Waits for ends of data transfer"]
pub type WAITDEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSMEN` reader - Command state machine (CSM) enable bit"]
pub type CSMEN_R = crate::BitReader;
#[doc = "Field `CSMEN` writer - Command state machine (CSM) enable bit"]
pub type CSMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SUSPEND` reader - SD I/O suspend command(SD I/O only)"]
pub type SUSPEND_R = crate::BitReader;
#[doc = "Field `SUSPEND` writer - SD I/O suspend command(SD I/O only)"]
pub type SUSPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENCMDC` reader - CMD completion signal enabled (CE-ATA only)"]
pub type ENCMDC_R = crate::BitReader;
#[doc = "Field `ENCMDC` writer - CMD completion signal enabled (CE-ATA only)"]
pub type ENCMDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NINTEN` reader - No CE-ATA Interrupt (CE-ATA only)"]
pub type NINTEN_R = crate::BitReader;
#[doc = "Field `NINTEN` writer - No CE-ATA Interrupt (CE-ATA only)"]
pub type NINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ATAEN` reader - CE-ATA command enable(CE-ATA only)"]
pub type ATAEN_R = crate::BitReader;
#[doc = "Field `ATAEN` writer - CE-ATA command enable(CE-ATA only)"]
pub type ATAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[must_use]
    pub fn cmdidx(&mut self) -> CMDIDX_W<CMDCTL_SPEC, 0> {
        CMDIDX_W::new(self)
    }
    #[doc = "Bits 6:7 - Command response type bits"]
    #[inline(always)]
    #[must_use]
    pub fn cmdresp(&mut self) -> CMDRESP_W<CMDCTL_SPEC, 6> {
        CMDRESP_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt wait instead of timeout"]
    #[inline(always)]
    #[must_use]
    pub fn intwait(&mut self) -> INTWAIT_W<CMDCTL_SPEC, 8> {
        INTWAIT_W::new(self)
    }
    #[doc = "Bit 9 - Waits for ends of data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn waitdend(&mut self) -> WAITDEND_W<CMDCTL_SPEC, 9> {
        WAITDEND_W::new(self)
    }
    #[doc = "Bit 10 - Command state machine (CSM) enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn csmen(&mut self) -> CSMEN_W<CMDCTL_SPEC, 10> {
        CSMEN_W::new(self)
    }
    #[doc = "Bit 11 - SD I/O suspend command(SD I/O only)"]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SUSPEND_W<CMDCTL_SPEC, 11> {
        SUSPEND_W::new(self)
    }
    #[doc = "Bit 12 - CMD completion signal enabled (CE-ATA only)"]
    #[inline(always)]
    #[must_use]
    pub fn encmdc(&mut self) -> ENCMDC_W<CMDCTL_SPEC, 12> {
        ENCMDC_W::new(self)
    }
    #[doc = "Bit 13 - No CE-ATA Interrupt (CE-ATA only)"]
    #[inline(always)]
    #[must_use]
    pub fn ninten(&mut self) -> NINTEN_W<CMDCTL_SPEC, 13> {
        NINTEN_W::new(self)
    }
    #[doc = "Bit 14 - CE-ATA command enable(CE-ATA only)"]
    #[inline(always)]
    #[must_use]
    pub fn ataen(&mut self) -> ATAEN_W<CMDCTL_SPEC, 14> {
        ATAEN_W::new(self)
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
#[doc = "Command control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDCTL_SPEC;
impl crate::RegisterSpec for CMDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdctl::R`](R) reader structure"]
impl crate::Readable for CMDCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmdctl::W`](W) writer structure"]
impl crate::Writable for CMDCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDCTL to value 0"]
impl crate::Resettable for CMDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
