use std::path::Path;

use lunio_core::{EngineRuntime, fs::id::generate_file_id};

#[test]
fn thumbnail_works() {
    let engine = EngineRuntime::new("./.lunio-cache-test".into());

    engine.full_scan("C:\\Users\\hreet\\Pictures");

    let path = Path::new("C:\\Users\\hreet\\Pictures\\WhatsApp Image 2025-08-09 at 18.40.39 (1).jpeg");

    let id = generate_file_id(path);

    // ✅ Request generation
    engine.request_thumbnail(id);

    // ✅ Give worker time to process
    std::thread::sleep(std::time::Duration::from_millis(300));

    // ✅ Now retrieve
    let results = engine.get_thumbnail(id);

    println!("{:?}", results.is_some());

    assert!(results.is_some());
}
