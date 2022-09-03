#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection"]
pub type ADDM7_R = crate::BitReader<bool>;
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection"]
pub type ADDM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `STOP` reader - STOP bits"]
pub type STOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP` writer - STOP bits"]
pub type STOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `SWAP` reader - Swap TX/RX pins"]
pub type SWAP_R = crate::BitReader<bool>;
#[doc = "Field `SWAP` writer - Swap TX/RX pins"]
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RXINV` reader - RX pin active level inversion"]
pub type RXINV_R = crate::BitReader<bool>;
#[doc = "Field `RXINV` writer - RX pin active level inversion"]
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TXINV` reader - TX pin active level inversion"]
pub type TXINV_R = crate::BitReader<bool>;
#[doc = "Field `TXINV` writer - TX pin active level inversion"]
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAINV` reader - Binary data inversion"]
pub type TAINV_R = crate::BitReader<bool>;
#[doc = "Field `TAINV` writer - Binary data inversion"]
pub type TAINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `MSBFIRST` reader - Most significant bit first"]
pub type MSBFIRST_R = crate::BitReader<bool>;
#[doc = "Field `MSBFIRST` writer - Most significant bit first"]
pub type MSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `ADD0_3` reader - Address of the USART node"]
pub type ADD0_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD0_3` writer - Address of the USART node"]
pub type ADD0_3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADD4_7` reader - Address of the USART node"]
pub type ADD4_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD4_7` writer - Address of the USART node"]
pub type ADD4_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn tainv(&self) -> TAINV_R {
        TAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Address of the USART node"]
    #[inline(always)]
    pub fn add0_3(&self) -> ADD0_3_R {
        ADD0_3_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Address of the USART node"]
    #[inline(always)]
    pub fn add4_7(&self) -> ADD4_7_R {
        ADD4_7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&mut self) -> ADDM7_W<4> {
        ADDM7_W::new(self)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<12> {
        STOP_W::new(self)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W<15> {
        SWAP_W::new(self)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W<16> {
        RXINV_W::new(self)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W<17> {
        TXINV_W::new(self)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn tainv(&mut self) -> TAINV_W<18> {
        TAINV_W::new(self)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<19> {
        MSBFIRST_W::new(self)
    }
    #[doc = "Bits 24:27 - Address of the USART node"]
    #[inline(always)]
    pub fn add0_3(&mut self) -> ADD0_3_W<24> {
        ADD0_3_W::new(self)
    }
    #[doc = "Bits 28:31 - Address of the USART node"]
    #[inline(always)]
    pub fn add4_7(&mut self) -> ADD4_7_W<28> {
        ADD4_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
