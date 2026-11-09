// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::io::{self, Write};

// it's possible that using a smaller or larger buffer might provide better performance
pub(super) const BUF_SIZE: usize = 16 * 1024;

#[cfg(not(unix))]
pub fn exec(bytes: &[u8]) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    loop {
        stdout.write_all(bytes)?;
    }
}

#[cfg(unix)]
pub fn exec(bytes: &[u8]) -> io::Result<()> {
    // catch `EBADF` when stdout is read-only.
    let mut stdout = uucore::io::RawWriter(rustix::stdio::stdout());
    loop {
        stdout.write_all(bytes)?;
    }
}
