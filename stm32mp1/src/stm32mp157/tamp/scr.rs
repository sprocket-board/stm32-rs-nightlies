#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTAMP1F` writer - CTAMP1F"]
pub type CTAMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CTAMP2F` writer - CTAMP2F"]
pub type CTAMP2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CTAMP3F` writer - CTAMP3F"]
pub type CTAMP3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP1F` writer - CITAMP1F"]
pub type CITAMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP2F` writer - CITAMP2F"]
pub type CITAMP2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP3F` writer - CITAMP3F"]
pub type CITAMP3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP4F` writer - CITAMP4F"]
pub type CITAMP4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP5F` writer - CITAMP5F"]
pub type CITAMP5F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP8F` writer - CITAMP8F"]
pub type CITAMP8F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - CTAMP1F"]
    #[inline(always)]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W<0> {
        CTAMP1F_W::new(self)
    }
    #[doc = "Bit 1 - CTAMP2F"]
    #[inline(always)]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W<1> {
        CTAMP2F_W::new(self)
    }
    #[doc = "Bit 2 - CTAMP3F"]
    #[inline(always)]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W<2> {
        CTAMP3F_W::new(self)
    }
    #[doc = "Bit 16 - CITAMP1F"]
    #[inline(always)]
    pub fn citamp1f(&mut self) -> CITAMP1F_W<16> {
        CITAMP1F_W::new(self)
    }
    #[doc = "Bit 17 - CITAMP2F"]
    #[inline(always)]
    pub fn citamp2f(&mut self) -> CITAMP2F_W<17> {
        CITAMP2F_W::new(self)
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<18> {
        CITAMP3F_W::new(self)
    }
    #[doc = "Bit 19 - CITAMP4F"]
    #[inline(always)]
    pub fn citamp4f(&mut self) -> CITAMP4F_W<19> {
        CITAMP4F_W::new(self)
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<20> {
        CITAMP5F_W::new(self)
    }
    #[doc = "Bit 23 - CITAMP8F"]
    #[inline(always)]
    pub fn citamp8f(&mut self) -> CITAMP8F_W<23> {
        CITAMP8F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP status clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
