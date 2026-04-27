use std::path::PathBuf;

pub fn expand_path(path: &str) -> PathBuf {
    if path.starts_with("~") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    PathBuf::from(path)
}



pub fn get_default_ascii() -> String {
    r#"
__  __
  \ \/ /
   \  /
   /  \
  /_/\_\
 /____/linux
---------BEGIN PUBLIC KEY----------
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMII
BCgKCAQEAwtU/XOS/xOf/FakeKeyDataFor
ArtPutHere+Of/XOSLINUXDISTRO/gD4t4+
N4ToR3aL/K3yG3NkR/v3RyL0ngD4t4+XoSy
LiNuX/R0cK/M0r3+L1nUx/D1sTr0/R4cK3a
XYS/aBcDeFgHiJkLmNoPqRsTuVwXyZ01234
56789+/woXosLinuxR0xOsD1sTr0d1stR0f
0/c00L/k3Y/wER3aL63nD/v3RyL0ngD4t4a
ABcD/EfGhIjK/xOsLiNuX/D1sTr0/R0cK3a
LmNoPqRsTuVwXyZ0/1nUx/wER3aL63nD/v3
RyL0ngD4t4+XoSy/M0r3+L1nUx/D1sTr0/R
4cK3a/XYS/aBcDeFgHiJkLmNoPqRsTuVwXy
Z0123456789+/woXos/Linux/Rocks/==
----------END PUBLIC KEY-----------
"#.trim().to_string()
}
