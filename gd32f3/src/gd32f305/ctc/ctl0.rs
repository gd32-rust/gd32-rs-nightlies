#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `CKOKIE` reader - Clock trim ok interrupt enable"]
pub type CKOKIE_R = crate::BitReader;
#[doc = "Field `CKOKIE` writer - Clock trim ok interrupt enable"]
pub type CKOKIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKWARNIE` reader - Clock trim warning interrupt enable"]
pub type CKWARNIE_R = crate::BitReader;
#[doc = "Field `CKWARNIE` writer - Clock trim warning interrupt enable"]
pub type CKWARNIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EREFIE` reader - EREFIF interrupt enable"]
pub type EREFIE_R = crate::BitReader;
#[doc = "Field `EREFIE` writer - EREFIF interrupt enable"]
pub type EREFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNTEN` reader - CTC counter enable"]
pub type CNTEN_R = crate::BitReader;
#[doc = "Field `CNTEN` writer - CTC counter enable"]
pub type CNTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOTRIM` reader - Hardware automatically trim mode"]
pub type AUTOTRIM_R = crate::BitReader;
#[doc = "Field `AUTOTRIM` writer - Hardware automatically trim mode"]
pub type AUTOTRIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREFPUL` reader - Software reference source sync pulse"]
pub type SWREFPUL_R = crate::BitReader;
#[doc = "Field `SWREFPUL` writer - Software reference source sync pulse"]
pub type SWREFPUL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRIMVALUE` reader - IRC48M trim value"]
pub type TRIMVALUE_R = crate::FieldReader;
#[doc = "Field `TRIMVALUE` writer - IRC48M trim value"]
pub type TRIMVALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
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
    #[must_use]
    pub fn ckokie(&mut self) -> CKOKIE_W<CTL0_SPEC, 0> {
        CKOKIE_W::new(self)
    }
    #[doc = "Bit 1 - Clock trim warning interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckwarnie(&mut self) -> CKWARNIE_W<CTL0_SPEC, 1> {
        CKWARNIE_W::new(self)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTL0_SPEC, 2> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 3 - EREFIF interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn erefie(&mut self) -> EREFIE_W<CTL0_SPEC, 3> {
        EREFIE_W::new(self)
    }
    #[doc = "Bit 5 - CTC counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnten(&mut self) -> CNTEN_W<CTL0_SPEC, 5> {
        CNTEN_W::new(self)
    }
    #[doc = "Bit 6 - Hardware automatically trim mode"]
    #[inline(always)]
    #[must_use]
    pub fn autotrim(&mut self) -> AUTOTRIM_W<CTL0_SPEC, 6> {
        AUTOTRIM_W::new(self)
    }
    #[doc = "Bit 7 - Software reference source sync pulse"]
    #[inline(always)]
    #[must_use]
    pub fn swrefpul(&mut self) -> SWREFPUL_W<CTL0_SPEC, 7> {
        SWREFPUL_W::new(self)
    }
    #[doc = "Bits 8:13 - IRC48M trim value"]
    #[inline(always)]
    #[must_use]
    pub fn trimvalue(&mut self) -> TRIMVALUE_W<CTL0_SPEC, 8> {
        TRIMVALUE_W::new(self)
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
#[doc = "Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0x2000"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
