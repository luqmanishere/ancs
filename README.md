# ancs
[![crates.io](https://img.shields.io/crates/v/ancs.svg)](https://crates.io/crates/ancs)
[![Released API docs](https://docs.rs/ancs/badge.svg)](https://docs.rs/ancs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![codecov](https://codecov.io/gh/ianmarmour/ancs/branch/main/graph/badge.svg?token=KVF3X22UFA)](https://codecov.io/gh/ianmarmour/ancs)
[![CI](https://github.com/ianmarmour/ancs/workflows/CI/badge.svg)](https://github.com/ianmarmour/ancs/actions/workflows/ci.yml)

An Apple Notification Control Service protocol library for Rust, with minimal dependencies.

## What is ANCS

> The purpose of the Apple Notification Center Service (ANCS) is to give Bluetooth accessories (that connect to iOS devices through a Bluetooth low-energy link) a simple and convenient way to access many kinds of notifications that are generated on iOS devices.

## What is This Library

This library contains all the required types and functions to work with the Bluetooth Low Energy Apple Notification Control Service Protocol. This is a low-level library, meant to be a building block for
other applications that interface with the Apple Notification Control Service.

## How Do I Use This Library

Please see the [Apple ANCS Specification](https://developer.apple.com/library/archive/documentation/CoreBluetooth/Reference/AppleNotificationCenterServiceSpecification/Introduction/Introduction.html#//apple_ref/doc/uid/TP40013460-CH2-SW1) for how to interface with their BLE protocol. This library strives to keep all terminology in line with the official documentation and should be easy to work with by following this specification alongside other ble libraries for Rust such as [btleplug](https://github.com/deviceplug/btleplug).