#![cfg(not(windows))]

/// Integration tests for exercising the hook and config recompilation
/// behavior of the Supervisor
extern crate biome_core as hcore;

#[macro_use]
extern crate lazy_static;

mod utils;

// The fixture location is derived from the name of this test
// suite. By convention, it is the same as the file name.
lazy_static! {
    static ref FIXTURE_ROOT: utils::FixtureRoot = utils::FixtureRoot::new("integration");
}

#[test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
fn config_only_packages_restart_on_config_application() {
    let bio_root = utils::BioRoot::new("config_only_packages_restart_on_config_application");

    let origin_name = "sup-integration-test";
    let package_name = "config-only";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &bio_root);

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&bio_root, origin_name, package_name, service_group);

    test_sup.start();
    utils::sleep_seconds(3);

    let pid_before_apply = bio_root.pid_of(package_name);
    let config_before_apply = bio_root.compiled_config_contents(&package_name, "config.toml");

    test_sup.apply_config(r#"config_value = "something new and different""#);
    utils::sleep_seconds(2);

    let pid_after_apply = bio_root.pid_of(package_name);
    let config_after_apply = bio_root.compiled_config_contents(&package_name, "config.toml");

    assert_ne!(config_before_apply, config_after_apply);
    assert_ne!(pid_before_apply, pid_after_apply);
}

#[test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
fn hook_only_packages_restart_on_config_application() {
    let bio_root = utils::BioRoot::new("hook_only_packages_restart_on_config_application");

    let origin_name = "sup-integration-test";
    let package_name = "no-configs-only-hooks";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &bio_root);

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&bio_root, origin_name, package_name, service_group);

    test_sup.start();
    utils::sleep_seconds(3);

    let pid_before_apply = bio_root.pid_of(package_name);
    let hook_before_apply = bio_root.compiled_hook_contents(&package_name, "health-check");

    test_sup.apply_config(r#"hook_value = "something new and different""#);
    utils::sleep_seconds(2);

    let pid_after_apply = bio_root.pid_of(package_name);
    let hook_after_apply = bio_root.compiled_hook_contents(&package_name, "health-check");

    assert_ne!(hook_before_apply, hook_after_apply);
    assert_ne!(pid_before_apply, pid_after_apply);
}

#[test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
fn config_files_change_but_hooks_do_not_still_restarts() {
    let bio_root = utils::BioRoot::new("config_files_change_but_hooks_do_not_still_restarts");

    let origin_name = "sup-integration-test";
    let package_name = "config-changes-hooks-do-not";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &bio_root);

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&bio_root, origin_name, package_name, service_group);

    test_sup.start();
    utils::sleep_seconds(3);

    let pid_before_apply = bio_root.pid_of(package_name);
    let hook_before_apply = bio_root.compiled_hook_contents(&package_name, "health-check");
    let config_before_apply = bio_root.compiled_config_contents(&package_name, "config.toml");

    test_sup.apply_config(
                          r#"
config_value = "applied"
hook_value = "default"
"#,
    );
    utils::sleep_seconds(2);

    let pid_after_apply = bio_root.pid_of(package_name);
    let hook_after_apply = bio_root.compiled_hook_contents(&package_name, "health-check");
    let config_after_apply = bio_root.compiled_config_contents(&package_name, "config.toml");

    assert_ne!(config_before_apply, config_after_apply);
    assert_eq!(hook_before_apply, hook_after_apply);
    assert_ne!(pid_before_apply, pid_after_apply);
}

#[test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
fn hooks_change_but_config_files_do_not_still_restarts() {
    let bio_root = utils::BioRoot::new("hooks_change_but_config_files_do_not_still_restarts");

    let origin_name = "sup-integration-test";
    let package_name = "hook-changes-config-does-not";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &bio_root);

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&bio_root, origin_name, package_name, service_group);

    test_sup.start();
    utils::sleep_seconds(3);

    let pid_before_apply = bio_root.pid_of(package_name);
    let hook_before_apply = bio_root.compiled_hook_contents(&package_name, "health-check");
    let config_before_apply = bio_root.compiled_config_contents(&package_name, "config.toml");

    test_sup.apply_config(
                          r#"
config_value = "default"
hook_value = "applied"
"#,
    );
    utils::sleep_seconds(2);

    let pid_after_apply = bio_root.pid_of(package_name);
    let hook_after_apply = bio_root.compiled_hook_contents(&package_name, "health-check");
    let config_after_apply = bio_root.compiled_config_contents(&package_name, "config.toml");

    assert_eq!(config_before_apply, config_after_apply);
    assert_ne!(hook_before_apply, hook_after_apply);
    assert_ne!(pid_before_apply, pid_after_apply);
}

