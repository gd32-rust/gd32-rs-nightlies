#[doc = "Register `PIOTCFG3` reader"]
pub struct R(crate::R<PIOTCFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIOTCFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIOTCFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIOTCFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIOTCFG3` writer"]
pub struct W(crate::W<PIOTCFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIOTCFG3_SPEC>;
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
impl From<crate::W<PIOTCFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIOTCFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOHIZ` reader - IO space data bus HiZ time"]
pub type IOHIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IOHIZ` writer - IO space data bus HiZ time"]
pub type IOHIZ_W<'a> = crate::FieldWriter<'a, u32, PIOTCFG3_SPEC, u8, u8, 8, 24>;
#[doc = "Field `IOHLD` reader - IO space hold time"]
pub type IOHLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IOHLD` writer - IO space hold time"]
pub type IOHLD_W<'a> = crate::FieldWriter<'a, u32, PIOTCFG3_SPEC, u8, u8, 8, 16>;
#[doc = "Field `IOWAIT` reader - IO space wait time"]
pub type IOWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IOWAIT` writer - IO space wait time"]
pub type IOWAIT_W<'a> = crate::FieldWriter<'a, u32, PIOTCFG3_SPEC, u8, u8, 8, 8>;
#[doc = "Field `IOSET` reader - IO space setup time"]
pub type IOSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IOSET` writer - IO space setup time"]
pub type IOSET_W<'a> = crate::FieldWriter<'a, u32, PIOTCFG3_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 24:31 - IO space data bus HiZ time"]
    #[inline(always)]
    pub fn iohiz(&self) -> IOHIZ_R {
        IOHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - IO space hold time"]
    #[inline(always)]
    pub fn iohld(&self) -> IOHLD_R {
        IOHLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - IO space wait time"]
    #[inline(always)]
    pub fn iowait(&self) -> IOWAIT_R {
        IOWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - IO space setup time"]
    #[inline(always)]
    pub fn ioset(&self) -> IOSET_R {
        IOSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - IO space data bus HiZ time"]
    #[inline(always)]
    pub fn iohiz(&mut self) -> IOHIZ_W {
        IOHIZ_W::new(self)
    }
    #[doc = "Bits 16:23 - IO space hold time"]
    #[inline(always)]
    pub fn iohld(&mut self) -> IOHLD_W {
        IOHLD_W::new(self)
    }
    #[doc = "Bits 8:15 - IO space wait time"]
    #[inline(always)]
    pub fn iowait(&mut self) -> IOWAIT_W {
        IOWAIT_W::new(self)
    }
    #[doc = "Bits 0:7 - IO space setup time"]
    #[inline(always)]
    pub fn ioset(&mut self) -> IOSET_W {
        IOSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PC card I/O space timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [piotcfg3](index.html) module"]
pub struct PIOTCFG3_SPEC;
impl crate::RegisterSpec for PIOTCFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [piotcfg3::R](R) reader structure"]
impl crate::Readable for PIOTCFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [piotcfg3::W](W) writer structure"]
impl crate::Writable for PIOTCFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIOTCFG3 to value 0xfcfc_fcfc"]
impl crate::Resettable for PIOTCFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}
