#[doc = "Register `AHB2RST` reader"]
pub type R = crate::R<AHB2RST_SPEC>;
#[doc = "Register `AHB2RST` writer"]
pub type W = crate::W<AHB2RST_SPEC>;
#[doc = "Field `DCIRST` reader - DCI reset"]
pub type DCIRST_R = crate::BitReader;
#[doc = "Field `DCIRST` writer - DCI reset"]
pub type DCIRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAURST` reader - CAU reset"]
pub type CAURST_R = crate::BitReader;
#[doc = "Field `CAURST` writer - CAU reset"]
pub type CAURST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HAURST` reader - HAU reset"]
pub type HAURST_R = crate::BitReader;
#[doc = "Field `HAURST` writer - HAU reset"]
pub type HAURST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRNGRST` reader - TRNG reset"]
pub type TRNGRST_R = crate::BitReader;
#[doc = "Field `TRNGRST` writer - TRNG reset"]
pub type TRNGRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DCI reset"]
    #[inline(always)]
    pub fn dcirst(&self) -> DCIRST_R {
        DCIRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CAU reset"]
    #[inline(always)]
    pub fn caurst(&self) -> CAURST_R {
        CAURST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HAU reset"]
    #[inline(always)]
    pub fn haurst(&self) -> HAURST_R {
        HAURST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TRNG reset"]
    #[inline(always)]
    pub fn trngrst(&self) -> TRNGRST_R {
        TRNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCI reset"]
    #[inline(always)]
    #[must_use]
    pub fn dcirst(&mut self) -> DCIRST_W<AHB2RST_SPEC, 0> {
        DCIRST_W::new(self)
    }
    #[doc = "Bit 4 - CAU reset"]
    #[inline(always)]
    #[must_use]
    pub fn caurst(&mut self) -> CAURST_W<AHB2RST_SPEC, 4> {
        CAURST_W::new(self)
    }
    #[doc = "Bit 5 - HAU reset"]
    #[inline(always)]
    #[must_use]
    pub fn haurst(&mut self) -> HAURST_W<AHB2RST_SPEC, 5> {
        HAURST_W::new(self)
    }
    #[doc = "Bit 6 - TRNG reset"]
    #[inline(always)]
    #[must_use]
    pub fn trngrst(&mut self) -> TRNGRST_W<AHB2RST_SPEC, 6> {
        TRNGRST_W::new(self)
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
#[doc = "AHB2 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2RST_SPEC;
impl crate::RegisterSpec for AHB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rst::R`](R) reader structure"]
impl crate::Readable for AHB2RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb2rst::W`](W) writer structure"]
impl crate::Writable for AHB2RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB2RST to value 0"]
impl crate::Resettable for AHB2RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
