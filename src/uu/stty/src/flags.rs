// * This file is part of the uutils coreutils package.
// *
// * For the full copyright and license information, please view the LICENSE file
// * that was distributed with this source code.

// spell-checker:ignore parenb parodd cmspar hupcl cstopb cread clocal crtscts CSIZE
// spell-checker:ignore ignbrk brkint ignpar parmrk inpck istrip inlcr igncr icrnl ixoff ixon iuclc ixany imaxbel iutf
// spell-checker:ignore opost olcuc ocrnl onlcr onocr onlret ofill ofdel nldly crdly tabdly bsdly vtdly ffdly
// spell-checker:ignore isig icanon iexten echoe crterase echok echonl noflsh xcase tostop echoprt prterase echoctl ctlecho echoke crtkill flusho extproc

use crate::Flag;
use nix::sys::termios::{
    BaudRate, ControlFlags as C, InputFlags as I, LocalFlags as L, OutputFlags as O,
};

pub const CONTROL_FLAGS: [Flag<C>; 12] = [
    Flag::new("parenb", C::PARENB),
    Flag::new("parodd", C::PARODD),
    Flag::new("cmspar", C::CMSPAR),
    Flag::new_grouped("cs5", C::CS5, C::CSIZE),
    Flag::new_grouped("cs6", C::CS6, C::CSIZE),
    Flag::new_grouped("cs7", C::CS7, C::CSIZE),
    Flag::new_grouped("cs8", C::CS8, C::CSIZE).sane(),
    Flag::new("hupcl", C::HUPCL),
    Flag::new("cstopb", C::CSTOPB),
    Flag::new("cread", C::CREAD).sane(),
    Flag::new("clocal", C::CLOCAL),
    Flag::new("crtscts", C::CRTSCTS),
];

pub const INPUT_FLAGS: [Flag<I>; 15] = [
    Flag::new("ignbrk", I::IGNBRK),
    Flag::new("brkint", I::BRKINT).sane(),
    Flag::new("ignpar", I::IGNPAR),
    Flag::new("parmrk", I::PARMRK),
    Flag::new("inpck", I::INPCK),
    Flag::new("istrip", I::ISTRIP),
    Flag::new("inlcr", I::INLCR),
    Flag::new("igncr", I::IGNCR),
    Flag::new("icrnl", I::ICRNL).sane(),
    Flag::new("ixoff", I::IXOFF),
    Flag::new("tandem", I::IXOFF),
    Flag::new("ixon", I::IXON),
    // not supported by nix
    // Flag::new("iuclc", I::IUCLC),
    Flag::new("ixany", I::IXANY),
    Flag::new("imaxbel", I::IMAXBEL).sane(),
    Flag::new("iutf8", I::IUTF8),
];

pub const OUTPUT_FLAGS: [Flag<O>; 24] = [
    Flag::new("opost", O::OPOST).sane(),
    Flag::new("olcuc", O::OLCUC),
    Flag::new("ocrnl", O::OCRNL),
    Flag::new("onlcr", O::ONLCR).sane(),
    Flag::new("onocr", O::ONOCR),
    Flag::new("onlret", O::ONLRET),
    Flag::new("ofill", O::OFILL),
    Flag::new("ofdel", O::OFDEL),
    Flag::new_grouped("nl0", O::NL0, O::NLDLY).sane(),
    Flag::new_grouped("nl1", O::NL1, O::NLDLY),
    Flag::new_grouped("cr0", O::CR0, O::CRDLY).sane(),
    Flag::new_grouped("cr1", O::CR1, O::CRDLY),
    Flag::new_grouped("cr2", O::CR2, O::CRDLY),
    Flag::new_grouped("cr3", O::CR3, O::CRDLY),
    Flag::new_grouped("tab0", O::TAB0, O::TABDLY).sane(),
    Flag::new_grouped("tab1", O::TAB1, O::TABDLY),
    Flag::new_grouped("tab2", O::TAB2, O::TABDLY),
    Flag::new_grouped("tab3", O::TAB3, O::TABDLY),
    Flag::new_grouped("bs0", O::BS0, O::BSDLY).sane(),
    Flag::new_grouped("bs1", O::BS1, O::BSDLY),
    Flag::new_grouped("vt0", O::VT0, O::VTDLY).sane(),
    Flag::new_grouped("vt1", O::VT1, O::VTDLY),
    Flag::new_grouped("ff0", O::FF0, O::FFDLY).sane(),
    Flag::new_grouped("ff1", O::FF1, O::FFDLY),
];

