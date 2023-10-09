#[doc = "Register `INT` reader"]
pub type R = crate::R<INT_SPEC>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<INT_SPEC>;
#[doc = "Field `IRC40KSTBIF` reader - IRC40K stabilization interrupt flag"]
pub type IRC40KSTBIF_R = crate::BitReader;
#[doc = "Field `LXTALSTBIF` reader - LXTAL stabilization interrupt flag"]
pub type LXTALSTBIF_R = crate::BitReader;
#[doc = "Field `IRC8MSTBIF` reader - IRC8M stabilization interrupt flag"]
pub type IRC8MSTBIF_R = crate::BitReader;
#[doc = "Field `HXTALSTBIF` reader - HXTAL stabilization interrupt flag"]
pub type HXTALSTBIF_R = crate::BitReader;
#[doc = "Field `PLLSTBIF` reader - PLL stabilization interrupt flag"]
pub type PLLSTBIF_R = crate::BitReader;
#[doc = "Field `CKMIF` reader - HXTAL Clock Stuck Interrupt Flag"]
pub type CKMIF_R = crate::BitReader;
#[doc = "Field `IRC40KSTBIE` reader - IRC40K Stabilization interrupt enable"]
pub type IRC40KSTBIE_R = crate::BitReader;
#[doc = "Field `IRC40KSTBIE` writer - IRC40K Stabilization interrupt enable"]
pub type IRC40KSTBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LXTALSTBIE` reader - LXTAL Stabilization Interrupt Enable"]
pub type LXTALSTBIE_R = crate::BitReader;
#[doc = "Field `LXTALSTBIE` writer - LXTAL Stabilization Interrupt Enable"]
pub type LXTALSTBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IRC8MSTBIE` reader - IRC8M Stabilization Interrupt Enable"]
pub type IRC8MSTBIE_R = crate::BitReader;
#[doc = "Field `IRC8MSTBIE` writer - IRC8M Stabilization Interrupt Enable"]
pub type IRC8MSTBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HXTALSTBIE` reader - HXTAL Stabilization Interrupt Enable"]
pub type HXTALSTBIE_R = crate::BitReader;
#[doc = "Field `HXTALSTBIE` writer - HXTAL Stabilization Interrupt Enable"]
pub type HXTALSTBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLSTBIE` reader - PLL Stabilization Interrupt Enable"]
pub type PLLSTBIE_R = crate::BitReader;
#[doc = "Field `PLLSTBIE` writer - PLL Stabilization Interrupt Enable"]
pub type PLLSTBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IRC40KSTBIC` writer - IRC40K Stabilization Interrupt Clear"]
pub type IRC40KSTBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LXTALSTBIC` writer - LXTAL Stabilization Interrupt Clear"]
pub type LXTALSTBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IRC8MSTBIC` writer - IRC8M Stabilization Interrupt Clear"]
pub type IRC8MSTBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HXTALSTBIC` writer - HXTAL Stabilization Interrupt Clear"]
pub type HXTALSTBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLSTBIC` writer - PLL stabilization Interrupt Clear"]
pub type PLLSTBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKMIC` writer - HXTAL Clock Stuck Interrupt Clear"]
pub type CKMIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - IRC40K stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc40kstbif(&self) -> IRC40KSTBIF_R {
        IRC40KSTBIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn lxtalstbif(&self) -> LXTALSTBIF_R {
        LXTALSTBIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRC8M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc8mstbif(&self) -> IRC8MSTBIF_R {
        IRC8MSTBIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn hxtalstbif(&self) -> HXTALSTBIF_R {
        HXTALSTBIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL stabilization interrupt flag"]
    #[inline(always)]
    pub fn pllstbif(&self) -> PLLSTBIF_R {
        PLLSTBIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - HXTAL Clock Stuck Interrupt Flag"]
    #[inline(always)]
    pub fn ckmif(&self) -> CKMIF_R {
        CKMIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc40kstbie(&self) -> IRC40KSTBIE_R {
        IRC40KSTBIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&self) -> LXTALSTBIE_R {
        LXTALSTBIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc8mstbie(&self) -> IRC8MSTBIE_R {
        IRC8MSTBIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&self) -> HXTALSTBIE_R {
        HXTALSTBIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&self) -> PLLSTBIE_R {
        PLLSTBIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc40kstbie(&mut self) -> IRC40KSTBIE_W<INT_SPEC, 8> {
        IRC40KSTBIE_W::new(self)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalstbie(&mut self) -> LXTALSTBIE_W<INT_SPEC, 9> {
        LXTALSTBIE_W::new(self)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc8mstbie(&mut self) -> IRC8MSTBIE_W<INT_SPEC, 10> {
        IRC8MSTBIE_W::new(self)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalstbie(&mut self) -> HXTALSTBIE_W<INT_SPEC, 11> {
        HXTALSTBIE_W::new(self)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllstbie(&mut self) -> PLLSTBIE_W<INT_SPEC, 12> {
        PLLSTBIE_W::new(self)
    }
    #[doc = "Bit 16 - IRC40K Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn irc40kstbic(&mut self) -> IRC40KSTBIC_W<INT_SPEC, 16> {
        IRC40KSTBIC_W::new(self)
    }
    #[doc = "Bit 17 - LXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalstbic(&mut self) -> LXTALSTBIC_W<INT_SPEC, 17> {
        LXTALSTBIC_W::new(self)
    }
    #[doc = "Bit 18 - IRC8M Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn irc8mstbic(&mut self) -> IRC8MSTBIC_W<INT_SPEC, 18> {
        IRC8MSTBIC_W::new(self)
    }
    #[doc = "Bit 19 - HXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalstbic(&mut self) -> HXTALSTBIC_W<INT_SPEC, 19> {
        HXTALSTBIC_W::new(self)
    }
    #[doc = "Bit 20 - PLL stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllstbic(&mut self) -> PLLSTBIC_W<INT_SPEC, 20> {
        PLLSTBIC_W::new(self)
    }
    #[doc = "Bit 23 - HXTAL Clock Stuck Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ckmic(&mut self) -> CKMIC_W<INT_SPEC, 23> {
        CKMIC_W::new(self)
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
#[doc = "Clock interrupt register (RCU_INT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_SPEC;
impl crate::RegisterSpec for INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
