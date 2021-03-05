# smartos-lx-img-builder
Build smartos lx images from userland tar files.

## Warning

This is experimental and a work in progress.  The goal is to take generic Linux userland tarballs
and kick out a zfs dataset and manifest that can be used on a SmartOS host.

## Example

```
$ pfexec target/debug/smartos-lx-img-builder -d "testing image
 creation" -k 4.3 -m 20210106T005452Z -n lx-ubuntu-20.04 -t /var/tmp/lx-ubuntu-20.04-2020-11-27_15-44-08
.tar.xz -u "https://images.joyent.com" -z rpool
created dataset rpool/lx-ubuntu-20.04-20210305
creating dir /rpool/lx-ubuntu-20.04-20210305/root
set permissions for /rpool/lx-ubuntu-20.04-20210305/root to owner: 0 group: 0 mode: 755
created zroot /rpool/lx-ubuntu-20.04-20210305/root
extracted /var/tmp/lx-ubuntu-20.04-2020-11-27_15-44-08.tar.xz into /rpool/lx-ubuntu-20.04-20210305/root
creating dir /rpool/lx-ubuntu-20.04-20210305/root/native/dev
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/native/dev to owner: 0 group: 0 mode: 755
creating dir /rpool/lx-ubuntu-20.04-20210305/root/native/etc/default
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/native/etc/default to owner: 0 group: 0 mode: 755
creating dir /rpool/lx-ubuntu-20.04-20210305/root/native/etc/svc/volatile
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/native/etc/svc/volatile to owner: 0 group: 0 mode: 755
creating dir /rpool/lx-ubuntu-20.04-20210305/root/native/lib
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/native/lib to owner: 0 group: 0 mode: 755
creating dir /rpool/lx-ubuntu-20.04-20210305/root/native/proc
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/native/proc to owner: 0 group: 0 mode: 755
creating dir /rpool/lx-ubuntu-20.04-20210305/root/native/tmp
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/native/tmp to owner: 0 group: 0 mode: 755
creating dir /rpool/lx-ubuntu-20.04-20210305/root/native/usr
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/native/usr to owner: 0 group: 0 mode: 755
creating dir /rpool/lx-ubuntu-20.04-20210305/root/native/var
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/native/var to owner: 0 group: 0 mode: 755
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/native/tmp to owner: 0 group: 0 mode: 1777
creating file /rpool/lx-ubuntu-20.04-20210305/root/etc/fstab
creating symlink from /native/usr/sbin/mdata-get to /rpool/lx-ubuntu-20.04-20210305/root/usr/sbin/mdata-get
/rpool/lx-ubuntu-20.04-20210305/root/usr/sbin/mdata-get changed ownership to owner: 0 group: 0
creating symlink from /native/usr/sbin/mdata-put to /rpool/lx-ubuntu-20.04-20210305/root/usr/sbin/mdata-put
/rpool/lx-ubuntu-20.04-20210305/root/usr/sbin/mdata-put changed ownership to owner: 0 group: 0
creating symlink from /native/usr/sbin/mdata-delete to /rpool/lx-ubuntu-20.04-20210305/root/usr/sbin/mdata-delete
/rpool/lx-ubuntu-20.04-20210305/root/usr/sbin/mdata-delete changed ownership to owner: 0 group: 0
creating symlink from /native/usr/sbin/mdata-list to /rpool/lx-ubuntu-20.04-20210305/root/usr/sbin/mdata-list
/rpool/lx-ubuntu-20.04-20210305/root/usr/sbin/mdata-list changed ownership to owner: 0 group: 0
copying guest/etc/profile.d/native_manpath.sh to /rpool/lx-ubuntu-20.04-20210305/root/etc/profile.d/native_manpath.sh
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/etc/profile.d/native_manpath.sh to owner: 0 group: 0 mode: 744
creating dir /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc to owner: 0 group: 0 mode: 755
copying guest/lib/smartdc/common.lib to /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/common.lib
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/common.lib to owner: 0 group: 0 mode: 755
copying guest/lib/smartdc/debian to /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/debian
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/debian to owner: 0 group: 0 mode: 755
copying guest/lib/smartdc/mdata-execute to /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/mdata-execute
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/mdata-execute to owner: 0 group: 0 mode: 755
copying guest/lib/smartdc/void to /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/void
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/void to owner: 0 group: 0 mode: 755
copying guest/lib/smartdc/set-provision-state to /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/set-provision-state
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/set-provision-state to owner: 0 group: 0 mode: 755
copying guest/lib/smartdc/joyent_rc.local to /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/joyent_rc.local
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/joyent_rc.local to owner: 0 group: 0 mode: 755
copying guest/lib/smartdc/mount-zfs to /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/mount-zfs
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/mount-zfs to owner: 0 group: 0 mode: 755
copying guest/lib/smartdc/redhat to /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/redhat
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/redhat to owner: 0 group: 0 mode: 755
copying guest/lib/smartdc/mdata-fetch to /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/mdata-fetch
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/mdata-fetch to owner: 0 group: 0 mode: 755
copying guest/lib/smartdc/alpine to /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/alpine
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/alpine to owner: 0 group: 0 mode: 755
copying guest/lib/smartdc/mdata-image to /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/mdata-image
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/mdata-image to owner: 0 group: 0 mode: 755
copying guest/lib/smartdc/archlinux to /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/archlinux
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/lib/smartdc/archlinux to owner: 0 group: 0 mode: 755
detected distro as Debian
copying guest/lib/smartdc/joyent_rc.local to /rpool/lx-ubuntu-20.04-20210305/root/etc/rc.local
set permissions for /rpool/lx-ubuntu-20.04-20210305/root/etc/rc.local to owner: 0 group: 0 mode: 755
snapshot created: rpool/lx-ubuntu-20.04-20210305@final
created zfs gzip at lx-ubuntu-20.04-20210305.zfs.gz
created manifest at lx-ubuntu-20.04-20210305.json
destroyed dataset rpool/lx-ubuntu-20.04-20210305



========== Output ==========

filesystem: /home/mike/src/img-builder/lx-ubuntu-20.04-20210305.zfs.gz
manifest: /home/mike/src/img-builder/lx-ubuntu-20.04-20210305.json
```
