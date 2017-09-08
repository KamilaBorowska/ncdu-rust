extern crate gcc;

fn main() {
    gcc::Build::new()
        .file("src/browser.c")
        .file("src/delete.c")
        .file("src/dir_common.c")
        .file("src/dir_export.c")
        .file("src/dir_import.c")
        .file("src/dirlist.c")
        .file("src/dir_mem.c")
        .file("src/dir_scan.c")
        .file("src/exclude.c")
        .file("src/help.c")
        .file("src/main.c")
        .file("src/path.c")
        .file("src/shell.c")
        .file("src/util.c")
        .include(".")
        .include("deps")
        .compile("ncdu");
}
