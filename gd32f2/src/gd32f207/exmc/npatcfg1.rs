#[doc = "Register `NPATCFG1` reader"]
pub struct R(crate::R<NPATCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NPATCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NPATCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NPATCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NPATCFG1` writer"]
pub struct W(crate::W<NPATCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NPATCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<NPATCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NPATCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTHIZ` reader - Attribute memory data bus HiZ time"]
pub type ATTHIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTHIZ` writer - Attribute memory data bus HiZ time"]
pub type ATTHIZ_W<'a> = crate::FieldWriter<'a, u32, NPATCFG1_SPEC, u8, u8, 8, 24>;
#[doc = "Field `ATTHLD` reader - Attribute memory hold time"]
pub type ATTHLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTHLD` writer - Attribute memory hold time"]
pub type ATTHLD_W<'a> = crate::FieldWriter<'a, u32, NPATCFG1_SPEC, u8, u8, 8, 16>;
#[doc = "Field `ATTWAIT` reader - Attribute memory wait time"]
pub type ATTWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTWAIT` writer - Attribute memory wait time"]
pub type ATTWAIT_W<'a> = crate::FieldWriter<'a, u32, NPATCFG1_SPEC, u8, u8, 8, 8>;
#[doc = "Field `ATTSET` reader - Attribute memory setup time"]
pub type ATTSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTSET` writer - Attribute memory setup time"]
pub type ATTSET_W<'a> = crate::FieldWriter<'a, u32, NPATCFG1_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 24:31 - Attribute memory data bus HiZ time"]
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Attribute memory hold time"]
    #[inline(always)]
    pub fn atthld(&self) -> ATTHLD_R {
        ATTHLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Attribute memory wait time"]
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Attribute memory setup time"]
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Attribute memory data bus HiZ time"]
    #[inline(always)]
    pub fn atthiz(&mut self) -> ATTHIZ_W {
        ATTHIZ_W::new(self)
    }
    #[doc = "Bits 16:23 - Attribute memory hold time"]
    #[inline(always)]
    pub fn atthld(&mut self) -> ATTHLD_W {
        ATTHLD_W::new(self)
    }
    #[doc = "Bits 8:15 - Attribute memory wait time"]
    #[inline(always)]
    pub fn attwait(&mut self) -> ATTWAIT_W {
        ATTWAIT_W::new(self)
    }
    #[doc = "Bits 0:7 - Attribute memory setup time"]
    #[inline(always)]
    pub fn attset(&mut self) -> ATTSET_W {
        ATTSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NAND flash/PC card attribute space timing configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npatcfg1](index.html) module"]
pub struct NPATCFG1_SPEC;
impl crate::RegisterSpec for NPATCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [npatcfg1::R](R) reader structure"]
impl crate::Readable for NPATCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [npatcfg1::W](W) writer structure"]
impl crate::Writable for NPATCFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NPATCFG1 to value 0xfcfc_fcfc"]
impl crate::Resettable for NPATCFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}
