# apk-decompiler

[![Build Status](https://travis-ci.org/robertohuertasm/apk-decompiler.svg?branch=master)](https://travis-ci.org/robertohuertasm/apk-decompiler)

Small utility to decompile your `apks` so you don't have to worry using lots of different tools.

## How to install it

At the moment, it only works in Mac and Linux. Check out the [releases section](https://github.com/robertohuertasm/apk-decompiler/releases) and download the specific file for your system

## How to get your apk

First of all you need a rooted phone so you can access it via `adb shell`.

Once you're there, if you want to list the packages installed in your device:

```sh
pm list packages
# or if you're looking for something specific
pm list packages -f instagram
# or
pm list packages | awk -F':' '{print $2}' | grep instagram
```

Once you know which is the name of the package you want to access:

```sh
pm path <name-of-the-package>
# e.g com.company.app
# this may output something similar to this:
# /data/app/com.company.app/BBhSG-3w3_vAghNyy2LsKg==/base.apk
```

In order to extract a package we'll use `adb` again:

```sh
adb pull <path-to-the-package>
```

This will extract the package into your current folder, normally with the following name: `base.apk`.

## How to use it

Just execute:

`./apk-decompiler <name-of-your-apk>`

You'll get a new folder called `output` which contains the following folders:

1. **decompiled**: This is the output of running [dex2jar](https://github.com/pxb1988/dex2jar).
1. **extracted**: This folder contains the output of `unzipping` the `apk`.
1. **xml**: This is basically the output of running [apktool](https://ibotpeaches.github.io/Apktool/).
1. **package-name-error.zip**: Optional file that you will get in case there were some errors during the decompilation process.

## Known issues

If you have folders with empty spaces this may be a problem for [jd-cli](https://github.com/kwart/jd-cmd) which is one of the dependencies of this project. So... just try to avoid them ;P
