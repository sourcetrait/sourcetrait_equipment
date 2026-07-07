use std::{path::PathBuf};
use sourcetrait_equipment as equipment;
use sourcetrait_testing::prelude::*;
use pretty_assertions::{assert_eq};

static TESTING: testing::Module = testing::module!(Integration, {
    .using_fixture_dir()
});

#[tested]
fn test_read() {
    let test = testing::test!({
        .inherit_fixture_dir()
    });

    let expected: equipment::Library = equipment::Library {
        name: equipment::Key::from("empower"),
        title: equipment::Title::from("Empower"),
        version: equipment::Version::from("0.0.0-4"),
        provider: equipment::Provider::GitHub(equipment::GitHubProvider),
        author: equipment::Author {
            name: equipment::Key::from("sourcetrait"),
            title: equipment::Title::from("SourceTrait"),
            email: equipment::Email::from("development@sourcetrait.com"),
        },
        description: equipment::Description {
            summary: equipment::Summary::from("Empower platform library"),
            keywords: Vec::from([
                equipment::Keyword::from("ai"),
            ]),
        },
        license: equipment::License {
            spdx: Some(equipment::Spdx::from("AGPL-3.0-or-later")),
            file: PathBuf::from("LICENSE-AGPL-3.txt"),
        },
        imports: equipment::LibraryImports {
            libraries: Vec::from([
                equipment::LibraryImport {
                    name: equipment::Key::from("equip"),
                    author: equipment::Key::from("sourcetrait"),
                    version: equipment::VersionReq::from("0"),
                    provider: equipment::Key::from("github"),
                    path: None, 
                },
            ]),
        },
    };
    
    let actual = equipment::Library::read(
            test.fixture_dir().join(equipment::LibraryToml::LIBRARY_TOML)
        ).expect("reads");

    assert_eq!(expected, actual);
}