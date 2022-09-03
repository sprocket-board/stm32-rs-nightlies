#[doc = "Register `CNDTR4` reader"]
pub struct R(crate::R<CNDTR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDTR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDTR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDTR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNDTR4` writer"]
pub struct W(crate::W<CNDTR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDTR4_SPEC>;
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
impl From<crate::W<CNDTR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDTR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDT` reader - number of data to transfer (0 to 218 - 1)"]
pub type NDT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NDT` writer - number of data to transfer (0 to 218 - 1)"]
pub type NDT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CNDTR4_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - number of data to transfer (0 to 218 - 1)"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - number of data to transfer (0 to 218 - 1)"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<0> {
        NDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel x number of data to transfer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr4](index.html) module"]
pub struct CNDTR4_SPEC;
impl crate::RegisterSpec for CNDTR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cndtr4::R](R) reader structure"]
impl crate::Readable for CNDTR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cndtr4::W](W) writer structure"]
impl crate::Writable for CNDTR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNDTR4 to value 0"]
impl crate::Resettable for CNDTR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
