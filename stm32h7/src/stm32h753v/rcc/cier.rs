#[doc = "Register `CIER` reader"]
pub struct R(crate::R<CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIER` writer"]
pub struct W(crate::W<CIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIER_SPEC>;
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
impl From<crate::W<CIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI ready Interrupt Enable"]
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE_A>;
#[doc = "LSI ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<LSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYIE_A {
        match self.bits {
            false => LSIRDYIE_A::Disabled,
            true => LSIRDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIE_A::Enabled
    }
}
#[doc = "Field `LSIRDYIE` writer - LSI ready Interrupt Enable"]
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, LSIRDYIE_A, O>;
impl<'a, const O: u8> LSIRDYIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::Enabled)
    }
}
#[doc = "Field `LSERDYIE` reader - LSE ready Interrupt Enable"]
pub use LSIRDYIE_R as LSERDYIE_R;
#[doc = "Field `HSIRDYIE` reader - HSI ready Interrupt Enable"]
pub use LSIRDYIE_R as HSIRDYIE_R;
#[doc = "Field `HSERDYIE` reader - HSE ready Interrupt Enable"]
pub use LSIRDYIE_R as HSERDYIE_R;
#[doc = "Field `CSIRDYIE` reader - CSI ready Interrupt Enable"]
pub use LSIRDYIE_R as CSIRDYIE_R;
#[doc = "Field `HSI48RDYIE` reader - RC48 ready Interrupt Enable"]
pub use LSIRDYIE_R as HSI48RDYIE_R;
#[doc = "Field `PLL1RDYIE` reader - PLL1 ready Interrupt Enable"]
pub use LSIRDYIE_R as PLL1RDYIE_R;
#[doc = "Field `PLL2RDYIE` reader - PLL2 ready Interrupt Enable"]
pub use LSIRDYIE_R as PLL2RDYIE_R;
#[doc = "Field `PLL3RDYIE` reader - PLL3 ready Interrupt Enable"]
pub use LSIRDYIE_R as PLL3RDYIE_R;
#[doc = "Field `LSECSSIE` reader - LSE clock security system Interrupt Enable"]
pub use LSIRDYIE_R as LSECSSIE_R;
#[doc = "Field `LSERDYIE` writer - LSE ready Interrupt Enable"]
pub use LSIRDYIE_W as LSERDYIE_W;
#[doc = "Field `HSIRDYIE` writer - HSI ready Interrupt Enable"]
pub use LSIRDYIE_W as HSIRDYIE_W;
#[doc = "Field `HSERDYIE` writer - HSE ready Interrupt Enable"]
pub use LSIRDYIE_W as HSERDYIE_W;
#[doc = "Field `CSIRDYIE` writer - CSI ready Interrupt Enable"]
pub use LSIRDYIE_W as CSIRDYIE_W;
#[doc = "Field `HSI48RDYIE` writer - RC48 ready Interrupt Enable"]
pub use LSIRDYIE_W as HSI48RDYIE_W;
#[doc = "Field `PLL1RDYIE` writer - PLL1 ready Interrupt Enable"]
pub use LSIRDYIE_W as PLL1RDYIE_W;
#[doc = "Field `PLL2RDYIE` writer - PLL2 ready Interrupt Enable"]
pub use LSIRDYIE_W as PLL2RDYIE_W;
#[doc = "Field `PLL3RDYIE` writer - PLL3 ready Interrupt Enable"]
pub use LSIRDYIE_W as PLL3RDYIE_W;
#[doc = "Field `LSECSSIE` writer - LSE clock security system Interrupt Enable"]
pub use LSIRDYIE_W as LSECSSIE_W;
impl R {
    #[doc = "Bit 0 - LSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn csirdyie(&self) -> CSIRDYIE_R {
        CSIRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll1rdyie(&self) -> PLL1RDYIE_R {
        PLL1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll2rdyie(&self) -> PLL2RDYIE_R {
        PLL2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll3rdyie(&self) -> PLL3RDYIE_R {
        PLL3RDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<0> {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Enable"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<1> {
        LSERDYIE_W::new(self)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<2> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Enable"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<3> {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn csirdyie(&mut self) -> CSIRDYIE_W<4> {
        CSIRDYIE_W::new(self)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<5> {
        HSI48RDYIE_W::new(self)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll1rdyie(&mut self) -> PLL1RDYIE_W<6> {
        PLL1RDYIE_W::new(self)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll2rdyie(&mut self) -> PLL2RDYIE_W<7> {
        PLL2RDYIE_W::new(self)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll3rdyie(&mut self) -> PLL3RDYIE_W<8> {
        PLL3RDYIE_W::new(self)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Enable"]
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<9> {
        LSECSSIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Clock Source Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cier](index.html) module"]
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cier::R](R) reader structure"]
impl crate::Readable for CIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cier::W](W) writer structure"]
impl crate::Writable for CIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
