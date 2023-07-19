/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright 2022 Joyent, Inc.
 * Copyright 2023 MNX Cloud, Inc.
 */

extern crate os_release;

use anyhow::{Context, Result};
use chrono::prelude::*;
use os_release::OsRelease;
use std::fs;
use std::path::Path;
use uuid::Uuid;

mod actions;
mod cli;
mod guest;
mod manifest;
mod utils;

use actions::*;
use manifest::*;

macro_rules! run_action {
    ($fn:expr, $ds:expr) => {{
        match $fn {
            Ok(v) => v,
            Err(e) => {
                actions::destroy_dataset($ds);
                return Err(e);
            }
        }
    }};
}

fn read_os_release<P: AsRef<Path>>(zroot: P) -> Result<OsRelease> {
    let zroot = zroot.as_ref();
    let path = &zroot.join("etc/os-release");
    let release = OsRelease::new_from(path)?;
    Ok(release)
}

fn get_zfs_parent(s: &str) -> String {
    let zonename = zonename::getzonename().expect("failed to get zonename");
    if s.is_empty() {
        if zonename == "global" {
            // This bakes in a smartos-ism, and poorly. It would be better to
            // query SMF for the blessed system_pool like we're supposed to.
            // It may also be a good idea to search for alternate pools
            // (e.g., rpool). For now at least, they can' use -z to specify.
            return "zones".to_string();
        } else {
            return format!("zones/{}/data", &zonename);
        }
    } else {
        return s.to_string();
    }
}

fn main() -> Result<()> {
    let opts = cli::get_opts();
    let utc: DateTime<Utc> = Utc::now();
    let build_date = utc.format("%Y%m%d").to_string();
    let uuid = Uuid::new_v4();
    let iuuid = format!("{}-{}", &uuid, &build_date);
    let zfs_parent = get_zfs_parent(&opts.zfs_parent);

    let dataset = format!("{}/{}", &zfs_parent, &iuuid);
    let zroot = create_dataset(&dataset)?;
    run_action!(install_tar(&zroot, &opts.tar), &dataset);

    let os_release = read_os_release(&zroot).context("failed to read os-release")?;
    let name: String;
    if opts.image_name.to_string().len() > 0 {
        name = opts.image_name;
    } else {
        name = format!("{}-{}", os_release.id, os_release.version_id)
            .trim_end_matches("-")
            .to_string();
    }
    let zfs_tar = format!("output/{}-{}.zfs.gz", &name, &build_date);
    let image_manifest = &format!("output/{}-{}.json", &name, &build_date);

    let desc = format!(
        "Container-native {} 64-bit image. {}",
        os_release.pretty_name, &opts.description
    );
    let manifest = Manifest {
        name: &name,
        version: &build_date,
        description: &desc.trim(),
        homepage: &opts.url,
        min_platform: &opts.min_platform,
        uuid: &uuid,
        os: "linux",
        kernel: &opts.kernel,
        tar_file: &zfs_tar,
    };
    let product = format!(
        r#"Name: Triton Instance
Image: {} {}
Documentation: {}
Description: {}

"#,
        &os_release.pretty_name, &build_date, &opts.url, &desc
    );

    let motd = format!(
        r#"         *--+--*--*
         |\ |\ |\ |\
         | \| \| \| \     #####  ####   #  #####  ###   #   # TM
         +--*--+--*--*      #    #   #  #    #   #   #  ##  #
         |\ |\ |\ |\ |      #    ####   #    #   #   #  # # #
         | \| \| \| \|      #    #  #   #    #   #   #  #  ##
         *--+--+--+--+      #    #   #  #    #    ###   #   #
          \ |\ |\ |\ |
           \| \| \| \|     LX Instance ({} {})
            *--+--*--*     {}

"#, &os_release.pretty_name, &build_date, &opts.url
    );

    fs::create_dir_all("output")
        .expect("Unable to create output directory");
    run_action!(modify_image(&zroot, &product, &motd), &dataset);
    run_action!(install_guest_tools(&zroot), &dataset);
    run_action!(create_dataset_gzip(&dataset, &zfs_tar), &dataset);
    run_action!(create_manifest(manifest, &image_manifest), &dataset);
    destroy_dataset(dataset);

    print!("\n\n\n========== Output ==========\n\n");
    println!("filesystem: {}", std::fs::canonicalize(&zfs_tar)?.display());
    println!(
        "manifest: {}",
        std::fs::canonicalize(&image_manifest)?.display()
    );

    Ok(())
}
