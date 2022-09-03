#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)"]
    pub otg_fs_gotgctl: OTG_FS_GOTGCTL,
    #[doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)"]
    pub otg_fs_gotgint: OTG_FS_GOTGINT,
    #[doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
    pub otg_fs_gahbcfg: OTG_FS_GAHBCFG,
    #[doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
    pub otg_fs_gusbcfg: OTG_FS_GUSBCFG,
    #[doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)"]
    pub otg_fs_grstctl: OTG_FS_GRSTCTL,
    #[doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
    pub otg_fs_gintsts: OTG_FS_GINTSTS,
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
    pub otg_fs_gintmsk: OTG_FS_GINTMSK,
    _reserved_7_otg_fs_grxstsr: [u8; 0x04],
    _reserved_8_otg_fs_grxstsp: [u8; 0x04],
    #[doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
    pub otg_fs_grxfsiz: OTG_FS_GRXFSIZ,
    _reserved_10_otg_fs: [u8; 0x04],
    #[doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
    pub otg_fs_hnptxsts: OTG_FS_HNPTXSTS,
    #[doc = "0x30 - OTG I2C access register"]
    pub otg_fs_gi2cctl: OTG_FS_GI2CCTL,
    _reserved13: [u8; 0x04],
    #[doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)"]
    pub otg_fs_gccfg: OTG_FS_GCCFG,
    #[doc = "0x3c - core ID register"]
    pub otg_fs_cid: OTG_FS_CID,
    _reserved15: [u8; 0x14],
    #[doc = "0x54 - OTG core LPM configuration register"]
    pub otg_fs_glpmcfg: OTG_FS_GLPMCFG,
    #[doc = "0x58 - OTG power down register"]
    pub otg_fs_gpwrdn: OTG_FS_GPWRDN,
    _reserved17: [u8; 0x04],
    #[doc = "0x60 - OTG ADP timer, control and status register"]
    pub otg_fs_gadpctl: OTG_FS_GADPCTL,
    _reserved18: [u8; 0x9c],
    #[doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
    pub otg_fs_hptxfsiz: OTG_FS_HPTXFSIZ,
    #[doc = "0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)"]
    pub otg_fs_dieptxf1: OTG_FS_DIEPTXF1,
    #[doc = "0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
    pub otg_fs_dieptxf2: OTG_FS_DIEPTXF2,
    #[doc = "0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
    pub otg_fs_dieptxf3: OTG_FS_DIEPTXF3,
    #[doc = "0x110 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
    pub otg_fs_dieptxf4: OTG_FS_DIEPTXF4,
    #[doc = "0x114 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)"]
    pub otg_fs_dieptxf5: OTG_FS_DIEPTXF5,
}
impl RegisterBlock {
    #[doc = "0x1c - OTG_FS Receive status debug read(Host mode)"]
    #[inline(always)]
    pub fn otg_fs_grxstsr_host(&self) -> &OTG_FS_GRXSTSR_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize) as *const OTG_FS_GRXSTSR_HOST)
        }
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    #[inline(always)]
    pub fn otg_fs_grxstsr_device(&self) -> &OTG_FS_GRXSTSR_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize) as *const OTG_FS_GRXSTSR_DEVICE)
        }
    }
    #[doc = "0x20 - OTG status read and pop register (Host mode)"]
    #[inline(always)]
    pub fn otg_fs_grxstsp_host(&self) -> &OTG_FS_GRXSTSP_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize) as *const OTG_FS_GRXSTSP_HOST)
        }
    }
    #[doc = "0x20 - OTG status read and pop register (Device mode)"]
    #[inline(always)]
    pub fn otg_fs_grxstsp_device(&self) -> &OTG_FS_GRXSTSP_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize) as *const OTG_FS_GRXSTSP_DEVICE)
        }
    }
    #[doc = "0x28 - OTG_FS Host non-periodic transmit FIFO size register"]
    #[inline(always)]
    pub fn otg_fs_hnptxfsiz_host(&self) -> &OTG_FS_HNPTXFSIZ_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize) as *const OTG_FS_HNPTXFSIZ_HOST)
        }
    }
    #[doc = "0x28 - OTG_FS Endpoint 0 Transmit FIFO size"]
    #[inline(always)]
    pub fn otg_fs_dieptxf0_device(&self) -> &OTG_FS_DIEPTXF0_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize) as *const OTG_FS_DIEPTXF0_DEVICE)
        }
    }
}
#[doc = "OTG_FS_GOTGCTL (rw) register accessor: an alias for `Reg<OTG_FS_GOTGCTL_SPEC>`"]
pub type OTG_FS_GOTGCTL = crate::Reg<otg_fs_gotgctl::OTG_FS_GOTGCTL_SPEC>;
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub mod otg_fs_gotgctl;
#[doc = "OTG_FS_GOTGINT (rw) register accessor: an alias for `Reg<OTG_FS_GOTGINT_SPEC>`"]
pub type OTG_FS_GOTGINT = crate::Reg<otg_fs_gotgint::OTG_FS_GOTGINT_SPEC>;
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub mod otg_fs_gotgint;
#[doc = "OTG_FS_GAHBCFG (rw) register accessor: an alias for `Reg<OTG_FS_GAHBCFG_SPEC>`"]
pub type OTG_FS_GAHBCFG = crate::Reg<otg_fs_gahbcfg::OTG_FS_GAHBCFG_SPEC>;
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub mod otg_fs_gahbcfg;
#[doc = "OTG_FS_GUSBCFG (rw) register accessor: an alias for `Reg<OTG_FS_GUSBCFG_SPEC>`"]
pub type OTG_FS_GUSBCFG = crate::Reg<otg_fs_gusbcfg::OTG_FS_GUSBCFG_SPEC>;
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub mod otg_fs_gusbcfg;
#[doc = "OTG_FS_GRSTCTL (rw) register accessor: an alias for `Reg<OTG_FS_GRSTCTL_SPEC>`"]
pub type OTG_FS_GRSTCTL = crate::Reg<otg_fs_grstctl::OTG_FS_GRSTCTL_SPEC>;
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub mod otg_fs_grstctl;
#[doc = "OTG_FS_GINTSTS (rw) register accessor: an alias for `Reg<OTG_FS_GINTSTS_SPEC>`"]
pub type OTG_FS_GINTSTS = crate::Reg<otg_fs_gintsts::OTG_FS_GINTSTS_SPEC>;
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub mod otg_fs_gintsts;
#[doc = "OTG_FS_GINTMSK (rw) register accessor: an alias for `Reg<OTG_FS_GINTMSK_SPEC>`"]
pub type OTG_FS_GINTMSK = crate::Reg<otg_fs_gintmsk::OTG_FS_GINTMSK_SPEC>;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod otg_fs_gintmsk;
#[doc = "OTG_FS_GRXSTSR_Device (r) register accessor: an alias for `Reg<OTG_FS_GRXSTSR_DEVICE_SPEC>`"]
pub type OTG_FS_GRXSTSR_DEVICE = crate::Reg<otg_fs_grxstsr_device::OTG_FS_GRXSTSR_DEVICE_SPEC>;
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub mod otg_fs_grxstsr_device;
#[doc = "OTG_FS_GRXSTSR_Host (r) register accessor: an alias for `Reg<OTG_FS_GRXSTSR_HOST_SPEC>`"]
pub type OTG_FS_GRXSTSR_HOST = crate::Reg<otg_fs_grxstsr_host::OTG_FS_GRXSTSR_HOST_SPEC>;
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub mod otg_fs_grxstsr_host;
#[doc = "OTG_FS_GRXFSIZ (rw) register accessor: an alias for `Reg<OTG_FS_GRXFSIZ_SPEC>`"]
pub type OTG_FS_GRXFSIZ = crate::Reg<otg_fs_grxfsiz::OTG_FS_GRXFSIZ_SPEC>;
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub mod otg_fs_grxfsiz;
#[doc = "OTG_FS_DIEPTXF0_Device (rw) register accessor: an alias for `Reg<OTG_FS_DIEPTXF0_DEVICE_SPEC>`"]
pub type OTG_FS_DIEPTXF0_DEVICE = crate::Reg<otg_fs_dieptxf0_device::OTG_FS_DIEPTXF0_DEVICE_SPEC>;
#[doc = "OTG_FS Endpoint 0 Transmit FIFO size"]
pub mod otg_fs_dieptxf0_device;
#[doc = "OTG_FS_HNPTXFSIZ_Host (rw) register accessor: an alias for `Reg<OTG_FS_HNPTXFSIZ_HOST_SPEC>`"]
pub type OTG_FS_HNPTXFSIZ_HOST = crate::Reg<otg_fs_hnptxfsiz_host::OTG_FS_HNPTXFSIZ_HOST_SPEC>;
#[doc = "OTG_FS Host non-periodic transmit FIFO size register"]
pub mod otg_fs_hnptxfsiz_host;
#[doc = "OTG_FS_HNPTXSTS (r) register accessor: an alias for `Reg<OTG_FS_HNPTXSTS_SPEC>`"]
pub type OTG_FS_HNPTXSTS = crate::Reg<otg_fs_hnptxsts::OTG_FS_HNPTXSTS_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub mod otg_fs_hnptxsts;
#[doc = "OTG_FS_GCCFG (rw) register accessor: an alias for `Reg<OTG_FS_GCCFG_SPEC>`"]
pub type OTG_FS_GCCFG = crate::Reg<otg_fs_gccfg::OTG_FS_GCCFG_SPEC>;
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub mod otg_fs_gccfg;
#[doc = "OTG_FS_CID (rw) register accessor: an alias for `Reg<OTG_FS_CID_SPEC>`"]
pub type OTG_FS_CID = crate::Reg<otg_fs_cid::OTG_FS_CID_SPEC>;
#[doc = "core ID register"]
pub mod otg_fs_cid;
#[doc = "OTG_FS_HPTXFSIZ (rw) register accessor: an alias for `Reg<OTG_FS_HPTXFSIZ_SPEC>`"]
pub type OTG_FS_HPTXFSIZ = crate::Reg<otg_fs_hptxfsiz::OTG_FS_HPTXFSIZ_SPEC>;
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub mod otg_fs_hptxfsiz;
#[doc = "OTG_FS_DIEPTXF1 (rw) register accessor: an alias for `Reg<OTG_FS_DIEPTXF1_SPEC>`"]
pub type OTG_FS_DIEPTXF1 = crate::Reg<otg_fs_dieptxf1::OTG_FS_DIEPTXF1_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)"]
pub mod otg_fs_dieptxf1;
#[doc = "OTG_FS_DIEPTXF2 (rw) register accessor: an alias for `Reg<OTG_FS_DIEPTXF2_SPEC>`"]
pub type OTG_FS_DIEPTXF2 = crate::Reg<otg_fs_dieptxf2::OTG_FS_DIEPTXF2_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub mod otg_fs_dieptxf2;
#[doc = "OTG_FS_DIEPTXF3 (rw) register accessor: an alias for `Reg<OTG_FS_DIEPTXF3_SPEC>`"]
pub type OTG_FS_DIEPTXF3 = crate::Reg<otg_fs_dieptxf3::OTG_FS_DIEPTXF3_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub mod otg_fs_dieptxf3;
#[doc = "OTG_FS_GRXSTSP_Device (r) register accessor: an alias for `Reg<OTG_FS_GRXSTSP_DEVICE_SPEC>`"]
pub type OTG_FS_GRXSTSP_DEVICE = crate::Reg<otg_fs_grxstsp_device::OTG_FS_GRXSTSP_DEVICE_SPEC>;
#[doc = "OTG status read and pop register (Device mode)"]
pub mod otg_fs_grxstsp_device;
#[doc = "OTG_FS_GRXSTSP_Host (r) register accessor: an alias for `Reg<OTG_FS_GRXSTSP_HOST_SPEC>`"]
pub type OTG_FS_GRXSTSP_HOST = crate::Reg<otg_fs_grxstsp_host::OTG_FS_GRXSTSP_HOST_SPEC>;
#[doc = "OTG status read and pop register (Host mode)"]
pub mod otg_fs_grxstsp_host;
#[doc = "OTG_FS_GI2CCTL (rw) register accessor: an alias for `Reg<OTG_FS_GI2CCTL_SPEC>`"]
pub type OTG_FS_GI2CCTL = crate::Reg<otg_fs_gi2cctl::OTG_FS_GI2CCTL_SPEC>;
#[doc = "OTG I2C access register"]
pub mod otg_fs_gi2cctl;
#[doc = "OTG_FS_GPWRDN (rw) register accessor: an alias for `Reg<OTG_FS_GPWRDN_SPEC>`"]
pub type OTG_FS_GPWRDN = crate::Reg<otg_fs_gpwrdn::OTG_FS_GPWRDN_SPEC>;
#[doc = "OTG power down register"]
pub mod otg_fs_gpwrdn;
#[doc = "OTG_FS_GADPCTL (rw) register accessor: an alias for `Reg<OTG_FS_GADPCTL_SPEC>`"]
pub type OTG_FS_GADPCTL = crate::Reg<otg_fs_gadpctl::OTG_FS_GADPCTL_SPEC>;
#[doc = "OTG ADP timer, control and status register"]
pub mod otg_fs_gadpctl;
#[doc = "OTG_FS_DIEPTXF4 (rw) register accessor: an alias for `Reg<OTG_FS_DIEPTXF4_SPEC>`"]
pub type OTG_FS_DIEPTXF4 = crate::Reg<otg_fs_dieptxf4::OTG_FS_DIEPTXF4_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub mod otg_fs_dieptxf4;
#[doc = "OTG_FS_DIEPTXF5 (rw) register accessor: an alias for `Reg<OTG_FS_DIEPTXF5_SPEC>`"]
pub type OTG_FS_DIEPTXF5 = crate::Reg<otg_fs_dieptxf5::OTG_FS_DIEPTXF5_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)"]
pub mod otg_fs_dieptxf5;
#[doc = "OTG_FS_GLPMCFG (rw) register accessor: an alias for `Reg<OTG_FS_GLPMCFG_SPEC>`"]
pub type OTG_FS_GLPMCFG = crate::Reg<otg_fs_glpmcfg::OTG_FS_GLPMCFG_SPEC>;
#[doc = "OTG core LPM configuration register"]
pub mod otg_fs_glpmcfg;
