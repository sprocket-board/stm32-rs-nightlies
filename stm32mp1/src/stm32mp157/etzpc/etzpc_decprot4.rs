#[doc = "Register `ETZPC_DECPROT4` reader"]
pub struct R(crate::R<ETZPC_DECPROT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETZPC_DECPROT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETZPC_DECPROT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETZPC_DECPROT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETZPC_DECPROT4` writer"]
pub struct W(crate::W<ETZPC_DECPROT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETZPC_DECPROT4_SPEC>;
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
impl From<crate::W<ETZPC_DECPROT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETZPC_DECPROT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DECPROT0` reader - DECPROT0"]
pub type DECPROT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT0` writer - DECPROT0"]
pub type DECPROT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT1` reader - DECPROT1"]
pub type DECPROT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT1` writer - DECPROT1"]
pub type DECPROT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT2` reader - DECPROT2"]
pub type DECPROT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT2` writer - DECPROT2"]
pub type DECPROT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT3` reader - DECPROT3"]
pub type DECPROT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT3` writer - DECPROT3"]
pub type DECPROT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT4` reader - DECPROT4"]
pub type DECPROT4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT4` writer - DECPROT4"]
pub type DECPROT4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT5` reader - DECPROT5"]
pub type DECPROT5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT5` writer - DECPROT5"]
pub type DECPROT5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT6` reader - DECPROT6"]
pub type DECPROT6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT6` writer - DECPROT6"]
pub type DECPROT6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT7` reader - DECPROT7"]
pub type DECPROT7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT7` writer - DECPROT7"]
pub type DECPROT7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT8` reader - DECPROT8"]
pub type DECPROT8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT8` writer - DECPROT8"]
pub type DECPROT8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT9` reader - DECPROT9"]
pub type DECPROT9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT9` writer - DECPROT9"]
pub type DECPROT9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT10` reader - DECPROT10"]
pub type DECPROT10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT10` writer - DECPROT10"]
pub type DECPROT10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT11` reader - DECPROT11"]
pub type DECPROT11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT11` writer - DECPROT11"]
pub type DECPROT11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT12` reader - DECPROT12"]
pub type DECPROT12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT12` writer - DECPROT12"]
pub type DECPROT12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT13` reader - DECPROT13"]
pub type DECPROT13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT13` writer - DECPROT13"]
pub type DECPROT13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT14` reader - DECPROT14"]
pub type DECPROT14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT14` writer - DECPROT14"]
pub type DECPROT14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECPROT15` reader - DECPROT15"]
pub type DECPROT15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECPROT15` writer - DECPROT15"]
pub type DECPROT15_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_DECPROT4_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - DECPROT0"]
    #[inline(always)]
    pub fn decprot0(&self) -> DECPROT0_R {
        DECPROT0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DECPROT1"]
    #[inline(always)]
    pub fn decprot1(&self) -> DECPROT1_R {
        DECPROT1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DECPROT2"]
    #[inline(always)]
    pub fn decprot2(&self) -> DECPROT2_R {
        DECPROT2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DECPROT3"]
    #[inline(always)]
    pub fn decprot3(&self) -> DECPROT3_R {
        DECPROT3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DECPROT4"]
    #[inline(always)]
    pub fn decprot4(&self) -> DECPROT4_R {
        DECPROT4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DECPROT5"]
    #[inline(always)]
    pub fn decprot5(&self) -> DECPROT5_R {
        DECPROT5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - DECPROT6"]
    #[inline(always)]
    pub fn decprot6(&self) -> DECPROT6_R {
        DECPROT6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DECPROT7"]
    #[inline(always)]
    pub fn decprot7(&self) -> DECPROT7_R {
        DECPROT7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - DECPROT8"]
    #[inline(always)]
    pub fn decprot8(&self) -> DECPROT8_R {
        DECPROT8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - DECPROT9"]
    #[inline(always)]
    pub fn decprot9(&self) -> DECPROT9_R {
        DECPROT9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - DECPROT10"]
    #[inline(always)]
    pub fn decprot10(&self) -> DECPROT10_R {
        DECPROT10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - DECPROT11"]
    #[inline(always)]
    pub fn decprot11(&self) -> DECPROT11_R {
        DECPROT11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - DECPROT12"]
    #[inline(always)]
    pub fn decprot12(&self) -> DECPROT12_R {
        DECPROT12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - DECPROT13"]
    #[inline(always)]
    pub fn decprot13(&self) -> DECPROT13_R {
        DECPROT13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - DECPROT14"]
    #[inline(always)]
    pub fn decprot14(&self) -> DECPROT14_R {
        DECPROT14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - DECPROT15"]
    #[inline(always)]
    pub fn decprot15(&self) -> DECPROT15_R {
        DECPROT15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DECPROT0"]
    #[inline(always)]
    pub fn decprot0(&mut self) -> DECPROT0_W<0> {
        DECPROT0_W::new(self)
    }
    #[doc = "Bits 2:3 - DECPROT1"]
    #[inline(always)]
    pub fn decprot1(&mut self) -> DECPROT1_W<2> {
        DECPROT1_W::new(self)
    }
    #[doc = "Bits 4:5 - DECPROT2"]
    #[inline(always)]
    pub fn decprot2(&mut self) -> DECPROT2_W<4> {
        DECPROT2_W::new(self)
    }
    #[doc = "Bits 6:7 - DECPROT3"]
    #[inline(always)]
    pub fn decprot3(&mut self) -> DECPROT3_W<6> {
        DECPROT3_W::new(self)
    }
    #[doc = "Bits 8:9 - DECPROT4"]
    #[inline(always)]
    pub fn decprot4(&mut self) -> DECPROT4_W<8> {
        DECPROT4_W::new(self)
    }
    #[doc = "Bits 10:11 - DECPROT5"]
    #[inline(always)]
    pub fn decprot5(&mut self) -> DECPROT5_W<10> {
        DECPROT5_W::new(self)
    }
    #[doc = "Bits 12:13 - DECPROT6"]
    #[inline(always)]
    pub fn decprot6(&mut self) -> DECPROT6_W<12> {
        DECPROT6_W::new(self)
    }
    #[doc = "Bits 14:15 - DECPROT7"]
    #[inline(always)]
    pub fn decprot7(&mut self) -> DECPROT7_W<14> {
        DECPROT7_W::new(self)
    }
    #[doc = "Bits 16:17 - DECPROT8"]
    #[inline(always)]
    pub fn decprot8(&mut self) -> DECPROT8_W<16> {
        DECPROT8_W::new(self)
    }
    #[doc = "Bits 18:19 - DECPROT9"]
    #[inline(always)]
    pub fn decprot9(&mut self) -> DECPROT9_W<18> {
        DECPROT9_W::new(self)
    }
    #[doc = "Bits 20:21 - DECPROT10"]
    #[inline(always)]
    pub fn decprot10(&mut self) -> DECPROT10_W<20> {
        DECPROT10_W::new(self)
    }
    #[doc = "Bits 22:23 - DECPROT11"]
    #[inline(always)]
    pub fn decprot11(&mut self) -> DECPROT11_W<22> {
        DECPROT11_W::new(self)
    }
    #[doc = "Bits 24:25 - DECPROT12"]
    #[inline(always)]
    pub fn decprot12(&mut self) -> DECPROT12_W<24> {
        DECPROT12_W::new(self)
    }
    #[doc = "Bits 26:27 - DECPROT13"]
    #[inline(always)]
    pub fn decprot13(&mut self) -> DECPROT13_W<26> {
        DECPROT13_W::new(self)
    }
    #[doc = "Bits 28:29 - DECPROT14"]
    #[inline(always)]
    pub fn decprot14(&mut self) -> DECPROT14_W<28> {
        DECPROT14_W::new(self)
    }
    #[doc = "Bits 30:31 - DECPROT15"]
    #[inline(always)]
    pub fn decprot15(&mut self) -> DECPROT15_W<30> {
        DECPROT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register reset values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot4](index.html) module"]
pub struct ETZPC_DECPROT4_SPEC;
impl crate::RegisterSpec for ETZPC_DECPROT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etzpc_decprot4::R](R) reader structure"]
impl crate::Readable for ETZPC_DECPROT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot4::W](W) writer structure"]
impl crate::Writable for ETZPC_DECPROT4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETZPC_DECPROT4 to value 0"]
impl crate::Resettable for ETZPC_DECPROT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
