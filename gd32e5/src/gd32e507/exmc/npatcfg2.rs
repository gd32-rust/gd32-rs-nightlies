#[doc = "Register `NPATCFG2` reader"]
pub type R = crate::R<NPATCFG2_SPEC>;
#[doc = "Register `NPATCFG2` writer"]
pub type W = crate::W<NPATCFG2_SPEC>;
#[doc = "Field `ATTSET` reader - Attribute memory setup time"]
pub type ATTSET_R = crate::FieldReader;
#[doc = "Field `ATTSET` writer - Attribute memory setup time"]
pub type ATTSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ATTWAIT` reader - Attribute memory wait time"]
pub type ATTWAIT_R = crate::FieldReader;
#[doc = "Field `ATTWAIT` writer - Attribute memory wait time"]
pub type ATTWAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ATTHLD` reader - Attribute memory hold time"]
pub type ATTHLD_R = crate::FieldReader;
#[doc = "Field `ATTHLD` writer - Attribute memory hold time"]
pub type ATTHLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ATTHIZ` reader - Attribute memory data bus HiZ time"]
pub type ATTHIZ_R = crate::FieldReader;
#[doc = "Field `ATTHIZ` writer - Attribute memory data bus HiZ time"]
pub type ATTHIZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Attribute memory setup time"]
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Attribute memory wait time"]
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Attribute memory hold time"]
    #[inline(always)]
    pub fn atthld(&self) -> ATTHLD_R {
        ATTHLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Attribute memory data bus HiZ time"]
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Attribute memory setup time"]
    #[inline(always)]
    #[must_use]
    pub fn attset(&mut self) -> ATTSET_W<NPATCFG2_SPEC, 0> {
        ATTSET_W::new(self)
    }
    #[doc = "Bits 8:15 - Attribute memory wait time"]
    #[inline(always)]
    #[must_use]
    pub fn attwait(&mut self) -> ATTWAIT_W<NPATCFG2_SPEC, 8> {
        ATTWAIT_W::new(self)
    }
    #[doc = "Bits 16:23 - Attribute memory hold time"]
    #[inline(always)]
    #[must_use]
    pub fn atthld(&mut self) -> ATTHLD_W<NPATCFG2_SPEC, 16> {
        ATTHLD_W::new(self)
    }
    #[doc = "Bits 24:31 - Attribute memory data bus HiZ time"]
    #[inline(always)]
    #[must_use]
    pub fn atthiz(&mut self) -> ATTHIZ_W<NPATCFG2_SPEC, 24> {
        ATTHIZ_W::new(self)
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
#[doc = "NAND flash/PC card attribute space timing configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npatcfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npatcfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NPATCFG2_SPEC;
impl crate::RegisterSpec for NPATCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`npatcfg2::R`](R) reader structure"]
impl crate::Readable for NPATCFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`npatcfg2::W`](W) writer structure"]
impl crate::Writable for NPATCFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NPATCFG2 to value 0xffff_ffff"]
impl crate::Resettable for NPATCFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
