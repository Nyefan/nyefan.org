---
layout: single
title: "Renicing in Unprivileged Containers"
author: "Nyefan"
categories:
  - "software"
tags:
  - "original content"
  - "tools"
---

### What is Niceness

In Linux, you can think of `niceness` as the inverse of process priority.  The default `niceness` of any process is it's
parent process's `niceness` or `0` for pid 1.  `niceness` can take any value from -20 to 20, and processes with a higher
`niceness` will be deferred by the cpu scheduler while processes with lower `niceness` are running.  Raising `niceness`
can be done by any user on their owned processes, but lowering niceness is forbidden without root, which raises the 
question, "How can we give way to existing processes during container startup but then return to the default `niceness`
once we are able to serve traffic?".  The normal answer would be to set root uid on the `renice` program to allow an
unprivileged user to set `niceness`.  However, if you're using Alpine containers, then `renice <args>` is actually 
`busybox renice <args>`, so setting root uid on `renice` would completely give up on the idea of non-root users.  Here 
is once possible solution.

### Code

```rust
/// This exists exclusively so we can add `CAP_SYS_NICE` to the output binary
/// We cannot set `CAP_SYS_NICE` on a shell script since that drops once it's passed to the shebang
/// We cannot just set root uid on a helper shell script because that uid is dropped when it's
///     passed to the shebang
/// We do not want to simply set `CAP_SYS_NICE` on /usr/bin/renice because that is a symlink on
///     alpine to `/bin/busybox`
/// For the same reason, we do not want to just set root uid on `/usr/bin/renice`
/// We also do not want to set `CAP_SYS_NICE` on `/bin/sh` or equivalent
/// We cannot give CAP_SYS_NICE to the uwsgi user because alpine doesn't use PAM
/// There is no way that shell scripts can regain `CAP_*` once they've been dropped unless that
///     `CAP_*` is set in the inode _and_ the user's `CapEff` allows it, and k8s currently has no
///     mechanism for setting `CapInh` and `CapPrm`, `CapEff`, only `CapBnd`
/// After this is built, ownership needs to be set to root:root, the uid, gid, and `CAP_SYS_NICE`
///     bits must be set on the output binary, and permissions should be set to 0555
/// This only works if the container or pod definition includes in the securityContext:
///   \`\`\`yaml
///     capabilities:
///       add:
///         - SYS_NICE
///   \`\`\`
/// This only works on alpine linux
use std::os::unix::process::CommandExt;

fn main() {
    std::process::Command::new("/bin/busybox")
        .arg0("renice")
        .args(std::env::args_os().skip(1))
        .exec();
    std::process::exit(127);
}
```