#[doc = "Register `TIMEOUT` reader"]
pub type R = crate::R<TIMEOUT_SPEC>;
#[doc = "Register `TIMEOUT` writer"]
pub type W = crate::W<TIMEOUT_SPEC>;
#[doc = "Field `BUSTOA` reader - Bus timeout A"]
pub type BUSTOA_R = crate::FieldReader<u16>;
#[doc = "Field `BUSTOA` writer - Bus timeout A"]
pub type BUSTOA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `TOIDLE` reader - Idle clock timeout detection"]
pub type TOIDLE_R = crate::BitReader;
#[doc = "Field `TOIDLE` writer - Idle clock timeout detection"]
pub type TOIDLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOEN` reader - Clock timeout detection enable"]
pub type TOEN_R = crate::BitReader;
#[doc = "Field `TOEN` writer - Clock timeout detection enable"]
pub type TOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUSTOB` reader - Bus timeout B"]
pub type BUSTOB_R = crate::FieldReader<u16>;
#[doc = "Field `BUSTOB` writer - Bus timeout B"]
pub type BUSTOB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `EXTOEN` reader - Extended clock timeout detection enable"]
pub type EXTOEN_R = crate::BitReader;
#[doc = "Field `EXTOEN` writer - Extended clock timeout detection enable"]
pub type EXTOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11 - Bus timeout A"]
    #[inline(always)]
    pub fn bustoa(&self) -> BUSTOA_R {
        BUSTOA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Idle clock timeout detection"]
    #[inline(always)]
    pub fn toidle(&self) -> TOIDLE_R {
        TOIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock timeout detection enable"]
    #[inline(always)]
    pub fn toen(&self) -> TOEN_R {
        TOEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Bus timeout B"]
    #[inline(always)]
    pub fn bustob(&self) -> BUSTOB_R {
        BUSTOB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Extended clock timeout detection enable"]
    #[inline(always)]
    pub fn extoen(&self) -> EXTOEN_R {
        EXTOEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bus timeout A"]
    #[inline(always)]
    #[must_use]
    pub fn bustoa(&mut self) -> BUSTOA_W<TIMEOUT_SPEC, 0> {
        BUSTOA_W::new(self)
    }
    #[doc = "Bit 12 - Idle clock timeout detection"]
    #[inline(always)]
    #[must_use]
    pub fn toidle(&mut self) -> TOIDLE_W<TIMEOUT_SPEC, 12> {
        TOIDLE_W::new(self)
    }
    #[doc = "Bit 15 - Clock timeout detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn toen(&mut self) -> TOEN_W<TIMEOUT_SPEC, 15> {
        TOEN_W::new(self)
    }
    #[doc = "Bits 16:27 - Bus timeout B"]
    #[inline(always)]
    #[must_use]
    pub fn bustob(&mut self) -> BUSTOB_W<TIMEOUT_SPEC, 16> {
        BUSTOB_W::new(self)
    }
    #[doc = "Bit 31 - Extended clock timeout detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn extoen(&mut self) -> EXTOEN_W<TIMEOUT_SPEC, 31> {
        EXTOEN_W::new(self)
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
#[doc = "timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEOUT_SPEC;
impl crate::RegisterSpec for TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout::R`](R) reader structure"]
impl crate::Readable for TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timeout::W`](W) writer structure"]
impl crate::Writable for TIMEOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMEOUT to value 0"]
impl crate::Resettable for TIMEOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
