#[doc = "Register `HSEM_KEYR` reader"]
pub struct R(crate::R<HSEM_KEYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_KEYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_KEYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_KEYR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSEM_KEYR` writer"]
pub struct W(crate::W<HSEM_KEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSEM_KEYR_SPEC>;
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
impl From<crate::W<HSEM_KEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSEM_KEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - KEY"]
pub type KEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `KEY` writer - KEY"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSEM_KEYR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - KEY"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<16> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_keyr](index.html) module"]
pub struct HSEM_KEYR_SPEC;
impl crate::RegisterSpec for HSEM_KEYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsem_keyr::R](R) reader structure"]
impl crate::Readable for HSEM_KEYR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsem_keyr::W](W) writer structure"]
impl crate::Writable for HSEM_KEYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSEM_KEYR to value 0"]
impl crate::Resettable for HSEM_KEYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}