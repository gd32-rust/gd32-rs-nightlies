#[doc = "Register `INT` reader"]
pub struct R(crate::R<INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT` writer"]
pub struct W(crate::W<INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SPEC>;
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
impl From<crate::W<INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKMIC` writer - HXTAL Clock Stuck Interrupt Clear"]
pub type CKMIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 23>;
#[doc = "Field `IRC28MSTBIC` writer - IRC28M stabilization Interrupt Clear"]
pub type IRC28MSTBIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 21>;
#[doc = "Field `PLLSTBIC` writer - PLL stabilization Interrupt Clear"]
pub type PLLSTBIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 20>;
#[doc = "Field `HXTALSTBIC` writer - HXTAL Stabilization Interrupt Clear"]
pub type HXTALSTBIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 19>;
#[doc = "Field `IRC8MSTBIC` writer - IRC8M Stabilization Interrupt Clear"]
pub type IRC8MSTBIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 18>;
#[doc = "Field `LXTALSTBIC` writer - LXTAL Stabilization Interrupt Clear"]
pub type LXTALSTBIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 17>;
#[doc = "Field `IRC40KSTBIC` writer - IRC40K Stabilization Interrupt Clear"]
pub type IRC40KSTBIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 16>;
#[doc = "Field `IRC28MSTBIE` reader - IRC28M Stabilization Interrupt Enable"]
pub type IRC28MSTBIE_R = crate::BitReader<bool>;
#[doc = "Field `IRC28MSTBIE` writer - IRC28M Stabilization Interrupt Enable"]
pub type IRC28MSTBIE_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 13>;
#[doc = "Field `PLLSTBIE` reader - PLL Stabilization Interrupt Enable"]
pub type PLLSTBIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLSTBIE` writer - PLL Stabilization Interrupt Enable"]
pub type PLLSTBIE_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 12>;
#[doc = "Field `HXTALSTBIE` reader - HXTAL Stabilization Interrupt Enable"]
pub type HXTALSTBIE_R = crate::BitReader<bool>;
#[doc = "Field `HXTALSTBIE` writer - HXTAL Stabilization Interrupt Enable"]
pub type HXTALSTBIE_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 11>;
#[doc = "Field `IRC8MSTBIE` reader - IRC8M Stabilization Interrupt Enable"]
pub type IRC8MSTBIE_R = crate::BitReader<bool>;
#[doc = "Field `IRC8MSTBIE` writer - IRC8M Stabilization Interrupt Enable"]
pub type IRC8MSTBIE_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 10>;
#[doc = "Field `LXTALSTBIE` reader - LXTAL Stabilization Interrupt Enable"]
pub type LXTALSTBIE_R = crate::BitReader<bool>;
#[doc = "Field `LXTALSTBIE` writer - LXTAL Stabilization Interrupt Enable"]
pub type LXTALSTBIE_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 9>;
#[doc = "Field `IRC40KSTBIE` reader - IRC40K Stabilization interrupt enable"]
pub type IRC40KSTBIE_R = crate::BitReader<bool>;
#[doc = "Field `IRC40KSTBIE` writer - IRC40K Stabilization interrupt enable"]
pub type IRC40KSTBIE_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, bool, 8>;
#[doc = "Field `CKMIF` reader - HXTAL Clock Stuck Interrupt Flag"]
pub type CKMIF_R = crate::BitReader<bool>;
#[doc = "Field `IRC28MSTBIF` reader - IRC28M stabilization interrupt flag"]
pub type IRC28MSTBIF_R = crate::BitReader<bool>;
#[doc = "Field `PLLSTBIF` reader - PLL stabilization interrupt flag"]
pub type PLLSTBIF_R = crate::BitReader<bool>;
#[doc = "Field `HXTALSTBIF` reader - HXTAL stabilization interrupt flag"]
pub type HXTALSTBIF_R = crate::BitReader<bool>;
#[doc = "Field `IRC8MSTBIF` reader - IRC8M stabilization interrupt flag"]
pub type IRC8MSTBIF_R = crate::BitReader<bool>;
#[doc = "Field `LXTALSTBIF` reader - LXTAL stabilization interrupt flag"]
pub type LXTALSTBIF_R = crate::BitReader<bool>;
#[doc = "Field `IRC40KSTBIF` reader - IRC40K stabilization interrupt flag"]
pub type IRC40KSTBIF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 13 - IRC28M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc28mstbie(&self) -> IRC28MSTBIE_R {
        IRC28MSTBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&self) -> PLLSTBIE_R {
        PLLSTBIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&self) -> HXTALSTBIE_R {
        HXTALSTBIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc8mstbie(&self) -> IRC8MSTBIE_R {
        IRC8MSTBIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&self) -> LXTALSTBIE_R {
        LXTALSTBIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc40kstbie(&self) -> IRC40KSTBIE_R {
        IRC40KSTBIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - HXTAL Clock Stuck Interrupt Flag"]
    #[inline(always)]
    pub fn ckmif(&self) -> CKMIF_R {
        CKMIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 5 - IRC28M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc28mstbif(&self) -> IRC28MSTBIF_R {
        IRC28MSTBIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL stabilization interrupt flag"]
    #[inline(always)]
    pub fn pllstbif(&self) -> PLLSTBIF_R {
        PLLSTBIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - HXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn hxtalstbif(&self) -> HXTALSTBIF_R {
        HXTALSTBIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - IRC8M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc8mstbif(&self) -> IRC8MSTBIF_R {
        IRC8MSTBIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - LXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn lxtalstbif(&self) -> LXTALSTBIF_R {
        LXTALSTBIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - IRC40K stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc40kstbif(&self) -> IRC40KSTBIF_R {
        IRC40KSTBIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - HXTAL Clock Stuck Interrupt Clear"]
    #[inline(always)]
    pub fn ckmic(&mut self) -> CKMIC_W {
        CKMIC_W::new(self)
    }
    #[doc = "Bit 21 - IRC28M stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc28mstbic(&mut self) -> IRC28MSTBIC_W {
        IRC28MSTBIC_W::new(self)
    }
    #[doc = "Bit 20 - PLL stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn pllstbic(&mut self) -> PLLSTBIC_W {
        PLLSTBIC_W::new(self)
    }
    #[doc = "Bit 19 - HXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn hxtalstbic(&mut self) -> HXTALSTBIC_W {
        HXTALSTBIC_W::new(self)
    }
    #[doc = "Bit 18 - IRC8M Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc8mstbic(&mut self) -> IRC8MSTBIC_W {
        IRC8MSTBIC_W::new(self)
    }
    #[doc = "Bit 17 - LXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn lxtalstbic(&mut self) -> LXTALSTBIC_W {
        LXTALSTBIC_W::new(self)
    }
    #[doc = "Bit 16 - IRC40K Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc40kstbic(&mut self) -> IRC40KSTBIC_W {
        IRC40KSTBIC_W::new(self)
    }
    #[doc = "Bit 13 - IRC28M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc28mstbie(&mut self) -> IRC28MSTBIE_W {
        IRC28MSTBIE_W::new(self)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&mut self) -> PLLSTBIE_W {
        PLLSTBIE_W::new(self)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&mut self) -> HXTALSTBIE_W {
        HXTALSTBIE_W::new(self)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc8mstbie(&mut self) -> IRC8MSTBIE_W {
        IRC8MSTBIE_W::new(self)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&mut self) -> LXTALSTBIE_W {
        LXTALSTBIE_W::new(self)
    }
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc40kstbie(&mut self) -> IRC40KSTBIE_W {
        IRC40KSTBIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock interrupt register (RCU_INT)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](index.html) module"]
pub struct INT_SPEC;
impl crate::RegisterSpec for INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int::R](R) reader structure"]
impl crate::Readable for INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int::W](W) writer structure"]
impl crate::Writable for INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
