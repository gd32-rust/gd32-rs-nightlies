#[doc = "Register `L1CKEY` reader"]
pub type R = crate::R<L1CKEY_SPEC>;
#[doc = "Register `L1CKEY` writer"]
pub type W = crate::W<L1CKEY_SPEC>;
#[doc = "Field `CKEYB` reader - Color Key Blue"]
pub type CKEYB_R = crate::FieldReader;
#[doc = "Field `CKEYB` writer - Color Key Blue"]
pub type CKEYB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CKEYG` reader - Color Key Green"]
pub type CKEYG_R = crate::FieldReader;
#[doc = "Field `CKEYG` writer - Color Key Green"]
pub type CKEYG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CKEYR` reader - Color Key Red"]
pub type CKEYR_R = crate::FieldReader;
#[doc = "Field `CKEYR` writer - Color Key Red"]
pub type CKEYR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Color Key Blue"]
    #[inline(always)]
    pub fn ckeyb(&self) -> CKEYB_R {
        CKEYB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Color Key Green"]
    #[inline(always)]
    pub fn ckeyg(&self) -> CKEYG_R {
        CKEYG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Color Key Red"]
    #[inline(always)]
    pub fn ckeyr(&self) -> CKEYR_R {
        CKEYR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Key Blue"]
    #[inline(always)]
    #[must_use]
    pub fn ckeyb(&mut self) -> CKEYB_W<L1CKEY_SPEC, 0> {
        CKEYB_W::new(self)
    }
    #[doc = "Bits 8:15 - Color Key Green"]
    #[inline(always)]
    #[must_use]
    pub fn ckeyg(&mut self) -> CKEYG_W<L1CKEY_SPEC, 8> {
        CKEYG_W::new(self)
    }
    #[doc = "Bits 16:23 - Color Key Red"]
    #[inline(always)]
    #[must_use]
    pub fn ckeyr(&mut self) -> CKEYR_W<L1CKEY_SPEC, 16> {
        CKEYR_W::new(self)
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
#[doc = "Layer 1 color key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ckey::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ckey::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CKEY_SPEC;
impl crate::RegisterSpec for L1CKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1ckey::R`](R) reader structure"]
impl crate::Readable for L1CKEY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1ckey::W`](W) writer structure"]
impl crate::Writable for L1CKEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1CKEY to value 0"]
impl crate::Resettable for L1CKEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
