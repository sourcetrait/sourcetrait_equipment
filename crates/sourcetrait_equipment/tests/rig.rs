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
        key: equipment::BiKey::fixed(("sourcetrait", "empower")),
        title: equipment::Title::fixed("SourceTrait Empower"),
        version: equipment::Version::fixed((0,0,0, 4)),
        author: equipment::Author {
            name: equipment::Key::fixed("sourcetrait"),
            title: equipment::Title::fixed("SourceTrait"),
            email: equipment::Email::fixed("development@sourcetrait.com"),
        },
        details: equipment::Details {
            summary: equipment::Summary::fixed("Empower platform library"),
            keywords: Vec::from([
                equipment::Keyword::fixed("ai"),
            ]),
        },
        license: equipment::License {
            spdx: Some(equipment::Spdx::fixed("AGPL-3.0-or-later")),
            file: PathBuf::from("LICENSE-AGPL-3.txt"),
        },
        imports: Vec::from([
            equipment::RigImport {
                key: equipment::BiKey::fixed(("sourcetrait", "equipment")),
                repository: equipment::TriKey::fixed(("git", "sourcetrait", "equipment")),
                version: equipment::VersionReq::fixed("0"),
            }.into(),
        ]),
    };
    
    let actual = equipment::Rig::read_toml(
            test.fixture_dir().join(equipment::RigToml::RIG_TOML)
        ).annotated().expect("TOML");

    assert_eq!(expected, actual);
}