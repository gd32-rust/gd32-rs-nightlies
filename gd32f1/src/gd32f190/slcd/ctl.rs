#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `SLCDON` reader - SLCD controller start"]
pub type SLCDON_R = crate::BitReader;
#[doc = "Field `SLCDON` writer - SLCD controller start"]
pub type SLCDON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VSRC` reader - SLCD voltage source"]
pub type VSRC_R = crate::BitReader;
#[doc = "Field `VSRC` writer - SLCD voltage source"]
pub type VSRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY` reader - Duty select"]
pub type DUTY_R = crate::FieldReader;
#[doc = "Field `DUTY` writer - Duty select"]
pub type DUTY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BIAS` reader - Bias select"]
pub type BIAS_R = crate::FieldReader;
#[doc = "Field `BIAS` writer - Bias select"]
pub type BIAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `COMS` reader - Common/segment padselect"]
pub type COMS_R = crate::BitReader;
#[doc = "Field `COMS` writer - Common/segment padselect"]
pub type COMS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SLCD controller start"]
    #[inline(always)]
    pub fn slcdon(&self) -> SLCDON_R {
        SLCDON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SLCD voltage source"]
    #[inline(always)]
    pub fn vsrc(&self) -> VSRC_R {
        VSRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Duty select"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - Bias select"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Common/segment padselect"]
    #[inline(always)]
    pub fn coms(&self) -> COMS_R {
        COMS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SLCD controller start"]
    #[inline(always)]
    #[must_use]
    pub fn slcdon(&mut self) -> SLCDON_W<CTL_SPEC, 0> {
        SLCDON_W::new(self)
    }
    #[doc = "Bit 1 - SLCD voltage source"]
    #[inline(always)]
    #[must_use]
    pub fn vsrc(&mut self) -> VSRC_W<CTL_SPEC, 1> {
        VSRC_W::new(self)
    }
    #[doc = "Bits 2:4 - Duty select"]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<CTL_SPEC, 2> {
        DUTY_W::new(self)
    }
    #[doc = "Bits 5:6 - Bias select"]
    #[inline(always)]
    #[must_use]
    pub fn bias(&mut self) -> BIAS_W<CTL_SPEC, 5> {
        BIAS_W::new(self)
    }
    #[doc = "Bit 7 - Common/segment padselect"]
    #[inline(always)]
    #[must_use]
    pub fn coms(&mut self) -> COMS_W<CTL_SPEC, 7> {
        COMS_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
