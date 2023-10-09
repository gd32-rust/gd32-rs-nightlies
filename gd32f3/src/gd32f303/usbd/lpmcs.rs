#[doc = "Register `LPMCS` reader"]
pub type R = crate::R<LPMCS_SPEC>;
#[doc = "Register `LPMCS` writer"]
pub type W = crate::W<LPMCS_SPEC>;
#[doc = "Field `LPMEN` reader - LPM support enable"]
pub type LPMEN_R = crate::BitReader;
#[doc = "Field `LPMEN` writer - LPM support enable"]
pub type LPMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPMACK` reader - LPM token acknowledge enable"]
pub type LPMACK_R = crate::BitReader;
#[doc = "Field `LPMACK` writer - LPM token acknowledge enable"]
pub type LPMACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REMWK` reader - bRemoteWake value"]
pub type REMWK_R = crate::BitReader;
#[doc = "Field `REMWK` writer - bRemoteWake value"]
pub type REMWK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLSTAT` reader - bLinkState value"]
pub type BLSTAT_R = crate::FieldReader;
#[doc = "Field `BLSTAT` writer - bLinkState value"]
pub type BLSTAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwk(&self) -> REMWK_R {
        REMWK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - bLinkState value"]
    #[inline(always)]
    pub fn blstat(&self) -> BLSTAT_R {
        BLSTAT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmen(&mut self) -> LPMEN_W<LPMCS_SPEC, 0> {
        LPMEN_W::new(self)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmack(&mut self) -> LPMACK_W<LPMCS_SPEC, 1> {
        LPMACK_W::new(self)
    }
    #[doc = "Bit 3 - bRemoteWake value"]
    #[inline(always)]
    #[must_use]
    pub fn remwk(&mut self) -> REMWK_W<LPMCS_SPEC, 3> {
        REMWK_W::new(self)
    }
    #[doc = "Bits 4:7 - bLinkState value"]
    #[inline(always)]
    #[must_use]
    pub fn blstat(&mut self) -> BLSTAT_W<LPMCS_SPEC, 4> {
        BLSTAT_W::new(self)
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
#[doc = "USB LPM control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPMCS_SPEC;
impl crate::RegisterSpec for LPMCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpmcs::R`](R) reader structure"]
impl crate::Readable for LPMCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpmcs::W`](W) writer structure"]
impl crate::Writable for LPMCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMCS to value 0"]
impl crate::Resettable for LPMCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
