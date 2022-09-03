#[doc = "Register `HDP_GPOVAL` reader"]
pub struct R(crate::R<HDP_GPOVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDP_GPOVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDP_GPOVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDP_GPOVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HDP_GPOVAL` writer"]
pub struct W(crate::W<HDP_GPOVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDP_GPOVAL_SPEC>;
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
impl From<crate::W<HDP_GPOVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDP_GPOVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDPGPOVAL` reader - HDPGPOVAL"]
pub type HDPGPOVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HDPGPOVAL` writer - HDPGPOVAL"]
pub type HDPGPOVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HDP_GPOVAL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - HDPGPOVAL"]
    #[inline(always)]
    pub fn hdpgpoval(&self) -> HDPGPOVAL_R {
        HDPGPOVAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HDPGPOVAL"]
    #[inline(always)]
    pub fn hdpgpoval(&mut self) -> HDPGPOVAL_W<0> {
        HDPGPOVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HDP GPO value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_gpoval](index.html) module"]
pub struct HDP_GPOVAL_SPEC;
impl crate::RegisterSpec for HDP_GPOVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hdp_gpoval::R](R) reader structure"]
impl crate::Readable for HDP_GPOVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hdp_gpoval::W](W) writer structure"]
impl crate::Writable for HDP_GPOVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HDP_GPOVAL to value 0"]
impl crate::Resettable for HDP_GPOVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
