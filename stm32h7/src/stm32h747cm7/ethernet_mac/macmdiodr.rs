#[doc = "Register `MACMDIODR` reader"]
pub struct R(crate::R<MACMDIODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACMDIODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACMDIODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACMDIODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACMDIODR` writer"]
pub struct W(crate::W<MACMDIODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACMDIODR_SPEC>;
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
impl From<crate::W<MACMDIODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACMDIODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD` reader - MII Data"]
pub type MD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MD` writer - MII Data"]
pub type MD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMDIODR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RA` reader - Register Address"]
pub type RA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RA` writer - Register Address"]
pub type RA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMDIODR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - MII Data"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Register Address"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII Data"]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<0> {
        MD_W::new(self)
    }
    #[doc = "Bits 16:31 - Register Address"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<16> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIO data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macmdiodr](index.html) module"]
pub struct MACMDIODR_SPEC;
impl crate::RegisterSpec for MACMDIODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macmdiodr::R](R) reader structure"]
impl crate::Readable for MACMDIODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macmdiodr::W](W) writer structure"]
impl crate::Writable for MACMDIODR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACMDIODR to value 0"]
impl crate::Resettable for MACMDIODR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
