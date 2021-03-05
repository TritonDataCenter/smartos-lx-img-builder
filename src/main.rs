use anyhow::Result;
use chrono::prelude::*;
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

fn main() -> Result<()> {
    let opts = cli::get_opts();

    let utc: DateTime<Utc> = Utc::now();
    let build_date = utc.format("%Y%m%d").to_string();
    let iuuid = format!("{}-{}", &opts.name, &build_date);
    let dataset = format!("{}/{}", &opts.zpool, &iuuid);
    let zfs_tar = format!("{}.zfs.gz", &iuuid);
    let image_manifest = &format!("{}.json", &iuuid);
    let manifest = Manifest {
        name: &opts.name,
        version: &build_date,
        description: &opts.description,
        homepage: &opts.url,
        min_platform: &opts.min_platform,
        uuid: &Uuid::new_v4(),
        os: "linux",
        kernel: &opts.kernel,
        tar_file: &zfs_tar,
    };

    let zroot = create_dataset(&dataset)?;
    run_action!(install_tar(&zroot, &opts.tar), &dataset);
    run_action!(modify_image(&zroot), &dataset);
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
