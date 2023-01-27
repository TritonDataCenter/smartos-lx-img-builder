/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright 2022 Joyent, Inc.
 * Copyright 2023 MNX Cloud, Inc.
 */

use anyhow::{bail, Context, Result};
use errno::errno;
use std::ffi::CString;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn set_permissions<P: AsRef<Path>>(path: P, mode: u32) -> Result<()> {
    let path = path.as_ref();
    let perms = fs::Permissions::from_mode(mode);
    fs::set_permissions(&path, perms).with_context(|| {
        format!(
            "failed to set permissions to {:o} on {}",
            mode,
            &path.display()
        )
    })?;

    Ok(())
}

fn chown<P: AsRef<Path>>(path: P, owner: u32, group: u32) -> Result<()> {
    let path = path.as_ref();
    let cstring = CString::new(path.as_os_str().as_bytes())
        .with_context(|| format!("path {} contains nul bytes", &path.display()))?;

    let (r, e) = unsafe {
        let r = libc::lchown(cstring.as_ptr(), owner, group);
        let e = errno();
        (r, e)
    };

    if r != 0 {
        bail!(
            "lchown({}, {}, {}): errno {}",
            &path.display(),
            owner,
            group,
            e
        );
    }

    Ok(())
}

pub fn mkdirp<P: AsRef<Path>>(path: P, owner: u32, group: u32, mode: u32) -> Result<()> {
    let path = path.as_ref();
    println!("creating dir {}", &path.display());
    fs::create_dir_all(&path).with_context(|| format!("mkdir -p {}", &path.display()))?;
    change_perms(&path, owner, group, mode)?;
    Ok(())
}

pub fn create_file_contents<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()> {
    let path = path.as_ref();

    println!("creating file {}", &path.display());
    let contents = contents.as_ref();
    fs::write(path, contents).context("writing file contents")?;
    Ok(())
}

pub fn create_symlink<S: AsRef<Path>, D: AsRef<Path>>(
    src: S,
    dst: D,
    owner: u32,
    group: u32,
) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    println!(
        "creating symlink from {} to {}",
        src.display(),
        dst.display()
    );
    std::os::unix::fs::symlink(src, dst)?;
    change_perms(dst, owner, group, 0o777)?;

    Ok(())
}

pub fn change_perms<P: AsRef<Path>>(path: P, owner: u32, group: u32, mode: u32) -> Result<()> {
    let path = path.as_ref();

    // symlinks don't have permissions so skip them
    let attr = fs::symlink_metadata(&path)?;
    if attr.file_type().is_symlink() {
        chown(&path, owner, group)?;
        println!(
            "{} changed ownership to owner: {} group: {}",
            &path.display(),
            owner,
            group,
        );
        return Ok(());
    }

    chown(&path, owner, group)?;
    set_permissions(&path, mode)?;
    println!(
        "set permissions for {} to owner: {} group: {} mode: {:o}",
        &path.display(),
        owner,
        group,
        mode
    );

    Ok(())
}

pub fn copy_file<S: AsRef<Path>, D: AsRef<Path>>(
    src: S,
    dst: D,
    owner: u32,
    group: u32,
    mode: u32,
) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    println!("copying {} to {}", src.display(), dst.display());
    fs::copy(src, dst).with_context(|| format!("copy {} to {}", src.display(), dst.display()))?;
    change_perms(dst, owner, group, mode)?;

    Ok(())
}
