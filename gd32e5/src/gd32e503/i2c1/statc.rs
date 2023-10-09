#[doc = "Register `STATC` reader"]
pub type R = crate::R<STATC_SPEC>;
#[doc = "Register `STATC` writer"]
pub type W = crate::W<STATC_SPEC>;
#[doc = "Field `SBSENDC` reader - Start send status clear"]
pub type SBSENDC_R = crate::BitReader;
#[doc = "Field `SBSENDC` writer - Start send status clear"]
pub type SBSENDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDSENDC` reader - ADDSEND status clear"]
pub type ADDSENDC_R = crate::BitReader;
#[doc = "Field `ADDSENDC` writer - ADDSEND status clear"]
pub type ADDSENDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BTCC` reader - BTC status clear"]
pub type BTCC_R = crate::BitReader;
#[doc = "Field `BTCC` writer - BTC status clear"]
pub type BTCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADD10SENDC` reader - ADD10SEND status clear"]
pub type ADD10SENDC_R = crate::BitReader;
#[doc = "Field `ADD10SENDC` writer - ADD10SEND status clear"]
pub type ADD10SENDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOPFC` reader - STOPF status clear"]
pub type STOPFC_R = crate::BitReader;
#[doc = "Field `STOPFC` writer - STOPF status clear"]
pub type STOPFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRCEN` reader - Status register clear enable"]
pub type SRCEN_R = crate::BitReader;
#[doc = "Field `SRCEN` writer - Status register clear enable"]
pub type SRCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Start send status clear"]
    #[inline(always)]
    pub fn sbsendc(&self) -> SBSENDC_R {
        SBSENDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADDSEND status clear"]
    #[inline(always)]
    pub fn addsendc(&self) -> ADDSENDC_R {
        ADDSENDC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BTC status clear"]
    #[inline(always)]
    pub fn btcc(&self) -> BTCC_R {
        BTCC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADD10SEND status clear"]
    #[inline(always)]
    pub fn add10sendc(&self) -> ADD10SENDC_R {
        ADD10SENDC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STOPF status clear"]
    #[inline(always)]
    pub fn stopfc(&self) -> STOPFC_R {
        STOPFC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 15 - Status register clear enable"]
    #[inline(always)]
    pub fn srcen(&self) -> SRCEN_R {
        SRCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start send status clear"]
    #[inline(always)]
    #[must_use]
    pub fn sbsendc(&mut self) -> SBSENDC_W<STATC_SPEC, 0> {
        SBSENDC_W::new(self)
    }
    #[doc = "Bit 1 - ADDSEND status clear"]
    #[inline(always)]
    #[must_use]
    pub fn addsendc(&mut self) -> ADDSENDC_W<STATC_SPEC, 1> {
        ADDSENDC_W::new(self)
    }
    #[doc = "Bit 2 - BTC status clear"]
    #[inline(always)]
    #[must_use]
    pub fn btcc(&mut self) -> BTCC_W<STATC_SPEC, 2> {
        BTCC_W::new(self)
    }
    #[doc = "Bit 3 - ADD10SEND status clear"]
    #[inline(always)]
    #[must_use]
    pub fn add10sendc(&mut self) -> ADD10SENDC_W<STATC_SPEC, 3> {
        ADD10SENDC_W::new(self)
    }
    #[doc = "Bit 4 - STOPF status clear"]
    #[inline(always)]
    #[must_use]
    pub fn stopfc(&mut self) -> STOPFC_W<STATC_SPEC, 4> {
        STOPFC_W::new(self)
    }
    #[doc = "Bit 15 - Status register clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn srcen(&mut self) -> SRCEN_W<STATC_SPEC, 15> {
        SRCEN_W::new(self)
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
#[doc = "Status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATC_SPEC;
impl crate::RegisterSpec for STATC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statc::R`](R) reader structure"]
impl crate::Readable for STATC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`statc::W`](W) writer structure"]
impl crate::Writable for STATC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATC to value 0"]
impl crate::Resettable for STATC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
