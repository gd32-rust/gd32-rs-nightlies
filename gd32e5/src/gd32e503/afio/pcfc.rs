#[doc = "Register `PCFC` reader"]
pub type R = crate::R<PCFC_SPEC>;
#[doc = "Register `PCFC` writer"]
pub type W = crate::W<PCFC_SPEC>;
#[doc = "Field `PC0_AFCFG` reader - PC0 AF function configuration bitse"]
pub type PC0_AFCFG_R = crate::BitReader;
#[doc = "Field `PC0_AFCFG` writer - PC0 AF function configuration bitse"]
pub type PC0_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PC2_AFCFG` reader - PC2 AF function configuration bitse"]
pub type PC2_AFCFG_R = crate::FieldReader;
#[doc = "Field `PC2_AFCFG` writer - PC2 AF function configuration bitse"]
pub type PC2_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PC3_AFCFG` reader - PC3 AF function configuration bitse"]
pub type PC3_AFCFG_R = crate::BitReader;
#[doc = "Field `PC3_AFCFG` writer - PC3 AF function configuration bitse"]
pub type PC3_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PC6_AFCFG` reader - PC6 AF function configuration bitse"]
pub type PC6_AFCFG_R = crate::FieldReader;
#[doc = "Field `PC6_AFCFG` writer - PC6 AF function configuration bitse"]
pub type PC6_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PC7_AFCFG` reader - PC7 AF function configuration bitse"]
pub type PC7_AFCFG_R = crate::FieldReader;
#[doc = "Field `PC7_AFCFG` writer - PC7 AF function configuration bitse"]
pub type PC7_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PC8_AFCFG` reader - PC8 AF function configuration bitse"]
pub type PC8_AFCFG_R = crate::FieldReader;
#[doc = "Field `PC8_AFCFG` writer - PC8 AF function configuration bitse"]
pub type PC8_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PC9_AFCFG` reader - PC9 AF function configuration bitse"]
pub type PC9_AFCFG_R = crate::FieldReader;
#[doc = "Field `PC9_AFCFG` writer - PC9 AF function configuration bitse"]
pub type PC9_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PC10_AFCFG` reader - PC10 AF function configuration bitse"]
pub type PC10_AFCFG_R = crate::BitReader;
#[doc = "Field `PC10_AFCFG` writer - PC10 AF function configuration bitse"]
pub type PC10_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PC11_AFCFG` reader - PC11 AF function configuration bitse"]
pub type PC11_AFCFG_R = crate::FieldReader;
#[doc = "Field `PC11_AFCFG` writer - PC11 AF function configuration bitse"]
pub type PC11_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PC12_AFCFG` reader - PC12 AF function configuration bitse"]
pub type PC12_AFCFG_R = crate::BitReader;
#[doc = "Field `PC12_AFCFG` writer - PC12 AF function configuration bitse"]
pub type PC12_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PC0 AF function configuration bitse"]
    #[inline(always)]
    pub fn pc0_afcfg(&self) -> PC0_AFCFG_R {
        PC0_AFCFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - PC2 AF function configuration bitse"]
    #[inline(always)]
    pub fn pc2_afcfg(&self) -> PC2_AFCFG_R {
        PC2_AFCFG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - PC3 AF function configuration bitse"]
    #[inline(always)]
    pub fn pc3_afcfg(&self) -> PC3_AFCFG_R {
        PC3_AFCFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 12:13 - PC6 AF function configuration bitse"]
    #[inline(always)]
    pub fn pc6_afcfg(&self) -> PC6_AFCFG_R {
        PC6_AFCFG_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PC7 AF function configuration bitse"]
    #[inline(always)]
    pub fn pc7_afcfg(&self) -> PC7_AFCFG_R {
        PC7_AFCFG_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PC8 AF function configuration bitse"]
    #[inline(always)]
    pub fn pc8_afcfg(&self) -> PC8_AFCFG_R {
        PC8_AFCFG_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PC9 AF function configuration bitse"]
    #[inline(always)]
    pub fn pc9_afcfg(&self) -> PC9_AFCFG_R {
        PC9_AFCFG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - PC10 AF function configuration bitse"]
    #[inline(always)]
    pub fn pc10_afcfg(&self) -> PC10_AFCFG_R {
        PC10_AFCFG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - PC11 AF function configuration bitse"]
    #[inline(always)]
    pub fn pc11_afcfg(&self) -> PC11_AFCFG_R {
        PC11_AFCFG_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - PC12 AF function configuration bitse"]
    #[inline(always)]
    pub fn pc12_afcfg(&self) -> PC12_AFCFG_R {
        PC12_AFCFG_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PC0 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pc0_afcfg(&mut self) -> PC0_AFCFG_W<PCFC_SPEC, 0> {
        PC0_AFCFG_W::new(self)
    }
    #[doc = "Bits 4:5 - PC2 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pc2_afcfg(&mut self) -> PC2_AFCFG_W<PCFC_SPEC, 4> {
        PC2_AFCFG_W::new(self)
    }
    #[doc = "Bit 6 - PC3 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pc3_afcfg(&mut self) -> PC3_AFCFG_W<PCFC_SPEC, 6> {
        PC3_AFCFG_W::new(self)
    }
    #[doc = "Bits 12:13 - PC6 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pc6_afcfg(&mut self) -> PC6_AFCFG_W<PCFC_SPEC, 12> {
        PC6_AFCFG_W::new(self)
    }
    #[doc = "Bits 14:15 - PC7 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pc7_afcfg(&mut self) -> PC7_AFCFG_W<PCFC_SPEC, 14> {
        PC7_AFCFG_W::new(self)
    }
    #[doc = "Bits 16:17 - PC8 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pc8_afcfg(&mut self) -> PC8_AFCFG_W<PCFC_SPEC, 16> {
        PC8_AFCFG_W::new(self)
    }
    #[doc = "Bits 18:19 - PC9 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pc9_afcfg(&mut self) -> PC9_AFCFG_W<PCFC_SPEC, 18> {
        PC9_AFCFG_W::new(self)
    }
    #[doc = "Bit 20 - PC10 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pc10_afcfg(&mut self) -> PC10_AFCFG_W<PCFC_SPEC, 20> {
        PC10_AFCFG_W::new(self)
    }
    #[doc = "Bits 22:23 - PC11 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pc11_afcfg(&mut self) -> PC11_AFCFG_W<PCFC_SPEC, 22> {
        PC11_AFCFG_W::new(self)
    }
    #[doc = "Bit 24 - PC12 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pc12_afcfg(&mut self) -> PC12_AFCFG_W<PCFC_SPEC, 24> {
        PC12_AFCFG_W::new(self)
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
#[doc = "AFIO port configuration register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCFC_SPEC;
impl crate::RegisterSpec for PCFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcfc::R`](R) reader structure"]
impl crate::Readable for PCFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcfc::W`](W) writer structure"]
impl crate::Writable for PCFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCFC to value 0"]
impl crate::Resettable for PCFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
