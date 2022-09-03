#[doc = "Register `WPCR3` reader"]
pub struct R(crate::R<WPCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR3` writer"]
pub struct W(crate::W<WPCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR3_SPEC>;
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
impl From<crate::W<WPCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THSZERO` reader - tHS-ZERO"]
pub type THSZERO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THSZERO` writer - tHS-ZERO"]
pub type THSZERO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `TLPXD` reader - tLPX for Data lanes"]
pub type TLPXD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TLPXD` writer - tLPX for Data lanes"]
pub type TLPXD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `THSEXIT` reader - tHSEXIT"]
pub type THSEXIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THSEXIT` writer - tHSEXIT"]
pub type THSEXIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `TLPXC` reader - tLPXC for Clock lane"]
pub type TLPXC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TLPXC` writer - tLPXC for Clock lane"]
pub type TLPXC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - tHS-ZERO"]
    #[inline(always)]
    pub fn thszero(&self) -> THSZERO_R {
        THSZERO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - tLPX for Data lanes"]
    #[inline(always)]
    pub fn tlpxd(&self) -> TLPXD_R {
        TLPXD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - tHSEXIT"]
    #[inline(always)]
    pub fn thsexit(&self) -> THSEXIT_R {
        THSEXIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - tLPXC for Clock lane"]
    #[inline(always)]
    pub fn tlpxc(&self) -> TLPXC_R {
        TLPXC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - tHS-ZERO"]
    #[inline(always)]
    pub fn thszero(&mut self) -> THSZERO_W<0> {
        THSZERO_W::new(self)
    }
    #[doc = "Bits 8:15 - tLPX for Data lanes"]
    #[inline(always)]
    pub fn tlpxd(&mut self) -> TLPXD_W<8> {
        TLPXD_W::new(self)
    }
    #[doc = "Bits 16:23 - tHSEXIT"]
    #[inline(always)]
    pub fn thsexit(&mut self) -> THSEXIT_W<16> {
        THSEXIT_W::new(self)
    }
    #[doc = "Bits 24:31 - tLPXC for Clock lane"]
    #[inline(always)]
    pub fn tlpxc(&mut self) -> TLPXC_W<24> {
        TLPXC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI_WPCR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr3](index.html) module"]
pub struct WPCR3_SPEC;
impl crate::RegisterSpec for WPCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr3::R](R) reader structure"]
impl crate::Readable for WPCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr3::W](W) writer structure"]
impl crate::Writable for WPCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCR3 to value 0"]
impl crate::Resettable for WPCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
