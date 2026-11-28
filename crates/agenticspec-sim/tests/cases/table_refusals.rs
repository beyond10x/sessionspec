//! The settings table refuses, when it is read, an entry its own description does not allow: a
//! bound above the other, a narrowed port no contract has, a key of another type, an enum without
//! words. Each case pairs a valid entry (accepted, so the head is sound) with the entry that must
//! be refused.

use agenticspec_sim::settings;

fn table_with(entry: &str) -> String {
    format!(
        "format: agenticspec-settings/1\nevery_part: []\nblocks:\n  - kind: k\n    domain: agentic.k\n    settings:\n      - {entry}\n"
    )
}

#[test]
fn a_bound_whose_minimum_is_above_its_maximum_is_refused_by_the_table() {
    settings::parse(&table_with("{name: a, type: integer, min: 1, max: 5}"))
        .expect("a valid bound parses");
    match settings::parse(&table_with("{name: a, type: integer, min: 5, max: 1}")) {
        Err(e) => assert!(e.contains("min") || e.contains("max"), "{e}"),
        Ok(_) => panic!("min 5 above max 1 is accepted; every value of `a` is then refused"),
    }
}

#[test]
fn a_narrowed_port_that_no_port_of_the_contract_has_is_refused_by_the_table() {
    settings::parse(&table_with(
        "{name: a, type: integer, narrows: [gateway.call]}",
    ))
    .expect("a port of the contract parses");
    match settings::parse(&table_with(
        "{name: a, type: integer, narrows: [gateway.cal]}",
    )) {
        Err(e) => assert!(e.contains("gateway.cal"), "{e}"),
        Ok(_) => panic!("a narrowed port `gateway.cal` that no contract has is accepted"),
    }
}

#[test]
fn min_and_max_are_refused_on_a_type_that_does_not_take_them() {
    settings::parse(&table_with("{name: a, type: boolean}")).expect("a boolean parses");
    match settings::parse(&table_with("{name: a, type: boolean, min: 5}")) {
        Err(e) => assert!(e.contains("min"), "{e}"),
        Ok(_) => panic!(
            "min on a boolean is accepted; the table takes bounds on number and integer only"
        ),
    }
}

#[test]
fn an_enum_whose_values_are_not_words_is_refused_by_the_table() {
    settings::parse(&table_with("{name: a, type: enum, values: [x, y]}"))
        .expect("an enum of words parses");
    match settings::parse(&table_with("{name: a, type: enum, values: [1, 2]}")) {
        Err(e) => assert!(e.contains("values") || e.contains("enum"), "{e}"),
        Ok(_) => panic!(
            "an enum of numbers parses with no variant; the table then refuses every value of `a`"
        ),
    }
}

#[test]
fn a_narrowed_item_that_is_not_a_port_name_is_refused_by_the_table() {
    settings::parse(&table_with(
        "{name: a, type: integer, narrows: [gateway.call]}",
    ))
    .expect("a port name parses");
    match settings::parse(&table_with("{name: a, type: integer, narrows: [3]}")) {
        Err(e) => assert!(e.contains("narrows"), "{e}"),
        Ok(_) => {
            panic!("narrows [3] parses and narrows no port: the item is dropped without a word")
        }
    }
}

/// Each key that only some types take, on a type that does not take it, and each such key with a
/// value of the wrong kind; beside each, the same key where it belongs.
#[test]
fn a_key_of_another_type_is_refused_by_the_table() {
    for fine in [
        "{name: a, type: number, unit: share, min: 0, max: 1}",
        "{name: a, type: duration, min: 1, max: 1d}",
        "{name: a, type: string, list: true}",
        "{name: a, type: part, list: false}",
        "{name: a, type: enum, values: [x]}",
    ] {
        settings::parse(&table_with(fine)).unwrap_or_else(|e| panic!("{fine}: {e}"));
    }
    for (entry, key) in [
        ("{name: a, type: string, min: 1}", "min"),
        ("{name: a, type: enum, values: [x], max: 1}", "max"),
        ("{name: a, type: boolean, unit: slots}", "unit"),
        ("{name: a, type: duration, unit: slots}", "unit"),
        ("{name: a, type: integer, list: true}", "list"),
        ("{name: a, type: string, list: yes}", "list"),
        ("{name: a, type: integer, values: [x]}", "values"),
        (
            "{name: a, type: string, enum: agentic.shared.Sharing}",
            "enum",
        ),
        ("{name: a, type: number, unit: 3}", "unit"),
        ("{name: a, type: enum, values: []}", "values"),
        ("{name: a, type: enum, values: [x, x]}", "values"),
    ] {
        match settings::parse(&table_with(entry)) {
            Err(e) => assert!(e.contains(key), "{entry}: {e}"),
            Ok(_) => panic!("{entry} is accepted"),
        }
    }
}

/// The committed table holds every rule above.
#[test]
fn the_committed_table_parses() {
    settings::parse(settings::SOURCE).unwrap_or_else(|e| panic!("blocks/settings.yaml: {e}"));
}
