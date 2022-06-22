#[doc = "Register `NPCTCFG1` reader"]
pub struct R(crate::R<NPCTCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NPCTCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NPCTCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NPCTCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NPCTCFG1` writer"]
pub struct W(crate::W<NPCTCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NPCTCFG1_SPEC>;
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
impl From<crate::W<NPCTCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NPCTCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMHIZ` reader - Common memory data bus HiZ time"]
pub type COMHIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMHIZ` writer - Common memory data bus HiZ time"]
pub type COMHIZ_W<'a> = crate::FieldWriter<'a, u32, NPCTCFG1_SPEC, u8, u8, 8, 24>;
#[doc = "Field `COMHLD` reader - Common memory hold time"]
pub type COMHLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMHLD` writer - Common memory hold time"]
pub type COMHLD_W<'a> = crate::FieldWriter<'a, u32, NPCTCFG1_SPEC, u8, u8, 8, 16>;
#[doc = "Field `COMWAIT` reader - Common memory wait time"]
pub type COMWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMWAIT` writer - Common memory wait time"]
pub type COMWAIT_W<'a> = crate::FieldWriter<'a, u32, NPCTCFG1_SPEC, u8, u8, 8, 8>;
#[doc = "Field `COMSET` reader - Common memory setup time"]
pub type COMSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMSET` writer - Common memory setup time"]
pub type COMSET_W<'a> = crate::FieldWriter<'a, u32, NPCTCFG1_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 24:31 - Common memory data bus HiZ time"]
    #[inline(always)]
    pub fn comhiz(&self) -> COMHIZ_R {
        COMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Common memory hold time"]
    #[inline(always)]
    pub fn comhld(&self) -> COMHLD_R {
        COMHLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Common memory wait time"]
    #[inline(always)]
    pub fn comwait(&self) -> COMWAIT_R {
        COMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Common memory setup time"]
    #[inline(always)]
    pub fn comset(&self) -> COMSET_R {
        COMSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Common memory data bus HiZ time"]
    #[inline(always)]
    pub fn comhiz(&mut self) -> COMHIZ_W {
        COMHIZ_W::new(self)
    }
    #[doc = "Bits 16:23 - Common memory hold time"]
    #[inline(always)]
    pub fn comhld(&mut self) -> COMHLD_W {
        COMHLD_W::new(self)
    }
    #[doc = "Bits 8:15 - Common memory wait time"]
    #[inline(always)]
    pub fn comwait(&mut self) -> COMWAIT_W {
        COMWAIT_W::new(self)
    }
    #[doc = "Bits 0:7 - Common memory setup time"]
    #[inline(always)]
    pub fn comset(&mut self) -> COMSET_W {
        COMSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NAND flash/PC card common space timing configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npctcfg1](index.html) module"]
pub struct NPCTCFG1_SPEC;
impl crate::RegisterSpec for NPCTCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [npctcfg1::R](R) reader structure"]
impl crate::Readable for NPCTCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [npctcfg1::W](W) writer structure"]
impl crate::Writable for NPCTCFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NPCTCFG1 to value 0xfcfc_fcfc"]
impl crate::Resettable for NPCTCFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}
