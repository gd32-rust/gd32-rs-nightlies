#[doc = "Register `L1DC` reader"]
pub type R = crate::R<L1DC_SPEC>;
#[doc = "Register `L1DC` writer"]
pub type W = crate::W<L1DC_SPEC>;
#[doc = "Field `DCB` reader - The default color blue"]
pub type DCB_R = crate::FieldReader;
#[doc = "Field `DCB` writer - The default color blue"]
pub type DCB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DCG` reader - The default color green"]
pub type DCG_R = crate::FieldReader;
#[doc = "Field `DCG` writer - The default color green"]
pub type DCG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DCR` reader - The default color red"]
pub type DCR_R = crate::FieldReader;
#[doc = "Field `DCR` writer - The default color red"]
pub type DCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DCA` reader - The default color ALPHA"]
pub type DCA_R = crate::FieldReader;
#[doc = "Field `DCA` writer - The default color ALPHA"]
pub type DCA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The default color blue"]
    #[inline(always)]
    pub fn dcb(&self) -> DCB_R {
        DCB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The default color green"]
    #[inline(always)]
    pub fn dcg(&self) -> DCG_R {
        DCG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The default color red"]
    #[inline(always)]
    pub fn dcr(&self) -> DCR_R {
        DCR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The default color ALPHA"]
    #[inline(always)]
    pub fn dca(&self) -> DCA_R {
        DCA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The default color blue"]
    #[inline(always)]
    #[must_use]
    pub fn dcb(&mut self) -> DCB_W<L1DC_SPEC, 0> {
        DCB_W::new(self)
    }
    #[doc = "Bits 8:15 - The default color green"]
    #[inline(always)]
    #[must_use]
    pub fn dcg(&mut self) -> DCG_W<L1DC_SPEC, 8> {
        DCG_W::new(self)
    }
    #[doc = "Bits 16:23 - The default color red"]
    #[inline(always)]
    #[must_use]
    pub fn dcr(&mut self) -> DCR_W<L1DC_SPEC, 16> {
        DCR_W::new(self)
    }
    #[doc = "Bits 24:31 - The default color ALPHA"]
    #[inline(always)]
    #[must_use]
    pub fn dca(&mut self) -> DCA_W<L1DC_SPEC, 24> {
        DCA_W::new(self)
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
#[doc = "Layer 1 default color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1dc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1dc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1DC_SPEC;
impl crate::RegisterSpec for L1DC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1dc::R`](R) reader structure"]
impl crate::Readable for L1DC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1dc::W`](W) writer structure"]
impl crate::Writable for L1DC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1DC to value 0"]
impl crate::Resettable for L1DC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
