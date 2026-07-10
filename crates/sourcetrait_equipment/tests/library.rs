use std::{path::PathBuf};
use sourcetrait_equipment::{self as equipment, prelude::*};
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

    let expected: equipment::Rig = equipment::Rig {
        key: equipment::Key::from("empower"),
        title: equipment::Title::from("SourceTrait Empower"),
        version: equipment::Version::from("0.0.0-4"),
        author: equipment::Author {
            name: equipment::Key::from("sourcetrait"),
            title: equipment::Title::from("SourceTrait"),
            email: equipment::Email::from("development@sourcetrait.com"),
        },
        details: equipment::Details {
            summary: equipment::Summary::from("Empower platform library"),
            keywords: Vec::from([
                equipment::Keyword::from("ai"),
            ]),
        },
        license: equipment::License {
            spdx: Some(equipment::Spdx::from("AGPL-3.0-or-later")),
            file: PathBuf::from("LICENSE-AGPL-3.txt"),
        },
        imports: equipment::RigImports {
            libraries: Vec::from([
                equipment::BiKey::from(("sourcetrait", "equipment"))
            ]),
        },
    };
    
    let actual = equipment::Rig::read(
            test.fixture_dir().join(equipment::LibraryToml::LIBRARY_TOML)
        ).annotated().expect("TOML");

    assert_eq!(expected, actual);
}