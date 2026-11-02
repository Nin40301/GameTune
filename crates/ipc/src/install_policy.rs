//! Conservative ACL policy, independently testable without changing host ACLs.
pub fn trusted_installer_sid(sid: &str) -> bool {
    matches!(
        sid,
        "S-1-5-18"
            | "S-1-5-32-544"
            | "S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464"
    )
}
pub fn local_install_path(path: &str) -> bool {
    let path = path.strip_prefix(r"\\?\").unwrap_or(path).as_bytes();
    path.len() >= 3 && path[0].is_ascii_alphabetic() && path[1] == b':' && path[2] == b'\\'
}
pub fn unsafe_allow(mask: u32, sid: &str, ancestor: bool) -> bool {
    // GENERIC_ALL/WRITE, WRITE_DAC/OWNER, DELETE, DELETE_CHILD. Installation
    // directory/file additionally disallows writing/creating content and attributes.
    let rights = 0x1000_0000 | 0x4000_0000 | 0x000d_0000 | 0x40 | if ancestor { 0 } else { 0x116 };
    !trusted_installer_sid(sid) && mask & rights != 0
}
pub fn helper_hash_matches(expected: Option<&str>, bytes: &[u8]) -> bool {
    use sha2::{Digest, Sha256};
    expected.is_some_and(|expected| {
        let actual = format!("{:x}", Sha256::digest(bytes));
        expected.len() == 64 && actual == expected
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn standard_users_and_owners_cannot_replace_helper() {
        for sid in ["S-1-1-0", "S-1-5-11", "S-1-5-32-545", "S-1-5-21-1"] {
            for right in [
                2, 4, 0x10, 0x100, 0x40, 0x10000, 0x40000, 0x80000, 0x40000000, 0x10000000,
            ] {
                assert!(unsafe_allow(right, sid, false));
            }
            assert!(!trusted_installer_sid(sid));
        }
        assert!(!unsafe_allow(0x1200a9, "S-1-5-32-545", false));
        assert!(!unsafe_allow(4, "S-1-5-32-545", true));
        assert!(unsafe_allow(0x40, "S-1-5-32-545", true));
        assert!(!unsafe_allow(u32::MAX, "S-1-5-18", false));
    }
    #[test]
    fn missing_hash_and_substituted_helper_fail_closed() {
        assert!(!helper_hash_matches(None, b"helper"));
        assert!(!helper_hash_matches(Some(&"0".repeat(64)), b"helper"));
        use sha2::{Digest, Sha256};
        let hash = format!("{:x}", Sha256::digest(b"helper"));
        assert!(helper_hash_matches(Some(&hash), b"helper"));
        assert!(!helper_hash_matches(Some(&hash), b"replacement"));
    }
    #[test]
    fn device_remote_and_relative_install_paths_are_rejected() {
        for path in [
            r"\\server\share\helper.exe",
            r"\\?\UNC\server\share\helper.exe",
            r"\\?\GLOBALROOT\Device\helper.exe",
            "helper.exe",
            r"C:helper.exe",
        ] {
            assert!(!local_install_path(path));
        }
        assert!(local_install_path(r"C:\Program Files\Ametista\helper.exe"));
        assert!(local_install_path(
            r"\\?\C:\Program Files\Ametista\helper.exe"
        ));
    }
}
