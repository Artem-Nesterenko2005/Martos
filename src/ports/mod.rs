use crate::timer::TickType;
#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
#[cfg(feature = "network")]
use esp_wifi::esp_now::EspNow;

/// PortTrait contains all the platform specific functions.
pub trait PortTrait {
    /// Function is called when timer is created. Can be used to set configuration.
    fn setup_hardware_timer();
    /// Function used to get amount of ticks from the start of a timer
    fn get_tick_counter() -> TickType;

    /// Function is called when heap is created. Can be used to set configuration.
    fn init_heap();
    #[cfg(feature = "network")]
    /// Function for initializing network settings.
    fn init_network();
    #[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
    #[cfg(feature = "network")]
    /// Function for getting esp-now object for network.
    fn get_esp_now() -> EspNow<'static>;

    // TODO: split to separate trait?
    #[cfg(feature = "preemptive")]
    fn setup_interrupt();
    #[cfg(feature = "preemptive")]
    fn setup_stack(thread: &mut crate::task_manager::preemptive::Thread);
    #[cfg(feature = "preemptive")]
    fn save_ctx(thread_ctx: &mut TrapFrame, isr_ctx: &TrapFrame);
    #[cfg(feature = "preemptive")]
    fn load_ctx(thread_ctx: &TrapFrame, isr_ctx: &mut TrapFrame);
}

/// Port is an alias of PortTrait implementation for a current platform

#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
pub mod xtensa_esp32;
#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
mod arch {
    use super::xtensa_esp32;
    pub type Port = crate::ports::xtensa_esp32::XtensaEsp32;
    #[cfg(feature = "preemptive")]
    pub type TrapFrame = crate::ports::xtensa_esp32::TrapFrame;
}

#[cfg(all(
    not(any(target_arch = "riscv32", target_arch = "xtensa")),
    not(target_arch = "mips64")
))]
pub mod mok;
#[cfg(all(
    not(any(target_arch = "riscv32", target_arch = "xtensa")),
    not(target_arch = "mips64")
))]
mod arch {
    use super::mok;
    pub type Port = mok::Mok;
    #[cfg(feature = "preemptive")]
    pub type TrapFrame = mok::TrapFrame;
}

#[cfg(target_arch = "mips64")]
pub mod mips64;
#[cfg(target_arch = "mips64")]
mod arch {
    use super::mips64;
    pub type Port = mips64::Mips64;
    #[cfg(feature = "preemptive")]
    pub type TrapFrame = ();
}

pub use arch::*;
