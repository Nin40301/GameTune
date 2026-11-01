#![forbid(unsafe_code)]
#[cfg(windows)]
#[test]
fn readonly_snapshot_has_current_process_and_stable_boot_and_power()
-> Result<(), gametune_metrics::Error> {
    let mut c = gametune_metrics::Collector::default();
    let first = c.sample()?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    let second = c.sample()?;
    assert!(first.total_bytes >= first.available_bytes && first.total_bytes > 0);
    assert!(first.app_private_bytes > 0);
    assert!(first.app_processes >= 1);
    assert!(second.cpu_percent.is_some());
    assert_eq!(first.boot, second.boot);
    assert_eq!(first.power_plan, second.power_plan);
    Ok(())
}
