# purp

[![Build Status](https://travis-ci.org/justinwoo/purp.svg?branch=master)](https://travis-ci.org/justinwoo/purp)

This is my convenience tool for doing PureScript things with Psc-Package. It doesn't try to do everything.

Make new issues or submit PRs if you want anything improved/different.

![](https://i.imgur.com/L6lArMv.jpg)

Normally would be named "Bongbong", but that would be hard to type.

## Usage

This has the following:

### Build

```sh
> purp build # aka `purp` atm
Building...
Installing 80 packages...
Success.
```

### Test

```sh
> purp test [--main Test.Main]
Building...
Installing 80 packages...
Success. Running tests.
You should add some tests.
Success.
```

### Run

```sh
> purp run [--skip-build (if you've already run build)]
           [--main Main]
Whatever output from your Main
```

### Bundle

```sh
> purp bundle [--skip-build (if you've already run build)]
              [--main Main]
  >> my-bundle.js # you probably want to pipe the output
```
