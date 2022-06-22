#[doc = "Register `ADDAPB1EN` reader"]
pub struct R(crate::R<ADDAPB1EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDAPB1EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDAPB1EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDAPB1EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDAPB1EN` writer"]
pub struct W(crate::W<ADDAPB1EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDAPB1EN_SPEC>;
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
impl From<crate::W<ADDAPB1EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDAPB1EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTCEN` reader - CTC clock enable"]
pub type CTCEN_R = crate::BitReader<bool>;
#[doc = "Field `CTCEN` writer - CTC clock enable"]
pub type CTCEN_W<'a> = crate::BitWriter<'a, u32, ADDAPB1EN_SPEC, bool, 27>;
impl R {
    #[doc = "Bit 27 - CTC clock enable"]
    #[inline(always)]
    pub fn ctcen(&self) -> CTCEN_R {
        CTCEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - CTC clock enable"]
    #[inline(always)]
    pub fn ctcen(&mut self) -> CTCEN_W {
        CTCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 additional enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addapb1en](index.html) module"]
pub struct ADDAPB1EN_SPEC;
impl crate::RegisterSpec for ADDAPB1EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addapb1en::R](R) reader structure"]
impl crate::Readable for ADDAPB1EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addapb1en::W](W) writer structure"]
impl crate::Writable for ADDAPB1EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDAPB1EN to value 0"]
impl crate::Resettable for ADDAPB1EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
