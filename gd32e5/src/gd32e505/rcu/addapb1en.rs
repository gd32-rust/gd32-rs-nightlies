#[doc = "Register `ADDAPB1EN` reader"]
pub type R = crate::R<ADDAPB1EN_SPEC>;
#[doc = "Register `ADDAPB1EN` writer"]
pub type W = crate::W<ADDAPB1EN_SPEC>;
#[doc = "Field `CTCEN` reader - CTC clock enable"]
pub type CTCEN_R = crate::BitReader;
#[doc = "Field `CTCEN` writer - CTC clock enable"]
pub type CTCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN2EN` reader - CNA2 clock enable"]
pub type CAN2EN_R = crate::BitReader;
#[doc = "Field `CAN2EN` writer - CNA2 clock enable"]
pub type CAN2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 27 - CTC clock enable"]
    #[inline(always)]
    pub fn ctcen(&self) -> CTCEN_R {
        CTCEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - CNA2 clock enable"]
    #[inline(always)]
    pub fn can2en(&self) -> CAN2EN_R {
        CAN2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - CTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctcen(&mut self) -> CTCEN_W<ADDAPB1EN_SPEC, 27> {
        CTCEN_W::new(self)
    }
    #[doc = "Bit 31 - CNA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can2en(&mut self) -> CAN2EN_W<ADDAPB1EN_SPEC, 31> {
        CAN2EN_W::new(self)
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
#[doc = "APB1 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDAPB1EN_SPEC;
impl crate::RegisterSpec for ADDAPB1EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb1en::R`](R) reader structure"]
impl crate::Readable for ADDAPB1EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addapb1en::W`](W) writer structure"]
impl crate::Writable for ADDAPB1EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDAPB1EN to value 0"]
impl crate::Resettable for ADDAPB1EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
