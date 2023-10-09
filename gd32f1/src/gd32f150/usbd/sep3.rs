#[doc = "Register `SEP3` reader"]
pub type R = crate::R<SEP3_SPEC>;
#[doc = "Register `SEP3` writer"]
pub type W = crate::W<SEP3_SPEC>;
#[doc = "Field `SUBPID_ATTR` reader - LPM Token bmAttribute Field."]
pub type SUBPID_ATTR_R = crate::FieldReader<u16>;
#[doc = "Field `SUBPID_ATTR` writer - LPM Token bmAttribute Field."]
pub type SUBPID_ATTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `SUB_STA` reader - Status bits, for the handshake of receiving subpid LPM"]
pub type SUB_STA_R = crate::FieldReader;
#[doc = "Field `SUB_STA` writer - Status bits, for the handshake of receiving subpid LPM"]
pub type SUB_STA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SUB_ST` reader - Successful Receive for LPM Token"]
pub type SUB_ST_R = crate::BitReader;
#[doc = "Field `SUB_ST` writer - Successful Receive for LPM Token"]
pub type SUB_ST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:10 - LPM Token bmAttribute Field."]
    #[inline(always)]
    pub fn subpid_attr(&self) -> SUBPID_ATTR_R {
        SUBPID_ATTR_R::new(self.bits & 0x07ff)
    }
    #[doc = "Bits 12:13 - Status bits, for the handshake of receiving subpid LPM"]
    #[inline(always)]
    pub fn sub_sta(&self) -> SUB_STA_R {
        SUB_STA_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Successful Receive for LPM Token"]
    #[inline(always)]
    pub fn sub_st(&self) -> SUB_ST_R {
        SUB_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - LPM Token bmAttribute Field."]
    #[inline(always)]
    #[must_use]
    pub fn subpid_attr(&mut self) -> SUBPID_ATTR_W<SEP3_SPEC, 0> {
        SUBPID_ATTR_W::new(self)
    }
    #[doc = "Bits 12:13 - Status bits, for the handshake of receiving subpid LPM"]
    #[inline(always)]
    #[must_use]
    pub fn sub_sta(&mut self) -> SUB_STA_W<SEP3_SPEC, 12> {
        SUB_STA_W::new(self)
    }
    #[doc = "Bit 15 - Successful Receive for LPM Token"]
    #[inline(always)]
    #[must_use]
    pub fn sub_st(&mut self) -> SUB_ST_W<SEP3_SPEC, 15> {
        SUB_ST_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB sub-endpoint 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEP3_SPEC;
impl crate::RegisterSpec for SEP3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sep3::R`](R) reader structure"]
impl crate::Readable for SEP3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sep3::W`](W) writer structure"]
impl crate::Writable for SEP3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEP3 to value 0"]
impl crate::Resettable for SEP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
