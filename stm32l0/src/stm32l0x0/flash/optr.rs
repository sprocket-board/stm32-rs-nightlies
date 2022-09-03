#[doc = "Register `OPTR` reader"]
pub struct R(crate::R<OPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDPROT` reader - Read protection"]
pub type RDPROT_R = crate::FieldReader<u8, RDPROT_A>;
#[doc = "Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RDPROT_A {
    #[doc = "0: Level 1"]
    Level1 = 0,
    #[doc = "170: Level 0"]
    Level0 = 170,
    #[doc = "204: Level 2"]
    Level2 = 204,
}
impl From<RDPROT_A> for u8 {
    #[inline(always)]
    fn from(variant: RDPROT_A) -> Self {
        variant as _
    }
}
impl RDPROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RDPROT_A> {
        match self.bits {
            0 => Some(RDPROT_A::Level1),
            170 => Some(RDPROT_A::Level0),
            204 => Some(RDPROT_A::Level2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Level1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDPROT_A::Level1
    }
    #[doc = "Checks if the value of the field is `Level0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDPROT_A::Level0
    }
    #[doc = "Checks if the value of the field is `Level2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDPROT_A::Level2
    }
}
#[doc = "Field `WPRMOD` reader - WPRMOD"]
pub type WPRMOD_R = crate::BitReader<WPRMOD_A>;
#[doc = "WPRMOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPRMOD_A {
    #[doc = "0: PCROP disabled. The WRPROT bits are used as a write protection on a sector."]
    Disabled = 0,
    #[doc = "1: PCROP enabled. The WRPROT bits are used as a read protection on a sector."]
    Enabled = 1,
}
impl From<WPRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: WPRMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl WPRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPRMOD_A {
        match self.bits {
            false => WPRMOD_A::Disabled,
            true => WPRMOD_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WPRMOD_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WPRMOD_A::Enabled
    }
}
#[doc = "Field `BOR_LEV` reader - BOR_LEV"]
pub type BOR_LEV_R = crate::FieldReader<u8, BOR_LEV_A>;
#[doc = "BOR_LEV\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOR_LEV_A {
    #[doc = "0: This is the reset threshold level for the 1.45 V - 1.55 V voltage range (power-down only)"]
    BorOff = 0,
    #[doc = "1: Reset threshold level for VBOR0 (around 1.8 V)"]
    BorLevel1 = 1,
    #[doc = "2: Reset threshold level for VBOR1 (around 2.0 V)"]
    BorLevel2 = 2,
    #[doc = "3: Reset threshold level for VBOR2 (around 2.5 V)"]
    BorLevel3 = 3,
    #[doc = "4: Reset threshold level for VBOR3 (around 2.7 V)"]
    BorLevel4 = 4,
    #[doc = "5: Reset threshold level for VBOR4 (around 3.0 V)"]
    BorLevel5 = 5,
}
impl From<BOR_LEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BOR_LEV_A) -> Self {
        variant as _
    }
}
impl BOR_LEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BOR_LEV_A> {
        match self.bits {
            0 => Some(BOR_LEV_A::BorOff),
            1 => Some(BOR_LEV_A::BorLevel1),
            2 => Some(BOR_LEV_A::BorLevel2),
            3 => Some(BOR_LEV_A::BorLevel3),
            4 => Some(BOR_LEV_A::BorLevel4),
            5 => Some(BOR_LEV_A::BorLevel5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BorOff`"]
    #[inline(always)]
    pub fn is_bor_off(&self) -> bool {
        *self == BOR_LEV_A::BorOff
    }
    #[doc = "Checks if the value of the field is `BorLevel1`"]
    #[inline(always)]
    pub fn is_bor_level1(&self) -> bool {
        *self == BOR_LEV_A::BorLevel1
    }
    #[doc = "Checks if the value of the field is `BorLevel2`"]
    #[inline(always)]
    pub fn is_bor_level2(&self) -> bool {
        *self == BOR_LEV_A::BorLevel2
    }
    #[doc = "Checks if the value of the field is `BorLevel3`"]
    #[inline(always)]
    pub fn is_bor_level3(&self) -> bool {
        *self == BOR_LEV_A::BorLevel3
    }
    #[doc = "Checks if the value of the field is `BorLevel4`"]
    #[inline(always)]
    pub fn is_bor_level4(&self) -> bool {
        *self == BOR_LEV_A::BorLevel4
    }
    #[doc = "Checks if the value of the field is `BorLevel5`"]
    #[inline(always)]
    pub fn is_bor_level5(&self) -> bool {
        *self == BOR_LEV_A::BorLevel5
    }
}
#[doc = "Field `WDG_SW` reader - WDG_SW"]
pub type WDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type N_RST_STOP_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type N_RST_STDBY_R = crate::BitReader<bool>;
#[doc = "Field `BFB2` reader - BFB2"]
pub type BFB2_R = crate::BitReader<bool>;
#[doc = "Field `nBOOT1` reader - nBOOT1"]
pub type N_BOOT1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - Read protection"]
    #[inline(always)]
    pub fn rdprot(&self) -> RDPROT_R {
        RDPROT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - WPRMOD"]
    #[inline(always)]
    pub fn wprmod(&self) -> WPRMOD_R {
        WPRMOD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - BOR_LEV"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - BFB2"]
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - nBOOT1"]
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Option byte register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optr](index.html) module"]
pub struct OPTR_SPEC;
impl crate::RegisterSpec for OPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optr::R](R) reader structure"]
impl crate::Readable for OPTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OPTR to value 0x00f8_0000"]
impl crate::Resettable for OPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00f8_0000
    }
}
