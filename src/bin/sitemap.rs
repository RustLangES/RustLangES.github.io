fn main() {
    /*
     * for windows:
     *
     * you can use busybox
     * @see https://busybox.net/
     *
     * try:
     * $ scoop info busybox
     */
    std::process::Command::new("wget")
        .arg("https://github.com/ph4un00b/sitemap-rustico/releases/download/latest/sitemap.xml")
        .output()
        .expect("Failed to execute wget");

    match std::fs::rename("sitemap.xml", "public/sitemap.xml") {
        Ok(_) => println!("sitemap.xml moved successfully!"),
        Err(err) => println!("Error: {}", err),
    }
}
