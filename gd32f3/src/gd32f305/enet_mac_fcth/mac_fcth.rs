#[doc = "Register `MAC_FCTH` reader"]
pub type R = crate::R<MAC_FCTH_SPEC>;
#[doc = "Register `MAC_FCTH` writer"]
pub type W = crate::W<MAC_FCTH_SPEC>;
#[doc = "Field `RFA` reader - Threshold of active flow control"]
pub type RFA_R = crate::FieldReader;
#[doc = "Field `RFA` writer - Threshold of active flow control"]
pub type RFA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RFD` reader - Threshold of deactive flow control"]
pub type RFD_R = crate::FieldReader;
#[doc = "Field `RFD` writer - Threshold of deactive flow control"]
pub type RFD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Threshold of active flow control"]
    #[inline(always)]
    pub fn rfa(&self) -> RFA_R {
        RFA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Threshold of deactive flow control"]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Threshold of active flow control"]
    #[inline(always)]
    #[must_use]
    pub fn rfa(&mut self) -> RFA_W<MAC_FCTH_SPEC, 0> {
        RFA_W::new(self)
    }
    #[doc = "Bits 4:6 - Threshold of deactive flow control"]
    #[inline(always)]
    #[must_use]
    pub fn rfd(&mut self) -> RFD_W<MAC_FCTH_SPEC, 4> {
        RFD_W::new(self)
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
#[doc = "Ethernet MAC flow control threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_fcth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_fcth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_FCTH_SPEC;
impl crate::RegisterSpec for MAC_FCTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_fcth::R`](R) reader structure"]
impl crate::Readable for MAC_FCTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_fcth::W`](W) writer structure"]
impl crate::Writable for MAC_FCTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_FCTH to value 0x15"]
impl crate::Resettable for MAC_FCTH_SPEC {
    const RESET_VALUE: Self::Ux = 0x15;
}
