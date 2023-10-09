#[doc = "Register `PWRD` reader"]
pub type R = crate::R<PWRD_SPEC>;
#[doc = "Register `PWRD` writer"]
pub type W = crate::W<PWRD_SPEC>;
#[doc = "Field `ADPMEN` reader - ADP module enable"]
pub type ADPMEN_R = crate::BitReader;
#[doc = "Field `ADPMEN` writer - ADP module enable"]
pub type ADPMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADPF` reader - ADP event interrupt flag"]
pub type ADPF_R = crate::BitReader;
#[doc = "Field `ADPF` writer - ADP event interrupt flag"]
pub type ADPF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    pub fn adpmen(&self) -> ADPMEN_R {
        ADPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 23 - ADP event interrupt flag"]
    #[inline(always)]
    pub fn adpf(&self) -> ADPF_R {
        ADPF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    #[must_use]
    pub fn adpmen(&mut self) -> ADPMEN_W<PWRD_SPEC, 0> {
        ADPMEN_W::new(self)
    }
    #[doc = "Bit 23 - ADP event interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adpf(&mut self) -> ADPF_W<PWRD_SPEC, 23> {
        ADPF_W::new(self)
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
#[doc = "Power down register (USBHS_PWRD)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRD_SPEC;
impl crate::RegisterSpec for PWRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrd::R`](R) reader structure"]
impl crate::Readable for PWRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwrd::W`](W) writer structure"]
impl crate::Writable for PWRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRD to value 0"]
impl crate::Resettable for PWRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
