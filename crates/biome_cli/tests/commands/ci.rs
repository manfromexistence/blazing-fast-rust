use crate::configs::{
    CONFIG_DISABLED_FORMATTER, CONFIG_DISABLED_FORMATTER_JSONC, CONFIG_FILE_SIZE_LIMIT,
    CONFIG_LINTER_DISABLED,
};
use crate::snap_test::{SnapshotPayload, assert_file_contents};
use crate::{
    CUSTOM_FORMAT_BEFORE, FORMATTED, LINT_ERROR, PARSE_ERROR, UNFORMATTED, assert_cli_snapshot,
    run_cli,
};
use biome_console::{BufferConsole, MarkupBuf};
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::{Utf8Path, Utf8PathBuf};

const INCORRECT_CODE: &str = "let a = !b || !c";

const UNFORMATTED_AND_INCORRECT: &str = "statement(    ) ; let a = !b || !c;";

const CI_CONFIGURATION: &str = r#"
{
    "formatter": {
        "enabled": true
    },
    "linter": {
        "enabled": true,
        "rules": {
            "recommended": true
        }
    }
}
"#;

#[test]
fn ci_help() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let (fs, result) = run_cli(fs, &mut console, Args::from(["ci", "--help"].as_slice()));

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_help",
        fs,
        console,
        result,
    ));
}

#[test]
fn ok() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("ci.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, FORMATTED);

    if console.out_buffer.len() != 1 {
        panic!("unexpected console content: {:#?}", console.out_buffer);
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn formatting_error() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("ci.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formatting_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_parse_error() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("ci.js");
    fs.insert(file_path.into(), PARSE_ERROR.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_parse_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_lint_error() {
    let fs = MemoryFileSystem::default();

    let file_path = Utf8Path::new("ci.js");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_lint_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_run_formatter() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8PathBuf::from("biome.json"),
        CONFIG_DISABLED_FORMATTER.as_bytes(),
    );

    let input_file = Utf8Path::new("file.js");

    fs.insert(input_file.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", input_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, input_file, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_run_formatter",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_run_formatter_biome_jsonc() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8PathBuf::from("biome.jsonc"),
        CONFIG_DISABLED_FORMATTER_JSONC.as_bytes(),
    );

    let input_file = Utf8Path::new("file.js");

    fs.insert(input_file.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", input_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, input_file, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_run_formatter_biome_jsonc",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_run_formatter_via_cli() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let input_file = Utf8Path::new("file.js");
    fs.insert(input_file.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--formatter-enabled=false", input_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, input_file, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_run_formatter_via_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_run_linter() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8PathBuf::from("biome.json"),
        CONFIG_LINTER_DISABLED.as_bytes(),
    );

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_FORMAT_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, CUSTOM_FORMAT_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_run_linter",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_run_linter_via_cli() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), UNFORMATTED_AND_INCORRECT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--linter-enabled=false", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, UNFORMATTED_AND_INCORRECT);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_run_linter_via_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_organize_imports_via_cli() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");

    let content = r#"import { lorem, foom, bar } from "foo";
import * as something from "../something";
"#;
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--assist-enabled=false", file_path.as_str()].as_slice()),
    );

    // assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, content);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_organize_imports_via_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_errors_for_all_disabled_checks() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), CI_CONFIGURATION.as_bytes());

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), UNFORMATTED_AND_INCORRECT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "ci",
                "--linter-enabled=false",
                "--formatter-enabled=false",
                "--assist-enabled=false",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, UNFORMATTED_AND_INCORRECT);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_errors_for_all_disabled_checks",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("ci.js");
    fs.insert(file_path.into(), "statement();\n".repeat(80660).as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    // Do not store the content of the file in the snapshot
    fs.remove(file_path);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_too_large",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large_config_limit() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(Utf8PathBuf::from("biome.json"), CONFIG_FILE_SIZE_LIMIT);

    let file_path = Utf8Path::new("ci.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_too_large_config_limit",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large_cli_limit() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("ci.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--files-max-size=16", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_too_large_cli_limit",
        fs,
        console,
        result,
    ));
}

#[test]
fn files_max_size_parse_error() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("ci.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--files-max-size=-1", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "files_max_size_parse_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_runs_linter_not_formatter_issue_3495() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), CONFIG_DISABLED_FORMATTER.as_bytes());

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), INCORRECT_CODE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_runs_linter_not_formatter_issue_3495",
        fs,
        console,
        result,
    ));
}

#[test]
fn max_diagnostics_default() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..60 {
        let file_path = Utf8PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, UNFORMATTED.as_bytes());
    }

    let (fs, result) = run_cli(fs, &mut console, Args::from(["ci", "src"].as_slice()));

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut diagnostic_count = 0;
    let mut filtered_messages = Vec::new();

    for msg in console.out_buffer {
        let MarkupBuf(nodes) = &msg.content;
        let is_diagnostic = nodes.iter().any(|node| {
            node.content
                .contains("File content differs from formatting output")
                || node.content.contains("format")
                || node.content.contains("lint")
                || node.content.contains("ci")
        });

        if is_diagnostic {
            diagnostic_count += 1;
        } else {
            filtered_messages.push(msg);
        }
    }

    console.out_buffer = filtered_messages;

    for i in 0..60 {
        let file_path = format!("src/file_{i}.js");
        fs.remove(Utf8Path::new(&file_path));
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "max_diagnostics_default",
        fs,
        console,
        result,
    ));

    assert_eq!(diagnostic_count, 20);
}

