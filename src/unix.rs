/// Not properly tested.
pub(crate) fn is_elevated() -> bool {
    unsafe {
        let uid = libc::getuid();
        let euid = libc::geteuid();

        uid == 0 || uid != euid
    }
}
