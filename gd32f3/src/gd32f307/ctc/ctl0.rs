#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKOKIE` reader - Clock trim ok interrupt enable"]
pub type CKOKIE_R = crate::BitReader<bool>;
#[doc = "Field `CKOKIE` writer - Clock trim ok interrupt enable"]
pub type CKOKIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 0>;
#[doc = "Field `CKWARNIE` reader - Clock trim warning interrupt enable"]
pub type CKWARNIE_R = crate::BitReader<bool>;
#[doc = "Field `CKWARNIE` writer - Clock trim warning interrupt enable"]
pub type CKWARNIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 1>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 2>;
#[doc = "Field `EREFIE` reader - EREFIF interrupt enable"]
pub type EREFIE_R = crate::BitReader<bool>;
#[doc = "Field `EREFIE` writer - EREFIF interrupt enable"]
pub type EREFIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 3>;
#[doc = "Field `CNTEN` reader - CTC counter enable"]
pub type CNTEN_R = crate::BitReader<bool>;
#[doc = "Field `CNTEN` writer - CTC counter enable"]
pub type CNTEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 5>;
#[doc = "Field `AUTOTRIM` reader - Hardware automatically trim mode"]
pub type AUTOTRIM_R = crate::BitReader<bool>;
#[doc = "Field `AUTOTRIM` writer - Hardware automatically trim mode"]
pub type AUTOTRIM_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 6>;
#[doc = "Field `SWREFPUL` reader - Software reference source sync pulse"]
pub type SWREFPUL_R = crate::BitReader<bool>;
#[doc = "Field `SWREFPUL` writer - Software reference source sync pulse"]
pub type SWREFPUL_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 7>;
#[doc = "Field `TRIMVALUE` reader - IRC48M trim value"]
pub type TRIMVALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMVALUE` writer - IRC48M trim value"]
pub type TRIMVALUE_W<'a> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 6, 8>;
impl R {
    #[doc = "Bit 0 - Clock trim ok interrupt enable"]
    #[inline(always)]
    pub fn ckokie(&self) -> CKOKIE_R {
        CKOKIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock trim warning interrupt enable"]
    #[inline(always)]
    pub fn ckwarnie(&self) -> CKWARNIE_R {
        CKWARNIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EREFIF interrupt enable"]
    #[inline(always)]
    pub fn erefie(&self) -> EREFIE_R {
        EREFIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - CTC counter enable"]
    #[inline(always)]
    pub fn cnten(&self) -> CNTEN_R {
        CNTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hardware automatically trim mode"]
    #[inline(always)]
    pub fn autotrim(&self) -> AUTOTRIM_R {
        AUTOTRIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software reference source sync pulse"]
    #[inline(always)]
    pub fn swrefpul(&self) -> SWREFPUL_R {
        SWREFPUL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - IRC48M trim value"]
    #[inline(always)]
    pub fn trimvalue(&self) -> TRIMVALUE_R {
        TRIMVALUE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock trim ok interrupt enable"]
    #[inline(always)]
    pub fn ckokie(&mut self) -> CKOKIE_W {
        CKOKIE_W::new(self)
    }
    #[doc = "Bit 1 - Clock trim warning interrupt enable"]
    #[inline(always)]
    pub fn ckwarnie(&mut self) -> CKWARNIE_W {
        CKWARNIE_W::new(self)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 3 - EREFIF interrupt enable"]
    #[inline(always)]
    pub fn erefie(&mut self) -> EREFIE_W {
        EREFIE_W::new(self)
    }
    #[doc = "Bit 5 - CTC counter enable"]
    #[inline(always)]
    pub fn cnten(&mut self) -> CNTEN_W {
        CNTEN_W::new(self)
    }
    #[doc = "Bit 6 - Hardware automatically trim mode"]
    #[inline(always)]
    pub fn autotrim(&mut self) -> AUTOTRIM_W {
        AUTOTRIM_W::new(self)
    }
    #[doc = "Bit 7 - Software reference source sync pulse"]
    #[inline(always)]
    pub fn swrefpul(&mut self) -> SWREFPUL_W {
        SWREFPUL_W::new(self)
    }
    #[doc = "Bits 8:13 - IRC48M trim value"]
    #[inline(always)]
    pub fn trimvalue(&mut self) -> TRIMVALUE_W {
        TRIMVALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL0 to value 0x2000"]
impl crate::Resettable for CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
