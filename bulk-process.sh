#!/usr/bin/env bash
curl -o latest.json -fL https://api.github.com/repos/TritonDataCenter/lx-images/releases/latest

RELEASE=$( <latest.json json .tag_name )

mkdir -p $RELEASE

cd $RELEASE
<../latest.json json .assets | json -a .browser_download_url | xargs -t -n 1 -- curl -fLO
cd ..

for file in $RELEASE/* ; do
	target/debug/smartos-lx-img-builder --tar $file
done

rm latest.json
