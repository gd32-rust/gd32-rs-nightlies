#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "Field `SLCD_DECA` reader - Decoupling capacitance connection for SLCD"]
pub type SLCD_DECA_R = crate::FieldReader;
#[doc = "Field `SLCD_DECA` writer - Decoupling capacitance connection for SLCD"]
pub type SLCD_DECA_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 1:3 - Decoupling capacitance connection for SLCD"]
    #[inline(always)]
    pub fn slcd_deca(&self) -> SLCD_DECA_R {
        SLCD_DECA_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 1:3 - Decoupling capacitance connection for SLCD"]
    #[inline(always)]
    #[must_use]
    pub fn slcd_deca(&mut self) -> SLCD_DECA_W<CFG1_SPEC, 1> {
        SLCD_DECA_W::new(self)
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
#[doc = "System configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
