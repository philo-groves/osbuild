use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RunnerArgs {
    pub executable: PathBuf
}

#[derive(Debug, Clone)]
pub struct ConfigurationArgs {
    pub name: String,
    pub bootloader: BootloaderOption,
    pub vm: VirtualMachineOption,
    pub build: BuildArgs
}

#[derive(Debug, Clone)]
pub struct BuildArgs {
    pub prepare: PhaseArgs,
    pub compile: PhaseArgs,
    pub pack: PhaseArgs,
    pub boot: PhaseArgs
}

#[derive(Debug, Clone)]
pub struct PhaseArgs {
    pub prevent_defaults: bool,
    pub commands: Vec<PhaseCommandArgs>
}

#[derive(Debug, Clone)]
pub struct PhaseCommandArgs {
    pub command: String,
    pub when: WhenOption
}

#[derive(Debug, Clone)]
pub enum BootloaderOption {
    Limine,
    Grub,
    None
}

impl BootloaderOption {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "limine" => Some(Self::Limine),
            "grub" => Some(Self::Grub),
            "none" => Some(Self::None),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub enum VirtualMachineOption {
    Qemu,
    Kvm,
    VirtualBox,
    None
}

impl VirtualMachineOption {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "qemu" => Some(Self::Qemu),
            "kvm" => Some(Self::Kvm),
            "virtualbox" => Some(Self::VirtualBox),
            "none" => Some(Self::None),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub enum WhenOption {
    Before,
    After
}

impl WhenOption {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "before" => Some(Self::Before),
            "after" => Some(Self::After),
            _ => None
        }
    }
}