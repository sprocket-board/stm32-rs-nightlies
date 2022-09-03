#[doc = "Register `S3NDTR` reader"]
pub struct R(crate::R<S3NDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S3NDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S3NDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S3NDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S3NDTR` writer"]
pub struct W(crate::W<S3NDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S3NDTR_SPEC>;
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
impl From<crate::W<S3NDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S3NDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDT` reader - Number of data items to transfer"]
pub type NDT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NDT` writer - Number of data items to transfer"]
pub type NDT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, S3NDTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
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
#[doc = "stream x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s3ndtr](index.html) module"]
pub struct S3NDTR_SPEC;
impl crate::RegisterSpec for S3NDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s3ndtr::R](R) reader structure"]
impl crate::Readable for S3NDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s3ndtr::W](W) writer structure"]
impl crate::Writable for S3NDTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S3NDTR to value 0"]
impl crate::Resettable for S3NDTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
