/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright 2022 Joyent, Inc.
 */

use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

use crate::utils::*;

#[derive(Copy, Debug, Clone)]
enum Distro {
    Alpine,
    Arch,
    Debian,
    Redhat,
    Void,
    Unknown,
}

impl Distro {
    fn detect<P: AsRef<Path>>(zroot: P) -> Self {
        let zroot = zroot.as_ref();

        let supported = [
            (Distro::Alpine, "etc/alpine-release"),
            (Distro::Arch, "etc/arch-release"),
            (Distro::Debian, "etc/debian_version"),
            (Distro::Redhat, "etc/redhat-release"),
            (Distro::Void, "etc/void-release"),
        ];

        if let Some((d, _)) = supported.iter().find(|(_, p)| zroot.join(p).exists()) {
            println!("detected distro as {:?}", d);
            return *d;
        }

        Distro::Unknown
    }

    fn install<P: AsRef<Path>>(&self, zroot: P) -> Result<()> {
        let zroot = zroot.as_ref();

        match self {
            Self::Alpine => {
                let rclocal = zroot.join("etc/rc.local");
                copy_file("guest/lib/smartdc/joyent_rc.local", &rclocal, 0, 0, 0o755)?;
                let shutdown = zroot.join("sbin/shutdown");
                copy_file("guest/sbin/shutdown", &shutdown, 0, 0, 0o755)?;
                copy_file(
                    "guest/lib/smartdc/alpine",
                    zroot.join("lib/smartdc/alpine"),
                    0,
                    0,
                    0o755,
                )?;
            }
            Self::Arch => {
                let system = zroot.join("etc/systemd/system");
                mkdirp(&system, 0, 0, 0o755)?;
                let service = &system.join("joyent.service");
                copy_file("etc/systemd/system/joyent.service", &service, 0, 0, 0o644)?;
                let enable =
                    zroot.join("etc/systemd/system/multi-user.target.wants/joyent.service");
                create_symlink(&service, &enable, 0, 0)?;
                copy_file(
                    "guest/lib/smartdc/arch",
                    zroot.join("lib/smartdc/arch"),
                    0,
                    0,
                    0o755,
                )?;
            }
            Self::Debian => {
                let rclocal = zroot.join("etc/rc.local");
                copy_file("guest/lib/smartdc/joyent_rc.local", &rclocal, 0, 0, 0o755)?;
                copy_file(
                    "guest/lib/smartdc/debian",
                    zroot.join("lib/smartdc/debian"),
                    0,
                    0,
                    0o755,
                )?;
            }
            Self::Redhat => {
                let dst = zroot.join("etc/rc.local");
                copy_file("guest/lib/smartdc/joyent_rc.local", &dst, 0, 0, 0o755)?;
                copy_file(
                    "guest/lib/smartdc/redhat",
                    zroot.join("lib/smartdc/redhat"),
                    0,
                    0,
                    0o755,
                )?;
            }
            Self::Void => {
                let rclocal = zroot.join("etc/rc.local");
                copy_file("guest/lib/smartdc/joyent_rc.local", &rclocal, 0, 0, 0o755)?;
                let shutdown = zroot.join("sbin/shutdown");
                copy_file("guest/sbin/shutdown", &shutdown, 0, 0, 0o755)?;
                copy_file(
                    "guest/lib/smartdc/void",
                    zroot.join("lib/smartdc/void"),
                    0,
                    0,
                    0o755,
                )?;
            }
            Self::Unknown => {
                bail!("failed to detect supported Linux Distribution");
            }
        };

        Ok(())
    }
}

fn install_native_manpath<P: AsRef<Path>>(zroot: P) -> Result<()> {
    let zroot = zroot.as_ref();

    copy_file(
        "guest/etc/profile.d/native_manpath.sh",
        zroot.join("etc/profile.d/native_manpath.sh"),
        0,
        0,
        0o744,
    )?;
    Ok(())
}

fn install_smartdc<P: AsRef<Path>>(zroot: P) -> Result<()> {
    let zroot = zroot.as_ref();

    mkdirp(zroot.join("lib/smartdc"), 0, 0, 0o755)?;
    let paths = [
        "lib/smartdc/common.lib",
        "lib/smartdc/mdata-execute",
        "lib/smartdc/mdata-fetch",
        "lib/smartdc/mdata-image",
        "lib/smartdc/mount-zfs",
        "lib/smartdc/set-provision-state",
    ];

    let guest = Path::new("guest");
    for p in &paths {
        let src = guest.join(p);
        let dst = zroot.join(p);
        copy_file(&src, &dst, 0, 0, 0o755)?;
    }

    Ok(())
}

fn install_distro<P: AsRef<Path>>(zroot: P) -> Result<()> {
    let zroot = zroot.as_ref();

    let distro = Distro::detect(zroot);
    distro.install(zroot)?;

    Ok(())
}

fn install_mdata_commands<P: AsRef<Path>>(zroot: P) -> Result<()> {
    let zroot = zroot.as_ref();

    let paths = [
        "usr/sbin/mdata-get",
        "usr/sbin/mdata-put",
        "usr/sbin/mdata-delete",
        "usr/sbin/mdata-list",
    ];

    for p in &paths {
        let dst = zroot.join(p);
        let src = Path::new("/native").join(p);
        if dst.exists() {
            fs::remove_file(&dst)
                .with_context(|| format!("failed to unlink {}", &src.display()))?;
            println!("unlinked {}", &src.display());
        }
        create_symlink(&src, &dst, 0, 0)?;
    }

    Ok(())
}

pub fn install_tools<P: AsRef<Path>>(zroot: P) -> Result<()> {
    let zroot = zroot.as_ref();

    install_mdata_commands(zroot)?;
    install_native_manpath(zroot)?;
    install_smartdc(zroot)?;
    install_distro(&zroot)?;
    Ok(())
}
