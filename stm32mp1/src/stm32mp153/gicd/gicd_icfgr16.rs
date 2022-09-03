#[doc = "Register `GICD_ICFGR16` reader"]
pub struct R(crate::R<GICD_ICFGR16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICFGR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICFGR16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICFGR16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICFGR16` writer"]
pub struct W(crate::W<GICD_ICFGR16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICFGR16_SPEC>;
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
impl From<crate::W<GICD_ICFGR16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICFGR16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_CONFIG0` reader - INT_CONFIG0"]
pub type INT_CONFIG0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG0` writer - INT_CONFIG0"]
pub type INT_CONFIG0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG1` reader - INT_CONFIG1"]
pub type INT_CONFIG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG1` writer - INT_CONFIG1"]
pub type INT_CONFIG1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG2` reader - INT_CONFIG2"]
pub type INT_CONFIG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG2` writer - INT_CONFIG2"]
pub type INT_CONFIG2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG3` reader - INT_CONFIG3"]
pub type INT_CONFIG3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG3` writer - INT_CONFIG3"]
pub type INT_CONFIG3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG4` reader - INT_CONFIG4"]
pub type INT_CONFIG4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG4` writer - INT_CONFIG4"]
pub type INT_CONFIG4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG5` reader - INT_CONFIG5"]
pub type INT_CONFIG5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG5` writer - INT_CONFIG5"]
pub type INT_CONFIG5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG6` reader - INT_CONFIG6"]
pub type INT_CONFIG6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG6` writer - INT_CONFIG6"]
pub type INT_CONFIG6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG7` reader - INT_CONFIG7"]
pub type INT_CONFIG7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG7` writer - INT_CONFIG7"]
pub type INT_CONFIG7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG8` reader - INT_CONFIG8"]
pub type INT_CONFIG8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG8` writer - INT_CONFIG8"]
pub type INT_CONFIG8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG9` reader - INT_CONFIG9"]
pub type INT_CONFIG9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG9` writer - INT_CONFIG9"]
pub type INT_CONFIG9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG10` reader - INT_CONFIG10"]
pub type INT_CONFIG10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG10` writer - INT_CONFIG10"]
pub type INT_CONFIG10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG11` reader - INT_CONFIG11"]
pub type INT_CONFIG11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG11` writer - INT_CONFIG11"]
pub type INT_CONFIG11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG12` reader - INT_CONFIG12"]
pub type INT_CONFIG12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG12` writer - INT_CONFIG12"]
pub type INT_CONFIG12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG13` reader - INT_CONFIG13"]
pub type INT_CONFIG13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG13` writer - INT_CONFIG13"]
pub type INT_CONFIG13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG14` reader - INT_CONFIG14"]
pub type INT_CONFIG14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG14` writer - INT_CONFIG14"]
pub type INT_CONFIG14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT_CONFIG15` reader - INT_CONFIG15"]
pub type INT_CONFIG15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CONFIG15` writer - INT_CONFIG15"]
pub type INT_CONFIG15_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICFGR16_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - INT_CONFIG0"]
    #[inline(always)]
    pub fn int_config0(&self) -> INT_CONFIG0_R {
        INT_CONFIG0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - INT_CONFIG1"]
    #[inline(always)]
    pub fn int_config1(&self) -> INT_CONFIG1_R {
        INT_CONFIG1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - INT_CONFIG2"]
    #[inline(always)]
    pub fn int_config2(&self) -> INT_CONFIG2_R {
        INT_CONFIG2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - INT_CONFIG3"]
    #[inline(always)]
    pub fn int_config3(&self) -> INT_CONFIG3_R {
        INT_CONFIG3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - INT_CONFIG4"]
    #[inline(always)]
    pub fn int_config4(&self) -> INT_CONFIG4_R {
        INT_CONFIG4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - INT_CONFIG5"]
    #[inline(always)]
    pub fn int_config5(&self) -> INT_CONFIG5_R {
        INT_CONFIG5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - INT_CONFIG6"]
    #[inline(always)]
    pub fn int_config6(&self) -> INT_CONFIG6_R {
        INT_CONFIG6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - INT_CONFIG7"]
    #[inline(always)]
    pub fn int_config7(&self) -> INT_CONFIG7_R {
        INT_CONFIG7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - INT_CONFIG8"]
    #[inline(always)]
    pub fn int_config8(&self) -> INT_CONFIG8_R {
        INT_CONFIG8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - INT_CONFIG9"]
    #[inline(always)]
    pub fn int_config9(&self) -> INT_CONFIG9_R {
        INT_CONFIG9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - INT_CONFIG10"]
    #[inline(always)]
    pub fn int_config10(&self) -> INT_CONFIG10_R {
        INT_CONFIG10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - INT_CONFIG11"]
    #[inline(always)]
    pub fn int_config11(&self) -> INT_CONFIG11_R {
        INT_CONFIG11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - INT_CONFIG12"]
    #[inline(always)]
    pub fn int_config12(&self) -> INT_CONFIG12_R {
        INT_CONFIG12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - INT_CONFIG13"]
    #[inline(always)]
    pub fn int_config13(&self) -> INT_CONFIG13_R {
        INT_CONFIG13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - INT_CONFIG14"]
    #[inline(always)]
    pub fn int_config14(&self) -> INT_CONFIG14_R {
        INT_CONFIG14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - INT_CONFIG15"]
    #[inline(always)]
    pub fn int_config15(&self) -> INT_CONFIG15_R {
        INT_CONFIG15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - INT_CONFIG0"]
    #[inline(always)]
    pub fn int_config0(&mut self) -> INT_CONFIG0_W<0> {
        INT_CONFIG0_W::new(self)
    }
    #[doc = "Bits 2:3 - INT_CONFIG1"]
    #[inline(always)]
    pub fn int_config1(&mut self) -> INT_CONFIG1_W<2> {
        INT_CONFIG1_W::new(self)
    }
    #[doc = "Bits 4:5 - INT_CONFIG2"]
    #[inline(always)]
    pub fn int_config2(&mut self) -> INT_CONFIG2_W<4> {
        INT_CONFIG2_W::new(self)
    }
    #[doc = "Bits 6:7 - INT_CONFIG3"]
    #[inline(always)]
    pub fn int_config3(&mut self) -> INT_CONFIG3_W<6> {
        INT_CONFIG3_W::new(self)
    }
    #[doc = "Bits 8:9 - INT_CONFIG4"]
    #[inline(always)]
    pub fn int_config4(&mut self) -> INT_CONFIG4_W<8> {
        INT_CONFIG4_W::new(self)
    }
    #[doc = "Bits 10:11 - INT_CONFIG5"]
    #[inline(always)]
    pub fn int_config5(&mut self) -> INT_CONFIG5_W<10> {
        INT_CONFIG5_W::new(self)
    }
    #[doc = "Bits 12:13 - INT_CONFIG6"]
    #[inline(always)]
    pub fn int_config6(&mut self) -> INT_CONFIG6_W<12> {
        INT_CONFIG6_W::new(self)
    }
    #[doc = "Bits 14:15 - INT_CONFIG7"]
    #[inline(always)]
    pub fn int_config7(&mut self) -> INT_CONFIG7_W<14> {
        INT_CONFIG7_W::new(self)
    }
    #[doc = "Bits 16:17 - INT_CONFIG8"]
    #[inline(always)]
    pub fn int_config8(&mut self) -> INT_CONFIG8_W<16> {
        INT_CONFIG8_W::new(self)
    }
    #[doc = "Bits 18:19 - INT_CONFIG9"]
    #[inline(always)]
    pub fn int_config9(&mut self) -> INT_CONFIG9_W<18> {
        INT_CONFIG9_W::new(self)
    }
    #[doc = "Bits 20:21 - INT_CONFIG10"]
    #[inline(always)]
    pub fn int_config10(&mut self) -> INT_CONFIG10_W<20> {
        INT_CONFIG10_W::new(self)
    }
    #[doc = "Bits 22:23 - INT_CONFIG11"]
    #[inline(always)]
    pub fn int_config11(&mut self) -> INT_CONFIG11_W<22> {
        INT_CONFIG11_W::new(self)
    }
    #[doc = "Bits 24:25 - INT_CONFIG12"]
    #[inline(always)]
    pub fn int_config12(&mut self) -> INT_CONFIG12_W<24> {
        INT_CONFIG12_W::new(self)
    }
    #[doc = "Bits 26:27 - INT_CONFIG13"]
    #[inline(always)]
    pub fn int_config13(&mut self) -> INT_CONFIG13_W<26> {
        INT_CONFIG13_W::new(self)
    }
    #[doc = "Bits 28:29 - INT_CONFIG14"]
    #[inline(always)]
    pub fn int_config14(&mut self) -> INT_CONFIG14_W<28> {
        INT_CONFIG14_W::new(self)
    }
    #[doc = "Bits 30:31 - INT_CONFIG15"]
    #[inline(always)]
    pub fn int_config15(&mut self) -> INT_CONFIG15_W<30> {
        INT_CONFIG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICD interrupt configuration register 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr16](index.html) module"]
pub struct GICD_ICFGR16_SPEC;
impl crate::RegisterSpec for GICD_ICFGR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icfgr16::R](R) reader structure"]
impl crate::Readable for GICD_ICFGR16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr16::W](W) writer structure"]
impl crate::Writable for GICD_ICFGR16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_ICFGR16 to value 0x5555_5555"]
impl crate::Resettable for GICD_ICFGR16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5555_5555
    }
}
