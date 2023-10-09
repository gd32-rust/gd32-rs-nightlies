#[doc = "Register `HCH0LEN` reader"]
pub type R = crate::R<HCH0LEN_SPEC>;
#[doc = "Register `HCH0LEN` writer"]
pub type W = crate::W<HCH0LEN_SPEC>;
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TLEN_R = crate::FieldReader<u32>;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 19, O, u32>;
#[doc = "Field `PCNT` reader - Packet count"]
pub type PCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader;
#[doc = "Field `DPID` writer - Data PID"]
pub type DPID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PING` reader - PING token request"]
pub type PING_R = crate::BitReader;
#[doc = "Field `PING` writer - PING token request"]
pub type PING_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - PING token request"]
    #[inline(always)]
    pub fn ping(&self) -> PING_R {
        PING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    #[must_use]
    pub fn tlen(&mut self) -> TLEN_W<HCH0LEN_SPEC, 0> {
        TLEN_W::new(self)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt(&mut self) -> PCNT_W<HCH0LEN_SPEC, 19> {
        PCNT_W::new(self)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    #[must_use]
    pub fn dpid(&mut self) -> DPID_W<HCH0LEN_SPEC, 29> {
        DPID_W::new(self)
    }
    #[doc = "Bit 31 - PING token request"]
    #[inline(always)]
    #[must_use]
    pub fn ping(&mut self) -> PING_W<HCH0LEN_SPEC, 31> {
        PING_W::new(self)
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
#[doc = "host channel-0 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCH0LEN_SPEC;
impl crate::RegisterSpec for HCH0LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch0len::R`](R) reader structure"]
impl crate::Readable for HCH0LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hch0len::W`](W) writer structure"]
impl crate::Writable for HCH0LEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCH0LEN to value 0"]
impl crate::Resettable for HCH0LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
