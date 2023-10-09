#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "Field `HXTALPREDV` reader - CK_HXTAL or CK_IRC48M divider previous PLL"]
pub type HXTALPREDV_R = crate::FieldReader<HXTALPREDV_A>;
#[doc = "CK_HXTAL or CK_IRC48M divider previous PLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HXTALPREDV_A {
    #[doc = "0: HXTAL input to PLL not divided"]
    DIV1 = 0,
    #[doc = "1: HXTAL input to PLL divided by 2"]
    DIV2 = 1,
    #[doc = "2: HXTAL input to PLL divided by 3"]
    DIV3 = 2,
    #[doc = "3: HXTAL input to PLL divided by 4"]
    DIV4 = 3,
    #[doc = "4: HXTAL input to PLL divided by 5"]
    DIV5 = 4,
    #[doc = "5: HXTAL input to PLL divided by 6"]
    DIV6 = 5,
    #[doc = "6: HXTAL input to PLL divided by 7"]
    DIV7 = 6,
    #[doc = "7: HXTAL input to PLL divided by 8"]
    DIV8 = 7,
    #[doc = "8: HXTAL input to PLL divided by 9"]
    DIV9 = 8,
    #[doc = "9: HXTAL input to PLL divided by 10"]
    DIV10 = 9,
    #[doc = "10: HXTAL input to PLL divided by 11"]
    DIV11 = 10,
    #[doc = "11: HXTAL input to PLL divided by 12"]
    DIV12 = 11,
    #[doc = "12: HXTAL input to PLL divided by 13"]
    DIV13 = 12,
    #[doc = "13: HXTAL input to PLL divided by 14"]
    DIV14 = 13,
    #[doc = "14: HXTAL input to PLL divided by 15"]
    DIV15 = 14,
    #[doc = "15: HXTAL input to PLL divided by 16"]
    DIV16 = 15,
}
impl From<HXTALPREDV_A> for u8 {
    #[inline(always)]
    fn from(variant: HXTALPREDV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HXTALPREDV_A {
    type Ux = u8;
}
impl HXTALPREDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTALPREDV_A {
        match self.bits {
            0 => HXTALPREDV_A::DIV1,
            1 => HXTALPREDV_A::DIV2,
            2 => HXTALPREDV_A::DIV3,
            3 => HXTALPREDV_A::DIV4,
            4 => HXTALPREDV_A::DIV5,
            5 => HXTALPREDV_A::DIV6,
            6 => HXTALPREDV_A::DIV7,
            7 => HXTALPREDV_A::DIV8,
            8 => HXTALPREDV_A::DIV9,
            9 => HXTALPREDV_A::DIV10,
            10 => HXTALPREDV_A::DIV11,
            11 => HXTALPREDV_A::DIV12,
            12 => HXTALPREDV_A::DIV13,
            13 => HXTALPREDV_A::DIV14,
            14 => HXTALPREDV_A::DIV15,
            15 => HXTALPREDV_A::DIV16,
            _ => unreachable!(),
        }
    }
    #[doc = "HXTAL input to PLL not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HXTALPREDV_A::DIV1
    }
    #[doc = "HXTAL input to PLL divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HXTALPREDV_A::DIV2
    }
    #[doc = "HXTAL input to PLL divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == HXTALPREDV_A::DIV3
    }
    #[doc = "HXTAL input to PLL divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HXTALPREDV_A::DIV4
    }
    #[doc = "HXTAL input to PLL divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == HXTALPREDV_A::DIV5
    }
    #[doc = "HXTAL input to PLL divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == HXTALPREDV_A::DIV6
    }
    #[doc = "HXTAL input to PLL divided by 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == HXTALPREDV_A::DIV7
    }
    #[doc = "HXTAL input to PLL divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HXTALPREDV_A::DIV8
    }
    #[doc = "HXTAL input to PLL divided by 9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == HXTALPREDV_A::DIV9
    }
    #[doc = "HXTAL input to PLL divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == HXTALPREDV_A::DIV10
    }
    #[doc = "HXTAL input to PLL divided by 11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == HXTALPREDV_A::DIV11
    }
    #[doc = "HXTAL input to PLL divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == HXTALPREDV_A::DIV12
    }
    #[doc = "HXTAL input to PLL divided by 13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == HXTALPREDV_A::DIV13
    }
    #[doc = "HXTAL input to PLL divided by 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == HXTALPREDV_A::DIV14
    }
    #[doc = "HXTAL input to PLL divided by 15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == HXTALPREDV_A::DIV15
    }
    #[doc = "HXTAL input to PLL divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HXTALPREDV_A::DIV16
    }
}
#[doc = "Field `HXTALPREDV` writer - CK_HXTAL or CK_IRC48M divider previous PLL"]
pub type HXTALPREDV_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, HXTALPREDV_A>;
impl<'a, REG, const O: u8> HXTALPREDV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HXTAL input to PLL not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV1)
    }
    #[doc = "HXTAL input to PLL divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV2)
    }
    #[doc = "HXTAL input to PLL divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV3)
    }
    #[doc = "HXTAL input to PLL divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV4)
    }
    #[doc = "HXTAL input to PLL divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV5)
    }
    #[doc = "HXTAL input to PLL divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV6)
    }
    #[doc = "HXTAL input to PLL divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV7)
    }
    #[doc = "HXTAL input to PLL divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV8)
    }
    #[doc = "HXTAL input to PLL divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV9)
    }
    #[doc = "HXTAL input to PLL divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV10)
    }
    #[doc = "HXTAL input to PLL divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV11)
    }
    #[doc = "HXTAL input to PLL divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV12)
    }
    #[doc = "HXTAL input to PLL divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV13)
    }
    #[doc = "HXTAL input to PLL divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV14)
    }
    #[doc = "HXTAL input to PLL divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV15)
    }
    #[doc = "HXTAL input to PLL divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALPREDV_A::DIV16)
    }
}
impl R {
    #[doc = "Bits 0:3 - CK_HXTAL or CK_IRC48M divider previous PLL"]
    #[inline(always)]
    pub fn hxtalpredv(&self) -> HXTALPREDV_R {
        HXTALPREDV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CK_HXTAL or CK_IRC48M divider previous PLL"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalpredv(&mut self) -> HXTALPREDV_W<CFG1_SPEC, 0> {
        HXTALPREDV_W::new(self)
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
#[doc = "Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
