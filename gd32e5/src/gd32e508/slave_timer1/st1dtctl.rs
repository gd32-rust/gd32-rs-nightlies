#[doc = "Register `ST1DTCTL` reader"]
pub type R = crate::R<ST1DTCTL_SPEC>;
#[doc = "Register `ST1DTCTL` writer"]
pub type W = crate::W<ST1DTCTL_SPEC>;
#[doc = "Field `DTRCFG` reader - Falling edge dead-time value"]
pub type DTRCFG_R = crate::FieldReader<u16>;
#[doc = "Field `DTRCFG` writer - Falling edge dead-time value"]
pub type DTRCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `DTRS` reader - The sign of falling edge dead-time value"]
pub type DTRS_R = crate::BitReader;
#[doc = "Field `DTRS` writer - The sign of falling edge dead-time value"]
pub type DTRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTGCKDIV` reader - Dead time generator clock division"]
pub type DTGCKDIV_R = crate::FieldReader;
#[doc = "Field `DTGCKDIV` writer - Dead time generator clock division"]
pub type DTGCKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DTRSPROT` reader - Dead-time rising edge protection for sign"]
pub type DTRSPROT_R = crate::BitReader;
#[doc = "Field `DTRSPROT` writer - Dead-time rising edge protection for sign"]
pub type DTRSPROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTRSVPROT` reader - Dead-time rising edge protection for value and sign"]
pub type DTRSVPROT_R = crate::BitReader;
#[doc = "Field `DTRSVPROT` writer - Dead-time rising edge protection for value and sign"]
pub type DTRSVPROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTFCFG` reader - Falling edge dead-time value"]
pub type DTFCFG_R = crate::FieldReader<u16>;
#[doc = "Field `DTFCFG` writer - Falling edge dead-time value"]
pub type DTFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `DTFS` reader - The sign of falling edge dead-time value"]
pub type DTFS_R = crate::BitReader;
#[doc = "Field `DTFS` writer - The sign of falling edge dead-time value"]
pub type DTFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTFSPROT` reader - Dead-time falling edge protection for sign"]
pub type DTFSPROT_R = crate::BitReader;
#[doc = "Field `DTFSPROT` writer - Dead-time falling edge protection for sign"]
pub type DTFSPROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTFSVPROT` reader - Dead-time falling edge protection for value and sign"]
pub type DTFSVPROT_R = crate::BitReader;
#[doc = "Field `DTFSVPROT` writer - Dead-time falling edge protection for value and sign"]
pub type DTFSVPROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:8 - Falling edge dead-time value"]
    #[inline(always)]
    pub fn dtrcfg(&self) -> DTRCFG_R {
        DTRCFG_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - The sign of falling edge dead-time value"]
    #[inline(always)]
    pub fn dtrs(&self) -> DTRS_R {
        DTRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Dead time generator clock division"]
    #[inline(always)]
    pub fn dtgckdiv(&self) -> DTGCKDIV_R {
        DTGCKDIV_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Dead-time rising edge protection for sign"]
    #[inline(always)]
    pub fn dtrsprot(&self) -> DTRSPROT_R {
        DTRSPROT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Dead-time rising edge protection for value and sign"]
    #[inline(always)]
    pub fn dtrsvprot(&self) -> DTRSVPROT_R {
        DTRSVPROT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Falling edge dead-time value"]
    #[inline(always)]
    pub fn dtfcfg(&self) -> DTFCFG_R {
        DTFCFG_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - The sign of falling edge dead-time value"]
    #[inline(always)]
    pub fn dtfs(&self) -> DTFS_R {
        DTFS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - Dead-time falling edge protection for sign"]
    #[inline(always)]
    pub fn dtfsprot(&self) -> DTFSPROT_R {
        DTFSPROT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Dead-time falling edge protection for value and sign"]
    #[inline(always)]
    pub fn dtfsvprot(&self) -> DTFSVPROT_R {
        DTFSVPROT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Falling edge dead-time value"]
    #[inline(always)]
    #[must_use]
    pub fn dtrcfg(&mut self) -> DTRCFG_W<ST1DTCTL_SPEC, 0> {
        DTRCFG_W::new(self)
    }
    #[doc = "Bit 9 - The sign of falling edge dead-time value"]
    #[inline(always)]
    #[must_use]
    pub fn dtrs(&mut self) -> DTRS_W<ST1DTCTL_SPEC, 9> {
        DTRS_W::new(self)
    }
    #[doc = "Bits 10:13 - Dead time generator clock division"]
    #[inline(always)]
    #[must_use]
    pub fn dtgckdiv(&mut self) -> DTGCKDIV_W<ST1DTCTL_SPEC, 10> {
        DTGCKDIV_W::new(self)
    }
    #[doc = "Bit 14 - Dead-time rising edge protection for sign"]
    #[inline(always)]
    #[must_use]
    pub fn dtrsprot(&mut self) -> DTRSPROT_W<ST1DTCTL_SPEC, 14> {
        DTRSPROT_W::new(self)
    }
    #[doc = "Bit 15 - Dead-time rising edge protection for value and sign"]
    #[inline(always)]
    #[must_use]
    pub fn dtrsvprot(&mut self) -> DTRSVPROT_W<ST1DTCTL_SPEC, 15> {
        DTRSVPROT_W::new(self)
    }
    #[doc = "Bits 16:24 - Falling edge dead-time value"]
    #[inline(always)]
    #[must_use]
    pub fn dtfcfg(&mut self) -> DTFCFG_W<ST1DTCTL_SPEC, 16> {
        DTFCFG_W::new(self)
    }
    #[doc = "Bit 25 - The sign of falling edge dead-time value"]
    #[inline(always)]
    #[must_use]
    pub fn dtfs(&mut self) -> DTFS_W<ST1DTCTL_SPEC, 25> {
        DTFS_W::new(self)
    }
    #[doc = "Bit 30 - Dead-time falling edge protection for sign"]
    #[inline(always)]
    #[must_use]
    pub fn dtfsprot(&mut self) -> DTFSPROT_W<ST1DTCTL_SPEC, 30> {
        DTFSPROT_W::new(self)
    }
    #[doc = "Bit 31 - Dead-time falling edge protection for value and sign"]
    #[inline(always)]
    #[must_use]
    pub fn dtfsvprot(&mut self) -> DTFSVPROT_W<ST1DTCTL_SPEC, 31> {
        DTFSVPROT_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMER1 dead-time control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1dtctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1dtctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST1DTCTL_SPEC;
impl crate::RegisterSpec for ST1DTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1dtctl::R`](R) reader structure"]
impl crate::Readable for ST1DTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st1dtctl::W`](W) writer structure"]
impl crate::Writable for ST1DTCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1DTCTL to value 0"]
impl crate::Resettable for ST1DTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
