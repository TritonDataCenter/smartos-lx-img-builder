/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright 2022 Joyent, Inc.
 * Copyright 2022 MNX Cloud, Inc.
 */

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(
        name = "tar",
        long = "tar",
        short = "t",
        help = "lx userland tar file",
        required = true
    )]
    pub tar: String,
    #[structopt(
        name = "kernel",
        long = "kernel",
        short = "k",
        help = "the kernel version",
        default_value = "5.10.0"
    )]
    pub kernel: String,
    #[structopt(
        name = "min_platform",
        long = "min",
        short = "m",
        help = "the minimum platform required for the image",
        default_value = "20210826T002459Z"
    )]
    pub min_platform: String,
    #[structopt(
        name = "description",
        long = "description",
        short = "d",
        help = "text to append to the description of the image as it would appear in the manifest",
        default_value = ""
    )]
    pub description: String,
    #[structopt(
        name = "url",
        long = "url",
        short = "u",
        help = "the url to information about the image as it would appear in the manifest",
        default_value = "https://docs.tritondatacenter.com/public-cloud/instances/infrastructure/images"
    )]
    pub url: String,
    #[structopt(
        name = "zfs_parent",
        long = "zfs-parent",
        short = "z",
        help = "the parent zfs dataset to use when creating our temporary image",
        default_value = ""
    )]
    pub zfs_parent: String,
}

pub fn get_opts() -> Opts {
    Opts::from_args()
}