pub const LOCAL_FLAGS: [Flag<L>; 18] = [
    Flag::new("isig", L::ISIG).sane(),
    Flag::new("icanon", L::ICANON).sane(),
    Flag::new("iexten", L::IEXTEN).sane(),
    Flag::new("echo", L::ECHO).sane(),
    Flag::new("echoe", L::ECHOE).sane(),
    Flag::new("crterase", L::ECHOE).hidden().sane(),
    Flag::new("echok", L::ECHOK).sane(),
    Flag::new("echonl", L::ECHONL),
    Flag::new("noflsh", L::NOFLSH),
    // Not supported by nix
    // Flag::new("xcase", L::XCASE),
    Flag::new("tostop", L::TOSTOP),
    Flag::new("echoprt", L::ECHOPRT),
    Flag::new("prterase", L::ECHOPRT).hidden(),
    Flag::new("echoctl", L::ECHOCTL).sane(),
    Flag::new("ctlecho", L::ECHOCTL).sane().hidden(),
    Flag::new("echoke", L::ECHOKE).sane(),
    Flag::new("crtkill", L::ECHOKE).sane().hidden(),
    Flag::new("flusho", L::FLUSHO),
    Flag::new("extproc", L::EXTPROC),
];

pub const BAUD_RATES: &[(&str, BaudRate)] = &[
    ("0", BaudRate::B0),
    ("50", BaudRate::B50),
    ("75", BaudRate::B75),
    ("110", BaudRate::B110),
    ("134", BaudRate::B134),
    ("150", BaudRate::B150),
    ("200", BaudRate::B200),
    ("300", BaudRate::B300),
    ("600", BaudRate::B600),
    ("1200", BaudRate::B1200),
    ("1800", BaudRate::B1800),
    ("2400", BaudRate::B2400),
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    ("4800", BaudRate::B4800),
    ("9600", BaudRate::B9600),
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    ("14400", BaudRate::B14400),
    ("19200", BaudRate::B19200),
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    ("28800", BaudRate::B28800),
    ("38400", BaudRate::B38400),
    ("57600", BaudRate::B57600),
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    ("76800", BaudRate::B76800),
    ("115200", BaudRate::B115200),
    ("230400", BaudRate::B230400),
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    ("460800", BaudRate::B460800),
    #[cfg(any(target_os = "android", target_os = "linux"))]
    ("500000", BaudRate::B500000),
    #[cfg(any(target_os = "android", target_os = "linux"))]
    ("576000", BaudRate::B576000),
    #[cfg(any(
        target_os = "android",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "netbsd"
    ))]
    ("921600", BaudRate::B921600),
    #[cfg(any(target_os = "android", target_os = "linux"))]
    ("1000000", BaudRate::B1000000),
    #[cfg(any(target_os = "android", target_os = "linux"))]
    ("1152000", BaudRate::B1152000),
    #[cfg(any(target_os = "android", target_os = "linux"))]
    ("1500000", BaudRate::B1500000),
    #[cfg(any(target_os = "android", target_os = "linux"))]
    ("2000000", BaudRate::B2000000),
    #[cfg(any(
        target_os = "android",
        all(target_os = "linux", not(target_arch = "sparc64"))
    ))]
    ("2500000", BaudRate::B2500000),
    #[cfg(any(
        target_os = "android",
        all(target_os = "linux", not(target_arch = "sparc64"))
    ))]
    ("3000000", BaudRate::B3000000),
    #[cfg(any(
        target_os = "android",
        all(target_os = "linux", not(target_arch = "sparc64"))
    ))]
    ("3500000", BaudRate::B3500000),
    #[cfg(any(
        target_os = "android",
        all(target_os = "linux", not(target_arch = "sparc64"))
    ))]
    ("4000000", BaudRate::B4000000),
];
