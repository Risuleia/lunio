use std::path::Path;

use lunio_core::EngineRuntime;

#[test]
fn listing_works() {
    let engine = EngineRuntime::new("./.lunio-cache-test".into(), None, None);

    let results = engine.list_dir(Path::new(&"C:\\Users\\hreet\\Desktop".to_string()));
    
    println!("{:?}", results);
    assert!(results.len() > 0);
}