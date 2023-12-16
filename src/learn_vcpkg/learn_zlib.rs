use crate::bindgen::zlib::*;
use std::ffi::CStr;

pub fn test_zlib() {
    /* zlib 已经成为了一种事实上的业界标准，数以千计的应用程序直接或间接依靠zlib压缩库。
     * 包括：
     *   Linux核心：使用zlib以实现网络协议的压缩、文件系统的压缩以及引导时解压缩自身的核心。
     *   libpng，用于PNG图形格式的一个实现，对bitmap数据规定了DEFLATE作为流压缩方法。
     *   Apache：使用zlib实现http 1.1。
     *   OpenSSH、OpenSSL：以zlib达到优化加密网络传输。
     *   FFmpeg：以zlib读写Matroska等以DEFLATE算法压缩的多媒体流格式。
     *   rsync：以zlib优化远程同步时的传输。
     *   Subversion、Git和CVS等版本控制系统，使用zlib来压缩和远端仓库的通讯流量。
     *   dpkg和RPM等包管理软件：以zlib解压缩RPM或者其他数据包。
     */
    let c_version = unsafe { zlibVersion() };
    let cstr_version = unsafe { CStr::from_ptr(c_version) };
    let version: String = cstr_version.to_string_lossy().into_owned();
    println!("zlib version: {}", version);
}
