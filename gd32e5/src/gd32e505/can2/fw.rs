#[doc = "Register `FW` reader"]
pub type R = crate::R<FW_SPEC>;
#[doc = "Register `FW` writer"]
pub type W = crate::W<FW_SPEC>;
#[doc = "Field `FW0` reader - Filter working"]
pub type FW0_R = crate::BitReader;
#[doc = "Field `FW0` writer - Filter working"]
pub type FW0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW1` reader - Filter working"]
pub type FW1_R = crate::BitReader;
#[doc = "Field `FW1` writer - Filter working"]
pub type FW1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW2` reader - Filter working"]
pub type FW2_R = crate::BitReader;
#[doc = "Field `FW2` writer - Filter working"]
pub type FW2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW3` reader - Filter working"]
pub type FW3_R = crate::BitReader;
#[doc = "Field `FW3` writer - Filter working"]
pub type FW3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW4` reader - Filter working"]
pub type FW4_R = crate::BitReader;
#[doc = "Field `FW4` writer - Filter working"]
pub type FW4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW5` reader - Filter working"]
pub type FW5_R = crate::BitReader;
#[doc = "Field `FW5` writer - Filter working"]
pub type FW5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW6` reader - Filter working"]
pub type FW6_R = crate::BitReader;
#[doc = "Field `FW6` writer - Filter working"]
pub type FW6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW7` reader - Filter working"]
pub type FW7_R = crate::BitReader;
#[doc = "Field `FW7` writer - Filter working"]
pub type FW7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW8` reader - Filter working"]
pub type FW8_R = crate::BitReader;
#[doc = "Field `FW8` writer - Filter working"]
pub type FW8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW9` reader - Filter working"]
pub type FW9_R = crate::BitReader;
#[doc = "Field `FW9` writer - Filter working"]
pub type FW9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW10` reader - Filter working"]
pub type FW10_R = crate::BitReader;
#[doc = "Field `FW10` writer - Filter working"]
pub type FW10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW11` reader - Filter working"]
pub type FW11_R = crate::BitReader;
#[doc = "Field `FW11` writer - Filter working"]
pub type FW11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW12` reader - Filter working"]
pub type FW12_R = crate::BitReader;
#[doc = "Field `FW12` writer - Filter working"]
pub type FW12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FW13` reader - Filter working"]
pub type FW13_R = crate::BitReader;
#[doc = "Field `FW13` writer - Filter working"]
pub type FW13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Filter working"]
    #[inline(always)]
    pub fn fw0(&self) -> FW0_R {
        FW0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter working"]
    #[inline(always)]
    pub fn fw1(&self) -> FW1_R {
        FW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter working"]
    #[inline(always)]
    pub fn fw2(&self) -> FW2_R {
        FW2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter working"]
    #[inline(always)]
    pub fn fw3(&self) -> FW3_R {
        FW3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter working"]
    #[inline(always)]
    pub fn fw4(&self) -> FW4_R {
        FW4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter working"]
    #[inline(always)]
    pub fn fw5(&self) -> FW5_R {
        FW5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter working"]
    #[inline(always)]
    pub fn fw6(&self) -> FW6_R {
        FW6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter working"]
    #[inline(always)]
    pub fn fw7(&self) -> FW7_R {
        FW7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter working"]
    #[inline(always)]
    pub fn fw8(&self) -> FW8_R {
        FW8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter working"]
    #[inline(always)]
    pub fn fw9(&self) -> FW9_R {
        FW9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter working"]
    #[inline(always)]
    pub fn fw10(&self) -> FW10_R {
        FW10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter working"]
    #[inline(always)]
    pub fn fw11(&self) -> FW11_R {
        FW11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter working"]
    #[inline(always)]
    pub fn fw12(&self) -> FW12_R {
        FW12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter working"]
    #[inline(always)]
    pub fn fw13(&self) -> FW13_R {
        FW13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw0(&mut self) -> FW0_W<FW_SPEC, 0> {
        FW0_W::new(self)
    }
    #[doc = "Bit 1 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw1(&mut self) -> FW1_W<FW_SPEC, 1> {
        FW1_W::new(self)
    }
    #[doc = "Bit 2 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw2(&mut self) -> FW2_W<FW_SPEC, 2> {
        FW2_W::new(self)
    }
    #[doc = "Bit 3 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw3(&mut self) -> FW3_W<FW_SPEC, 3> {
        FW3_W::new(self)
    }
    #[doc = "Bit 4 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw4(&mut self) -> FW4_W<FW_SPEC, 4> {
        FW4_W::new(self)
    }
    #[doc = "Bit 5 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw5(&mut self) -> FW5_W<FW_SPEC, 5> {
        FW5_W::new(self)
    }
    #[doc = "Bit 6 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw6(&mut self) -> FW6_W<FW_SPEC, 6> {
        FW6_W::new(self)
    }
    #[doc = "Bit 7 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw7(&mut self) -> FW7_W<FW_SPEC, 7> {
        FW7_W::new(self)
    }
    #[doc = "Bit 8 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw8(&mut self) -> FW8_W<FW_SPEC, 8> {
        FW8_W::new(self)
    }
    #[doc = "Bit 9 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw9(&mut self) -> FW9_W<FW_SPEC, 9> {
        FW9_W::new(self)
    }
    #[doc = "Bit 10 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw10(&mut self) -> FW10_W<FW_SPEC, 10> {
        FW10_W::new(self)
    }
    #[doc = "Bit 11 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw11(&mut self) -> FW11_W<FW_SPEC, 11> {
        FW11_W::new(self)
    }
    #[doc = "Bit 12 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw12(&mut self) -> FW12_W<FW_SPEC, 12> {
        FW12_W::new(self)
    }
    #[doc = "Bit 13 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw13(&mut self) -> FW13_W<FW_SPEC, 13> {
        FW13_W::new(self)
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
#[doc = "Filter working register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FW_SPEC;
impl crate::RegisterSpec for FW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fw::R`](R) reader structure"]
impl crate::Readable for FW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fw::W`](W) writer structure"]
impl crate::Writable for FW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FW to value 0"]
impl crate::Resettable for FW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
