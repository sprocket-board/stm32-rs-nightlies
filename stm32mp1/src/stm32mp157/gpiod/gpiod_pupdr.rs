#[doc = "Register `GPIOD_PUPDR` reader"]
pub struct R(crate::R<GPIOD_PUPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOD_PUPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOD_PUPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOD_PUPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOD_PUPDR` writer"]
pub struct W(crate::W<GPIOD_PUPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOD_PUPDR_SPEC>;
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
impl From<crate::W<GPIOD_PUPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOD_PUPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUPDR0` reader - PUPDR0"]
pub type PUPDR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR0` writer - PUPDR0"]
pub type PUPDR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR1` reader - PUPDR1"]
pub type PUPDR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR1` writer - PUPDR1"]
pub type PUPDR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR2` reader - PUPDR2"]
pub type PUPDR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR2` writer - PUPDR2"]
pub type PUPDR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR3` reader - PUPDR3"]
pub type PUPDR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR3` writer - PUPDR3"]
pub type PUPDR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR4` reader - PUPDR4"]
pub type PUPDR4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR4` writer - PUPDR4"]
pub type PUPDR4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR5` reader - PUPDR5"]
pub type PUPDR5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR5` writer - PUPDR5"]
pub type PUPDR5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR6` reader - PUPDR6"]
pub type PUPDR6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR6` writer - PUPDR6"]
pub type PUPDR6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR7` reader - PUPDR7"]
pub type PUPDR7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR7` writer - PUPDR7"]
pub type PUPDR7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR8` reader - PUPDR8"]
pub type PUPDR8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR8` writer - PUPDR8"]
pub type PUPDR8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR9` reader - PUPDR9"]
pub type PUPDR9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR9` writer - PUPDR9"]
pub type PUPDR9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR10` reader - PUPDR10"]
pub type PUPDR10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR10` writer - PUPDR10"]
pub type PUPDR10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR11` reader - PUPDR11"]
pub type PUPDR11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR11` writer - PUPDR11"]
pub type PUPDR11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR12` reader - PUPDR12"]
pub type PUPDR12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR12` writer - PUPDR12"]
pub type PUPDR12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR13` reader - PUPDR13"]
pub type PUPDR13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR13` writer - PUPDR13"]
pub type PUPDR13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR14` reader - PUPDR14"]
pub type PUPDR14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR14` writer - PUPDR14"]
pub type PUPDR14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPDR15` reader - PUPDR15"]
pub type PUPDR15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPDR15` writer - PUPDR15"]
pub type PUPDR15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOD_PUPDR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PUPDR0"]
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR0_R {
        PUPDR0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PUPDR1"]
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR1_R {
        PUPDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PUPDR2"]
    #[inline(always)]
    pub fn pupdr2(&self) -> PUPDR2_R {
        PUPDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PUPDR3"]
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PUPDR4"]
    #[inline(always)]
    pub fn pupdr4(&self) -> PUPDR4_R {
        PUPDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PUPDR5"]
    #[inline(always)]
    pub fn pupdr5(&self) -> PUPDR5_R {
        PUPDR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PUPDR6"]
    #[inline(always)]
    pub fn pupdr6(&self) -> PUPDR6_R {
        PUPDR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PUPDR7"]
    #[inline(always)]
    pub fn pupdr7(&self) -> PUPDR7_R {
        PUPDR7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PUPDR8"]
    #[inline(always)]
    pub fn pupdr8(&self) -> PUPDR8_R {
        PUPDR8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PUPDR9"]
    #[inline(always)]
    pub fn pupdr9(&self) -> PUPDR9_R {
        PUPDR9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PUPDR10"]
    #[inline(always)]
    pub fn pupdr10(&self) -> PUPDR10_R {
        PUPDR10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PUPDR11"]
    #[inline(always)]
    pub fn pupdr11(&self) -> PUPDR11_R {
        PUPDR11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PUPDR12"]
    #[inline(always)]
    pub fn pupdr12(&self) -> PUPDR12_R {
        PUPDR12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PUPDR13"]
    #[inline(always)]
    pub fn pupdr13(&self) -> PUPDR13_R {
        PUPDR13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PUPDR14"]
    #[inline(always)]
    pub fn pupdr14(&self) -> PUPDR14_R {
        PUPDR14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PUPDR15"]
    #[inline(always)]
    pub fn pupdr15(&self) -> PUPDR15_R {
        PUPDR15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PUPDR0"]
    #[inline(always)]
    pub fn pupdr0(&mut self) -> PUPDR0_W<0> {
        PUPDR0_W::new(self)
    }
    #[doc = "Bits 2:3 - PUPDR1"]
    #[inline(always)]
    pub fn pupdr1(&mut self) -> PUPDR1_W<2> {
        PUPDR1_W::new(self)
    }
    #[doc = "Bits 4:5 - PUPDR2"]
    #[inline(always)]
    pub fn pupdr2(&mut self) -> PUPDR2_W<4> {
        PUPDR2_W::new(self)
    }
    #[doc = "Bits 6:7 - PUPDR3"]
    #[inline(always)]
    pub fn pupdr3(&mut self) -> PUPDR3_W<6> {
        PUPDR3_W::new(self)
    }
    #[doc = "Bits 8:9 - PUPDR4"]
    #[inline(always)]
    pub fn pupdr4(&mut self) -> PUPDR4_W<8> {
        PUPDR4_W::new(self)
    }
    #[doc = "Bits 10:11 - PUPDR5"]
    #[inline(always)]
    pub fn pupdr5(&mut self) -> PUPDR5_W<10> {
        PUPDR5_W::new(self)
    }
    #[doc = "Bits 12:13 - PUPDR6"]
    #[inline(always)]
    pub fn pupdr6(&mut self) -> PUPDR6_W<12> {
        PUPDR6_W::new(self)
    }
    #[doc = "Bits 14:15 - PUPDR7"]
    #[inline(always)]
    pub fn pupdr7(&mut self) -> PUPDR7_W<14> {
        PUPDR7_W::new(self)
    }
    #[doc = "Bits 16:17 - PUPDR8"]
    #[inline(always)]
    pub fn pupdr8(&mut self) -> PUPDR8_W<16> {
        PUPDR8_W::new(self)
    }
    #[doc = "Bits 18:19 - PUPDR9"]
    #[inline(always)]
    pub fn pupdr9(&mut self) -> PUPDR9_W<18> {
        PUPDR9_W::new(self)
    }
    #[doc = "Bits 20:21 - PUPDR10"]
    #[inline(always)]
    pub fn pupdr10(&mut self) -> PUPDR10_W<20> {
        PUPDR10_W::new(self)
    }
    #[doc = "Bits 22:23 - PUPDR11"]
    #[inline(always)]
    pub fn pupdr11(&mut self) -> PUPDR11_W<22> {
        PUPDR11_W::new(self)
    }
    #[doc = "Bits 24:25 - PUPDR12"]
    #[inline(always)]
    pub fn pupdr12(&mut self) -> PUPDR12_W<24> {
        PUPDR12_W::new(self)
    }
    #[doc = "Bits 26:27 - PUPDR13"]
    #[inline(always)]
    pub fn pupdr13(&mut self) -> PUPDR13_W<26> {
        PUPDR13_W::new(self)
    }
    #[doc = "Bits 28:29 - PUPDR14"]
    #[inline(always)]
    pub fn pupdr14(&mut self) -> PUPDR14_W<28> {
        PUPDR14_W::new(self)
    }
    #[doc = "Bits 30:31 - PUPDR15"]
    #[inline(always)]
    pub fn pupdr15(&mut self) -> PUPDR15_W<30> {
        PUPDR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_pupdr](index.html) module"]
pub struct GPIOD_PUPDR_SPEC;
impl crate::RegisterSpec for GPIOD_PUPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiod_pupdr::R](R) reader structure"]
impl crate::Readable for GPIOD_PUPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiod_pupdr::W](W) writer structure"]
impl crate::Writable for GPIOD_PUPDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOD_PUPDR to value 0"]
impl crate::Resettable for GPIOD_PUPDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
