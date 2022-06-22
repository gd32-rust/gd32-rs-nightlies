#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHVSEL` reader - Write CHxVAL register selection"]
pub type CHVSEL_R = crate::BitReader<bool>;
#[doc = "Field `CHVSEL` writer - Write CHxVAL register selection"]
pub type CHVSEL_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 1>;
#[doc = "Field `OUTSEL` reader - The output value selection"]
pub type OUTSEL_R = crate::BitReader<bool>;
#[doc = "Field `OUTSEL` writer - The output value selection"]
pub type OUTSEL_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 1 - Write CHxVAL register selection"]
    #[inline(always)]
    pub fn chvsel(&self) -> CHVSEL_R {
        CHVSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - The output value selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write CHxVAL register selection"]
    #[inline(always)]
    pub fn chvsel(&mut self) -> CHVSEL_W {
        CHVSEL_W::new(self)
    }
    #[doc = "Bit 0 - The output value selection"]
    #[inline(always)]
    pub fn outsel(&mut self) -> OUTSEL_W {
        OUTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
