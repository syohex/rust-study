use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path;

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        let meta = fs::metadata(self).unwrap();
        let mode = meta.mode();
        (mode & 0o100) != 0
    }

    fn is_writeable(&self) -> bool {
        let meta = fs::metadata(self).unwrap();
        !meta.permissions().readonly()
    }

    fn exists(&self) -> bool {
        self.exists()
    }
}

fn main() {
    use std::path::Path;

    let p = Path::new("/usr/bin/python3");
    println!("readble:{}", p.is_readable());
    println!("writable:{}", p.is_writeable());
    println!("exists:{}", p.exists());

    // touch data/not_read && chmod -r data/not_read
    let r = Path::new("data/not_read");
    println!("exists:{}", r.exists());
    println!("readble:{}", r.is_readable());
    println!("writable:{}", p.is_writeable());
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    fs::remove_file(f.path()).unwrap();
}
