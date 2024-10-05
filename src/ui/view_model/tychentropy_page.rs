use dioxus::signals::Signal;
use getset::Getters;

#[derive(Debug, Getters, Clone)]
#[get = "pub with_prefix"]
pub struct TychentropyPageViewModel {
    pub range_local_view: Signal<u64>,
    pub target_entropy_bytes_local_view: Signal<u64>,
    pub range_and_target_entropy_bytes_accepted: Signal<bool>,
}

impl Default for TychentropyPageViewModel {
    fn default() -> Self {
        Self {
            range_local_view: Signal::new(6),
            target_entropy_bytes_local_view: Signal::new(32),
            range_and_target_entropy_bytes_accepted: Signal::new(true),
        }
    }
}
