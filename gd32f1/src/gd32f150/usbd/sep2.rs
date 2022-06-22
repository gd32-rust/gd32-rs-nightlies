#[doc = "Register `SEP2` reader"]
pub struct R(crate::R<SEP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEP2` writer"]
pub struct W(crate::W<SEP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEP2_SPEC>;
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
impl From<crate::W<SEP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUB_ST` reader - Successful Receive for LPM Token"]
pub type SUB_ST_R = crate::BitReader<bool>;
#[doc = "Field `SUB_ST` writer - Successful Receive for LPM Token"]
pub type SUB_ST_W<'a> = crate::BitWriter<'a, u16, SEP2_SPEC, bool, 15>;
#[doc = "Field `SUB_STA` reader - Status bits, for the handshake of receiving subpid LPM"]
pub type SUB_STA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUB_STA` writer - Status bits, for the handshake of receiving subpid LPM"]
pub type SUB_STA_W<'a> = crate::FieldWriter<'a, u16, SEP2_SPEC, u8, u8, 2, 12>;
#[doc = "Field `SUBPID_ATTR` reader - LPM Token bmAttribute Field."]
pub type SUBPID_ATTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SUBPID_ATTR` writer - LPM Token bmAttribute Field."]
pub type SUBPID_ATTR_W<'a> = crate::FieldWriter<'a, u16, SEP2_SPEC, u16, u16, 11, 0>;
impl R {
    #[doc = "Bit 15 - Successful Receive for LPM Token"]
    #[inline(always)]
    pub fn sub_st(&self) -> SUB_ST_R {
        SUB_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Status bits, for the handshake of receiving subpid LPM"]
    #[inline(always)]
    pub fn sub_sta(&self) -> SUB_STA_R {
        SUB_STA_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 0:10 - LPM Token bmAttribute Field."]
    #[inline(always)]
    pub fn subpid_attr(&self) -> SUBPID_ATTR_R {
        SUBPID_ATTR_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Successful Receive for LPM Token"]
    #[inline(always)]
    pub fn sub_st(&mut self) -> SUB_ST_W {
        SUB_ST_W::new(self)
    }
    #[doc = "Bits 12:13 - Status bits, for the handshake of receiving subpid LPM"]
    #[inline(always)]
    pub fn sub_sta(&mut self) -> SUB_STA_W {
        SUB_STA_W::new(self)
    }
    #[doc = "Bits 0:10 - LPM Token bmAttribute Field."]
    #[inline(always)]
    pub fn subpid_attr(&mut self) -> SUBPID_ATTR_W {
        SUBPID_ATTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB sub-endpoint 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sep2](index.html) module"]
pub struct SEP2_SPEC;
impl crate::RegisterSpec for SEP2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sep2::R](R) reader structure"]
impl crate::Readable for SEP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sep2::W](W) writer structure"]
impl crate::Writable for SEP2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEP2 to value 0"]
impl crate::Resettable for SEP2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