#[test]
fn max_diagnostics() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..60 {
        let file_path = Utf8PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, UNFORMATTED.as_bytes());
    }

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--max-diagnostics", "10", "src"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut diagnostic_count = 0;
    let mut filtered_messages = Vec::new();

    for msg in console.out_buffer {
        let MarkupBuf(nodes) = &msg.content;
        let is_diagnostic = nodes.iter().any(|node| {
            node.content
                .contains("File content differs from formatting output")
                || node.content.contains("format")
                || node.content.contains("ci")
        });

        if is_diagnostic {
            diagnostic_count += 1;
        } else {
            filtered_messages.push(msg);
        }
    }

    console.out_buffer = filtered_messages;

    for i in 0..60 {
        let file_path = format!("src/file_{i}.js");
        fs.remove(Utf8Path::new(&file_path));
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "max_diagnostics",
        fs,
        console,
        result,
    ));

    assert_eq!(diagnostic_count, 10);
}

#[test]
fn print_verbose() {
    let fs = MemoryFileSystem::default();

    let file_path = Utf8Path::new("ci.js");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let mut console = BufferConsole::default();
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--verbose", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "print_verbose",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_formatter_linter_organize_imports() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
    "linter": {
        "enabled": true,
        "rules": {
            "recommended": true
        }
    },
    "assist": {
        "enabled": true
    }
}"#;

    let input = r#"
import { B, C } from "b.js"
import A from "a.js"


something( )
    "#;

    let expect = r#"
import { B, C } from "b.js"
import A from "a.js"


something( )
    "#;

    let file_path = Utf8Path::new("biome.json");
    fs.insert(file_path.into(), rome_json.as_bytes());

    let file_path = Utf8Path::new("file.js");
    fs.insert(file_path.into(), input.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_file_contents(&fs, file_path, expect);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_formatter_linter_organize_imports",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignores_unknown_file() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Utf8Path::new("test.txt");
    fs.insert(file_path1.into(), *b"content");

    let file_path2 = Utf8Path::new("test.js");
    fs.insert(file_path2.into(), *b"console.log('bar');\n");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "ci",
                file_path1.as_str(),
                file_path2.as_str(),
                "--files-ignore-unknown=true",
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignores_unknown_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn correctly_handles_ignored_and_not_ignored_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = Utf8Path::new("biome.json");
    fs.insert(
        configuration.into(),
        r#"{
            "linter": {
                "includes": ["**", "!/linter-ignored/**"]
            },
            "formatter": {
                "includes": ["**", "!/formatter-ignored/**"]
            },
            "files": {
                "includes": ["**", "!/globally-ignored/**"]
            }
        }"#,
    );

    let file_path1 = Utf8Path::new("/formatter-ignored/test.js");
    fs.insert(file_path1.into(), UNFORMATTED_AND_INCORRECT.as_bytes());

    let file_path2 = Utf8Path::new("/linter-ignored/test.js");
    fs.insert(file_path2.into(), INCORRECT_CODE.as_bytes());

    let file_path3 = Utf8Path::new("/globally-ignored/test.js");
    fs.insert(file_path3.into(), UNFORMATTED_AND_INCORRECT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "ci",
                file_path1.as_str(),
                file_path2.as_str(),
                file_path3.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "correctly_handles_ignored_and_not_ignored_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn doesnt_error_if_no_files_were_processed() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--no-errors-on-unmatched", "file.js"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "doesnt_error_if_no_files_were_processed",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_error_with_only_warnings() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();

    let file_path = Utf8Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"
{
    "formatter": { "enabled": false},
  "linter": {
    "rules": {
        "recommended": true,
        "suspicious": {
            "noClassAssign": "warn"
        }
    }
  }
}
        "#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.js");
    fs.insert(
        file_path.into(),
        r#"class A {};
A = 0;
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--error-on-warnings", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_error_with_only_warnings",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_formatting_error_without_file_paths() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("ci.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(fs, &mut console, Args::from(["ci", ""].as_slice()));

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_formatting_error_without_file_paths",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_error_if_unchanged_files_only_with_changed_flag() {
    let mut console = BufferConsole::default();
    let fs = MemoryFileSystem::default();
    // Unchanged
    fs.insert(
        Utf8Path::new("file1.js").into(),
        r#"console.log('file1');"#.as_bytes(),
    );
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--changed", "--since=main"].as_slice()),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_error_if_unchanged_files_only_with_changed_flag",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_skip_parse_errors() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let valid = Utf8Path::new("valid.js");
    let invalid = Utf8Path::new("invalid.js");

    fs.insert(valid.into(), LINT_ERROR.as_bytes());
    fs.insert(invalid.into(), PARSE_ERROR.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "ci",
                "--skip-parse-errors",
                valid.as_str(),
                invalid.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_skip_parse_errors",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_enable_assist() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file = Utf8Path::new("file.js");

    fs.insert(
        file.into(),
        "import z from \"zod\"; \n import foo from \"foo\";".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["ci", "--assist-enabled=false", file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_enable_assist",
        fs,
        console,
        result,
    ));
}
