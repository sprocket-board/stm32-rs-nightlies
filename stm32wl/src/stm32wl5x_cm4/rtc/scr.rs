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
#[doc = "Clear alarm A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALRAF_AW {
    #[doc = "1: Clear interrupt flag by writing 1"]
    Clear = 1,
}
impl From<CALRAF_AW> for bool {
    #[inline(always)]
    fn from(variant: CALRAF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALRAF` writer - Clear alarm A flag"]
pub type CALRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, CALRAF_AW, O>;
impl<'a, const O: u8> CALRAF_W<'a, O> {
    #[doc = "Clear interrupt flag by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CALRAF_AW::Clear)
    }
}
#[doc = "Field `CALRBF` writer - Clear alarm B flag"]
pub use CALRAF_W as CALRBF_W;
#[doc = "Field `CWUTF` writer - Clear wakeup timer flag"]
pub use CALRAF_W as CWUTF_W;
#[doc = "Field `CTSF` writer - Clear timestamp flag"]
pub use CALRAF_W as CTSF_W;
#[doc = "Field `CTSOVF` writer - Clear timestamp overflow flag"]
pub use CALRAF_W as CTSOVF_W;
#[doc = "Field `CITSF` writer - Clear internal timestamp flag"]
pub use CALRAF_W as CITSF_W;
#[doc = "Field `CSSRUF` writer - Clear SSR underflow flag"]
pub use CALRAF_W as CSSRUF_W;
impl W {
    #[doc = "Bit 0 - Clear alarm A flag"]
    #[inline(always)]
    pub fn calraf(&mut self) -> CALRAF_W<0> {
        CALRAF_W::new(self)
    }
    #[doc = "Bit 1 - Clear alarm B flag"]
    #[inline(always)]
    pub fn calrbf(&mut self) -> CALRBF_W<1> {
        CALRBF_W::new(self)
    }
    #[doc = "Bit 2 - Clear wakeup timer flag"]
    #[inline(always)]
    pub fn cwutf(&mut self) -> CWUTF_W<2> {
        CWUTF_W::new(self)
    }
    #[doc = "Bit 3 - Clear timestamp flag"]
    #[inline(always)]
    pub fn ctsf(&mut self) -> CTSF_W<3> {
        CTSF_W::new(self)
    }
    #[doc = "Bit 4 - Clear timestamp overflow flag"]
    #[inline(always)]
    pub fn ctsovf(&mut self) -> CTSOVF_W<4> {
        CTSOVF_W::new(self)
    }
    #[doc = "Bit 5 - Clear internal timestamp flag"]
    #[inline(always)]
    pub fn citsf(&mut self) -> CITSF_W<5> {
        CITSF_W::new(self)
    }
    #[doc = "Bit 6 - Clear SSR underflow flag"]
    #[inline(always)]
    pub fn cssruf(&mut self) -> CSSRUF_W<6> {
        CSSRUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status clear register (interrupts)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
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
