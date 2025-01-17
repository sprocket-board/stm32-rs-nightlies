#[doc = "Register `UR3` reader"]
pub struct R(crate::R<UR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR3` writer"]
pub struct W(crate::W<UR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR3_SPEC>;
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
impl From<crate::W<UR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_ADD1` reader - Boot Address 1"]
pub type BOOT_ADD1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BOOT_ADD1` writer - Boot Address 1"]
pub type BOOT_ADD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UR3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - Boot Address 1"]
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Boot Address 1"]
    #[inline(always)]
    pub fn boot_add1(&mut self) -> BOOT_ADD1_W<16> {
        BOOT_ADD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG user register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur3](index.html) module"]
pub struct UR3_SPEC;
impl crate::RegisterSpec for UR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur3::R](R) reader structure"]
impl crate::Readable for UR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur3::W](W) writer structure"]
impl crate::Writable for UR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UR3 to value 0"]
impl crate::Resettable for UR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
