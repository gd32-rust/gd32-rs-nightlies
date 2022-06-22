#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCDON` reader - SLCD controller start"]
pub type SLCDON_R = crate::BitReader<bool>;
#[doc = "Field `SLCDON` writer - SLCD controller start"]
pub type SLCDON_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 0>;
#[doc = "Field `VSRC` reader - SLCD voltage source"]
pub type VSRC_R = crate::BitReader<bool>;
#[doc = "Field `VSRC` writer - SLCD voltage source"]
pub type VSRC_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 1>;
#[doc = "Field `DUTY` reader - Duty select"]
pub type DUTY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUTY` writer - Duty select"]
pub type DUTY_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 3, 2>;
#[doc = "Field `BIAS` reader - Bias select"]
pub type BIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIAS` writer - Bias select"]
pub type BIAS_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, 5>;
#[doc = "Field `COMS` reader - Common/segment padselect"]
pub type COMS_R = crate::BitReader<bool>;
#[doc = "Field `COMS` writer - Common/segment padselect"]
pub type COMS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 7>;
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
    pub fn slcdon(&mut self) -> SLCDON_W {
        SLCDON_W::new(self)
    }
    #[doc = "Bit 1 - SLCD voltage source"]
    #[inline(always)]
    pub fn vsrc(&mut self) -> VSRC_W {
        VSRC_W::new(self)
    }
    #[doc = "Bits 2:4 - Duty select"]
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W {
        DUTY_W::new(self)
    }
    #[doc = "Bits 5:6 - Bias select"]
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W {
        BIAS_W::new(self)
    }
    #[doc = "Bit 7 - Common/segment padselect"]
    #[inline(always)]
    pub fn coms(&mut self) -> COMS_W {
        COMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
