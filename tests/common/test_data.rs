pub const TEST_HASH_1: &str = "277a138cd1ee79fc1fdb2869c321b848d4861e45b82184487139ef66dd40b62d";
pub const TEST_HASH_2: &str = "9641a590e66d9f2e5137b6bcba07fdf6cec3ffaa54de2565c3afcc2125ad1160";
pub const EMPTY_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
pub const TEST_HASHES: &[&str] = &[TEST_HASH_1, TEST_HASH_2];

pub const TEST_URL_1: &str =
    "https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium";
pub const TEST_URL_2: &str = "https://yande.re/post/show/923576";
pub const TEST_URLS: &[&str] = &[TEST_URL_1, TEST_URL_2];

pub fn get_test_hashes() -> Vec<String> {
    TEST_HASHES.iter().map(|h| String::from(*h)).collect()
}

pub fn get_test_urls() -> Vec<String> {
    TEST_URLS.iter().map(|u| String::from(*u)).collect()
}
