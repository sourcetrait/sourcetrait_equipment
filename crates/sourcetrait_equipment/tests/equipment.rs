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

    let expected: equipment::Equipment = equipment::Equipment {
        key: equipment::BiKey::fixed(("sourcetrait", "empower")),
        title: equipment::Title::fixed("SourceTrait Empower"),
        version: equipment::Version::fixed((0,0,0, 2)),
        repositories: Vec::from([
            equipment::RepositorySet {
                key: equipment::TriKey::fixed(("git", "sourcetrait", "empower")),
                read: Some(equipment::GitRepository {
                    uri: equipment::GitUri::fixed("https://github.com/sourcetrait/empower.git"),
                    reference: equipment::GitReference::fixed("dev"),
                }),
                write: Some(equipment::GitRepository {
                    uri: equipment::GitUri::fixed("ssh://git@github.com/sourcetrait/empower.git"),
                    reference: equipment::GitReference::fixed("dev"),
                }),
                store: None,
            }.into(),
        ]),
        author: equipment::Author {
            name: equipment::Key::fixed("sourcetrait"),
            title: equipment::Title::fixed("SourceTrait"),
            email: equipment::Email::fixed("development@sourcetrait.com"),
        },
        details: equipment::Details {
            summary: equipment::Summary::fixed("Empower rig"),
            keywords: Vec::from([
                equipment::Keyword::fixed("ai"),
            ]),
        },
        license: equipment::License {
            spdx: Some(equipment::Spdx::fixed("AGPL-3.0-or-later")),
            file: PathBuf::from("LICENSE-AGPL-3.txt"),
        },
        exports: Vec::from([
            equipment::RigExport::fixed(("sourcetrait", "ant")).into(),
            equipment::RigExport::fixed(("sourcetrait", "drone")).into(),
            equipment::RigExport::fixed(("sourcetrait", "equip")).into(),
            equipment::RigExport::fixed(("sourcetrait", "empower")).into(),
            equipment::RigExport::fixed(("sourcetrait", "fae")).into(),
            equipment::RigExport::fixed(("sourcetrait", "queen")).into(),
        ]),
        support: equipment::EquipmentSupport {
            nushell: equipment::VersionReq::fixed("0.113"),
            equipment: equipment::VersionReq::fixed("0"),
        }, 
    };
    
    let actual = equipment::Equipment::read_toml(
        test.fixture_dir().join(equipment::EquipmentToml::EQUIPMENT_TOML)
    ).annotated().expect("TOML");

    assert_eq!(expected, actual);
}