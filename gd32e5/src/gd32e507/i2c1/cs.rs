#[doc = "Register `CS` reader"]
pub type R = crate::R<CS_SPEC>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CS_SPEC>;
#[doc = "Field `STLO` reader - Start lost flag"]
pub type STLO_R = crate::BitReader;
#[doc = "Field `STLO` writer - Start lost flag"]
pub type STLO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STPSEND` reader - Stop condition sent out in master mode"]
pub type STPSEND_R = crate::BitReader;
#[doc = "Field `STPSEND` writer - Stop condition sent out in master mode"]
pub type STPSEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STLOIE` reader - Interrupt enable for start lost"]
pub type STLOIE_R = crate::BitReader;
#[doc = "Field `STLOIE` writer - Interrupt enable for start lost"]
pub type STLOIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STPSENDIE` reader - Interrupt enable for stop condition sent"]
pub type STPSENDIE_R = crate::BitReader;
#[doc = "Field `STPSENDIE` writer - Interrupt enable for stop condition sent"]
pub type STPSENDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Start lost flag"]
    #[inline(always)]
    pub fn stlo(&self) -> STLO_R {
        STLO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop condition sent out in master mode"]
    #[inline(always)]
    pub fn stpsend(&self) -> STPSEND_R {
        STPSEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable for start lost"]
    #[inline(always)]
    pub fn stloie(&self) -> STLOIE_R {
        STLOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable for stop condition sent"]
    #[inline(always)]
    pub fn stpsendie(&self) -> STPSENDIE_R {
        STPSENDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn stlo(&mut self) -> STLO_W<CS_SPEC, 0> {
        STLO_W::new(self)
    }
    #[doc = "Bit 1 - Stop condition sent out in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn stpsend(&mut self) -> STPSEND_W<CS_SPEC, 1> {
        STPSEND_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt enable for start lost"]
    #[inline(always)]
    #[must_use]
    pub fn stloie(&mut self) -> STLOIE_W<CS_SPEC, 8> {
        STLOIE_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt enable for stop condition sent"]
    #[inline(always)]
    #[must_use]
    pub fn stpsendie(&mut self) -> STPSENDIE_W<CS_SPEC, 9> {
        STPSENDIE_W::new(self)
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
#[doc = "Control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
