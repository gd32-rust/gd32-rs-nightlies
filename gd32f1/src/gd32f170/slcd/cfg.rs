#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDEN` reader - High drive enable"]
pub type HDEN_R = crate::BitReader<bool>;
#[doc = "Field `HDEN` writer - High drive enable"]
pub type HDEN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 0>;
#[doc = "Field `SOFIE` reader - Start of frame interrupt enable"]
pub type SOFIE_R = crate::BitReader<bool>;
#[doc = "Field `SOFIE` writer - Start of frame interrupt enable"]
pub type SOFIE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 1>;
#[doc = "Field `UPDIE` reader - SLCD update done interrupt enable"]
pub type UPDIE_R = crate::BitReader<bool>;
#[doc = "Field `UPDIE` writer - SLCD update done interrupt enable"]
pub type UPDIE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 3>;
#[doc = "Field `PULSE` reader - Pulse on duration"]
pub type PULSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PULSE` writer - Pulse on duration"]
pub type PULSE_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, 4>;
#[doc = "Field `DTD` reader - Dead time duration"]
pub type DTD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTD` writer - Dead time duration"]
pub type DTD_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, 7>;
#[doc = "Field `CONR` reader - Contrast ratio"]
pub type CONR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONR` writer - Contrast ratio"]
pub type CONR_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, 10>;
#[doc = "Field `BLKDIV` reader - Blink frequency divider"]
pub type BLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLKDIV` writer - Blink frequency divider"]
pub type BLKDIV_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, 13>;
#[doc = "Field `BLKMOD` reader - Blink mode"]
pub type BLKMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLKMOD` writer - Blink mode"]
pub type BLKMOD_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, 16>;
#[doc = "Field `DIV` reader - SLCD clock divider"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - SLCD clock divider"]
pub type DIV_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, 18>;
#[doc = "Field `PSC` reader - SLCD clock prescaler"]
pub type PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSC` writer - SLCD clock prescaler"]
pub type PSC_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, 22>;
impl R {
    #[doc = "Bit 0 - High drive enable"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SLCD update done interrupt enable"]
    #[inline(always)]
    pub fn updie(&self) -> UPDIE_R {
        UPDIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Pulse on duration"]
    #[inline(always)]
    pub fn pulse(&self) -> PULSE_R {
        PULSE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Dead time duration"]
    #[inline(always)]
    pub fn dtd(&self) -> DTD_R {
        DTD_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - Contrast ratio"]
    #[inline(always)]
    pub fn conr(&self) -> CONR_R {
        CONR_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Blink frequency divider"]
    #[inline(always)]
    pub fn blkdiv(&self) -> BLKDIV_R {
        BLKDIV_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Blink mode"]
    #[inline(always)]
    pub fn blkmod(&self) -> BLKMOD_R {
        BLKMOD_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - SLCD clock divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - SLCD clock prescaler"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - High drive enable"]
    #[inline(always)]
    pub fn hden(&mut self) -> HDEN_W {
        HDEN_W::new(self)
    }
    #[doc = "Bit 1 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&mut self) -> SOFIE_W {
        SOFIE_W::new(self)
    }
    #[doc = "Bit 3 - SLCD update done interrupt enable"]
    #[inline(always)]
    pub fn updie(&mut self) -> UPDIE_W {
        UPDIE_W::new(self)
    }
    #[doc = "Bits 4:6 - Pulse on duration"]
    #[inline(always)]
    pub fn pulse(&mut self) -> PULSE_W {
        PULSE_W::new(self)
    }
    #[doc = "Bits 7:9 - Dead time duration"]
    #[inline(always)]
    pub fn dtd(&mut self) -> DTD_W {
        DTD_W::new(self)
    }
    #[doc = "Bits 10:12 - Contrast ratio"]
    #[inline(always)]
    pub fn conr(&mut self) -> CONR_W {
        CONR_W::new(self)
    }
    #[doc = "Bits 13:15 - Blink frequency divider"]
    #[inline(always)]
    pub fn blkdiv(&mut self) -> BLKDIV_W {
        BLKDIV_W::new(self)
    }
    #[doc = "Bits 16:17 - Blink mode"]
    #[inline(always)]
    pub fn blkmod(&mut self) -> BLKMOD_W {
        BLKMOD_W::new(self)
    }
    #[doc = "Bits 18:21 - SLCD clock divider"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W::new(self)
    }
    #[doc = "Bits 22:25 - SLCD clock prescaler"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLCD configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
