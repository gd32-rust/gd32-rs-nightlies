#[doc = "Register `ST2CSCTL` reader"]
pub type R = crate::R<ST2CSCTL_SPEC>;
#[doc = "Register `ST2CSCTL` writer"]
pub type W = crate::W<ST2CSCTL_SPEC>;
#[doc = "Field `CSPRD` reader - Carrier signal period"]
pub type CSPRD_R = crate::FieldReader;
#[doc = "Field `CSPRD` writer - Carrier signal period"]
pub type CSPRD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CSDTY` reader - Carrier signal duty cycle"]
pub type CSDTY_R = crate::FieldReader;
#[doc = "Field `CSDTY` writer - Carrier signal duty cycle"]
pub type CSDTY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSFSTPW` reader - First carrier-signal pulse width"]
pub type CSFSTPW_R = crate::FieldReader;
#[doc = "Field `CSFSTPW` writer - First carrier-signal pulse width"]
pub type CSFSTPW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Carrier signal period"]
    #[inline(always)]
    pub fn csprd(&self) -> CSPRD_R {
        CSPRD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Carrier signal duty cycle"]
    #[inline(always)]
    pub fn csdty(&self) -> CSDTY_R {
        CSDTY_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:10 - First carrier-signal pulse width"]
    #[inline(always)]
    pub fn csfstpw(&self) -> CSFSTPW_R {
        CSFSTPW_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Carrier signal period"]
    #[inline(always)]
    #[must_use]
    pub fn csprd(&mut self) -> CSPRD_W<ST2CSCTL_SPEC, 0> {
        CSPRD_W::new(self)
    }
    #[doc = "Bits 4:6 - Carrier signal duty cycle"]
    #[inline(always)]
    #[must_use]
    pub fn csdty(&mut self) -> CSDTY_W<ST2CSCTL_SPEC, 4> {
        CSDTY_W::new(self)
    }
    #[doc = "Bits 7:10 - First carrier-signal pulse width"]
    #[inline(always)]
    #[must_use]
    pub fn csfstpw(&mut self) -> CSFSTPW_W<ST2CSCTL_SPEC, 7> {
        CSFSTPW_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx carrier-signal control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2csctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2csctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST2CSCTL_SPEC;
impl crate::RegisterSpec for ST2CSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2csctl::R`](R) reader structure"]
impl crate::Readable for ST2CSCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st2csctl::W`](W) writer structure"]
impl crate::Writable for ST2CSCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST2CSCTL to value 0"]
impl crate::Resettable for ST2CSCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
