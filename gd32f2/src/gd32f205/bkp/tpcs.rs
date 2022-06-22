#[doc = "Register `TPCS` reader"]
pub struct R(crate::R<TPCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPCS` writer"]
pub struct W(crate::W<TPCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPCS_SPEC>;
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
impl From<crate::W<TPCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIF1` reader - Tamper1 interrupt flag"]
pub type TIF1_R = crate::BitReader<bool>;
#[doc = "Field `TIF1` writer - Tamper1 interrupt flag"]
pub type TIF1_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 15>;
#[doc = "Field `TEF1` reader - Tamper1 event flag"]
pub type TEF1_R = crate::BitReader<bool>;
#[doc = "Field `TEF1` writer - Tamper1 event flag"]
pub type TEF1_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 14>;
#[doc = "Field `TIF0` reader - Tamper0 interrupt flag"]
pub type TIF0_R = crate::BitReader<bool>;
#[doc = "Field `TIF0` writer - Tamper0 interrupt flag"]
pub type TIF0_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 9>;
#[doc = "Field `TEF0` reader - Tamper0 event flag"]
pub type TEF0_R = crate::BitReader<bool>;
#[doc = "Field `TEF0` writer - Tamper0 event flag"]
pub type TEF0_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 8>;
#[doc = "Field `TPIE1` reader - Tamper1interrupt enable"]
pub type TPIE1_R = crate::BitReader<bool>;
#[doc = "Field `TPIE1` writer - Tamper1interrupt enable"]
pub type TPIE1_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 7>;
#[doc = "Field `TIR1` reader - Tamper1 interrupt reset"]
pub type TIR1_R = crate::BitReader<bool>;
#[doc = "Field `TIR1` writer - Tamper1 interrupt reset"]
pub type TIR1_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 6>;
#[doc = "Field `TER1` reader - Tamper1 event reset"]
pub type TER1_R = crate::BitReader<bool>;
#[doc = "Field `TER1` writer - Tamper1 event reset"]
pub type TER1_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 5>;
#[doc = "Field `TPIE0` reader - Tamper0 interrupt enable"]
pub type TPIE0_R = crate::BitReader<bool>;
#[doc = "Field `TPIE0` writer - Tamper0 interrupt enable"]
pub type TPIE0_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 2>;
#[doc = "Field `TIR0` reader - Tamper0 interrupt reset"]
pub type TIR0_R = crate::BitReader<bool>;
#[doc = "Field `TIR0` writer - Tamper0 interrupt reset"]
pub type TIR0_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 1>;
#[doc = "Field `TER0` reader - Tamper0 event reset"]
pub type TER0_R = crate::BitReader<bool>;
#[doc = "Field `TER0` writer - Tamper0 event reset"]
pub type TER0_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 15 - Tamper1 interrupt flag"]
    #[inline(always)]
    pub fn tif1(&self) -> TIF1_R {
        TIF1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Tamper1 event flag"]
    #[inline(always)]
    pub fn tef1(&self) -> TEF1_R {
        TEF1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper0 interrupt flag"]
    #[inline(always)]
    pub fn tif0(&self) -> TIF0_R {
        TIF0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper0 event flag"]
    #[inline(always)]
    pub fn tef0(&self) -> TEF0_R {
        TEF0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Tamper1interrupt enable"]
    #[inline(always)]
    pub fn tpie1(&self) -> TPIE1_R {
        TPIE1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Tamper1 interrupt reset"]
    #[inline(always)]
    pub fn tir1(&self) -> TIR1_R {
        TIR1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Tamper1 event reset"]
    #[inline(always)]
    pub fn ter1(&self) -> TER1_R {
        TER1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper0 interrupt enable"]
    #[inline(always)]
    pub fn tpie0(&self) -> TPIE0_R {
        TPIE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper0 interrupt reset"]
    #[inline(always)]
    pub fn tir0(&self) -> TIR0_R {
        TIR0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Tamper0 event reset"]
    #[inline(always)]
    pub fn ter0(&self) -> TER0_R {
        TER0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Tamper1 interrupt flag"]
    #[inline(always)]
    pub fn tif1(&mut self) -> TIF1_W {
        TIF1_W::new(self)
    }
    #[doc = "Bit 14 - Tamper1 event flag"]
    #[inline(always)]
    pub fn tef1(&mut self) -> TEF1_W {
        TEF1_W::new(self)
    }
    #[doc = "Bit 9 - Tamper0 interrupt flag"]
    #[inline(always)]
    pub fn tif0(&mut self) -> TIF0_W {
        TIF0_W::new(self)
    }
    #[doc = "Bit 8 - Tamper0 event flag"]
    #[inline(always)]
    pub fn tef0(&mut self) -> TEF0_W {
        TEF0_W::new(self)
    }
    #[doc = "Bit 7 - Tamper1interrupt enable"]
    #[inline(always)]
    pub fn tpie1(&mut self) -> TPIE1_W {
        TPIE1_W::new(self)
    }
    #[doc = "Bit 6 - Tamper1 interrupt reset"]
    #[inline(always)]
    pub fn tir1(&mut self) -> TIR1_W {
        TIR1_W::new(self)
    }
    #[doc = "Bit 5 - Tamper1 event reset"]
    #[inline(always)]
    pub fn ter1(&mut self) -> TER1_W {
        TER1_W::new(self)
    }
    #[doc = "Bit 2 - Tamper0 interrupt enable"]
    #[inline(always)]
    pub fn tpie0(&mut self) -> TPIE0_W {
        TPIE0_W::new(self)
    }
    #[doc = "Bit 1 - Tamper0 interrupt reset"]
    #[inline(always)]
    pub fn tir0(&mut self) -> TIR0_W {
        TIR0_W::new(self)
    }
    #[doc = "Bit 0 - Tamper0 event reset"]
    #[inline(always)]
    pub fn ter0(&mut self) -> TER0_W {
        TER0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpcs](index.html) module"]
pub struct TPCS_SPEC;
impl crate::RegisterSpec for TPCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpcs::R](R) reader structure"]
impl crate::Readable for TPCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpcs::W](W) writer structure"]
impl crate::Writable for TPCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPCS to value 0"]
impl crate::Resettable for TPCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
