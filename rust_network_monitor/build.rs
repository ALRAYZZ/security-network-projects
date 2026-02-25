fn main() {
    // This tells the linker exactly where to find the Npcap files
    println!(r"cargo:rustc-link-search=native=D:\.CyberSec\npcap-sdk-1.16\Lib\x64");
}