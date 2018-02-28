use super::Rule;
use severity::Severity;

pub fn get_builtin_rules() -> Vec<Rule> {
    let mut collection = vec![];

    collection.append(&mut get_builtin_rules_php());

    collection
}

fn get_builtin_rules_php() -> Vec<Rule> {
    vec![
        Rule::with_path_and_content("php::content::error_reporting", Severity::NOTICE, "\\.php", "error_reporting\\("),
        Rule::with_path_and_content("php::content::preg_", Severity::NOTICE, "\\.php", "='preg_"),
        Rule::with_path_and_content("php::content::preg_", Severity::NOTICE, "\\.php", "=\"preg_"),
        Rule::with_path_and_content("php::content::beval", Severity::NOTICE, "\\.php", "\\beval\\("),
        Rule::with_path_and_content("php::content::bgzinflate", Severity::NOTICE, "\\.php", "\\bgzinflate\\("),
        Rule::with_path_and_content("php::content::bsystem", Severity::NOTICE, "\\.php", "\\bsystem\\("),
        Rule::with_path_and_content("php::content::bexec", Severity::NOTICE, "\\.php", "\\bexec\\("),
        Rule::with_path_and_content("php::content::bcreate_function", Severity::NOTICE, "\\.php", "\\bcreate_function\\("),
        Rule::with_path_and_content("php::content::return", Severity::MAJOR, "\\.php", "\",\"\\.\");return;?>"),
        Rule::with_path_and_content("php::content::base64_decode", Severity::MAJOR, "\\.php", "eval\\(base64_decode\\("),
        Rule::with_path_and_content("php::content::6fbcb8b698317491a5fd7926f2c3b7de", Severity::CRITICAL, "\\.php", "6fbcb8b698317491a5fd7926f2c3b7de"),
        Rule::with_path_and_content("php::content::4ngel", Severity::CRITICAL, "\\.php", "Codz by angel\\(4ngel\\)"),
        Rule::with_path_and_content("php::content::dezmond", Severity::CRITICAL, "\\.php", "dezmond"),
        Rule::with_path_and_content("php::content::FilesMan", Severity::CRITICAL, "\\.php", "FilesMan"),
        Rule::with_path_and_content("php::content::raprap1", Severity::CRITICAL, "\\.php", "raprap1"),
        Rule::with_path("php::file::tx_mocfilemanager", Severity::CRITICAL, "tx_mocfilemanager.php"),
        Rule::with_path("php::file::cache", Severity::CRITICAL, "//cache\\.dat"),
        Rule::with_path("php::file::cache", Severity::CRITICAL, "//\\.cache\\.php"),
        Rule::with_path("php::file::ext_fpdf", Severity::CRITICAL, "ext_fpdf.php"),
        Rule::with_path("php::file::ixwstat", Severity::CRITICAL, "ixwstat.php"),
    ]
}
