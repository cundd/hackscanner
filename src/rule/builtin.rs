use super::Rule;
use severity::Severity;

pub fn get_builtin_rules() -> Vec<Rule> {
    let mut collection = vec![];

    collection.append(&mut get_builtin_rules_php());

    collection
}

fn get_builtin_rules_php() -> Vec<Rule> {
    vec![
        Rule::with_path_and_content(1, Severity::NOTICE, "\\.php", "error_reporting\\("),
        Rule::with_path_and_content(2, Severity::NOTICE, "\\.php", "='preg_"),
        Rule::with_path_and_content(3, Severity::NOTICE, "\\.php", "=\"preg_"),
        Rule::with_path_and_content(4, Severity::NOTICE, "\\.php", "\\beval\\("),
        Rule::with_path_and_content(5, Severity::NOTICE, "\\.php", "\\bgzinflate\\("),
        Rule::with_path_and_content(6, Severity::NOTICE, "\\.php", "\\bsystem\\("),
        Rule::with_path_and_content(7, Severity::NOTICE, "\\.php", "\\bexec\\("),
        Rule::with_path_and_content(8, Severity::NOTICE, "\\.php", "\\bcreate_function\\("),
        Rule::with_path_and_content(9, Severity::MAJOR, "\\.php", "\",\"\\.\");return;?>"),
        Rule::with_path_and_content(10, Severity::MAJOR, "\\.php", "eval\\(base64_decode\\("),
        Rule::with_path_and_content(11, Severity::CRITICAL, "\\.php", "6fbcb8b698317491a5fd7926f2c3b7de"),
        Rule::with_path_and_content(12, Severity::CRITICAL, "\\.php", "Codz by angel\\(4ngel\\)"),
        Rule::with_path_and_content(13, Severity::CRITICAL, "\\.php", "dezmond"),
        Rule::with_path_and_content(14, Severity::CRITICAL, "\\.php", "FilesMan"),
        Rule::with_path_and_content(15, Severity::CRITICAL, "\\.php", "raprap1"),
        Rule::with_path(20, Severity::CRITICAL, "tx_mocfilemanager.php"),
        Rule::with_path(21, Severity::CRITICAL, "//cache\\.dat"),
        Rule::with_path(22, Severity::CRITICAL, "\\.cache\\.php"),
        Rule::with_path(23, Severity::CRITICAL, "ext_fpdf.php"),
        Rule::with_path(24, Severity::CRITICAL, "ixwstat.php"),
    ]
}
