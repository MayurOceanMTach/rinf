#![allow(clippy::crate_in_macro_def)]

#[macro_export]
/// Writes the interface code
/// needed to communicate with Dart.
/// This should be used once, and only once,
/// at the root of the `hub` crate.
macro_rules! write_interface {
    () => {
        use crate::tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
        use rinf::externs::os_thread_local::ThreadLocal;
        use rinf::SharedCell;
        use std::cell::RefCell;
        use std::sync::mpsc::Receiver;
        use std::sync::{Arc, Mutex, OnceLock};

        struct ShutdownNotifier {
            shutdown_sender: UnboundedSender<()>,
            done_receiver: Receiver<()>,
        }

        #[cfg(not(target_family = "wasm"))]
        impl Drop for ShutdownNotifier {
            fn drop(&mut self) {
                let _ = self.shutdown_sender.send(());
                let _ = self.done_receiver.recv();
                interface_os::TOKIO_RUNTIME.with(|cell| {
                    cell.take();
                });
            }
        }

        // We use `os_thread_local` so that when the program fails
        // and the main thread exits unexpectedly,
        // the sender can be dropped and send the error signal accordingly.
        type NotifierType = OnceLock<ThreadLocal<RefCell<Option<ShutdownNotifier>>>>;
        static SHUTDOWN_NOTIFIER: NotifierType = OnceLock::new();

        static SHUTDOWN_RECEIVER: SharedCell<UnboundedReceiver<()>> = OnceLock::new();

        async fn dart_shutdown() {
            let mut receiver = SHUTDOWN_RECEIVER
                .get_or_init(|| Mutex::new(RefCell::new(None)))
                .lock()
                .unwrap()
                .take()
                .unwrap();
            receiver.recv().await;
        }

        #[cfg(not(target_family = "wasm"))]
        mod interface_os {
            use crate::tokio::runtime::Builder;
            use crate::tokio::runtime::Runtime;
            use crate::tokio::sync::mpsc::unbounded_channel;
            use rinf::externs::os_thread_local::ThreadLocal;
            use std::cell::RefCell;
            use std::panic::catch_unwind;
            use std::sync::mpsc::channel as std_channel;
            use std::sync::{Arc, Mutex};
            thread_local! {
                pub static TOKIO_RUNTIME: RefCell<Option<Runtime>> = RefCell::new(None);
            }

            #[no_mangle]
            pub extern "C" fn start_rust_logic_extern() {
                let _ = catch_unwind(|| {
                    // Enable backtrace output for panics.
                    #[cfg(debug_assertions)]
                    {
                        use rinf::debug_print;
                        use rinf::externs::backtrace::Backtrace;
                        std::panic::set_hook(Box::new(|panic_info| {
                            let backtrace = Backtrace::new();
                            debug_print!("A panic occurred in Rust.\n{panic_info}\n{backtrace:?}");
                        }));
                    }

                    // Prepare to notify Dart shutdown.
                    let (shutdown_sender, shutdown_receiver) = unbounded_channel();
                    let (done_sender, done_receiver) = std_channel::<()>();
                    let os_cell = crate::SHUTDOWN_NOTIFIER
                        .get_or_init(|| ThreadLocal::new(|| RefCell::new(None)));
                    os_cell.with(move |cell| {
                        cell.replace(Some(crate::ShutdownNotifier {
                            shutdown_sender,
                            done_receiver,
                        }));
                    });
                    let mut receiver = crate::SHUTDOWN_RECEIVER
                        .get_or_init(|| Mutex::new(RefCell::new(None)))
                        .lock()
                        .unwrap()
                        .replace(Some(shutdown_receiver));

                    // Run the main function.
                    let tokio_runtime = Builder::new_multi_thread().enable_all().build().unwrap();
                    tokio_runtime.spawn(async move {
                        crate::main().await;
                        let _ = done_sender.send(());
                    });
                    TOKIO_RUNTIME.with(move |cell| {
                        // If there was already a tokio runtime previously,
                        // most likely due to Dart's hot restart,
                        // its tasks as well as itself will be terminated,
                        // being replaced with the new one.
                        cell.replace(Some(tokio_runtime));
                    });
                });
            }

            #[no_mangle]
            pub extern "C" fn send_dart_signal_extern(
                message_id: i64,
                message_pointer: *const u8,
                message_size: usize,
                binary_pointer: *const u8,
                binary_size: usize,
            ) {
                let message_bytes = unsafe {
                    std::slice::from_raw_parts(message_pointer as *mut u8, message_size).to_vec()
                };
                let binary = unsafe {
                    std::slice::from_raw_parts(binary_pointer as *mut u8, binary_size).to_vec()
                };
                let _ = catch_unwind(|| {
                    crate::messages::generated::handle_dart_signal(
                        message_id as i32,
                        message_bytes,
                        binary,
                    );
                });
            }
        }

        #[cfg(target_family = "wasm")]
        mod interface_web {
            use crate::tokio;
            use std::panic::catch_unwind;
            use wasm_bindgen::prelude::wasm_bindgen;

            #[wasm_bindgen]
            pub fn start_rust_logic_extern() {
                let _ = catch_unwind(|| {
                    // Add kind description for panics.
                    #[cfg(debug_assertions)]
                    {
                        use rinf::debug_print;
                        std::panic::set_hook(Box::new(|panic_info| {
                            debug_print!("A panic occurred in Rust.\n{panic_info}");
                        }));
                    }

                    // Run the main function.
                    tokio::spawn(crate::main());
                });
            }

            #[wasm_bindgen]
            pub fn send_dart_signal_extern(message_id: i32, message_bytes: &[u8], binary: &[u8]) {
                let message_bytes = message_bytes.to_vec();
                let binary = binary.to_vec();
                let _ = catch_unwind(|| {
                    crate::messages::generated::handle_dart_signal(
                        message_id,
                        message_bytes,
                        binary,
                    );
                });
            }
        }
    };
}

/// Delegates the printing operation to Flutter,
/// which excels at handling various platforms
/// including web and mobile emulators.
/// When debugging, using this macro is recommended over `println!()`,
/// as it seamlessly adapts to different environments.
/// Note that this macro does nothing in release mode.
#[macro_export]
macro_rules! debug_print {
    ( $( $t:tt )* ) => {
        let rust_report = format!( $( $t )* );
        #[cfg(debug_assertions)]
        rinf::send_rust_signal(
            -1, // This is a special message ID for Rust reports
            Vec::new(),
            rust_report.into_bytes(),
        );
        #[cfg(not(debug_assertions))]
        let _ = rust_report;
    }
}
