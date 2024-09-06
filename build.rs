use std::{
    env::var,
    fs::write,
    path::{Path, PathBuf},
};

fn code_for_dbus_xml(xml: impl AsRef<Path>) -> String {
    dbus_codegen::generate(
        &std::fs::read_to_string(xml).unwrap(),
        &dbus_codegen::GenOpts {
            methodtype: None,
            connectiontype: dbus_codegen::ConnectionType::Blocking,
            ..Default::default()
        },
    )
    .unwrap()
}

fn main() {
    let systemd_dbus_interface_dir = var("SYSTEMD_DBUS_INTERFACE_DIR").unwrap();
    let systemd_dbus_interface_dir = Path::new(systemd_dbus_interface_dir.as_str());

    let out_path = PathBuf::from(var("OUT_DIR").unwrap());

    let systemd_manager_code =
        code_for_dbus_xml(systemd_dbus_interface_dir.join("org.freedesktop.systemd1.Manager.xml"));
    write(out_path.join("systemd_manager.rs"), systemd_manager_code).unwrap();

    let logind_manager_code =
        code_for_dbus_xml(systemd_dbus_interface_dir.join("org.freedesktop.login1.Manager.xml"));
    write(out_path.join("logind_manager.rs"), logind_manager_code).unwrap();
}
