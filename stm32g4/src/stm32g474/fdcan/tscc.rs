#[doc = "Register `TSCC` reader"]
pub struct R(crate::R<TSCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSCC` writer"]
pub struct W(crate::W<TSCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSCC_SPEC>;
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
impl From<crate::W<TSCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSS` reader - TSS"]
pub type TSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSS` writer - TSS"]
pub type TSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSCC_SPEC, u8, u8, 2, O>;
#[doc = "Field `TCP` reader - TCP"]
pub type TCP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCP` writer - TCP"]
pub type TCP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSCC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - TSS"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - TCP"]
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TSS"]
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W<0> {
        TSS_W::new(self)
    }
    #[doc = "Bits 16:19 - TCP"]
    #[inline(always)]
    pub fn tcp(&mut self) -> TCP_W<16> {
        TCP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Timestamp Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscc](index.html) module"]
pub struct TSCC_SPEC;
impl crate::RegisterSpec for TSCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tscc::R](R) reader structure"]
impl crate::Readable for TSCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tscc::W](W) writer structure"]
impl crate::Writable for TSCC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSCC to value 0"]
impl crate::Resettable for TSCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