#[test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
fn applying_identical_configuration_results_in_no_changes_and_no_restart() {
    let bio_root = utils::BioRoot::new(
        "applying_identical_configuration_results_in_no_changes_and_no_restart",
    );

    let origin_name = "sup-integration-test";
    let package_name = "no-changes-no-restart";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &bio_root);

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&bio_root, origin_name, package_name, service_group);

    test_sup.start();
    utils::sleep_seconds(3);

    let pid_before_apply = bio_root.pid_of(package_name);
    let hook_before_apply = bio_root.compiled_hook_contents(&package_name, "health-check");
    let config_before_apply = bio_root.compiled_config_contents(&package_name, "config.toml");

    test_sup.apply_config(
                          r#"
config_value = "default"
hook_value = "default"
"#,
    );
    utils::sleep_seconds(2);

    let pid_after_apply = bio_root.pid_of(package_name);
    let hook_after_apply = bio_root.compiled_hook_contents(&package_name, "health-check");
    let config_after_apply = bio_root.compiled_config_contents(&package_name, "config.toml");

    assert_eq!(config_before_apply, config_after_apply);
    assert_eq!(hook_before_apply, hook_after_apply);
    assert_eq!(pid_before_apply, pid_after_apply);
}

#[test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
fn install_hook_success() {
    let bio_root = utils::BioRoot::new("install_hook_success");

    let origin_name = "sup-integration-test";
    let package_name = "install-hook-succeeds";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &bio_root);

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&bio_root, origin_name, package_name, service_group);

    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_before = bio_root.install_status_created(origin_name, package_name);

    assert_eq!(bio_root.install_status_of(origin_name, package_name), 0);
    assert!(bio_root.pid_of(package_name) > 0);

    test_sup.stop();
    utils::sleep_seconds(3);
    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_after = bio_root.install_status_created(origin_name, package_name);

    assert_eq!(status_created_before, status_created_after);
}

#[test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
fn package_with_successful_install_hook_in_dependency_is_loaded() {
    let bio_root =
        utils::BioRoot::new("package_with_successful_install_hook_in_dependency_is_loaded");

    let origin_name = "sup-integration-test";
    let package_name = "install-hook-succeeds-with-dependency";
    let dep = "install-hook-succeeds";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &bio_root);

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&bio_root, origin_name, package_name, service_group);

    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_before = bio_root.install_status_created(origin_name, dep);

    assert_eq!(bio_root.install_status_of(origin_name, dep), 0);
    assert!(bio_root.pid_of(package_name) > 0);

    test_sup.stop();
    utils::sleep_seconds(3);
    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_after = bio_root.install_status_created(origin_name, dep);

    assert_eq!(status_created_before, status_created_after);
}

#[test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
fn install_hook_fails() {
    let bio_root = utils::BioRoot::new("install_hook_fails");

    let origin_name = "sup-integration-test";
    let package_name = "install-hook-fails";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &bio_root);

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&bio_root, origin_name, package_name, service_group);

    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_before = bio_root.install_status_created(origin_name, package_name);
    let result = std::panic::catch_unwind(|| bio_root.pid_of(package_name));

    assert_eq!(bio_root.install_status_of(origin_name, package_name), 1);
    assert!(result.is_err());

    test_sup.stop();
    utils::sleep_seconds(3);
    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_after = bio_root.install_status_created(origin_name, package_name);

    assert_ne!(status_created_before, status_created_after);
}

#[test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
fn package_with_failing_install_hook_in_dependency_is_not_loaded() {
    let bio_root =
        utils::BioRoot::new("package_with_failing_install_hook_in_dependency_is_not_loaded");

    let origin_name = "sup-integration-test";
    let package_name = "install-hook-fails-with-dependency";
    let dep = "install-hook-fails";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &bio_root);

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&bio_root, origin_name, package_name, service_group);

    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_before = bio_root.install_status_created(origin_name, dep);
    let result = std::panic::catch_unwind(|| bio_root.pid_of(package_name));

    assert_eq!(bio_root.install_status_of(origin_name, dep), 1);
    assert!(result.is_err());

    test_sup.stop();
    utils::sleep_seconds(3);
    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_after = bio_root.install_status_created(origin_name, dep);

    assert_ne!(status_created_before, status_created_after);
}
