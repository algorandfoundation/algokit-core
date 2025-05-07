# glibc and musl Support

One of the difficulties of shipping binaries is deciding which C standard library versions to support. This document goes over the options and how we might want to decide which versions to support.

## musl vs glibc

The main difference between musl and glibc is that glibc includes non-standard gnu-specific features whereas musl only supports POSIX standards. This results in musl being smaller but glibc being more compatible with modern Linux systems. Despite this, musl should still be supported because it is used in size-oriented distros, such as Alpine, which are commonly used in docker containers.

## glibc Versions

glibc versions are backwards but not forwards compatible, meaning a system with an older version of glibc will not be able to run a binary compiled with a newer version of glibc. This means we need to deicde how far back we should go for glibc support. The latest Rust compiler requires glibc 2.17 or higher. As of Feb 3, 2025 100% of Python package consumers are on glibc 2.17+ (source: [Manylinux Timeline](https://mayeut.github.io/manylinux-timeline/)) which makes it the ideal glibc version to support. Supporting older versions of glibc would result in compromises needing to be made in regards to which version of Rust we can use. Considering we are using bleeding edge tools, limiting ourselves in this way seems unreasonable for a very small percent (0% according to manylinux timeline) of users.

### Future Support

Eventually we will need likely need to move to a newer glibc version due to toolchain requirements. In general, a good rule of thumb is to support the glibc version from two RHEL releases ago. For example, as of the time of this writing the most recent stable RHEL release is 9, which means we should support the glibc version in RHEL 7, which is 2.17. Historically this roughly aligns with the End-of-Maintenance period for RHEL.
