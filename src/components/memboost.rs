use async_sleep_aki::async_sleep;
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Dp {
    pub total_memory_mb: Signal<u64>,
    pub free_memory_mb: Signal<u64>,
}
impl Dp {
    pub fn new() -> Self {
        Self {
            total_memory_mb: use_signal(|| 0u64),
            free_memory_mb: use_signal(|| 0u64),
        }
    }
    pub fn refresh(&mut self) {
        const MB: u64 = 1024 * 1024;
        use sysinfo::{MemoryRefreshKind, System};
        let mut system = System::new();
        system.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());
        self.total_memory_mb.set(system.total_memory() / MB);
        self.free_memory_mb.set(system.available_memory() / MB);
    }
}

#[component]
pub fn MemBoost() -> Element {
    let mut dp = Dp::new();

    use_future(move || async move {
        loop {
            async_sleep(1000).await;
            dp.refresh();
        }
    });

    rsx! {
        div { id: "memboost",
            div { id: "hello", "Hello" }
            div { id: "totalmemory", "{dp.free_memory_mb}/{dp.total_memory_mb} MiB" }
            button {
                id: "boost",
                onclick: move |_evt| async move {
                    process_boost(dp).await;
                    dp.refresh();
                },
                "BOOST"
            }
        }
    }
}

#[inline(never)]
async fn process_boost(mut dp: Dp) {
    const MB: u64 = 1024 * 1024;
    let max_mb = (dp.total_memory_mb * 3 / 4).max(dp.total_memory_mb - 800);
    let mut v: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    while i < max_mb {
        let vv = vec![123u8; 10 * MB as usize];
        dp.refresh();
        async_sleep(10).await;
        v.push(vv);
        i += 10;
    }
    dp.refresh();
    async_sleep(10).await;
}
