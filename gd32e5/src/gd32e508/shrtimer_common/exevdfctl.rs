#[doc = "Register `EXEVDFCTL` reader"]
pub type R = crate::R<EXEVDFCTL_SPEC>;
#[doc = "Register `EXEVDFCTL` writer"]
pub type W = crate::W<EXEVDFCTL_SPEC>;
#[doc = "Field `EXEV5FC` reader - External event 5 filter control"]
pub type EXEV5FC_R = crate::FieldReader;
#[doc = "Field `EXEV5FC` writer - External event 5 filter control"]
pub type EXEV5FC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEV6FC` reader - External event 6 filter control"]
pub type EXEV6FC_R = crate::FieldReader;
#[doc = "Field `EXEV6FC` writer - External event 6 filter control"]
pub type EXEV6FC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEV7FC` reader - External event 7 filter control"]
pub type EXEV7FC_R = crate::FieldReader;
#[doc = "Field `EXEV7FC` writer - External event 7 filter control"]
pub type EXEV7FC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEV8FC` reader - External event 8 filter control"]
pub type EXEV8FC_R = crate::FieldReader;
#[doc = "Field `EXEV8FC` writer - External event 8 filter control"]
pub type EXEV8FC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEV9FC` reader - External event 9 filter control"]
pub type EXEV9FC_R = crate::FieldReader;
#[doc = "Field `EXEV9FC` writer - External event 9 filter control"]
pub type EXEV9FC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEVFDIV` reader - External event digital filter clock division"]
pub type EXEVFDIV_R = crate::FieldReader;
#[doc = "Field `EXEVFDIV` writer - External event digital filter clock division"]
pub type EXEVFDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:3 - External event 5 filter control"]
    #[inline(always)]
    pub fn exev5fc(&self) -> EXEV5FC_R {
        EXEV5FC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - External event 6 filter control"]
    #[inline(always)]
    pub fn exev6fc(&self) -> EXEV6FC_R {
        EXEV6FC_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External event 7 filter control"]
    #[inline(always)]
    pub fn exev7fc(&self) -> EXEV7FC_R {
        EXEV7FC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - External event 8 filter control"]
    #[inline(always)]
    pub fn exev8fc(&self) -> EXEV8FC_R {
        EXEV8FC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - External event 9 filter control"]
    #[inline(always)]
    pub fn exev9fc(&self) -> EXEV9FC_R {
        EXEV9FC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - External event digital filter clock division"]
    #[inline(always)]
    pub fn exevfdiv(&self) -> EXEVFDIV_R {
        EXEVFDIV_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External event 5 filter control"]
    #[inline(always)]
    #[must_use]
    pub fn exev5fc(&mut self) -> EXEV5FC_W<EXEVDFCTL_SPEC, 0> {
        EXEV5FC_W::new(self)
    }
    #[doc = "Bits 6:9 - External event 6 filter control"]
    #[inline(always)]
    #[must_use]
    pub fn exev6fc(&mut self) -> EXEV6FC_W<EXEVDFCTL_SPEC, 6> {
        EXEV6FC_W::new(self)
    }
    #[doc = "Bits 12:15 - External event 7 filter control"]
    #[inline(always)]
    #[must_use]
    pub fn exev7fc(&mut self) -> EXEV7FC_W<EXEVDFCTL_SPEC, 12> {
        EXEV7FC_W::new(self)
    }
    #[doc = "Bits 18:21 - External event 8 filter control"]
    #[inline(always)]
    #[must_use]
    pub fn exev8fc(&mut self) -> EXEV8FC_W<EXEVDFCTL_SPEC, 18> {
        EXEV8FC_W::new(self)
    }
    #[doc = "Bits 24:27 - External event 9 filter control"]
    #[inline(always)]
    #[must_use]
    pub fn exev9fc(&mut self) -> EXEV9FC_W<EXEVDFCTL_SPEC, 24> {
        EXEV9FC_W::new(self)
    }
    #[doc = "Bits 30:31 - External event digital filter clock division"]
    #[inline(always)]
    #[must_use]
    pub fn exevfdiv(&mut self) -> EXEVFDIV_W<EXEVDFCTL_SPEC, 30> {
        EXEVFDIV_W::new(self)
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
#[doc = "SHRTIMER external event digital filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exevdfctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exevdfctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXEVDFCTL_SPEC;
impl crate::RegisterSpec for EXEVDFCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exevdfctl::R`](R) reader structure"]
impl crate::Readable for EXEVDFCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exevdfctl::W`](W) writer structure"]
impl crate::Writable for EXEVDFCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXEVDFCTL to value 0"]
impl crate::Resettable for EXEVDFCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
