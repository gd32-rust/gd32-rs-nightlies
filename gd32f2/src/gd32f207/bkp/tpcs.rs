#[doc = "Register `TPCS` reader"]
pub type R = crate::R<TPCS_SPEC>;
#[doc = "Register `TPCS` writer"]
pub type W = crate::W<TPCS_SPEC>;
#[doc = "Field `TER0` reader - Tamper0 event reset"]
pub type TER0_R = crate::BitReader;
#[doc = "Field `TER0` writer - Tamper0 event reset"]
pub type TER0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIR0` reader - Tamper0 interrupt reset"]
pub type TIR0_R = crate::BitReader;
#[doc = "Field `TIR0` writer - Tamper0 interrupt reset"]
pub type TIR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPIE0` reader - Tamper0 interrupt enable"]
pub type TPIE0_R = crate::BitReader;
#[doc = "Field `TPIE0` writer - Tamper0 interrupt enable"]
pub type TPIE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TER1` reader - Tamper1 event reset"]
pub type TER1_R = crate::BitReader;
#[doc = "Field `TER1` writer - Tamper1 event reset"]
pub type TER1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIR1` reader - Tamper1 interrupt reset"]
pub type TIR1_R = crate::BitReader;
#[doc = "Field `TIR1` writer - Tamper1 interrupt reset"]
pub type TIR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPIE1` reader - Tamper1interrupt enable"]
pub type TPIE1_R = crate::BitReader;
#[doc = "Field `TPIE1` writer - Tamper1interrupt enable"]
pub type TPIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEF0` reader - Tamper0 event flag"]
pub type TEF0_R = crate::BitReader;
#[doc = "Field `TEF0` writer - Tamper0 event flag"]
pub type TEF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIF0` reader - Tamper0 interrupt flag"]
pub type TIF0_R = crate::BitReader;
#[doc = "Field `TIF0` writer - Tamper0 interrupt flag"]
pub type TIF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEF1` reader - Tamper1 event flag"]
pub type TEF1_R = crate::BitReader;
#[doc = "Field `TEF1` writer - Tamper1 event flag"]
pub type TEF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIF1` reader - Tamper1 interrupt flag"]
pub type TIF1_R = crate::BitReader;
#[doc = "Field `TIF1` writer - Tamper1 interrupt flag"]
pub type TIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Tamper0 event reset"]
    #[inline(always)]
    pub fn ter0(&self) -> TER0_R {
        TER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper0 interrupt reset"]
    #[inline(always)]
    pub fn tir0(&self) -> TIR0_R {
        TIR0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper0 interrupt enable"]
    #[inline(always)]
    pub fn tpie0(&self) -> TPIE0_R {
        TPIE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Tamper1 event reset"]
    #[inline(always)]
    pub fn ter1(&self) -> TER1_R {
        TER1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tamper1 interrupt reset"]
    #[inline(always)]
    pub fn tir1(&self) -> TIR1_R {
        TIR1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tamper1interrupt enable"]
    #[inline(always)]
    pub fn tpie1(&self) -> TPIE1_R {
        TPIE1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper0 event flag"]
    #[inline(always)]
    pub fn tef0(&self) -> TEF0_R {
        TEF0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper0 interrupt flag"]
    #[inline(always)]
    pub fn tif0(&self) -> TIF0_R {
        TIF0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Tamper1 event flag"]
    #[inline(always)]
    pub fn tef1(&self) -> TEF1_R {
        TEF1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tamper1 interrupt flag"]
    #[inline(always)]
    pub fn tif1(&self) -> TIF1_R {
        TIF1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper0 event reset"]
    #[inline(always)]
    #[must_use]
    pub fn ter0(&mut self) -> TER0_W<TPCS_SPEC, 0> {
        TER0_W::new(self)
    }
    #[doc = "Bit 1 - Tamper0 interrupt reset"]
    #[inline(always)]
    #[must_use]
    pub fn tir0(&mut self) -> TIR0_W<TPCS_SPEC, 1> {
        TIR0_W::new(self)
    }
    #[doc = "Bit 2 - Tamper0 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpie0(&mut self) -> TPIE0_W<TPCS_SPEC, 2> {
        TPIE0_W::new(self)
    }
    #[doc = "Bit 5 - Tamper1 event reset"]
    #[inline(always)]
    #[must_use]
    pub fn ter1(&mut self) -> TER1_W<TPCS_SPEC, 5> {
        TER1_W::new(self)
    }
    #[doc = "Bit 6 - Tamper1 interrupt reset"]
    #[inline(always)]
    #[must_use]
    pub fn tir1(&mut self) -> TIR1_W<TPCS_SPEC, 6> {
        TIR1_W::new(self)
    }
    #[doc = "Bit 7 - Tamper1interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpie1(&mut self) -> TPIE1_W<TPCS_SPEC, 7> {
        TPIE1_W::new(self)
    }
    #[doc = "Bit 8 - Tamper0 event flag"]
    #[inline(always)]
    #[must_use]
    pub fn tef0(&mut self) -> TEF0_W<TPCS_SPEC, 8> {
        TEF0_W::new(self)
    }
    #[doc = "Bit 9 - Tamper0 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tif0(&mut self) -> TIF0_W<TPCS_SPEC, 9> {
        TIF0_W::new(self)
    }
    #[doc = "Bit 14 - Tamper1 event flag"]
    #[inline(always)]
    #[must_use]
    pub fn tef1(&mut self) -> TEF1_W<TPCS_SPEC, 14> {
        TEF1_W::new(self)
    }
    #[doc = "Bit 15 - Tamper1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tif1(&mut self) -> TIF1_W<TPCS_SPEC, 15> {
        TIF1_W::new(self)
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
#[doc = "Tamper control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPCS_SPEC;
impl crate::RegisterSpec for TPCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpcs::R`](R) reader structure"]
impl crate::Readable for TPCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpcs::W`](W) writer structure"]
impl crate::Writable for TPCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPCS to value 0"]
impl crate::Resettable for TPCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
