#[doc = "Register `PMEM4` reader"]
pub struct R(crate::R<PMEM4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMEM4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMEM4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMEM4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMEM4` writer"]
pub struct W(crate::W<PMEM4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMEM4_SPEC>;
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
impl From<crate::W<PMEM4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMEM4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMSETx` reader - MEMSETx"]
pub type MEMSETX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMSETx` writer - MEMSETx"]
pub type MEMSETX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM4_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEMWAITx` reader - MEMWAITx"]
pub type MEMWAITX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMWAITx` writer - MEMWAITx"]
pub type MEMWAITX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM4_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEMHOLDx` reader - MEMHOLDx"]
pub type MEMHOLDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMHOLDx` writer - MEMHOLDx"]
pub type MEMHOLDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM4_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEMHIZx` reader - MEMHIZx"]
pub type MEMHIZX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMHIZx` writer - MEMHIZx"]
pub type MEMHIZX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memsetx(&self) -> MEMSETX_R {
        MEMSETX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwaitx(&self) -> MEMWAITX_R {
        MEMWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memholdx(&self) -> MEMHOLDX_R {
        MEMHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhizx(&self) -> MEMHIZX_R {
        MEMHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memsetx(&mut self) -> MEMSETX_W<0> {
        MEMSETX_W::new(self)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwaitx(&mut self) -> MEMWAITX_W<8> {
        MEMWAITX_W::new(self)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memholdx(&mut self) -> MEMHOLDX_W<16> {
        MEMHOLDX_W::new(self)
    }
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhizx(&mut self) -> MEMHIZX_W<24> {
        MEMHIZX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common memory space timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem4](index.html) module"]
pub struct PMEM4_SPEC;
impl crate::RegisterSpec for PMEM4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmem4::R](R) reader structure"]
impl crate::Readable for PMEM4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmem4::W](W) writer structure"]
impl crate::Writable for PMEM4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMEM4 to value 0xfcfc_fcfc"]
impl crate::Resettable for PMEM4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}
