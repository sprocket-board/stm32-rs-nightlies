#[doc = "Register `ARGR` reader"]
pub struct R(crate::R<ARGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARGR` writer"]
pub struct W(crate::W<ARGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARGR_SPEC>;
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
impl From<crate::W<ARGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDARG` reader - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
pub type CMDARG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMDARG` writer - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
pub type CMDARG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARGR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
    #[inline(always)]
    pub fn cmdarg(&self) -> CMDARG_R {
        CMDARG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
    #[inline(always)]
    pub fn cmdarg(&mut self) -> CMDARG_W<0> {
        CMDARG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [argr](index.html) module"]
pub struct ARGR_SPEC;
impl crate::RegisterSpec for ARGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [argr::R](R) reader structure"]
impl crate::Readable for ARGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [argr::W](W) writer structure"]
impl crate::Writable for ARGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARGR to value 0"]
impl crate::Resettable for ARGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
