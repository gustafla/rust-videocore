fn main() {
    pkg_config::Config::new().probe("bcm_host").unwrap();
}
