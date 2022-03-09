/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright 2022 Joyent, Inc.
 */

use anyhow::Result;
use chrono::prelude::*;
use sha1::Sha1;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use uuid::Uuid;

const BUFSIZE: usize = 1024;

pub struct Manifest<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub description: &'a str,
    pub homepage: &'a str,
    pub min_platform: &'a str,
    pub uuid: &'a Uuid,
    pub os: &'a str,
    pub kernel: &'a str,
    pub tar_file: &'a str,
}

fn sha1_digest<R: Read>(mut reader: R) -> Result<String> {
    let mut hasher = Sha1::default();
    let mut buffer = [0; BUFSIZE];

    loop {
        let count = reader.read(&mut buffer)?;
        hasher.update(&buffer[..count]);
        if count == 0 || count < BUFSIZE {
            break;
        }
    }

    Ok(hasher.digest().to_string())
}

impl<'a> Manifest<'a> {
    pub fn to_writer<W: Write>(&self, writer: W) -> Result<()> {
        let file = File::open(&self.tar_file)?;
        let bufreader = BufReader::new(file);
        let shasum = sha1_digest(bufreader)?;
        let filesize = fs::metadata(&self.tar_file)?.len().to_string();
        let utc: DateTime<Utc> = Utc::now();
        let published_at = utc.format("%Y-%m-%dT%TZ").to_string();

        let manifest = serde_json::json!({
          "v": "2",
          "name": self.name,
          "version": self.version,
          "type": "lx-dataset",
          "description": self.description,
          "homepage": self.homepage,
          "published_at": published_at,
          "os": self.os,
          "files": [
            {
              "sha1": shasum,
              "size": filesize,
              "compression": "gzip"
            }
          ],
          "requirements": {
            "networks": [
              {
                "name": "net0",
                "description": "public"
              }
            ],
            "min_platform": {
                 "7.0": self.min_platform,
               },
            "brand": "lx"
          },
          "uuid": self.uuid,
          "public": false,
          "owner": "00000000-0000-0000-0000-000000000000",
          "tags": {
            "role": "os",
            "kernel_version": self.kernel
          }
        });

        Ok(serde_json::to_writer_pretty(writer, &manifest)?)
    }
}
