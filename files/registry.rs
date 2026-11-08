//! Windows registry read helpers for Nimbus Capture.

pub const APP_KEY: &str = "Software\\Nimbus\\Nimbus Capture";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Nimbus Capture";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Nimbus Capture") },
        RegValue { name: "Version",     value: String::from("v1.4.17") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Nimbus Capture") },
        RegValue { name: "Publisher",       value: String::from("Nimbus") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Nimbus Capture\\uninstall.exe\"") },
    ]
}
