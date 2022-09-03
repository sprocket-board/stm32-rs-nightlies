#[doc = "Register `PLL2FRACR` reader"]
pub struct R(crate::R<PLL2FRACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL2FRACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL2FRACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL2FRACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL2FRACR` writer"]
pub struct W(crate::W<PLL2FRACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL2FRACR_SPEC>;
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
impl From<crate::W<PLL2FRACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL2FRACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRACN2` reader - fractional part of the multiplication factor for PLL2 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL2VCOSEL = 0 150 to 420Â MHz if PLL2VCOSEL = 1 VCO output frequency = Fref2_ck x (DIVN2 + (FRACN2 / 213)), with DIVN2 between 8 and 420 FRACN2 can be between 0 and 213 - 1 The input frequency Fref2_ck must be between 1 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL2FRACEN to 0. Write the new fractional value into FRACN2. Set the bit PLL2FRACEN to 1."]
pub type FRACN2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRACN2` writer - fractional part of the multiplication factor for PLL2 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL2VCOSEL = 0 150 to 420Â MHz if PLL2VCOSEL = 1 VCO output frequency = Fref2_ck x (DIVN2 + (FRACN2 / 213)), with DIVN2 between 8 and 420 FRACN2 can be between 0 and 213 - 1 The input frequency Fref2_ck must be between 1 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL2FRACEN to 0. Write the new fractional value into FRACN2. Set the bit PLL2FRACEN to 1."]
pub type FRACN2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLL2FRACR_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 3:15 - fractional part of the multiplication factor for PLL2 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL2VCOSEL = 0 150 to 420Â MHz if PLL2VCOSEL = 1 VCO output frequency = Fref2_ck x (DIVN2 + (FRACN2 / 213)), with DIVN2 between 8 and 420 FRACN2 can be between 0 and 213 - 1 The input frequency Fref2_ck must be between 1 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL2FRACEN to 0. Write the new fractional value into FRACN2. Set the bit PLL2FRACEN to 1."]
    #[inline(always)]
    pub fn fracn2(&self) -> FRACN2_R {
        FRACN2_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - fractional part of the multiplication factor for PLL2 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL2VCOSEL = 0 150 to 420Â MHz if PLL2VCOSEL = 1 VCO output frequency = Fref2_ck x (DIVN2 + (FRACN2 / 213)), with DIVN2 between 8 and 420 FRACN2 can be between 0 and 213 - 1 The input frequency Fref2_ck must be between 1 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL2FRACEN to 0. Write the new fractional value into FRACN2. Set the bit PLL2FRACEN to 1."]
    #[inline(always)]
    pub fn fracn2(&mut self) -> FRACN2_W<3> {
        FRACN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll2fracr](index.html) module"]
pub struct PLL2FRACR_SPEC;
impl crate::RegisterSpec for PLL2FRACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll2fracr::R](R) reader structure"]
impl crate::Readable for PLL2FRACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll2fracr::W](W) writer structure"]
impl crate::Writable for PLL2FRACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL2FRACR to value 0"]
impl crate::Resettable for PLL2FRACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}