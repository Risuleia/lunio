use lunio_core::EngineRuntime;

#[test]
fn scan_and_search_works() {
    let engine = EngineRuntime::new("./.lunio-cache-test".into(), None, None);

    engine.full_scan(".");
    let results = engine.search("cargo", 50);

    println!("{:?}", results);

    assert!(results.len() > 0);
}