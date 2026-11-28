// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`
//! Every generated declaration, as JSON, in the renderings the published wire contracts fix.
//!
//! Generated from the model beside the types it crosses, so a field renamed in the specification
//! is renamed here in the same regeneration. An absent optional field is omitted rather
//! than sent as `null`, which is what the `required` list of the published schema says.

use crate::json;

/// Writes `agenticspec.cli.AgentScope` as JSON.
pub fn encode_agenticspec_cli_agent_scope(value: &agenticspec_types::cli::AgentScope, out: &mut String) {
    match value {
        agenticspec_types::cli::AgentScope::Main => json::push_text(out, "main"),
        agenticspec_types::cli::AgentScope::Sub => json::push_text(out, "sub"),
    }
}

/// Reads `agenticspec.cli.AgentScope` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_agent_scope(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::AgentScope, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `main`, `sub`")? {
        "main" => agenticspec_types::cli::AgentScope::Main,
        "sub" => agenticspec_types::cli::AgentScope::Sub,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `main`, `sub`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.cli.AnalyzeInput` as JSON.
pub fn encode_agenticspec_cli_analyze_input(value: &agenticspec_types::cli::AnalyzeInput, out: &mut String) {
    out.push('{');
    if let Some(held0) = &value.root {
        json::member(out, "root");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.since_hours {
        json::member(out, "since_hours");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.out {
        json::member(out, "out");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.AnalyzeInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_analyze_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::AnalyzeInput, json::DecodeError> {
    json::closed_at(value, at, &["root", "since_hours", "out"])?;
    Ok(agenticspec_types::cli::AnalyzeInput {
        root: match value.member("root") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "root");
                Some(json::text_at(member0, &at0, "a string")?.to_owned())
            }
        },
        since_hours: match value.member("since_hours") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "since_hours");
                Some(json::integer_at(member1, &at1, "an integer")?)
            }
        },
        out: match value.member("out") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "out");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.AnalyzeResult` as JSON.
pub fn encode_agenticspec_cli_analyze_result(value: &agenticspec_types::cli::AnalyzeResult, out: &mut String) {
    out.push('{');
    json::member(out, "path");
    json::push_text(out, &value.path);
    json::member(out, "sessions");
    json::push_integer(out, value.sessions);
    out.push('}');
}

/// Reads `agenticspec.cli.AnalyzeResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_analyze_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::AnalyzeResult, json::DecodeError> {
    json::closed_at(value, at, &["path", "sessions"])?;
    Ok(agenticspec_types::cli::AnalyzeResult {
        path: {
            let at0 = json::nested(at, "path");
            let member0 = json::member_at(value, at, "path")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        sessions: {
            let at1 = json::nested(at, "sessions");
            let member1 = json::member_at(value, at, "sessions")?;
            json::integer_at(member1, &at1, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.AutonomyFigures` as JSON.
pub fn encode_agenticspec_cli_autonomy_figures(value: &agenticspec_types::cli::AutonomyFigures, out: &mut String) {
    out.push('{');
    json::member(out, "working_seconds");
    json::push_integer(out, value.working_seconds);
    json::member(out, "waiting_on_human_seconds");
    json::push_integer(out, value.waiting_on_human_seconds);
    json::member(out, "away_seconds");
    json::push_integer(out, value.away_seconds);
    json::member(out, "prompts");
    out.push('[');
    for (index0, item0) in value.prompts.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_prompt_figures(&*item0, out);
    }
    out.push(']');
    json::member(out, "decisions");
    out.push('[');
    for (index0, item0) in value.decisions.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_decision_figures(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.AutonomyFigures` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_autonomy_figures(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::AutonomyFigures, json::DecodeError> {
    json::closed_at(value, at, &["working_seconds", "waiting_on_human_seconds", "away_seconds", "prompts", "decisions"])?;
    Ok(agenticspec_types::cli::AutonomyFigures {
        working_seconds: {
            let at0 = json::nested(at, "working_seconds");
            let member0 = json::member_at(value, at, "working_seconds")?;
            json::integer_at(member0, &at0, "an integer")?
        },
        waiting_on_human_seconds: {
            let at1 = json::nested(at, "waiting_on_human_seconds");
            let member1 = json::member_at(value, at, "waiting_on_human_seconds")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        away_seconds: {
            let at2 = json::nested(at, "away_seconds");
            let member2 = json::member_at(value, at, "away_seconds")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        prompts: {
            let at3 = json::nested(at, "prompts");
            let member3 = json::member_at(value, at, "prompts")?;
            {
                let mut items3 = Vec::new();
                for (index3, element3) in json::items_at(member3, &at3, "an array")?.iter().enumerate() {
                    let nested3 = json::nested(&at3, &index3.to_string());
                    items3.push(decode_agenticspec_cli_prompt_figures(element3, &nested3)?);
                }
                items3
            }
        },
        decisions: {
            let at4 = json::nested(at, "decisions");
            let member4 = json::member_at(value, at, "decisions")?;
            {
                let mut items4 = Vec::new();
                for (index4, element4) in json::items_at(member4, &at4, "an array")?.iter().enumerate() {
                    let nested4 = json::nested(&at4, &index4.to_string());
                    items4.push(decode_agenticspec_cli_decision_figures(element4, &nested4)?);
                }
                items4
            }
        },
    })
}

/// Writes `agenticspec.cli.BlocksLiftInput` as JSON.
pub fn encode_agenticspec_cli_blocks_lift_input(value: &agenticspec_types::cli::BlocksLiftInput, out: &mut String) {
    out.push('{');
    json::member(out, "transcript");
    json::push_text(out, &value.transcript);
    out.push('}');
}

/// Reads `agenticspec.cli.BlocksLiftInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_blocks_lift_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::BlocksLiftInput, json::DecodeError> {
    json::closed_at(value, at, &["transcript"])?;
    Ok(agenticspec_types::cli::BlocksLiftInput {
        transcript: {
            let at0 = json::nested(at, "transcript");
            let member0 = json::member_at(value, at, "transcript")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.BlocksLiftResult` as JSON.
pub fn encode_agenticspec_cli_blocks_lift_result(value: &agenticspec_types::cli::BlocksLiftResult, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    json::push_text(out, &value.session_id);
    json::member(out, "given");
    json::push_value(out, &value.given);
    json::member(out, "entities");
    out.push('[');
    for (index0, item0) in value.entities.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_entity_rows(&*item0, out);
    }
    out.push(']');
    json::member(out, "not_observable");
    out.push('[');
    for (index0, item0) in value.not_observable.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.BlocksLiftResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_blocks_lift_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::BlocksLiftResult, json::DecodeError> {
    json::closed_at(value, at, &["session_id", "given", "entities", "not_observable"])?;
    Ok(agenticspec_types::cli::BlocksLiftResult {
        session_id: {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        given: {
            let at1 = json::nested(at, "given");
            let member1 = json::member_at(value, at, "given")?;
            json::value_at(member1, &at1)
        },
        entities: {
            let at2 = json::nested(at, "entities");
            let member2 = json::member_at(value, at, "entities")?;
            {
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(decode_agenticspec_cli_entity_rows(element2, &nested2)?);
                }
                items2
            }
        },
        not_observable: {
            let at3 = json::nested(at, "not_observable");
            let member3 = json::member_at(value, at, "not_observable")?;
            {
                let mut items3 = Vec::new();
                for (index3, element3) in json::items_at(member3, &at3, "an array")?.iter().enumerate() {
                    let nested3 = json::nested(&at3, &index3.to_string());
                    items3.push(json::text_at(element3, &nested3, "a string")?.to_owned());
                }
                items3
            }
        },
    })
}

/// Writes `agenticspec.cli.BlocksScenarioInput` as JSON.
pub fn encode_agenticspec_cli_blocks_scenario_input(value: &agenticspec_types::cli::BlocksScenarioInput, out: &mut String) {
    out.push('{');
    json::member(out, "transcript");
    json::push_text(out, &value.transcript);
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "profile");
    json::push_text(out, &value.profile);
    json::member(out, "map");
    json::push_text(out, &value.map);
    json::member(out, "out");
    json::push_text(out, &value.out);
    if let Some(held0) = &value.name {
        json::member(out, "name");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.BlocksScenarioInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_blocks_scenario_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::BlocksScenarioInput, json::DecodeError> {
    json::closed_at(value, at, &["transcript", "architecture", "profile", "map", "out", "name"])?;
    Ok(agenticspec_types::cli::BlocksScenarioInput {
        transcript: {
            let at0 = json::nested(at, "transcript");
            let member0 = json::member_at(value, at, "transcript")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        architecture: {
            let at1 = json::nested(at, "architecture");
            let member1 = json::member_at(value, at, "architecture")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        profile: {
            let at2 = json::nested(at, "profile");
            let member2 = json::member_at(value, at, "profile")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        map: {
            let at3 = json::nested(at, "map");
            let member3 = json::member_at(value, at, "map")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        out: {
            let at4 = json::nested(at, "out");
            let member4 = json::member_at(value, at, "out")?;
            json::text_at(member4, &at4, "a string")?.to_owned()
        },
        name: match value.member("name") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "name");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.BlocksScenarioResult` as JSON.
pub fn encode_agenticspec_cli_blocks_scenario_result(value: &agenticspec_types::cli::BlocksScenarioResult, out: &mut String) {
    out.push('{');
    json::member(out, "out");
    json::push_text(out, &value.out);
    json::member(out, "scenario");
    json::push_text(out, &value.scenario);
    json::member(out, "inputs");
    json::push_integer(out, value.inputs);
    json::member(out, "expectations");
    json::push_integer(out, value.expectations);
    json::member(out, "unjoined");
    out.push('[');
    for (index0, item0) in value.unjoined.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "unmapped");
    out.push('[');
    for (index0, item0) in value.unmapped.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.BlocksScenarioResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_blocks_scenario_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::BlocksScenarioResult, json::DecodeError> {
    json::closed_at(value, at, &["out", "scenario", "inputs", "expectations", "unjoined", "unmapped"])?;
    Ok(agenticspec_types::cli::BlocksScenarioResult {
        out: {
            let at0 = json::nested(at, "out");
            let member0 = json::member_at(value, at, "out")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        scenario: {
            let at1 = json::nested(at, "scenario");
            let member1 = json::member_at(value, at, "scenario")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        inputs: {
            let at2 = json::nested(at, "inputs");
            let member2 = json::member_at(value, at, "inputs")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        expectations: {
            let at3 = json::nested(at, "expectations");
            let member3 = json::member_at(value, at, "expectations")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        unjoined: {
            let at4 = json::nested(at, "unjoined");
            let member4 = json::member_at(value, at, "unjoined")?;
            {
                let mut items4 = Vec::new();
                for (index4, element4) in json::items_at(member4, &at4, "an array")?.iter().enumerate() {
                    let nested4 = json::nested(&at4, &index4.to_string());
                    items4.push(json::text_at(element4, &nested4, "a string")?.to_owned());
                }
                items4
            }
        },
        unmapped: {
            let at5 = json::nested(at, "unmapped");
            let member5 = json::member_at(value, at, "unmapped")?;
            {
                let mut items5 = Vec::new();
                for (index5, element5) in json::items_at(member5, &at5, "an array")?.iter().enumerate() {
                    let nested5 = json::nested(&at5, &index5.to_string());
                    items5.push(json::text_at(element5, &nested5, "a string")?.to_owned());
                }
                items5
            }
        },
    })
}

/// Writes `agenticspec.cli.BustCause` as JSON.
pub fn encode_agenticspec_cli_bust_cause(value: &agenticspec_types::cli::BustCause, out: &mut String) {
    match value {
        agenticspec_types::cli::BustCause::TtlExpired => json::push_text(out, "ttl_expired"),
        agenticspec_types::cli::BustCause::ModelChanged => json::push_text(out, "model_changed"),
        agenticspec_types::cli::BustCause::Compacted => json::push_text(out, "compacted"),
        agenticspec_types::cli::BustCause::Unexplained => json::push_text(out, "unexplained"),
    }
}

/// Reads `agenticspec.cli.BustCause` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_bust_cause(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::BustCause, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `ttl_expired`, `model_changed`, `compacted`, `unexplained`")? {
        "ttl_expired" => agenticspec_types::cli::BustCause::TtlExpired,
        "model_changed" => agenticspec_types::cli::BustCause::ModelChanged,
        "compacted" => agenticspec_types::cli::BustCause::Compacted,
        "unexplained" => agenticspec_types::cli::BustCause::Unexplained,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `ttl_expired`, `model_changed`, `compacted`, `unexplained`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.cli.BustFigures` as JSON.
pub fn encode_agenticspec_cli_bust_figures(value: &agenticspec_types::cli::BustFigures, out: &mut String) {
    out.push('{');
    json::member(out, "scope");
    encode_agenticspec_cli_agent_scope(&value.scope, out);
    json::member(out, "cause");
    encode_agenticspec_cli_bust_cause(&value.cause, out);
    json::member(out, "count");
    json::push_integer(out, value.count);
    json::member(out, "tokens_written_again");
    json::push_integer(out, value.tokens_written_again);
    out.push('}');
}

/// Reads `agenticspec.cli.BustFigures` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_bust_figures(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::BustFigures, json::DecodeError> {
    json::closed_at(value, at, &["scope", "cause", "count", "tokens_written_again"])?;
    Ok(agenticspec_types::cli::BustFigures {
        scope: {
            let at0 = json::nested(at, "scope");
            let member0 = json::member_at(value, at, "scope")?;
            decode_agenticspec_cli_agent_scope(member0, &at0)?
        },
        cause: {
            let at1 = json::nested(at, "cause");
            let member1 = json::member_at(value, at, "cause")?;
            decode_agenticspec_cli_bust_cause(member1, &at1)?
        },
        count: {
            let at2 = json::nested(at, "count");
            let member2 = json::member_at(value, at, "count")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        tokens_written_again: {
            let at3 = json::nested(at, "tokens_written_again");
            let member3 = json::member_at(value, at, "tokens_written_again")?;
            json::integer_at(member3, &at3, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.CacheFigures` as JSON.
pub fn encode_agenticspec_cli_cache_figures(value: &agenticspec_types::cli::CacheFigures, out: &mut String) {
    out.push('{');
    json::member(out, "scope");
    encode_agenticspec_cli_agent_scope(&value.scope, out);
    json::member(out, "read_tokens");
    json::push_integer(out, value.read_tokens);
    json::member(out, "write_tokens");
    json::push_integer(out, value.write_tokens);
    json::member(out, "uncached_tokens");
    json::push_integer(out, value.uncached_tokens);
    json::member(out, "hit_permille");
    json::push_integer(out, value.hit_permille);
    out.push('}');
}

/// Reads `agenticspec.cli.CacheFigures` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_cache_figures(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::CacheFigures, json::DecodeError> {
    json::closed_at(value, at, &["scope", "read_tokens", "write_tokens", "uncached_tokens", "hit_permille"])?;
    Ok(agenticspec_types::cli::CacheFigures {
        scope: {
            let at0 = json::nested(at, "scope");
            let member0 = json::member_at(value, at, "scope")?;
            decode_agenticspec_cli_agent_scope(member0, &at0)?
        },
        read_tokens: {
            let at1 = json::nested(at, "read_tokens");
            let member1 = json::member_at(value, at, "read_tokens")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        write_tokens: {
            let at2 = json::nested(at, "write_tokens");
            let member2 = json::member_at(value, at, "write_tokens")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        uncached_tokens: {
            let at3 = json::nested(at, "uncached_tokens");
            let member3 = json::member_at(value, at, "uncached_tokens")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        hit_permille: {
            let at4 = json::nested(at, "hit_permille");
            let member4 = json::member_at(value, at, "hit_permille")?;
            json::integer_at(member4, &at4, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.Certainty` as JSON.
pub fn encode_agenticspec_cli_certainty(value: &agenticspec_types::cli::Certainty, out: &mut String) {
    match value {
        agenticspec_types::cli::Certainty::Definite => json::push_text(out, "definite"),
        agenticspec_types::cli::Certainty::Possible => json::push_text(out, "possible"),
    }
}

/// Reads `agenticspec.cli.Certainty` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_certainty(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::Certainty, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `definite`, `possible`")? {
        "definite" => agenticspec_types::cli::Certainty::Definite,
        "possible" => agenticspec_types::cli::Certainty::Possible,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `definite`, `possible`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.cli.CheckFigures` as JSON.
pub fn encode_agenticspec_cli_check_figures(value: &agenticspec_types::cli::CheckFigures, out: &mut String) {
    out.push('{');
    json::member(out, "check");
    json::push_text(out, &value.check);
    json::member(out, "about");
    json::push_text(out, &value.about);
    json::member(out, "severity");
    json::push_text(out, &value.severity);
    json::member(out, "ok");
    json::push_integer(out, value.ok);
    json::member(out, "gap");
    json::push_integer(out, value.gap);
    json::member(out, "unk");
    json::push_integer(out, value.unk);
    out.push('}');
}

/// Reads `agenticspec.cli.CheckFigures` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_check_figures(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::CheckFigures, json::DecodeError> {
    json::closed_at(value, at, &["check", "about", "severity", "ok", "gap", "unk"])?;
    Ok(agenticspec_types::cli::CheckFigures {
        check: {
            let at0 = json::nested(at, "check");
            let member0 = json::member_at(value, at, "check")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        about: {
            let at1 = json::nested(at, "about");
            let member1 = json::member_at(value, at, "about")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        severity: {
            let at2 = json::nested(at, "severity");
            let member2 = json::member_at(value, at, "severity")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        ok: {
            let at3 = json::nested(at, "ok");
            let member3 = json::member_at(value, at, "ok")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        gap: {
            let at4 = json::nested(at, "gap");
            let member4 = json::member_at(value, at, "gap")?;
            json::integer_at(member4, &at4, "an integer")?
        },
        unk: {
            let at5 = json::nested(at, "unk");
            let member5 = json::member_at(value, at, "unk")?;
            json::integer_at(member5, &at5, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.CheckVerdict` as JSON.
pub fn encode_agenticspec_cli_check_verdict(value: &agenticspec_types::cli::CheckVerdict, out: &mut String) {
    out.push('{');
    json::member(out, "check");
    json::push_text(out, &value.check);
    json::member(out, "session_id");
    json::push_text(out, &value.session_id);
    json::member(out, "verdict");
    encode_agenticspec_session_verdict(&value.verdict, out);
    json::member(out, "rows");
    json::push_integer(out, value.rows);
    out.push('}');
}

/// Reads `agenticspec.cli.CheckVerdict` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_check_verdict(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::CheckVerdict, json::DecodeError> {
    json::closed_at(value, at, &["check", "session_id", "verdict", "rows"])?;
    Ok(agenticspec_types::cli::CheckVerdict {
        check: {
            let at0 = json::nested(at, "check");
            let member0 = json::member_at(value, at, "check")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        session_id: {
            let at1 = json::nested(at, "session_id");
            let member1 = json::member_at(value, at, "session_id")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        verdict: {
            let at2 = json::nested(at, "verdict");
            let member2 = json::member_at(value, at, "verdict")?;
            decode_agenticspec_session_verdict(member2, &at2)?
        },
        rows: {
            let at3 = json::nested(at, "rows");
            let member3 = json::member_at(value, at, "rows")?;
            json::integer_at(member3, &at3, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.ConcurrencyFigures` as JSON.
pub fn encode_agenticspec_cli_concurrency_figures(value: &agenticspec_types::cli::ConcurrencyFigures, out: &mut String) {
    out.push('{');
    json::member(out, "scope");
    encode_agenticspec_cli_agent_scope(&value.scope, out);
    json::member(out, "p50");
    json::push_integer(out, value.p50);
    json::member(out, "p90");
    json::push_integer(out, value.p90);
    json::member(out, "max");
    json::push_integer(out, value.max);
    out.push('}');
}

/// Reads `agenticspec.cli.ConcurrencyFigures` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_concurrency_figures(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::ConcurrencyFigures, json::DecodeError> {
    json::closed_at(value, at, &["scope", "p50", "p90", "max"])?;
    Ok(agenticspec_types::cli::ConcurrencyFigures {
        scope: {
            let at0 = json::nested(at, "scope");
            let member0 = json::member_at(value, at, "scope")?;
            decode_agenticspec_cli_agent_scope(member0, &at0)?
        },
        p50: {
            let at1 = json::nested(at, "p50");
            let member1 = json::member_at(value, at, "p50")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        p90: {
            let at2 = json::nested(at, "p90");
            let member2 = json::member_at(value, at, "p90")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        max: {
            let at3 = json::nested(at, "max");
            let member3 = json::member_at(value, at, "max")?;
            json::integer_at(member3, &at3, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.CorpusAction` as JSON.
pub fn encode_agenticspec_cli_corpus_action(value: &agenticspec_types::cli::CorpusAction, out: &mut String) {
    match value {
        agenticspec_types::cli::CorpusAction::Run => json::push_text(out, "run"),
    }
}

/// Reads `agenticspec.cli.CorpusAction` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_corpus_action(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::CorpusAction, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `run`")? {
        "run" => agenticspec_types::cli::CorpusAction::Run,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `run`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.cli.Dashboard` as JSON.
pub fn encode_agenticspec_cli_dashboard(value: &agenticspec_types::cli::Dashboard, out: &mut String) {
    out.push('{');
    json::member(out, "generated_at");
    json::push_text(out, &value.generated_at);
    json::member(out, "overall");
    encode_agenticspec_cli_kpi_report(&value.overall, out);
    json::member(out, "sessions");
    out.push('[');
    for (index0, item0) in value.sessions.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_session_summary(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.Dashboard` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_dashboard(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::Dashboard, json::DecodeError> {
    json::closed_at(value, at, &["generated_at", "overall", "sessions"])?;
    Ok(agenticspec_types::cli::Dashboard {
        generated_at: {
            let at0 = json::nested(at, "generated_at");
            let member0 = json::member_at(value, at, "generated_at")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        overall: {
            let at1 = json::nested(at, "overall");
            let member1 = json::member_at(value, at, "overall")?;
            decode_agenticspec_cli_kpi_report(member1, &at1)?
        },
        sessions: {
            let at2 = json::nested(at, "sessions");
            let member2 = json::member_at(value, at, "sessions")?;
            {
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(decode_agenticspec_cli_session_summary(element2, &nested2)?);
                }
                items2
            }
        },
    })
}

/// Writes `agenticspec.cli.DecisionFigures` as JSON.
pub fn encode_agenticspec_cli_decision_figures(value: &agenticspec_types::cli::DecisionFigures, out: &mut String) {
    out.push('{');
    json::member(out, "decider");
    encode_agenticspec_session_decider(&value.decider, out);
    json::member(out, "decision");
    encode_agenticspec_session_decision(&value.decision, out);
    json::member(out, "count");
    json::push_integer(out, value.count);
    out.push('}');
}

/// Reads `agenticspec.cli.DecisionFigures` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_decision_figures(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::DecisionFigures, json::DecodeError> {
    json::closed_at(value, at, &["decider", "decision", "count"])?;
    Ok(agenticspec_types::cli::DecisionFigures {
        decider: {
            let at0 = json::nested(at, "decider");
            let member0 = json::member_at(value, at, "decider")?;
            decode_agenticspec_session_decider(member0, &at0)?
        },
        decision: {
            let at1 = json::nested(at, "decision");
            let member1 = json::member_at(value, at, "decision")?;
            decode_agenticspec_session_decision(member1, &at1)?
        },
        count: {
            let at2 = json::nested(at, "count");
            let member2 = json::member_at(value, at, "count")?;
            json::integer_at(member2, &at2, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.DeployBundleInput` as JSON.
pub fn encode_agenticspec_cli_deploy_bundle_input(value: &agenticspec_types::cli::DeployBundleInput, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "binding");
    json::push_text(out, &value.binding);
    if let Some(held0) = &value.out {
        json::member(out, "out");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.ports {
        json::member(out, "ports");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.accepts {
        json::member(out, "accepts");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.kpis {
        json::member(out, "kpis");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.DeployBundleInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_deploy_bundle_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::DeployBundleInput, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "binding", "out", "ports", "accepts", "kpis"])?;
    Ok(agenticspec_types::cli::DeployBundleInput {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        binding: {
            let at1 = json::nested(at, "binding");
            let member1 = json::member_at(value, at, "binding")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        out: match value.member("out") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "out");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
        ports: match value.member("ports") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "ports");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        accepts: match value.member("accepts") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "accepts");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
        kpis: match value.member("kpis") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "kpis");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.DeployBundleResult` as JSON.
pub fn encode_agenticspec_cli_deploy_bundle_result(value: &agenticspec_types::cli::DeployBundleResult, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "binding");
    json::push_text(out, &value.binding);
    json::member(out, "digest");
    json::push_text(out, &value.digest);
    json::member(out, "units");
    json::push_integer(out, value.units);
    json::member(out, "written");
    json::push_bool(out, value.written);
    if let Some(held0) = &value.out {
        json::member(out, "out");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.bundle {
        json::member(out, "bundle");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.DeployBundleResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_deploy_bundle_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::DeployBundleResult, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "binding", "digest", "units", "written", "out", "bundle"])?;
    Ok(agenticspec_types::cli::DeployBundleResult {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        binding: {
            let at1 = json::nested(at, "binding");
            let member1 = json::member_at(value, at, "binding")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        digest: {
            let at2 = json::nested(at, "digest");
            let member2 = json::member_at(value, at, "digest")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        units: {
            let at3 = json::nested(at, "units");
            let member3 = json::member_at(value, at, "units")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        written: {
            let at4 = json::nested(at, "written");
            let member4 = json::member_at(value, at, "written")?;
            json::bool_at(member4, &at4, "a boolean")?
        },
        out: match value.member("out") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "out");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
        bundle: match value.member("bundle") {
            None | Some(json::Value::Null) => None,
            Some(member6) => {
                let at6 = json::nested(at, "bundle");
                Some(json::text_at(member6, &at6, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.DeployCheckInput` as JSON.
pub fn encode_agenticspec_cli_deploy_check_input(value: &agenticspec_types::cli::DeployCheckInput, out: &mut String) {
    out.push('{');
    json::member(out, "bundle");
    json::push_text(out, &value.bundle);
    if let Some(held0) = &value.accepts {
        json::member(out, "accepts");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.DeployCheckInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_deploy_check_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::DeployCheckInput, json::DecodeError> {
    json::closed_at(value, at, &["bundle", "accepts"])?;
    Ok(agenticspec_types::cli::DeployCheckInput {
        bundle: {
            let at0 = json::nested(at, "bundle");
            let member0 = json::member_at(value, at, "bundle")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        accepts: match value.member("accepts") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "accepts");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.DeployCheckResult` as JSON.
pub fn encode_agenticspec_cli_deploy_check_result(value: &agenticspec_types::cli::DeployCheckResult, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "binding");
    json::push_text(out, &value.binding);
    json::member(out, "digest");
    json::push_text(out, &value.digest);
    json::member(out, "units");
    json::push_integer(out, value.units);
    out.push('}');
}

/// Reads `agenticspec.cli.DeployCheckResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_deploy_check_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::DeployCheckResult, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "binding", "digest", "units"])?;
    Ok(agenticspec_types::cli::DeployCheckResult {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        binding: {
            let at1 = json::nested(at, "binding");
            let member1 = json::member_at(value, at, "binding")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        digest: {
            let at2 = json::nested(at, "digest");
            let member2 = json::member_at(value, at, "digest")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        units: {
            let at3 = json::nested(at, "units");
            let member3 = json::member_at(value, at, "units")?;
            json::integer_at(member3, &at3, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.EntityRows` as JSON.
pub fn encode_agenticspec_cli_entity_rows(value: &agenticspec_types::cli::EntityRows, out: &mut String) {
    out.push('{');
    json::member(out, "entity");
    json::push_text(out, &value.entity);
    json::member(out, "rows");
    json::push_integer(out, value.rows);
    out.push('}');
}

/// Reads `agenticspec.cli.EntityRows` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_entity_rows(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::EntityRows, json::DecodeError> {
    json::closed_at(value, at, &["entity", "rows"])?;
    Ok(agenticspec_types::cli::EntityRows {
        entity: {
            let at0 = json::nested(at, "entity");
            let member0 = json::member_at(value, at, "entity")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        rows: {
            let at1 = json::nested(at, "rows");
            let member1 = json::member_at(value, at, "rows")?;
            json::integer_at(member1, &at1, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.ExportInput` as JSON.
pub fn encode_agenticspec_cli_export_input(value: &agenticspec_types::cli::ExportInput, out: &mut String) {
    out.push('{');
    json::member(out, "procedure");
    json::push_text(out, &value.procedure);
    json::member(out, "out");
    json::push_text(out, &value.out);
    out.push('}');
}

/// Reads `agenticspec.cli.ExportInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_export_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::ExportInput, json::DecodeError> {
    json::closed_at(value, at, &["procedure", "out"])?;
    Ok(agenticspec_types::cli::ExportInput {
        procedure: {
            let at0 = json::nested(at, "procedure");
            let member0 = json::member_at(value, at, "procedure")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        out: {
            let at1 = json::nested(at, "out");
            let member1 = json::member_at(value, at, "out")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.ExportResult` as JSON.
pub fn encode_agenticspec_cli_export_result(value: &agenticspec_types::cli::ExportResult, out: &mut String) {
    out.push('{');
    json::member(out, "protocol");
    json::push_text(out, &value.protocol);
    json::member(out, "bindings");
    json::push_text(out, &value.bindings);
    json::member(out, "validated");
    json::push_bool(out, value.validated);
    json::member(out, "detail");
    json::push_text(out, &value.detail);
    out.push('}');
}

/// Reads `agenticspec.cli.ExportResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_export_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::ExportResult, json::DecodeError> {
    json::closed_at(value, at, &["protocol", "bindings", "validated", "detail"])?;
    Ok(agenticspec_types::cli::ExportResult {
        protocol: {
            let at0 = json::nested(at, "protocol");
            let member0 = json::member_at(value, at, "protocol")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        bindings: {
            let at1 = json::nested(at, "bindings");
            let member1 = json::member_at(value, at, "bindings")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        validated: {
            let at2 = json::nested(at, "validated");
            let member2 = json::member_at(value, at, "validated")?;
            json::bool_at(member2, &at2, "a boolean")?
        },
        detail: {
            let at3 = json::nested(at, "detail");
            let member3 = json::member_at(value, at, "detail")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.ExposureFigures` as JSON.
pub fn encode_agenticspec_cli_exposure_figures(value: &agenticspec_types::cli::ExposureFigures, out: &mut String) {
    out.push('{');
    json::member(out, "channel");
    encode_agenticspec_session_exposure_channel(&value.channel, out);
    json::member(out, "class");
    encode_agenticspec_session_exposure_class(&value.class, out);
    json::member(out, "kind");
    json::push_text(out, &value.kind);
    json::member(out, "verdict");
    encode_agenticspec_session_verdict(&value.verdict, out);
    json::member(out, "count");
    json::push_integer(out, value.count);
    json::member(out, "sessions");
    json::push_integer(out, value.sessions);
    out.push('}');
}

/// Reads `agenticspec.cli.ExposureFigures` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_exposure_figures(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::ExposureFigures, json::DecodeError> {
    json::closed_at(value, at, &["channel", "class", "kind", "verdict", "count", "sessions"])?;
    Ok(agenticspec_types::cli::ExposureFigures {
        channel: {
            let at0 = json::nested(at, "channel");
            let member0 = json::member_at(value, at, "channel")?;
            decode_agenticspec_session_exposure_channel(member0, &at0)?
        },
        class: {
            let at1 = json::nested(at, "class");
            let member1 = json::member_at(value, at, "class")?;
            decode_agenticspec_session_exposure_class(member1, &at1)?
        },
        kind: {
            let at2 = json::nested(at, "kind");
            let member2 = json::member_at(value, at, "kind")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        verdict: {
            let at3 = json::nested(at, "verdict");
            let member3 = json::member_at(value, at, "verdict")?;
            decode_agenticspec_session_verdict(member3, &at3)?
        },
        count: {
            let at4 = json::nested(at, "count");
            let member4 = json::member_at(value, at, "count")?;
            json::integer_at(member4, &at4, "an integer")?
        },
        sessions: {
            let at5 = json::nested(at, "sessions");
            let member5 = json::member_at(value, at, "sessions")?;
            json::integer_at(member5, &at5, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.IngressDefaultInput` as JSON.
pub fn encode_agenticspec_cli_ingress_default_input(value: &agenticspec_types::cli::IngressDefaultInput, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    out.push('}');
}

/// Reads `agenticspec.cli.IngressDefaultInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ingress_default_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::IngressDefaultInput, json::DecodeError> {
    json::closed_at(value, at, &["architecture"])?;
    Ok(agenticspec_types::cli::IngressDefaultInput {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.IngressDefaultResult` as JSON.
pub fn encode_agenticspec_cli_ingress_default_result(value: &agenticspec_types::cli::IngressDefaultResult, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "defaults");
    out.push('[');
    for (index0, item0) in value.defaults.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "ingress");
    json::push_text(out, &value.ingress);
    out.push('}');
}

/// Reads `agenticspec.cli.IngressDefaultResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ingress_default_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::IngressDefaultResult, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "defaults", "ingress"])?;
    Ok(agenticspec_types::cli::IngressDefaultResult {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        defaults: {
            let at1 = json::nested(at, "defaults");
            let member1 = json::member_at(value, at, "defaults")?;
            {
                let mut items1 = Vec::new();
                for (index1, element1) in json::items_at(member1, &at1, "an array")?.iter().enumerate() {
                    let nested1 = json::nested(&at1, &index1.to_string());
                    items1.push(json::text_at(element1, &nested1, "a string")?.to_owned());
                }
                items1
            }
        },
        ingress: {
            let at2 = json::nested(at, "ingress");
            let member2 = json::member_at(value, at, "ingress")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.IngressProjectInput` as JSON.
pub fn encode_agenticspec_cli_ingress_project_input(value: &agenticspec_types::cli::IngressProjectInput, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    if let Some(held0) = &value.out {
        json::member(out, "out");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.check {
        json::member(out, "check");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.blocks {
        json::member(out, "blocks");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.ports {
        json::member(out, "ports");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.kpis {
        json::member(out, "kpis");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.IngressProjectInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ingress_project_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::IngressProjectInput, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "out", "check", "blocks", "ports", "kpis"])?;
    Ok(agenticspec_types::cli::IngressProjectInput {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        out: match value.member("out") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "out");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
        check: match value.member("check") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "check");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
        blocks: match value.member("blocks") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "blocks");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        ports: match value.member("ports") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "ports");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
        kpis: match value.member("kpis") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "kpis");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.IngressProjectResult` as JSON.
pub fn encode_agenticspec_cli_ingress_project_result(value: &agenticspec_types::cli::IngressProjectResult, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "exports");
    json::push_integer(out, value.exports);
    json::member(out, "files");
    out.push('[');
    for (index0, item0) in value.files.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "written");
    json::push_bool(out, value.written);
    out.push('}');
}

/// Reads `agenticspec.cli.IngressProjectResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ingress_project_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::IngressProjectResult, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "exports", "files", "written"])?;
    Ok(agenticspec_types::cli::IngressProjectResult {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        exports: {
            let at1 = json::nested(at, "exports");
            let member1 = json::member_at(value, at, "exports")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        files: {
            let at2 = json::nested(at, "files");
            let member2 = json::member_at(value, at, "files")?;
            {
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(json::text_at(element2, &nested2, "a string")?.to_owned());
                }
                items2
            }
        },
        written: {
            let at3 = json::nested(at, "written");
            let member3 = json::member_at(value, at, "written")?;
            json::bool_at(member3, &at3, "a boolean")?
        },
    })
}

/// Writes `agenticspec.cli.InvariantStatus` as JSON.
pub fn encode_agenticspec_cli_invariant_status(value: &agenticspec_types::cli::InvariantStatus, out: &mut String) {
    match value {
        agenticspec_types::cli::InvariantStatus::Held => json::push_text(out, "held"),
        agenticspec_types::cli::InvariantStatus::Violated => json::push_text(out, "violated"),
        agenticspec_types::cli::InvariantStatus::NotChecked => json::push_text(out, "not_checked"),
    }
}

/// Reads `agenticspec.cli.InvariantStatus` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_invariant_status(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::InvariantStatus, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `held`, `violated`, `not_checked`")? {
        "held" => agenticspec_types::cli::InvariantStatus::Held,
        "violated" => agenticspec_types::cli::InvariantStatus::Violated,
        "not_checked" => agenticspec_types::cli::InvariantStatus::NotChecked,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `held`, `violated`, `not_checked`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.cli.KpiInput` as JSON.
pub fn encode_agenticspec_cli_kpi_input(value: &agenticspec_types::cli::KpiInput, out: &mut String) {
    out.push('{');
    if let Some(held0) = &value.root {
        json::member(out, "root");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.since_hours {
        json::member(out, "since_hours");
        json::push_integer(out, *held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.KpiInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_kpi_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::KpiInput, json::DecodeError> {
    json::closed_at(value, at, &["root", "since_hours"])?;
    Ok(agenticspec_types::cli::KpiInput {
        root: match value.member("root") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "root");
                Some(json::text_at(member0, &at0, "a string")?.to_owned())
            }
        },
        since_hours: match value.member("since_hours") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "since_hours");
                Some(json::integer_at(member1, &at1, "an integer")?)
            }
        },
    })
}

/// Writes `agenticspec.cli.KpiReport` as JSON.
pub fn encode_agenticspec_cli_kpi_report(value: &agenticspec_types::cli::KpiReport, out: &mut String) {
    out.push('{');
    json::member(out, "window_from");
    json::push_text(out, &value.window_from);
    json::member(out, "window_to");
    json::push_text(out, &value.window_to);
    json::member(out, "transcripts");
    json::push_integer(out, value.transcripts);
    json::member(out, "sessions");
    json::push_integer(out, value.sessions);
    json::member(out, "sub_agents");
    json::push_integer(out, value.sub_agents);
    json::member(out, "refused_commands");
    json::push_integer(out, value.refused_commands);
    json::member(out, "cost_microdollars");
    json::push_integer(out, value.cost_microdollars);
    json::member(out, "unpriced_messages");
    json::push_integer(out, value.unpriced_messages);
    json::member(out, "models");
    out.push('[');
    for (index0, item0) in value.models.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_model_figures(&*item0, out);
    }
    out.push(']');
    json::member(out, "cache");
    out.push('[');
    for (index0, item0) in value.cache.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_cache_figures(&*item0, out);
    }
    out.push(']');
    json::member(out, "busts");
    out.push('[');
    for (index0, item0) in value.busts.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_bust_figures(&*item0, out);
    }
    out.push(']');
    json::member(out, "concurrency");
    out.push('[');
    for (index0, item0) in value.concurrency.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_concurrency_figures(&*item0, out);
    }
    out.push(']');
    json::member(out, "autonomy");
    encode_agenticspec_cli_autonomy_figures(&value.autonomy, out);
    json::member(out, "exposures");
    out.push('[');
    for (index0, item0) in value.exposures.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_exposure_figures(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.KpiReport` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_kpi_report(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::KpiReport, json::DecodeError> {
    json::closed_at(value, at, &["window_from", "window_to", "transcripts", "sessions", "sub_agents", "refused_commands", "cost_microdollars", "unpriced_messages", "models", "cache", "busts", "concurrency", "autonomy", "exposures"])?;
    Ok(agenticspec_types::cli::KpiReport {
        window_from: {
            let at0 = json::nested(at, "window_from");
            let member0 = json::member_at(value, at, "window_from")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        window_to: {
            let at1 = json::nested(at, "window_to");
            let member1 = json::member_at(value, at, "window_to")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        transcripts: {
            let at2 = json::nested(at, "transcripts");
            let member2 = json::member_at(value, at, "transcripts")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        sessions: {
            let at3 = json::nested(at, "sessions");
            let member3 = json::member_at(value, at, "sessions")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        sub_agents: {
            let at4 = json::nested(at, "sub_agents");
            let member4 = json::member_at(value, at, "sub_agents")?;
            json::integer_at(member4, &at4, "an integer")?
        },
        refused_commands: {
            let at5 = json::nested(at, "refused_commands");
            let member5 = json::member_at(value, at, "refused_commands")?;
            json::integer_at(member5, &at5, "an integer")?
        },
        cost_microdollars: {
            let at6 = json::nested(at, "cost_microdollars");
            let member6 = json::member_at(value, at, "cost_microdollars")?;
            json::integer_at(member6, &at6, "an integer")?
        },
        unpriced_messages: {
            let at7 = json::nested(at, "unpriced_messages");
            let member7 = json::member_at(value, at, "unpriced_messages")?;
            json::integer_at(member7, &at7, "an integer")?
        },
        models: {
            let at8 = json::nested(at, "models");
            let member8 = json::member_at(value, at, "models")?;
            {
                let mut items8 = Vec::new();
                for (index8, element8) in json::items_at(member8, &at8, "an array")?.iter().enumerate() {
                    let nested8 = json::nested(&at8, &index8.to_string());
                    items8.push(decode_agenticspec_cli_model_figures(element8, &nested8)?);
                }
                items8
            }
        },
        cache: {
            let at9 = json::nested(at, "cache");
            let member9 = json::member_at(value, at, "cache")?;
            {
                let mut items9 = Vec::new();
                for (index9, element9) in json::items_at(member9, &at9, "an array")?.iter().enumerate() {
                    let nested9 = json::nested(&at9, &index9.to_string());
                    items9.push(decode_agenticspec_cli_cache_figures(element9, &nested9)?);
                }
                items9
            }
        },
        busts: {
            let at10 = json::nested(at, "busts");
            let member10 = json::member_at(value, at, "busts")?;
            {
                let mut items10 = Vec::new();
                for (index10, element10) in json::items_at(member10, &at10, "an array")?.iter().enumerate() {
                    let nested10 = json::nested(&at10, &index10.to_string());
                    items10.push(decode_agenticspec_cli_bust_figures(element10, &nested10)?);
                }
                items10
            }
        },
        concurrency: {
            let at11 = json::nested(at, "concurrency");
            let member11 = json::member_at(value, at, "concurrency")?;
            {
                let mut items11 = Vec::new();
                for (index11, element11) in json::items_at(member11, &at11, "an array")?.iter().enumerate() {
                    let nested11 = json::nested(&at11, &index11.to_string());
                    items11.push(decode_agenticspec_cli_concurrency_figures(element11, &nested11)?);
                }
                items11
            }
        },
        autonomy: {
            let at12 = json::nested(at, "autonomy");
            let member12 = json::member_at(value, at, "autonomy")?;
            decode_agenticspec_cli_autonomy_figures(member12, &at12)?
        },
        exposures: {
            let at13 = json::nested(at, "exposures");
            let member13 = json::member_at(value, at, "exposures")?;
            {
                let mut items13 = Vec::new();
                for (index13, element13) in json::items_at(member13, &at13, "an array")?.iter().enumerate() {
                    let nested13 = json::nested(&at13, &index13.to_string());
                    items13.push(decode_agenticspec_cli_exposure_figures(element13, &nested13)?);
                }
                items13
            }
        },
    })
}

/// Writes `agenticspec.cli.LiftInput` as JSON.
pub fn encode_agenticspec_cli_lift_input(value: &agenticspec_types::cli::LiftInput, out: &mut String) {
    out.push('{');
    json::member(out, "transcript");
    json::push_text(out, &value.transcript);
    out.push('}');
}

/// Reads `agenticspec.cli.LiftInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_lift_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::LiftInput, json::DecodeError> {
    json::closed_at(value, at, &["transcript"])?;
    Ok(agenticspec_types::cli::LiftInput {
        transcript: {
            let at0 = json::nested(at, "transcript");
            let member0 = json::member_at(value, at, "transcript")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.LiftResult` as JSON.
pub fn encode_agenticspec_cli_lift_result(value: &agenticspec_types::cli::LiftResult, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    json::push_text(out, &value.session_id);
    json::member(out, "agents");
    json::push_integer(out, value.agents);
    json::member(out, "commands");
    out.push('[');
    for (index0, item0) in value.commands.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_lifted_command(&*item0, out);
    }
    out.push(']');
    json::member(out, "refused");
    json::push_integer(out, value.refused);
    out.push('}');
}

/// Reads `agenticspec.cli.LiftResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_lift_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::LiftResult, json::DecodeError> {
    json::closed_at(value, at, &["session_id", "agents", "commands", "refused"])?;
    Ok(agenticspec_types::cli::LiftResult {
        session_id: {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        agents: {
            let at1 = json::nested(at, "agents");
            let member1 = json::member_at(value, at, "agents")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        commands: {
            let at2 = json::nested(at, "commands");
            let member2 = json::member_at(value, at, "commands")?;
            {
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(decode_agenticspec_cli_lifted_command(element2, &nested2)?);
                }
                items2
            }
        },
        refused: {
            let at3 = json::nested(at, "refused");
            let member3 = json::member_at(value, at, "refused")?;
            json::integer_at(member3, &at3, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.LiftedCommand` as JSON.
pub fn encode_agenticspec_cli_lifted_command(value: &agenticspec_types::cli::LiftedCommand, out: &mut String) {
    out.push('{');
    json::member(out, "seq");
    json::push_integer(out, value.seq);
    json::member(out, "at");
    json::push_text(out, &value.at);
    json::member(out, "actor");
    json::push_text(out, &value.actor);
    json::member(out, "command");
    json::push_text(out, &value.command);
    json::member(out, "outcome");
    json::push_text(out, &value.outcome);
    json::member(out, "input");
    json::push_value(out, &value.input);
    out.push('}');
}

/// Reads `agenticspec.cli.LiftedCommand` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_lifted_command(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::LiftedCommand, json::DecodeError> {
    json::closed_at(value, at, &["seq", "at", "actor", "command", "outcome", "input"])?;
    Ok(agenticspec_types::cli::LiftedCommand {
        seq: {
            let at0 = json::nested(at, "seq");
            let member0 = json::member_at(value, at, "seq")?;
            json::integer_at(member0, &at0, "an integer")?
        },
        at: {
            let at1 = json::nested(at, "at");
            let member1 = json::member_at(value, at, "at")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        actor: {
            let at2 = json::nested(at, "actor");
            let member2 = json::member_at(value, at, "actor")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        command: {
            let at3 = json::nested(at, "command");
            let member3 = json::member_at(value, at, "command")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        outcome: {
            let at4 = json::nested(at, "outcome");
            let member4 = json::member_at(value, at, "outcome")?;
            json::text_at(member4, &at4, "a string")?.to_owned()
        },
        input: {
            let at5 = json::nested(at, "input");
            let member5 = json::member_at(value, at, "input")?;
            json::value_at(member5, &at5)
        },
    })
}

/// Writes `agenticspec.cli.MarkKind` as JSON.
pub fn encode_agenticspec_cli_mark_kind(value: &agenticspec_types::cli::MarkKind, out: &mut String) {
    match value {
        agenticspec_types::cli::MarkKind::Turn => json::push_text(out, "turn"),
        agenticspec_types::cli::MarkKind::HumanPrompt => json::push_text(out, "human_prompt"),
        agenticspec_types::cli::MarkKind::DelegatePrompt => json::push_text(out, "delegate_prompt"),
        agenticspec_types::cli::MarkKind::PeerPrompt => json::push_text(out, "peer_prompt"),
        agenticspec_types::cli::MarkKind::HarnessPrompt => json::push_text(out, "harness_prompt"),
        agenticspec_types::cli::MarkKind::Compaction => json::push_text(out, "compaction"),
        agenticspec_types::cli::MarkKind::CacheBust => json::push_text(out, "cache_bust"),
        agenticspec_types::cli::MarkKind::Refusal => json::push_text(out, "refusal"),
    }
}

/// Reads `agenticspec.cli.MarkKind` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_mark_kind(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::MarkKind, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `turn`, `human_prompt`, `delegate_prompt`, `peer_prompt`, `harness_prompt`, `compaction`, `cache_bust`, `refusal`")? {
        "turn" => agenticspec_types::cli::MarkKind::Turn,
        "human_prompt" => agenticspec_types::cli::MarkKind::HumanPrompt,
        "delegate_prompt" => agenticspec_types::cli::MarkKind::DelegatePrompt,
        "peer_prompt" => agenticspec_types::cli::MarkKind::PeerPrompt,
        "harness_prompt" => agenticspec_types::cli::MarkKind::HarnessPrompt,
        "compaction" => agenticspec_types::cli::MarkKind::Compaction,
        "cache_bust" => agenticspec_types::cli::MarkKind::CacheBust,
        "refusal" => agenticspec_types::cli::MarkKind::Refusal,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `turn`, `human_prompt`, `delegate_prompt`, `peer_prompt`, `harness_prompt`, `compaction`, `cache_bust`, `refusal`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.cli.MineInput` as JSON.
pub fn encode_agenticspec_cli_mine_input(value: &agenticspec_types::cli::MineInput, out: &mut String) {
    out.push('{');
    if let Some(held0) = &value.root {
        json::member(out, "root");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.since_hours {
        json::member(out, "since_hours");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.level {
        json::member(out, "level");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.min_sessions {
        json::member(out, "min_sessions");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.min_projects {
        json::member(out, "min_projects");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.top {
        json::member(out, "top");
        json::push_integer(out, *held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.MineInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_mine_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::MineInput, json::DecodeError> {
    json::closed_at(value, at, &["root", "since_hours", "level", "min_sessions", "min_projects", "top"])?;
    Ok(agenticspec_types::cli::MineInput {
        root: match value.member("root") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "root");
                Some(json::text_at(member0, &at0, "a string")?.to_owned())
            }
        },
        since_hours: match value.member("since_hours") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "since_hours");
                Some(json::integer_at(member1, &at1, "an integer")?)
            }
        },
        level: match value.member("level") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "level");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
        min_sessions: match value.member("min_sessions") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "min_sessions");
                Some(json::integer_at(member3, &at3, "an integer")?)
            }
        },
        min_projects: match value.member("min_projects") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "min_projects");
                Some(json::integer_at(member4, &at4, "an integer")?)
            }
        },
        top: match value.member("top") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "top");
                Some(json::integer_at(member5, &at5, "an integer")?)
            }
        },
    })
}

/// Writes `agenticspec.cli.MineReport` as JSON.
pub fn encode_agenticspec_cli_mine_report(value: &agenticspec_types::cli::MineReport, out: &mut String) {
    out.push('{');
    json::member(out, "format");
    json::push_text(out, &value.format);
    json::member(out, "level");
    json::push_text(out, &value.level);
    json::member(out, "sessions");
    json::push_integer(out, value.sessions);
    json::member(out, "turns");
    json::push_integer(out, value.turns);
    json::member(out, "patterns");
    out.push('[');
    for (index0, item0) in value.patterns.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_pattern(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.MineReport` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_mine_report(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::MineReport, json::DecodeError> {
    json::closed_at(value, at, &["format", "level", "sessions", "turns", "patterns"])?;
    Ok(agenticspec_types::cli::MineReport {
        format: {
            let at0 = json::nested(at, "format");
            let member0 = json::member_at(value, at, "format")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        level: {
            let at1 = json::nested(at, "level");
            let member1 = json::member_at(value, at, "level")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        sessions: {
            let at2 = json::nested(at, "sessions");
            let member2 = json::member_at(value, at, "sessions")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        turns: {
            let at3 = json::nested(at, "turns");
            let member3 = json::member_at(value, at, "turns")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        patterns: {
            let at4 = json::nested(at, "patterns");
            let member4 = json::member_at(value, at, "patterns")?;
            {
                let mut items4 = Vec::new();
                for (index4, element4) in json::items_at(member4, &at4, "an array")?.iter().enumerate() {
                    let nested4 = json::nested(&at4, &index4.to_string());
                    items4.push(decode_agenticspec_cli_pattern(element4, &nested4)?);
                }
                items4
            }
        },
    })
}

/// Writes `agenticspec.cli.ModelFigures` as JSON.
pub fn encode_agenticspec_cli_model_figures(value: &agenticspec_types::cli::ModelFigures, out: &mut String) {
    out.push('{');
    json::member(out, "scope");
    encode_agenticspec_cli_agent_scope(&value.scope, out);
    json::member(out, "model");
    json::push_text(out, &value.model);
    json::member(out, "messages");
    json::push_integer(out, value.messages);
    json::member(out, "output_tokens");
    json::push_integer(out, value.output_tokens);
    json::member(out, "context_p50");
    json::push_integer(out, value.context_p50);
    json::member(out, "context_p90");
    json::push_integer(out, value.context_p90);
    json::member(out, "context_max");
    json::push_integer(out, value.context_max);
    json::member(out, "cost_microdollars");
    json::push_integer(out, value.cost_microdollars);
    out.push('}');
}

/// Reads `agenticspec.cli.ModelFigures` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_model_figures(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::ModelFigures, json::DecodeError> {
    json::closed_at(value, at, &["scope", "model", "messages", "output_tokens", "context_p50", "context_p90", "context_max", "cost_microdollars"])?;
    Ok(agenticspec_types::cli::ModelFigures {
        scope: {
            let at0 = json::nested(at, "scope");
            let member0 = json::member_at(value, at, "scope")?;
            decode_agenticspec_cli_agent_scope(member0, &at0)?
        },
        model: {
            let at1 = json::nested(at, "model");
            let member1 = json::member_at(value, at, "model")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        messages: {
            let at2 = json::nested(at, "messages");
            let member2 = json::member_at(value, at, "messages")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        output_tokens: {
            let at3 = json::nested(at, "output_tokens");
            let member3 = json::member_at(value, at, "output_tokens")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        context_p50: {
            let at4 = json::nested(at, "context_p50");
            let member4 = json::member_at(value, at, "context_p50")?;
            json::integer_at(member4, &at4, "an integer")?
        },
        context_p90: {
            let at5 = json::nested(at, "context_p90");
            let member5 = json::member_at(value, at, "context_p90")?;
            json::integer_at(member5, &at5, "an integer")?
        },
        context_max: {
            let at6 = json::nested(at, "context_max");
            let member6 = json::member_at(value, at, "context_max")?;
            json::integer_at(member6, &at6, "an integer")?
        },
        cost_microdollars: {
            let at7 = json::nested(at, "cost_microdollars");
            let member7 = json::member_at(value, at, "cost_microdollars")?;
            json::integer_at(member7, &at7, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.PackingDefaultInput` as JSON.
pub fn encode_agenticspec_cli_packing_default_input(value: &agenticspec_types::cli::PackingDefaultInput, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    if let Some(held0) = &value.strategy {
        json::member(out, "strategy");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.PackingDefaultInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_packing_default_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::PackingDefaultInput, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "strategy"])?;
    Ok(agenticspec_types::cli::PackingDefaultInput {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        strategy: match value.member("strategy") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "strategy");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.PackingDefaultResult` as JSON.
pub fn encode_agenticspec_cli_packing_default_result(value: &agenticspec_types::cli::PackingDefaultResult, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "strategy");
    json::push_text(out, &value.strategy);
    json::member(out, "units");
    json::push_integer(out, value.units);
    json::member(out, "packing");
    json::push_text(out, &value.packing);
    out.push('}');
}

/// Reads `agenticspec.cli.PackingDefaultResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_packing_default_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::PackingDefaultResult, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "strategy", "units", "packing"])?;
    Ok(agenticspec_types::cli::PackingDefaultResult {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        strategy: {
            let at1 = json::nested(at, "strategy");
            let member1 = json::member_at(value, at, "strategy")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        units: {
            let at2 = json::nested(at, "units");
            let member2 = json::member_at(value, at, "units")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        packing: {
            let at3 = json::nested(at, "packing");
            let member3 = json::member_at(value, at, "packing")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.Pattern` as JSON.
pub fn encode_agenticspec_cli_pattern(value: &agenticspec_types::cli::Pattern, out: &mut String) {
    out.push('{');
    json::member(out, "id");
    json::push_text(out, &value.id);
    json::member(out, "tokens");
    out.push('[');
    for (index0, item0) in value.tokens.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "sessions");
    json::push_integer(out, value.sessions);
    json::member(out, "projects");
    json::push_integer(out, value.projects);
    json::member(out, "occurrences");
    json::push_integer(out, value.occurrences);
    json::member(out, "ok_permille");
    json::push_integer(out, value.ok_permille);
    json::member(out, "query");
    json::push_text(out, &value.query);
    out.push('}');
}

/// Reads `agenticspec.cli.Pattern` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_pattern(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::Pattern, json::DecodeError> {
    json::closed_at(value, at, &["id", "tokens", "sessions", "projects", "occurrences", "ok_permille", "query"])?;
    Ok(agenticspec_types::cli::Pattern {
        id: {
            let at0 = json::nested(at, "id");
            let member0 = json::member_at(value, at, "id")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        tokens: {
            let at1 = json::nested(at, "tokens");
            let member1 = json::member_at(value, at, "tokens")?;
            {
                let mut items1 = Vec::new();
                for (index1, element1) in json::items_at(member1, &at1, "an array")?.iter().enumerate() {
                    let nested1 = json::nested(&at1, &index1.to_string());
                    items1.push(json::text_at(element1, &nested1, "a string")?.to_owned());
                }
                items1
            }
        },
        sessions: {
            let at2 = json::nested(at, "sessions");
            let member2 = json::member_at(value, at, "sessions")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        projects: {
            let at3 = json::nested(at, "projects");
            let member3 = json::member_at(value, at, "projects")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        occurrences: {
            let at4 = json::nested(at, "occurrences");
            let member4 = json::member_at(value, at, "occurrences")?;
            json::integer_at(member4, &at4, "an integer")?
        },
        ok_permille: {
            let at5 = json::nested(at, "ok_permille");
            let member5 = json::member_at(value, at, "ok_permille")?;
            json::integer_at(member5, &at5, "an integer")?
        },
        query: {
            let at6 = json::nested(at, "query");
            let member6 = json::member_at(value, at, "query")?;
            json::text_at(member6, &at6, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.Procedure` as JSON.
pub fn encode_agenticspec_cli_procedure(value: &agenticspec_types::cli::Procedure, out: &mut String) {
    out.push('{');
    json::member(out, "format");
    json::push_text(out, &value.format);
    json::member(out, "id");
    json::push_text(out, &value.id);
    json::member(out, "description");
    json::push_text(out, &value.description);
    json::member(out, "pattern");
    out.push('[');
    for (index0, item0) in value.pattern.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "matches");
    json::push_integer(out, value.matches);
    json::member(out, "sessions");
    out.push('[');
    for (index0, item0) in value.sessions.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "parameters");
    out.push('[');
    for (index0, item0) in value.parameters.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_procedure_parameter(&*item0, out);
    }
    out.push(']');
    json::member(out, "steps");
    out.push('[');
    for (index0, item0) in value.steps.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_procedure_step(&*item0, out);
    }
    out.push(']');
    json::member(out, "query");
    json::push_text(out, &value.query);
    out.push('}');
}

/// Reads `agenticspec.cli.Procedure` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_procedure(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::Procedure, json::DecodeError> {
    json::closed_at(value, at, &["format", "id", "description", "pattern", "matches", "sessions", "parameters", "steps", "query"])?;
    Ok(agenticspec_types::cli::Procedure {
        format: {
            let at0 = json::nested(at, "format");
            let member0 = json::member_at(value, at, "format")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        id: {
            let at1 = json::nested(at, "id");
            let member1 = json::member_at(value, at, "id")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        description: {
            let at2 = json::nested(at, "description");
            let member2 = json::member_at(value, at, "description")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        pattern: {
            let at3 = json::nested(at, "pattern");
            let member3 = json::member_at(value, at, "pattern")?;
            {
                let mut items3 = Vec::new();
                for (index3, element3) in json::items_at(member3, &at3, "an array")?.iter().enumerate() {
                    let nested3 = json::nested(&at3, &index3.to_string());
                    items3.push(json::text_at(element3, &nested3, "a string")?.to_owned());
                }
                items3
            }
        },
        matches: {
            let at4 = json::nested(at, "matches");
            let member4 = json::member_at(value, at, "matches")?;
            json::integer_at(member4, &at4, "an integer")?
        },
        sessions: {
            let at5 = json::nested(at, "sessions");
            let member5 = json::member_at(value, at, "sessions")?;
            {
                let mut items5 = Vec::new();
                for (index5, element5) in json::items_at(member5, &at5, "an array")?.iter().enumerate() {
                    let nested5 = json::nested(&at5, &index5.to_string());
                    items5.push(json::text_at(element5, &nested5, "a string")?.to_owned());
                }
                items5
            }
        },
        parameters: {
            let at6 = json::nested(at, "parameters");
            let member6 = json::member_at(value, at, "parameters")?;
            {
                let mut items6 = Vec::new();
                for (index6, element6) in json::items_at(member6, &at6, "an array")?.iter().enumerate() {
                    let nested6 = json::nested(&at6, &index6.to_string());
                    items6.push(decode_agenticspec_cli_procedure_parameter(element6, &nested6)?);
                }
                items6
            }
        },
        steps: {
            let at7 = json::nested(at, "steps");
            let member7 = json::member_at(value, at, "steps")?;
            {
                let mut items7 = Vec::new();
                for (index7, element7) in json::items_at(member7, &at7, "an array")?.iter().enumerate() {
                    let nested7 = json::nested(&at7, &index7.to_string());
                    items7.push(decode_agenticspec_cli_procedure_step(element7, &nested7)?);
                }
                items7
            }
        },
        query: {
            let at8 = json::nested(at, "query");
            let member8 = json::member_at(value, at, "query")?;
            json::text_at(member8, &at8, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.ProcedureInput` as JSON.
pub fn encode_agenticspec_cli_procedure_input(value: &agenticspec_types::cli::ProcedureInput, out: &mut String) {
    out.push('{');
    json::member(out, "pattern");
    json::push_text(out, &value.pattern);
    if let Some(held0) = &value.root {
        json::member(out, "root");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.since_hours {
        json::member(out, "since_hours");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.level {
        json::member(out, "level");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.out {
        json::member(out, "out");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.ProcedureInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_procedure_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::ProcedureInput, json::DecodeError> {
    json::closed_at(value, at, &["pattern", "root", "since_hours", "level", "out"])?;
    Ok(agenticspec_types::cli::ProcedureInput {
        pattern: {
            let at0 = json::nested(at, "pattern");
            let member0 = json::member_at(value, at, "pattern")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        root: match value.member("root") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "root");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
        since_hours: match value.member("since_hours") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "since_hours");
                Some(json::integer_at(member2, &at2, "an integer")?)
            }
        },
        level: match value.member("level") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "level");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        out: match value.member("out") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "out");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.ProcedureParameter` as JSON.
pub fn encode_agenticspec_cli_procedure_parameter(value: &agenticspec_types::cli::ProcedureParameter, out: &mut String) {
    out.push('{');
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "step");
    json::push_text(out, &value.step);
    json::member(out, "kind");
    json::push_text(out, &value.kind);
    json::member(out, "values");
    out.push('[');
    for (index0, item0) in value.values.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.ProcedureParameter` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_procedure_parameter(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::ProcedureParameter, json::DecodeError> {
    json::closed_at(value, at, &["name", "step", "kind", "values"])?;
    Ok(agenticspec_types::cli::ProcedureParameter {
        name: {
            let at0 = json::nested(at, "name");
            let member0 = json::member_at(value, at, "name")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        step: {
            let at1 = json::nested(at, "step");
            let member1 = json::member_at(value, at, "step")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        kind: {
            let at2 = json::nested(at, "kind");
            let member2 = json::member_at(value, at, "kind")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        values: {
            let at3 = json::nested(at, "values");
            let member3 = json::member_at(value, at, "values")?;
            {
                let mut items3 = Vec::new();
                for (index3, element3) in json::items_at(member3, &at3, "an array")?.iter().enumerate() {
                    let nested3 = json::nested(&at3, &index3.to_string());
                    items3.push(json::text_at(element3, &nested3, "a string")?.to_owned());
                }
                items3
            }
        },
    })
}

/// Writes `agenticspec.cli.ProcedureStep` as JSON.
pub fn encode_agenticspec_cli_procedure_step(value: &agenticspec_types::cli::ProcedureStep, out: &mut String) {
    out.push('{');
    json::member(out, "id");
    json::push_text(out, &value.id);
    json::member(out, "kind");
    encode_agenticspec_cli_step_kind(&value.kind, out);
    json::member(out, "token");
    json::push_text(out, &value.token);
    json::member(out, "tool");
    json::push_text(out, &value.tool);
    json::member(out, "category");
    json::push_text(out, &value.category);
    if let Some(held0) = &value.shape {
        json::member(out, "shape");
        json::push_text(out, &*held0);
    }
    json::member(out, "effect");
    json::push_text(out, &value.effect);
    json::member(out, "after");
    out.push('[');
    for (index0, item0) in value.after.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "retry_max");
    json::push_integer(out, value.retry_max);
    json::member(out, "timeout_seconds");
    json::push_integer(out, value.timeout_seconds);
    json::member(out, "ok_permille");
    json::push_integer(out, value.ok_permille);
    out.push('}');
}

/// Reads `agenticspec.cli.ProcedureStep` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_procedure_step(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::ProcedureStep, json::DecodeError> {
    json::closed_at(value, at, &["id", "kind", "token", "tool", "category", "shape", "effect", "after", "retry_max", "timeout_seconds", "ok_permille"])?;
    Ok(agenticspec_types::cli::ProcedureStep {
        id: {
            let at0 = json::nested(at, "id");
            let member0 = json::member_at(value, at, "id")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        kind: {
            let at1 = json::nested(at, "kind");
            let member1 = json::member_at(value, at, "kind")?;
            decode_agenticspec_cli_step_kind(member1, &at1)?
        },
        token: {
            let at2 = json::nested(at, "token");
            let member2 = json::member_at(value, at, "token")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        tool: {
            let at3 = json::nested(at, "tool");
            let member3 = json::member_at(value, at, "tool")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        category: {
            let at4 = json::nested(at, "category");
            let member4 = json::member_at(value, at, "category")?;
            json::text_at(member4, &at4, "a string")?.to_owned()
        },
        shape: match value.member("shape") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "shape");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
        effect: {
            let at6 = json::nested(at, "effect");
            let member6 = json::member_at(value, at, "effect")?;
            json::text_at(member6, &at6, "a string")?.to_owned()
        },
        after: {
            let at7 = json::nested(at, "after");
            let member7 = json::member_at(value, at, "after")?;
            {
                let mut items7 = Vec::new();
                for (index7, element7) in json::items_at(member7, &at7, "an array")?.iter().enumerate() {
                    let nested7 = json::nested(&at7, &index7.to_string());
                    items7.push(json::text_at(element7, &nested7, "a string")?.to_owned());
                }
                items7
            }
        },
        retry_max: {
            let at8 = json::nested(at, "retry_max");
            let member8 = json::member_at(value, at, "retry_max")?;
            json::integer_at(member8, &at8, "an integer")?
        },
        timeout_seconds: {
            let at9 = json::nested(at, "timeout_seconds");
            let member9 = json::member_at(value, at, "timeout_seconds")?;
            json::integer_at(member9, &at9, "an integer")?
        },
        ok_permille: {
            let at10 = json::nested(at, "ok_permille");
            let member10 = json::member_at(value, at, "ok_permille")?;
            json::integer_at(member10, &at10, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.PromptFigures` as JSON.
pub fn encode_agenticspec_cli_prompt_figures(value: &agenticspec_types::cli::PromptFigures, out: &mut String) {
    out.push('{');
    json::member(out, "author");
    encode_agenticspec_session_prompt_author(&value.author, out);
    json::member(out, "attribution");
    encode_agenticspec_session_attribution(&value.attribution, out);
    json::member(out, "count");
    json::push_integer(out, value.count);
    out.push('}');
}

/// Reads `agenticspec.cli.PromptFigures` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_prompt_figures(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::PromptFigures, json::DecodeError> {
    json::closed_at(value, at, &["author", "attribution", "count"])?;
    Ok(agenticspec_types::cli::PromptFigures {
        author: {
            let at0 = json::nested(at, "author");
            let member0 = json::member_at(value, at, "author")?;
            decode_agenticspec_session_prompt_author(member0, &at0)?
        },
        attribution: {
            let at1 = json::nested(at, "attribution");
            let member1 = json::member_at(value, at, "attribution")?;
            decode_agenticspec_session_attribution(member1, &at1)?
        },
        count: {
            let at2 = json::nested(at, "count");
            let member2 = json::member_at(value, at, "count")?;
            json::integer_at(member2, &at2, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.QueryInput` as JSON.
pub fn encode_agenticspec_cli_query_input(value: &agenticspec_types::cli::QueryInput, out: &mut String) {
    out.push('{');
    if let Some(held0) = &value.module {
        json::member(out, "module");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.text {
        json::member(out, "text");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.transcript {
        json::member(out, "transcript");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.root {
        json::member(out, "root");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.since_hours {
        json::member(out, "since_hours");
        json::push_integer(out, *held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.QueryInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_query_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::QueryInput, json::DecodeError> {
    json::closed_at(value, at, &["module", "text", "transcript", "root", "since_hours"])?;
    Ok(agenticspec_types::cli::QueryInput {
        module: match value.member("module") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "module");
                Some(json::text_at(member0, &at0, "a string")?.to_owned())
            }
        },
        text: match value.member("text") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "text");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
        transcript: match value.member("transcript") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "transcript");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
        root: match value.member("root") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "root");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        since_hours: match value.member("since_hours") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "since_hours");
                Some(json::integer_at(member4, &at4, "an integer")?)
            }
        },
    })
}

/// Writes `agenticspec.cli.QueryReport` as JSON.
pub fn encode_agenticspec_cli_query_report(value: &agenticspec_types::cli::QueryReport, out: &mut String) {
    out.push('{');
    json::member(out, "format");
    json::push_text(out, &value.format);
    json::member(out, "sessions");
    json::push_integer(out, value.sessions);
    json::member(out, "queries");
    out.push('[');
    for (index0, item0) in value.queries.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "rows");
    out.push('[');
    for (index0, item0) in value.rows.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_query_row(&*item0, out);
    }
    out.push(']');
    json::member(out, "checks");
    out.push('[');
    for (index0, item0) in value.checks.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_check_figures(&*item0, out);
    }
    out.push(']');
    json::member(out, "verdicts");
    out.push('[');
    for (index0, item0) in value.verdicts.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_check_verdict(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.QueryReport` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_query_report(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::QueryReport, json::DecodeError> {
    json::closed_at(value, at, &["format", "sessions", "queries", "rows", "checks", "verdicts"])?;
    Ok(agenticspec_types::cli::QueryReport {
        format: {
            let at0 = json::nested(at, "format");
            let member0 = json::member_at(value, at, "format")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        sessions: {
            let at1 = json::nested(at, "sessions");
            let member1 = json::member_at(value, at, "sessions")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        queries: {
            let at2 = json::nested(at, "queries");
            let member2 = json::member_at(value, at, "queries")?;
            {
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(json::text_at(element2, &nested2, "a string")?.to_owned());
                }
                items2
            }
        },
        rows: {
            let at3 = json::nested(at, "rows");
            let member3 = json::member_at(value, at, "rows")?;
            {
                let mut items3 = Vec::new();
                for (index3, element3) in json::items_at(member3, &at3, "an array")?.iter().enumerate() {
                    let nested3 = json::nested(&at3, &index3.to_string());
                    items3.push(decode_agenticspec_cli_query_row(element3, &nested3)?);
                }
                items3
            }
        },
        checks: {
            let at4 = json::nested(at, "checks");
            let member4 = json::member_at(value, at, "checks")?;
            {
                let mut items4 = Vec::new();
                for (index4, element4) in json::items_at(member4, &at4, "an array")?.iter().enumerate() {
                    let nested4 = json::nested(&at4, &index4.to_string());
                    items4.push(decode_agenticspec_cli_check_figures(element4, &nested4)?);
                }
                items4
            }
        },
        verdicts: {
            let at5 = json::nested(at, "verdicts");
            let member5 = json::member_at(value, at, "verdicts")?;
            {
                let mut items5 = Vec::new();
                for (index5, element5) in json::items_at(member5, &at5, "an array")?.iter().enumerate() {
                    let nested5 = json::nested(&at5, &index5.to_string());
                    items5.push(decode_agenticspec_cli_check_verdict(element5, &nested5)?);
                }
                items5
            }
        },
    })
}

/// Writes `agenticspec.cli.QueryRow` as JSON.
pub fn encode_agenticspec_cli_query_row(value: &agenticspec_types::cli::QueryRow, out: &mut String) {
    out.push('{');
    json::member(out, "query");
    json::push_integer(out, value.query);
    json::member(out, "session_id");
    json::push_text(out, &value.session_id);
    json::member(out, "values");
    out.push('[');
    for (index0, item0) in value.values.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "certainty");
    encode_agenticspec_cli_certainty(&value.certainty, out);
    json::member(out, "unknown");
    out.push('[');
    for (index0, item0) in value.unknown.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.QueryRow` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_query_row(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::QueryRow, json::DecodeError> {
    json::closed_at(value, at, &["query", "session_id", "values", "certainty", "unknown"])?;
    Ok(agenticspec_types::cli::QueryRow {
        query: {
            let at0 = json::nested(at, "query");
            let member0 = json::member_at(value, at, "query")?;
            json::integer_at(member0, &at0, "an integer")?
        },
        session_id: {
            let at1 = json::nested(at, "session_id");
            let member1 = json::member_at(value, at, "session_id")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        values: {
            let at2 = json::nested(at, "values");
            let member2 = json::member_at(value, at, "values")?;
            {
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(json::text_at(element2, &nested2, "a string")?.to_owned());
                }
                items2
            }
        },
        certainty: {
            let at3 = json::nested(at, "certainty");
            let member3 = json::member_at(value, at, "certainty")?;
            decode_agenticspec_cli_certainty(member3, &at3)?
        },
        unknown: {
            let at4 = json::nested(at, "unknown");
            let member4 = json::member_at(value, at, "unknown")?;
            {
                let mut items4 = Vec::new();
                for (index4, element4) in json::items_at(member4, &at4, "an array")?.iter().enumerate() {
                    let nested4 = json::nested(&at4, &index4.to_string());
                    items4.push(json::text_at(element4, &nested4, "a string")?.to_owned());
                }
                items4
            }
        },
    })
}

/// Writes `agenticspec.cli.Refusal` as JSON.
pub fn encode_agenticspec_cli_refusal(value: &agenticspec_types::cli::Refusal, out: &mut String) {
    out.push('{');
    json::member(out, "detail");
    json::push_text(out, &value.detail);
    out.push('}');
}

/// Reads `agenticspec.cli.Refusal` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_refusal(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::Refusal, json::DecodeError> {
    json::closed_at(value, at, &["detail"])?;
    Ok(agenticspec_types::cli::Refusal {
        detail: {
            let at0 = json::nested(at, "detail");
            let member0 = json::member_at(value, at, "detail")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.ScenarioAction` as JSON.
pub fn encode_agenticspec_cli_scenario_action(value: &agenticspec_types::cli::ScenarioAction, out: &mut String) {
    match value {
        agenticspec_types::cli::ScenarioAction::Check => json::push_text(out, "check"),
        agenticspec_types::cli::ScenarioAction::Run => json::push_text(out, "run"),
    }
}

/// Reads `agenticspec.cli.ScenarioAction` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_scenario_action(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::ScenarioAction, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `check`, `run`")? {
        "check" => agenticspec_types::cli::ScenarioAction::Check,
        "run" => agenticspec_types::cli::ScenarioAction::Run,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `check`, `run`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.cli.ServeInput` as JSON.
pub fn encode_agenticspec_cli_serve_input(value: &agenticspec_types::cli::ServeInput, out: &mut String) {
    out.push('{');
    if let Some(held0) = &value.root {
        json::member(out, "root");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.since_hours {
        json::member(out, "since_hours");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.port {
        json::member(out, "port");
        json::push_integer(out, *held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.ServeInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_serve_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::ServeInput, json::DecodeError> {
    json::closed_at(value, at, &["root", "since_hours", "port"])?;
    Ok(agenticspec_types::cli::ServeInput {
        root: match value.member("root") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "root");
                Some(json::text_at(member0, &at0, "a string")?.to_owned())
            }
        },
        since_hours: match value.member("since_hours") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "since_hours");
                Some(json::integer_at(member1, &at1, "an integer")?)
            }
        },
        port: match value.member("port") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "port");
                Some(json::integer_at(member2, &at2, "an integer")?)
            }
        },
    })
}

/// Writes `agenticspec.cli.ServeResult` as JSON.
pub fn encode_agenticspec_cli_serve_result(value: &agenticspec_types::cli::ServeResult, out: &mut String) {
    out.push('{');
    json::member(out, "url");
    json::push_text(out, &value.url);
    out.push('}');
}

/// Reads `agenticspec.cli.ServeResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_serve_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::ServeResult, json::DecodeError> {
    json::closed_at(value, at, &["url"])?;
    Ok(agenticspec_types::cli::ServeResult {
        url: {
            let at0 = json::nested(at, "url");
            let member0 = json::member_at(value, at, "url")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.SessionSummary` as JSON.
pub fn encode_agenticspec_cli_session_summary(value: &agenticspec_types::cli::SessionSummary, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    json::push_text(out, &value.session_id);
    json::member(out, "project");
    json::push_text(out, &value.project);
    if let Some(held0) = &value.title {
        json::member(out, "title");
        json::push_text(out, &*held0);
    }
    json::member(out, "first_at");
    json::push_text(out, &value.first_at);
    json::member(out, "last_at");
    json::push_text(out, &value.last_at);
    json::member(out, "kpis");
    encode_agenticspec_cli_kpi_report(&value.kpis, out);
    json::member(out, "tools");
    out.push('[');
    for (index0, item0) in value.tools.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_tool_figures(&*item0, out);
    }
    out.push(']');
    json::member(out, "timeline");
    out.push('[');
    for (index0, item0) in value.timeline.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_timeline_mark(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.SessionSummary` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_session_summary(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SessionSummary, json::DecodeError> {
    json::closed_at(value, at, &["session_id", "project", "title", "first_at", "last_at", "kpis", "tools", "timeline"])?;
    Ok(agenticspec_types::cli::SessionSummary {
        session_id: {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        project: {
            let at1 = json::nested(at, "project");
            let member1 = json::member_at(value, at, "project")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        title: match value.member("title") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "title");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
        first_at: {
            let at3 = json::nested(at, "first_at");
            let member3 = json::member_at(value, at, "first_at")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        last_at: {
            let at4 = json::nested(at, "last_at");
            let member4 = json::member_at(value, at, "last_at")?;
            json::text_at(member4, &at4, "a string")?.to_owned()
        },
        kpis: {
            let at5 = json::nested(at, "kpis");
            let member5 = json::member_at(value, at, "kpis")?;
            decode_agenticspec_cli_kpi_report(member5, &at5)?
        },
        tools: {
            let at6 = json::nested(at, "tools");
            let member6 = json::member_at(value, at, "tools")?;
            {
                let mut items6 = Vec::new();
                for (index6, element6) in json::items_at(member6, &at6, "an array")?.iter().enumerate() {
                    let nested6 = json::nested(&at6, &index6.to_string());
                    items6.push(decode_agenticspec_cli_tool_figures(element6, &nested6)?);
                }
                items6
            }
        },
        timeline: {
            let at7 = json::nested(at, "timeline");
            let member7 = json::member_at(value, at, "timeline")?;
            {
                let mut items7 = Vec::new();
                for (index7, element7) in json::items_at(member7, &at7, "an array")?.iter().enumerate() {
                    let nested7 = json::nested(&at7, &index7.to_string());
                    items7.push(decode_agenticspec_cli_timeline_mark(element7, &nested7)?);
                }
                items7
            }
        },
    })
}

/// Writes `agenticspec.cli.SimCheckInput` as JSON.
pub fn encode_agenticspec_cli_sim_check_input(value: &agenticspec_types::cli::SimCheckInput, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    if let Some(held0) = &value.ports {
        json::member(out, "ports");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.kpis {
        json::member(out, "kpis");
        json::push_text(out, &*held0);
    }
    json::member(out, "json");
    json::push_bool(out, value.json);
    out.push('}');
}

/// Reads `agenticspec.cli.SimCheckInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_check_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimCheckInput, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "ports", "kpis", "json"])?;
    Ok(agenticspec_types::cli::SimCheckInput {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        ports: match value.member("ports") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "ports");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
        kpis: match value.member("kpis") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "kpis");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
        json: {
            let at3 = json::nested(at, "json");
            let member3 = json::member_at(value, at, "json")?;
            json::bool_at(member3, &at3, "a boolean")?
        },
    })
}

/// Writes `agenticspec.cli.SimCheckResult` as JSON.
pub fn encode_agenticspec_cli_sim_check_result(value: &agenticspec_types::cli::SimCheckResult, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "parts");
    json::push_integer(out, value.parts);
    json::member(out, "links");
    json::push_integer(out, value.links);
    json::member(out, "kpis");
    json::push_value(out, &value.kpis);
    json::member(out, "single_points");
    out.push('[');
    for (index0, item0) in value.single_points.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "nested");
    out.push('[');
    for (index0, item0) in value.nested.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_sim_nested(&*item0, out);
    }
    out.push(']');
    json::member(out, "ingresses");
    json::push_integer(out, value.ingresses);
    json::member(out, "requirements");
    json::push_value(out, &value.requirements);
    if let Some(held0) = &value.format {
        json::member(out, "format");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.valid {
        json::member(out, "valid");
        json::push_bool(out, *held0);
    }
    if let Some(held0) = &value.violations {
        json::member(out, "violations");
        out.push('[');
        for (index1, item1) in (*held0).iter().enumerate() {
            if index1 > 0 {
                out.push(',');
            }
            encode_agenticspec_cli_sim_check_violation(&*item1, out);
        }
        out.push(']');
    }
    if let Some(held0) = &value.counts {
        json::member(out, "counts");
        out.push('{');
        for (index1, (key1, item1)) in (*held0).iter().enumerate() {
            if index1 > 0 {
                out.push(',');
            }
            json::push_text(out, key1);
            out.push(':');
            json::push_integer(out, *item1);
        }
        out.push('}');
    }
    out.push('}');
}

/// Reads `agenticspec.cli.SimCheckResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_check_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimCheckResult, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "parts", "links", "kpis", "single_points", "nested", "ingresses", "requirements", "format", "valid", "violations", "counts"])?;
    Ok(agenticspec_types::cli::SimCheckResult {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        parts: {
            let at1 = json::nested(at, "parts");
            let member1 = json::member_at(value, at, "parts")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        links: {
            let at2 = json::nested(at, "links");
            let member2 = json::member_at(value, at, "links")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        kpis: {
            let at3 = json::nested(at, "kpis");
            let member3 = json::member_at(value, at, "kpis")?;
            json::value_at(member3, &at3)
        },
        single_points: {
            let at4 = json::nested(at, "single_points");
            let member4 = json::member_at(value, at, "single_points")?;
            {
                let mut items4 = Vec::new();
                for (index4, element4) in json::items_at(member4, &at4, "an array")?.iter().enumerate() {
                    let nested4 = json::nested(&at4, &index4.to_string());
                    items4.push(json::text_at(element4, &nested4, "a string")?.to_owned());
                }
                items4
            }
        },
        nested: {
            let at5 = json::nested(at, "nested");
            let member5 = json::member_at(value, at, "nested")?;
            {
                let mut items5 = Vec::new();
                for (index5, element5) in json::items_at(member5, &at5, "an array")?.iter().enumerate() {
                    let nested5 = json::nested(&at5, &index5.to_string());
                    items5.push(decode_agenticspec_cli_sim_nested(element5, &nested5)?);
                }
                items5
            }
        },
        ingresses: {
            let at6 = json::nested(at, "ingresses");
            let member6 = json::member_at(value, at, "ingresses")?;
            json::integer_at(member6, &at6, "an integer")?
        },
        requirements: {
            let at7 = json::nested(at, "requirements");
            let member7 = json::member_at(value, at, "requirements")?;
            json::value_at(member7, &at7)
        },
        format: match value.member("format") {
            None | Some(json::Value::Null) => None,
            Some(member8) => {
                let at8 = json::nested(at, "format");
                Some(json::text_at(member8, &at8, "a string")?.to_owned())
            }
        },
        valid: match value.member("valid") {
            None | Some(json::Value::Null) => None,
            Some(member9) => {
                let at9 = json::nested(at, "valid");
                Some(json::bool_at(member9, &at9, "a boolean")?)
            }
        },
        violations: match value.member("violations") {
            None | Some(json::Value::Null) => None,
            Some(member10) => {
                let at10 = json::nested(at, "violations");
                Some({
                let mut items10 = Vec::new();
                for (index10, element10) in json::items_at(member10, &at10, "an array")?.iter().enumerate() {
                    let nested10 = json::nested(&at10, &index10.to_string());
                    items10.push(decode_agenticspec_cli_sim_check_violation(element10, &nested10)?);
                }
                items10
            })
            }
        },
        counts: match value.member("counts") {
            None | Some(json::Value::Null) => None,
            Some(member11) => {
                let at11 = json::nested(at, "counts");
                Some({
                let mut entries11 = std::collections::BTreeMap::new();
                for (key11, element11) in json::members_at(member11, &at11, "an object")? {
                    let nested11 = json::nested(&at11, key11);
                    entries11.insert(key11.clone(), json::integer_at(element11, &nested11, "an integer")?);
                }
                entries11
            })
            }
        },
    })
}

/// Writes `agenticspec.cli.SimCheckViolation` as JSON.
pub fn encode_agenticspec_cli_sim_check_violation(value: &agenticspec_types::cli::SimCheckViolation, out: &mut String) {
    out.push('{');
    json::member(out, "rule");
    json::push_text(out, &value.rule);
    json::member(out, "gate");
    json::push_bool(out, value.gate);
    json::member(out, "subject");
    json::push_text(out, &value.subject);
    json::member(out, "part");
    json::push_text(out, &value.part);
    if let Some(held0) = &value.link {
        json::member(out, "link");
        json::push_integer(out, *held0);
    }
    json::member(out, "message");
    json::push_text(out, &value.message);
    out.push('}');
}

/// Reads `agenticspec.cli.SimCheckViolation` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_check_violation(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimCheckViolation, json::DecodeError> {
    json::closed_at(value, at, &["rule", "gate", "subject", "part", "link", "message"])?;
    Ok(agenticspec_types::cli::SimCheckViolation {
        rule: {
            let at0 = json::nested(at, "rule");
            let member0 = json::member_at(value, at, "rule")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        gate: {
            let at1 = json::nested(at, "gate");
            let member1 = json::member_at(value, at, "gate")?;
            json::bool_at(member1, &at1, "a boolean")?
        },
        subject: {
            let at2 = json::nested(at, "subject");
            let member2 = json::member_at(value, at, "subject")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        part: {
            let at3 = json::nested(at, "part");
            let member3 = json::member_at(value, at, "part")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        link: match value.member("link") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "link");
                Some(json::integer_at(member4, &at4, "an integer")?)
            }
        },
        message: {
            let at5 = json::nested(at, "message");
            let member5 = json::member_at(value, at, "message")?;
            json::text_at(member5, &at5, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.SimContractInput` as JSON.
pub fn encode_agenticspec_cli_sim_contract_input(value: &agenticspec_types::cli::SimContractInput, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    if let Some(held0) = &value.ports {
        json::member(out, "ports");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.out {
        json::member(out, "out");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.check {
        json::member(out, "check");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.suite {
        json::member(out, "suite");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.trace {
        json::member(out, "trace");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.kpis {
        json::member(out, "kpis");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.SimContractInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_contract_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimContractInput, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "ports", "out", "check", "suite", "trace", "kpis"])?;
    Ok(agenticspec_types::cli::SimContractInput {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        ports: match value.member("ports") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "ports");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
        out: match value.member("out") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "out");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
        check: match value.member("check") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "check");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        suite: match value.member("suite") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "suite");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
        trace: match value.member("trace") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "trace");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
        kpis: match value.member("kpis") {
            None | Some(json::Value::Null) => None,
            Some(member6) => {
                let at6 = json::nested(at, "kpis");
                Some(json::text_at(member6, &at6, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.SimContractResult` as JSON.
pub fn encode_agenticspec_cli_sim_contract_result(value: &agenticspec_types::cli::SimContractResult, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "ports");
    json::push_integer(out, value.ports);
    json::member(out, "invariants");
    json::push_integer(out, value.invariants);
    json::member(out, "ingresses");
    json::push_integer(out, value.ingresses);
    json::member(out, "written");
    json::push_bool(out, value.written);
    if let Some(held0) = &value.contract {
        json::member(out, "contract");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.suite_scenarios {
        json::member(out, "suite_scenarios");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.kept_scenarios {
        json::member(out, "kept_scenarios");
        json::push_integer(out, *held0);
    }
    json::member(out, "checks");
    out.push('[');
    for (index0, item0) in value.checks.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_sim_invariant_check(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.SimContractResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_contract_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimContractResult, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "ports", "invariants", "ingresses", "written", "contract", "suite_scenarios", "kept_scenarios", "checks"])?;
    Ok(agenticspec_types::cli::SimContractResult {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        ports: {
            let at1 = json::nested(at, "ports");
            let member1 = json::member_at(value, at, "ports")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        invariants: {
            let at2 = json::nested(at, "invariants");
            let member2 = json::member_at(value, at, "invariants")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        ingresses: {
            let at3 = json::nested(at, "ingresses");
            let member3 = json::member_at(value, at, "ingresses")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        written: {
            let at4 = json::nested(at, "written");
            let member4 = json::member_at(value, at, "written")?;
            json::bool_at(member4, &at4, "a boolean")?
        },
        contract: match value.member("contract") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "contract");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
        suite_scenarios: match value.member("suite_scenarios") {
            None | Some(json::Value::Null) => None,
            Some(member6) => {
                let at6 = json::nested(at, "suite_scenarios");
                Some(json::integer_at(member6, &at6, "an integer")?)
            }
        },
        kept_scenarios: match value.member("kept_scenarios") {
            None | Some(json::Value::Null) => None,
            Some(member7) => {
                let at7 = json::nested(at, "kept_scenarios");
                Some(json::integer_at(member7, &at7, "an integer")?)
            }
        },
        checks: {
            let at8 = json::nested(at, "checks");
            let member8 = json::member_at(value, at, "checks")?;
            {
                let mut items8 = Vec::new();
                for (index8, element8) in json::items_at(member8, &at8, "an array")?.iter().enumerate() {
                    let nested8 = json::nested(&at8, &index8.to_string());
                    items8.push(decode_agenticspec_cli_sim_invariant_check(element8, &nested8)?);
                }
                items8
            }
        },
    })
}

/// Writes `agenticspec.cli.SimCorpusInput` as JSON.
pub fn encode_agenticspec_cli_sim_corpus_input(value: &agenticspec_types::cli::SimCorpusInput, out: &mut String) {
    out.push('{');
    json::member(out, "action");
    encode_agenticspec_cli_corpus_action(&value.action, out);
    json::member(out, "corpus");
    json::push_text(out, &value.corpus);
    out.push('}');
}

/// Reads `agenticspec.cli.SimCorpusInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_corpus_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimCorpusInput, json::DecodeError> {
    json::closed_at(value, at, &["action", "corpus"])?;
    Ok(agenticspec_types::cli::SimCorpusInput {
        action: {
            let at0 = json::nested(at, "action");
            let member0 = json::member_at(value, at, "action")?;
            decode_agenticspec_cli_corpus_action(member0, &at0)?
        },
        corpus: {
            let at1 = json::nested(at, "corpus");
            let member1 = json::member_at(value, at, "corpus")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.SimCorpusResult` as JSON.
pub fn encode_agenticspec_cli_sim_corpus_result(value: &agenticspec_types::cli::SimCorpusResult, out: &mut String) {
    out.push('{');
    json::member(out, "corpus");
    json::push_text(out, &value.corpus);
    json::member(out, "entries");
    json::push_integer(out, value.entries);
    json::member(out, "held");
    json::push_integer(out, value.held);
    json::member(out, "failed");
    json::push_integer(out, value.failed);
    json::member(out, "rows");
    out.push('[');
    for (index0, item0) in value.rows.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_sim_corpus_row(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.SimCorpusResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_corpus_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimCorpusResult, json::DecodeError> {
    json::closed_at(value, at, &["corpus", "entries", "held", "failed", "rows"])?;
    Ok(agenticspec_types::cli::SimCorpusResult {
        corpus: {
            let at0 = json::nested(at, "corpus");
            let member0 = json::member_at(value, at, "corpus")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        entries: {
            let at1 = json::nested(at, "entries");
            let member1 = json::member_at(value, at, "entries")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        held: {
            let at2 = json::nested(at, "held");
            let member2 = json::member_at(value, at, "held")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        failed: {
            let at3 = json::nested(at, "failed");
            let member3 = json::member_at(value, at, "failed")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        rows: {
            let at4 = json::nested(at, "rows");
            let member4 = json::member_at(value, at, "rows")?;
            {
                let mut items4 = Vec::new();
                for (index4, element4) in json::items_at(member4, &at4, "an array")?.iter().enumerate() {
                    let nested4 = json::nested(&at4, &index4.to_string());
                    items4.push(decode_agenticspec_cli_sim_corpus_row(element4, &nested4)?);
                }
                items4
            }
        },
    })
}

/// Writes `agenticspec.cli.SimCorpusRow` as JSON.
pub fn encode_agenticspec_cli_sim_corpus_row(value: &agenticspec_types::cli::SimCorpusRow, out: &mut String) {
    out.push('{');
    json::member(out, "entry");
    json::push_text(out, &value.entry);
    json::member(out, "kind");
    json::push_text(out, &value.kind);
    json::member(out, "expected");
    json::push_text(out, &value.expected);
    json::member(out, "predicted");
    json::push_text(out, &value.predicted);
    json::member(out, "held");
    json::push_integer(out, value.held);
    json::member(out, "failed");
    json::push_integer(out, value.failed);
    json::member(out, "not_checked");
    json::push_integer(out, value.not_checked);
    json::member(out, "verdict");
    json::push_text(out, &value.verdict);
    out.push('}');
}

/// Reads `agenticspec.cli.SimCorpusRow` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_corpus_row(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimCorpusRow, json::DecodeError> {
    json::closed_at(value, at, &["entry", "kind", "expected", "predicted", "held", "failed", "not_checked", "verdict"])?;
    Ok(agenticspec_types::cli::SimCorpusRow {
        entry: {
            let at0 = json::nested(at, "entry");
            let member0 = json::member_at(value, at, "entry")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        kind: {
            let at1 = json::nested(at, "kind");
            let member1 = json::member_at(value, at, "kind")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        expected: {
            let at2 = json::nested(at, "expected");
            let member2 = json::member_at(value, at, "expected")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        predicted: {
            let at3 = json::nested(at, "predicted");
            let member3 = json::member_at(value, at, "predicted")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        held: {
            let at4 = json::nested(at, "held");
            let member4 = json::member_at(value, at, "held")?;
            json::integer_at(member4, &at4, "an integer")?
        },
        failed: {
            let at5 = json::nested(at, "failed");
            let member5 = json::member_at(value, at, "failed")?;
            json::integer_at(member5, &at5, "an integer")?
        },
        not_checked: {
            let at6 = json::nested(at, "not_checked");
            let member6 = json::member_at(value, at, "not_checked")?;
            json::integer_at(member6, &at6, "an integer")?
        },
        verdict: {
            let at7 = json::nested(at, "verdict");
            let member7 = json::member_at(value, at, "verdict")?;
            json::text_at(member7, &at7, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.SimDigestInput` as JSON.
pub fn encode_agenticspec_cli_sim_digest_input(value: &agenticspec_types::cli::SimDigestInput, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    out.push('}');
}

/// Reads `agenticspec.cli.SimDigestInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_digest_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimDigestInput, json::DecodeError> {
    json::closed_at(value, at, &["architecture"])?;
    Ok(agenticspec_types::cli::SimDigestInput {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.SimDigestResult` as JSON.
pub fn encode_agenticspec_cli_sim_digest_result(value: &agenticspec_types::cli::SimDigestResult, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "digest");
    json::push_text(out, &value.digest);
    out.push('}');
}

/// Reads `agenticspec.cli.SimDigestResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_digest_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimDigestResult, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "digest"])?;
    Ok(agenticspec_types::cli::SimDigestResult {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        digest: {
            let at1 = json::nested(at, "digest");
            let member1 = json::member_at(value, at, "digest")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.SimInvariantCheck` as JSON.
pub fn encode_agenticspec_cli_sim_invariant_check(value: &agenticspec_types::cli::SimInvariantCheck, out: &mut String) {
    out.push('{');
    json::member(out, "link");
    json::push_text(out, &value.link);
    json::member(out, "predicate");
    json::push_text(out, &value.predicate);
    json::member(out, "status");
    encode_agenticspec_cli_invariant_status(&value.status, out);
    json::member(out, "uses");
    json::push_integer(out, value.uses);
    if let Some(held0) = &value.event {
        json::member(out, "event");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.detail {
        json::member(out, "detail");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.SimInvariantCheck` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_invariant_check(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimInvariantCheck, json::DecodeError> {
    json::closed_at(value, at, &["link", "predicate", "status", "uses", "event", "detail"])?;
    Ok(agenticspec_types::cli::SimInvariantCheck {
        link: {
            let at0 = json::nested(at, "link");
            let member0 = json::member_at(value, at, "link")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        predicate: {
            let at1 = json::nested(at, "predicate");
            let member1 = json::member_at(value, at, "predicate")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        status: {
            let at2 = json::nested(at, "status");
            let member2 = json::member_at(value, at, "status")?;
            decode_agenticspec_cli_invariant_status(member2, &at2)?
        },
        uses: {
            let at3 = json::nested(at, "uses");
            let member3 = json::member_at(value, at, "uses")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        event: match value.member("event") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "event");
                Some(json::integer_at(member4, &at4, "an integer")?)
            }
        },
        detail: match value.member("detail") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "detail");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.SimLadderInput` as JSON.
pub fn encode_agenticspec_cli_sim_ladder_input(value: &agenticspec_types::cli::SimLadderInput, out: &mut String) {
    out.push('{');
    json::member(out, "ladder");
    json::push_text(out, &value.ladder);
    json::member(out, "out");
    json::push_text(out, &value.out);
    json::member(out, "check");
    json::push_bool(out, value.check);
    out.push('}');
}

/// Reads `agenticspec.cli.SimLadderInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_ladder_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimLadderInput, json::DecodeError> {
    json::closed_at(value, at, &["ladder", "out", "check"])?;
    Ok(agenticspec_types::cli::SimLadderInput {
        ladder: {
            let at0 = json::nested(at, "ladder");
            let member0 = json::member_at(value, at, "ladder")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        out: {
            let at1 = json::nested(at, "out");
            let member1 = json::member_at(value, at, "out")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        check: {
            let at2 = json::nested(at, "check");
            let member2 = json::member_at(value, at, "check")?;
            json::bool_at(member2, &at2, "a boolean")?
        },
    })
}

/// Writes `agenticspec.cli.SimLadderResult` as JSON.
pub fn encode_agenticspec_cli_sim_ladder_result(value: &agenticspec_types::cli::SimLadderResult, out: &mut String) {
    out.push('{');
    json::member(out, "out");
    json::push_text(out, &value.out);
    json::member(out, "rungs");
    json::push_integer(out, value.rungs);
    json::member(out, "written");
    json::push_bool(out, value.written);
    out.push('}');
}

/// Reads `agenticspec.cli.SimLadderResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_ladder_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimLadderResult, json::DecodeError> {
    json::closed_at(value, at, &["out", "rungs", "written"])?;
    Ok(agenticspec_types::cli::SimLadderResult {
        out: {
            let at0 = json::nested(at, "out");
            let member0 = json::member_at(value, at, "out")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        rungs: {
            let at1 = json::nested(at, "rungs");
            let member1 = json::member_at(value, at, "rungs")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        written: {
            let at2 = json::nested(at, "written");
            let member2 = json::member_at(value, at, "written")?;
            json::bool_at(member2, &at2, "a boolean")?
        },
    })
}

/// Writes `agenticspec.cli.SimNested` as JSON.
pub fn encode_agenticspec_cli_sim_nested(value: &agenticspec_types::cli::SimNested, out: &mut String) {
    out.push('{');
    json::member(out, "part");
    json::push_text(out, &value.part);
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "reference");
    json::push_text(out, &value.reference);
    json::member(out, "digest");
    json::push_text(out, &value.digest);
    json::member(out, "parts");
    json::push_integer(out, value.parts);
    json::member(out, "links");
    json::push_integer(out, value.links);
    out.push('}');
}

/// Reads `agenticspec.cli.SimNested` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_nested(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimNested, json::DecodeError> {
    json::closed_at(value, at, &["part", "architecture", "reference", "digest", "parts", "links"])?;
    Ok(agenticspec_types::cli::SimNested {
        part: {
            let at0 = json::nested(at, "part");
            let member0 = json::member_at(value, at, "part")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        architecture: {
            let at1 = json::nested(at, "architecture");
            let member1 = json::member_at(value, at, "architecture")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        reference: {
            let at2 = json::nested(at, "reference");
            let member2 = json::member_at(value, at, "reference")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        digest: {
            let at3 = json::nested(at, "digest");
            let member3 = json::member_at(value, at, "digest")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        parts: {
            let at4 = json::nested(at, "parts");
            let member4 = json::member_at(value, at, "parts")?;
            json::integer_at(member4, &at4, "an integer")?
        },
        links: {
            let at5 = json::nested(at, "links");
            let member5 = json::member_at(value, at, "links")?;
            json::integer_at(member5, &at5, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.SimPortsInput` as JSON.
pub fn encode_agenticspec_cli_sim_ports_input(value: &agenticspec_types::cli::SimPortsInput, out: &mut String) {
    out.push('{');
    if let Some(held0) = &value.blocks {
        json::member(out, "blocks");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.out {
        json::member(out, "out");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.check {
        json::member(out, "check");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.expect {
        json::member(out, "expect");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.SimPortsInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_ports_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimPortsInput, json::DecodeError> {
    json::closed_at(value, at, &["blocks", "out", "check", "expect"])?;
    Ok(agenticspec_types::cli::SimPortsInput {
        blocks: match value.member("blocks") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "blocks");
                Some(json::text_at(member0, &at0, "a string")?.to_owned())
            }
        },
        out: match value.member("out") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "out");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
        check: match value.member("check") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "check");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
        expect: match value.member("expect") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "expect");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.SimPortsResult` as JSON.
pub fn encode_agenticspec_cli_sim_ports_result(value: &agenticspec_types::cli::SimPortsResult, out: &mut String) {
    out.push('{');
    json::member(out, "ports");
    json::push_integer(out, value.ports);
    json::member(out, "sources_sha256");
    json::push_text(out, &value.sources_sha256);
    json::member(out, "written");
    json::push_bool(out, value.written);
    if let Some(held0) = &value.file {
        json::member(out, "file");
        json::push_text(out, &*held0);
    }
    json::member(out, "differences");
    out.push('[');
    for (index0, item0) in value.differences.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.SimPortsResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_ports_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimPortsResult, json::DecodeError> {
    json::closed_at(value, at, &["ports", "sources_sha256", "written", "file", "differences"])?;
    Ok(agenticspec_types::cli::SimPortsResult {
        ports: {
            let at0 = json::nested(at, "ports");
            let member0 = json::member_at(value, at, "ports")?;
            json::integer_at(member0, &at0, "an integer")?
        },
        sources_sha256: {
            let at1 = json::nested(at, "sources_sha256");
            let member1 = json::member_at(value, at, "sources_sha256")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        written: {
            let at2 = json::nested(at, "written");
            let member2 = json::member_at(value, at, "written")?;
            json::bool_at(member2, &at2, "a boolean")?
        },
        file: match value.member("file") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "file");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        differences: {
            let at4 = json::nested(at, "differences");
            let member4 = json::member_at(value, at, "differences")?;
            {
                let mut items4 = Vec::new();
                for (index4, element4) in json::items_at(member4, &at4, "an array")?.iter().enumerate() {
                    let nested4 = json::nested(&at4, &index4.to_string());
                    items4.push(json::text_at(element4, &nested4, "a string")?.to_owned());
                }
                items4
            }
        },
    })
}

/// Writes `agenticspec.cli.SimRenderInput` as JSON.
pub fn encode_agenticspec_cli_sim_render_input(value: &agenticspec_types::cli::SimRenderInput, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    if let Some(held0) = &value.result {
        json::member(out, "result");
        json::push_text(out, &*held0);
    }
    json::member(out, "out");
    json::push_text(out, &value.out);
    out.push('}');
}

/// Reads `agenticspec.cli.SimRenderInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_render_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimRenderInput, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "result", "out"])?;
    Ok(agenticspec_types::cli::SimRenderInput {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        result: match value.member("result") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "result");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
        out: {
            let at2 = json::nested(at, "out");
            let member2 = json::member_at(value, at, "out")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.SimRenderResult` as JSON.
pub fn encode_agenticspec_cli_sim_render_result(value: &agenticspec_types::cli::SimRenderResult, out: &mut String) {
    out.push('{');
    json::member(out, "out");
    json::push_text(out, &value.out);
    out.push('}');
}

/// Reads `agenticspec.cli.SimRenderResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_render_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimRenderResult, json::DecodeError> {
    json::closed_at(value, at, &["out"])?;
    Ok(agenticspec_types::cli::SimRenderResult {
        out: {
            let at0 = json::nested(at, "out");
            let member0 = json::member_at(value, at, "out")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.SimRunInput` as JSON.
pub fn encode_agenticspec_cli_sim_run_input(value: &agenticspec_types::cli::SimRunInput, out: &mut String) {
    out.push('{');
    json::member(out, "experiment");
    json::push_text(out, &value.experiment);
    json::member(out, "out");
    json::push_text(out, &value.out);
    if let Some(held0) = &value.threads {
        json::member(out, "threads");
        json::push_integer(out, *held0);
    }
    json::member(out, "events");
    json::push_bool(out, value.events);
    out.push('}');
}

/// Reads `agenticspec.cli.SimRunInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_run_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimRunInput, json::DecodeError> {
    json::closed_at(value, at, &["experiment", "out", "threads", "events"])?;
    Ok(agenticspec_types::cli::SimRunInput {
        experiment: {
            let at0 = json::nested(at, "experiment");
            let member0 = json::member_at(value, at, "experiment")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        out: {
            let at1 = json::nested(at, "out");
            let member1 = json::member_at(value, at, "out")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        threads: match value.member("threads") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "threads");
                Some(json::integer_at(member2, &at2, "an integer")?)
            }
        },
        events: {
            let at3 = json::nested(at, "events");
            let member3 = json::member_at(value, at, "events")?;
            json::bool_at(member3, &at3, "a boolean")?
        },
    })
}

/// Writes `agenticspec.cli.SimRunResult` as JSON.
pub fn encode_agenticspec_cli_sim_run_result(value: &agenticspec_types::cli::SimRunResult, out: &mut String) {
    out.push('{');
    json::member(out, "out");
    json::push_text(out, &value.out);
    json::member(out, "variants");
    json::push_integer(out, value.variants);
    json::member(out, "seeds");
    json::push_integer(out, value.seeds);
    json::member(out, "threads");
    json::push_integer(out, value.threads);
    json::member(out, "best_written");
    json::push_bool(out, value.best_written);
    if let Some(held0) = &value.events {
        json::member(out, "events");
        json::push_integer(out, *held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.SimRunResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_run_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimRunResult, json::DecodeError> {
    json::closed_at(value, at, &["out", "variants", "seeds", "threads", "best_written", "events"])?;
    Ok(agenticspec_types::cli::SimRunResult {
        out: {
            let at0 = json::nested(at, "out");
            let member0 = json::member_at(value, at, "out")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        variants: {
            let at1 = json::nested(at, "variants");
            let member1 = json::member_at(value, at, "variants")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        seeds: {
            let at2 = json::nested(at, "seeds");
            let member2 = json::member_at(value, at, "seeds")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        threads: {
            let at3 = json::nested(at, "threads");
            let member3 = json::member_at(value, at, "threads")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        best_written: {
            let at4 = json::nested(at, "best_written");
            let member4 = json::member_at(value, at, "best_written")?;
            json::bool_at(member4, &at4, "a boolean")?
        },
        events: match value.member("events") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "events");
                Some(json::integer_at(member5, &at5, "an integer")?)
            }
        },
    })
}

/// Writes `agenticspec.cli.SimScenarioInput` as JSON.
pub fn encode_agenticspec_cli_sim_scenario_input(value: &agenticspec_types::cli::SimScenarioInput, out: &mut String) {
    out.push('{');
    json::member(out, "action");
    encode_agenticspec_cli_scenario_action(&value.action, out);
    json::member(out, "scenario");
    json::push_text(out, &value.scenario);
    if let Some(held0) = &value.out {
        json::member(out, "out");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.SimScenarioInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_scenario_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimScenarioInput, json::DecodeError> {
    json::closed_at(value, at, &["action", "scenario", "out"])?;
    Ok(agenticspec_types::cli::SimScenarioInput {
        action: {
            let at0 = json::nested(at, "action");
            let member0 = json::member_at(value, at, "action")?;
            decode_agenticspec_cli_scenario_action(member0, &at0)?
        },
        scenario: {
            let at1 = json::nested(at, "scenario");
            let member1 = json::member_at(value, at, "scenario")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        out: match value.member("out") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "out");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.SimScenarioResult` as JSON.
pub fn encode_agenticspec_cli_sim_scenario_result(value: &agenticspec_types::cli::SimScenarioResult, out: &mut String) {
    out.push('{');
    json::member(out, "scenario");
    json::push_text(out, &value.scenario);
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "inputs");
    json::push_integer(out, value.inputs);
    json::member(out, "expectations");
    json::push_integer(out, value.expectations);
    if let Some(held0) = &value.out {
        json::member(out, "out");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.verdict {
        json::member(out, "verdict");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.predicted {
        json::member(out, "predicted");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.held {
        json::member(out, "held");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.failed {
        json::member(out, "failed");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.not_checked {
        json::member(out, "not_checked");
        json::push_integer(out, *held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.SimScenarioResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_scenario_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimScenarioResult, json::DecodeError> {
    json::closed_at(value, at, &["scenario", "architecture", "inputs", "expectations", "out", "verdict", "predicted", "held", "failed", "not_checked"])?;
    Ok(agenticspec_types::cli::SimScenarioResult {
        scenario: {
            let at0 = json::nested(at, "scenario");
            let member0 = json::member_at(value, at, "scenario")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        architecture: {
            let at1 = json::nested(at, "architecture");
            let member1 = json::member_at(value, at, "architecture")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        inputs: {
            let at2 = json::nested(at, "inputs");
            let member2 = json::member_at(value, at, "inputs")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        expectations: {
            let at3 = json::nested(at, "expectations");
            let member3 = json::member_at(value, at, "expectations")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        out: match value.member("out") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "out");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
        verdict: match value.member("verdict") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "verdict");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
        predicted: match value.member("predicted") {
            None | Some(json::Value::Null) => None,
            Some(member6) => {
                let at6 = json::nested(at, "predicted");
                Some(json::text_at(member6, &at6, "a string")?.to_owned())
            }
        },
        held: match value.member("held") {
            None | Some(json::Value::Null) => None,
            Some(member7) => {
                let at7 = json::nested(at, "held");
                Some(json::integer_at(member7, &at7, "an integer")?)
            }
        },
        failed: match value.member("failed") {
            None | Some(json::Value::Null) => None,
            Some(member8) => {
                let at8 = json::nested(at, "failed");
                Some(json::integer_at(member8, &at8, "an integer")?)
            }
        },
        not_checked: match value.member("not_checked") {
            None | Some(json::Value::Null) => None,
            Some(member9) => {
                let at9 = json::nested(at, "not_checked");
                Some(json::integer_at(member9, &at9, "an integer")?)
            }
        },
    })
}

/// Writes `agenticspec.cli.SimScoreInput` as JSON.
pub fn encode_agenticspec_cli_sim_score_input(value: &agenticspec_types::cli::SimScoreInput, out: &mut String) {
    out.push('{');
    if let Some(held0) = &value.architecture {
        json::member(out, "architecture");
        json::push_text(out, &*held0);
    }
    json::member(out, "profile");
    json::push_text(out, &value.profile);
    if let Some(held0) = &value.seed {
        json::member(out, "seed");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.horizon {
        json::member(out, "horizon");
        json::push_text(out, &*held0);
    }
    json::member(out, "batch");
    json::push_bool(out, value.batch);
    json::member(out, "documents");
    out.push('[');
    for (index0, item0) in value.documents.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.SimScoreInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_score_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimScoreInput, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "profile", "seed", "horizon", "batch", "documents"])?;
    Ok(agenticspec_types::cli::SimScoreInput {
        architecture: match value.member("architecture") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "architecture");
                Some(json::text_at(member0, &at0, "a string")?.to_owned())
            }
        },
        profile: {
            let at1 = json::nested(at, "profile");
            let member1 = json::member_at(value, at, "profile")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        seed: match value.member("seed") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "seed");
                Some(json::integer_at(member2, &at2, "an integer")?)
            }
        },
        horizon: match value.member("horizon") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "horizon");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        batch: {
            let at4 = json::nested(at, "batch");
            let member4 = json::member_at(value, at, "batch")?;
            json::bool_at(member4, &at4, "a boolean")?
        },
        documents: {
            let at5 = json::nested(at, "documents");
            let member5 = json::member_at(value, at, "documents")?;
            {
                let mut items5 = Vec::new();
                for (index5, element5) in json::items_at(member5, &at5, "an array")?.iter().enumerate() {
                    let nested5 = json::nested(&at5, &index5.to_string());
                    items5.push(json::text_at(element5, &nested5, "a string")?.to_owned());
                }
                items5
            }
        },
    })
}

/// Writes `agenticspec.cli.SimScoreResult` as JSON.
pub fn encode_agenticspec_cli_sim_score_result(value: &agenticspec_types::cli::SimScoreResult, out: &mut String) {
    out.push('{');
    if let Some(held0) = &value.kpis {
        json::member(out, "kpis");
        json::push_value(out, &*held0);
    }
    if let Some(held0) = &value.per_part {
        json::member(out, "per_part");
        json::push_value(out, &*held0);
    }
    if let Some(held0) = &value.results {
        json::member(out, "results");
        out.push('[');
        for (index1, item1) in (*held0).iter().enumerate() {
            if index1 > 0 {
                out.push(',');
            }
            json::push_value(out, &*item1);
        }
        out.push(']');
    }
    out.push('}');
}

/// Reads `agenticspec.cli.SimScoreResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_score_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimScoreResult, json::DecodeError> {
    json::closed_at(value, at, &["kpis", "per_part", "results"])?;
    Ok(agenticspec_types::cli::SimScoreResult {
        kpis: match value.member("kpis") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "kpis");
                Some(json::value_at(member0, &at0))
            }
        },
        per_part: match value.member("per_part") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "per_part");
                Some(json::value_at(member1, &at1))
            }
        },
        results: match value.member("results") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "results");
                Some({
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(json::value_at(element2, &nested2));
                }
                items2
            })
            }
        },
    })
}

/// Writes `agenticspec.cli.SimTraceInput` as JSON.
pub fn encode_agenticspec_cli_sim_trace_input(value: &agenticspec_types::cli::SimTraceInput, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "profile");
    json::push_text(out, &value.profile);
    if let Some(held0) = &value.seed {
        json::member(out, "seed");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.horizon {
        json::member(out, "horizon");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.out {
        json::member(out, "out");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.SimTraceInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_trace_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimTraceInput, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "profile", "seed", "horizon", "out"])?;
    Ok(agenticspec_types::cli::SimTraceInput {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        profile: {
            let at1 = json::nested(at, "profile");
            let member1 = json::member_at(value, at, "profile")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        seed: match value.member("seed") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "seed");
                Some(json::integer_at(member2, &at2, "an integer")?)
            }
        },
        horizon: match value.member("horizon") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "horizon");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        out: match value.member("out") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "out");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.SimTraceResult` as JSON.
pub fn encode_agenticspec_cli_sim_trace_result(value: &agenticspec_types::cli::SimTraceResult, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "seed");
    json::push_integer(out, value.seed);
    json::member(out, "uses");
    json::push_integer(out, value.uses);
    json::member(out, "events");
    json::push_integer(out, value.events);
    json::member(out, "written");
    json::push_bool(out, value.written);
    if let Some(held0) = &value.trace {
        json::member(out, "trace");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.SimTraceResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sim_trace_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SimTraceResult, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "seed", "uses", "events", "written", "trace"])?;
    Ok(agenticspec_types::cli::SimTraceResult {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        seed: {
            let at1 = json::nested(at, "seed");
            let member1 = json::member_at(value, at, "seed")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        uses: {
            let at2 = json::nested(at, "uses");
            let member2 = json::member_at(value, at, "uses")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        events: {
            let at3 = json::nested(at, "events");
            let member3 = json::member_at(value, at, "events")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        written: {
            let at4 = json::nested(at, "written");
            let member4 = json::member_at(value, at, "written")?;
            json::bool_at(member4, &at4, "a boolean")?
        },
        trace: match value.member("trace") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "trace");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.StepKind` as JSON.
pub fn encode_agenticspec_cli_step_kind(value: &agenticspec_types::cli::StepKind, out: &mut String) {
    match value {
        agenticspec_types::cli::StepKind::Program => json::push_text(out, "program"),
        agenticspec_types::cli::StepKind::Judgement => json::push_text(out, "judgement"),
        agenticspec_types::cli::StepKind::Person => json::push_text(out, "person"),
        agenticspec_types::cli::StepKind::Child => json::push_text(out, "child"),
    }
}

/// Reads `agenticspec.cli.StepKind` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_step_kind(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::StepKind, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `program`, `judgement`, `person`, `child`")? {
        "program" => agenticspec_types::cli::StepKind::Program,
        "judgement" => agenticspec_types::cli::StepKind::Judgement,
        "person" => agenticspec_types::cli::StepKind::Person,
        "child" => agenticspec_types::cli::StepKind::Child,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `program`, `judgement`, `person`, `child`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.cli.SyncInput` as JSON.
pub fn encode_agenticspec_cli_sync_input(value: &agenticspec_types::cli::SyncInput, out: &mut String) {
    out.push('{');
    if let Some(held0) = &value.root {
        json::member(out, "root");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.since_hours {
        json::member(out, "since_hours");
        json::push_integer(out, *held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.SyncInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sync_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SyncInput, json::DecodeError> {
    json::closed_at(value, at, &["root", "since_hours"])?;
    Ok(agenticspec_types::cli::SyncInput {
        root: match value.member("root") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "root");
                Some(json::text_at(member0, &at0, "a string")?.to_owned())
            }
        },
        since_hours: match value.member("since_hours") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "since_hours");
                Some(json::integer_at(member1, &at1, "an integer")?)
            }
        },
    })
}

/// Writes `agenticspec.cli.SyncResult` as JSON.
pub fn encode_agenticspec_cli_sync_result(value: &agenticspec_types::cli::SyncResult, out: &mut String) {
    out.push('{');
    json::member(out, "db");
    json::push_text(out, &value.db);
    json::member(out, "scanned");
    json::push_integer(out, value.scanned);
    json::member(out, "lifted");
    json::push_integer(out, value.lifted);
    json::member(out, "unchanged");
    json::push_integer(out, value.unchanged);
    json::member(out, "failed");
    json::push_integer(out, value.failed);
    json::member(out, "sessions");
    json::push_integer(out, value.sessions);
    json::member(out, "ms");
    json::push_integer(out, value.ms);
    out.push('}');
}

/// Reads `agenticspec.cli.SyncResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_sync_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::SyncResult, json::DecodeError> {
    json::closed_at(value, at, &["db", "scanned", "lifted", "unchanged", "failed", "sessions", "ms"])?;
    Ok(agenticspec_types::cli::SyncResult {
        db: {
            let at0 = json::nested(at, "db");
            let member0 = json::member_at(value, at, "db")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        scanned: {
            let at1 = json::nested(at, "scanned");
            let member1 = json::member_at(value, at, "scanned")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        lifted: {
            let at2 = json::nested(at, "lifted");
            let member2 = json::member_at(value, at, "lifted")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        unchanged: {
            let at3 = json::nested(at, "unchanged");
            let member3 = json::member_at(value, at, "unchanged")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        failed: {
            let at4 = json::nested(at, "failed");
            let member4 = json::member_at(value, at, "failed")?;
            json::integer_at(member4, &at4, "an integer")?
        },
        sessions: {
            let at5 = json::nested(at, "sessions");
            let member5 = json::member_at(value, at, "sessions")?;
            json::integer_at(member5, &at5, "an integer")?
        },
        ms: {
            let at6 = json::nested(at, "ms");
            let member6 = json::member_at(value, at, "ms")?;
            json::integer_at(member6, &at6, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.TimelineMark` as JSON.
pub fn encode_agenticspec_cli_timeline_mark(value: &agenticspec_types::cli::TimelineMark, out: &mut String) {
    out.push('{');
    json::member(out, "at");
    json::push_text(out, &value.at);
    json::member(out, "kind");
    encode_agenticspec_cli_mark_kind(&value.kind, out);
    json::member(out, "scope");
    encode_agenticspec_cli_agent_scope(&value.scope, out);
    json::member(out, "value");
    json::push_integer(out, value.value);
    out.push('}');
}

/// Reads `agenticspec.cli.TimelineMark` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_timeline_mark(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::TimelineMark, json::DecodeError> {
    json::closed_at(value, at, &["at", "kind", "scope", "value"])?;
    Ok(agenticspec_types::cli::TimelineMark {
        at: {
            let at0 = json::nested(at, "at");
            let member0 = json::member_at(value, at, "at")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        kind: {
            let at1 = json::nested(at, "kind");
            let member1 = json::member_at(value, at, "kind")?;
            decode_agenticspec_cli_mark_kind(member1, &at1)?
        },
        scope: {
            let at2 = json::nested(at, "scope");
            let member2 = json::member_at(value, at, "scope")?;
            decode_agenticspec_cli_agent_scope(member2, &at2)?
        },
        value: {
            let at3 = json::nested(at, "value");
            let member3 = json::member_at(value, at, "value")?;
            json::integer_at(member3, &at3, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.ToolFigures` as JSON.
pub fn encode_agenticspec_cli_tool_figures(value: &agenticspec_types::cli::ToolFigures, out: &mut String) {
    out.push('{');
    json::member(out, "category");
    encode_agenticspec_session_tool_category(&value.category, out);
    json::member(out, "calls");
    json::push_integer(out, value.calls);
    json::member(out, "refused");
    json::push_integer(out, value.refused);
    json::member(out, "failed");
    json::push_integer(out, value.failed);
    out.push('}');
}

/// Reads `agenticspec.cli.ToolFigures` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_tool_figures(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::ToolFigures, json::DecodeError> {
    json::closed_at(value, at, &["category", "calls", "refused", "failed"])?;
    Ok(agenticspec_types::cli::ToolFigures {
        category: {
            let at0 = json::nested(at, "category");
            let member0 = json::member_at(value, at, "category")?;
            decode_agenticspec_session_tool_category(member0, &at0)?
        },
        calls: {
            let at1 = json::nested(at, "calls");
            let member1 = json::member_at(value, at, "calls")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        refused: {
            let at2 = json::nested(at, "refused");
            let member2 = json::member_at(value, at, "refused")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        failed: {
            let at3 = json::nested(at, "failed");
            let member3 = json::member_at(value, at, "failed")?;
            json::integer_at(member3, &at3, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.UiBuildInput` as JSON.
pub fn encode_agenticspec_cli_ui_build_input(value: &agenticspec_types::cli::UiBuildInput, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    json::member(out, "contract");
    json::push_text(out, &value.contract);
    json::member(out, "out");
    json::push_text(out, &value.out);
    if let Some(held0) = &value.api_base {
        json::member(out, "api_base");
        json::push_text(out, &*held0);
    }
    json::member(out, "archive");
    json::push_bool(out, value.archive);
    json::member(out, "control_plane");
    json::push_bool(out, value.control_plane);
    if let Some(held0) = &value.prediction {
        json::member(out, "prediction");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.ingress {
        json::member(out, "ingress");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.UiBuildInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_build_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiBuildInput, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "contract", "out", "api_base", "archive", "control_plane", "prediction", "ingress"])?;
    Ok(agenticspec_types::cli::UiBuildInput {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        contract: {
            let at1 = json::nested(at, "contract");
            let member1 = json::member_at(value, at, "contract")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        out: {
            let at2 = json::nested(at, "out");
            let member2 = json::member_at(value, at, "out")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        api_base: match value.member("api_base") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "api_base");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        archive: {
            let at4 = json::nested(at, "archive");
            let member4 = json::member_at(value, at, "archive")?;
            json::bool_at(member4, &at4, "a boolean")?
        },
        control_plane: {
            let at5 = json::nested(at, "control_plane");
            let member5 = json::member_at(value, at, "control_plane")?;
            json::bool_at(member5, &at5, "a boolean")?
        },
        prediction: match value.member("prediction") {
            None | Some(json::Value::Null) => None,
            Some(member6) => {
                let at6 = json::nested(at, "prediction");
                Some(json::text_at(member6, &at6, "a string")?.to_owned())
            }
        },
        ingress: match value.member("ingress") {
            None | Some(json::Value::Null) => None,
            Some(member7) => {
                let at7 = json::nested(at, "ingress");
                Some(json::text_at(member7, &at7, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.UiBuildResult` as JSON.
pub fn encode_agenticspec_cli_ui_build_result(value: &agenticspec_types::cli::UiBuildResult, out: &mut String) {
    out.push('{');
    json::member(out, "out");
    json::push_text(out, &value.out);
    json::member(out, "files");
    json::push_integer(out, value.files);
    json::member(out, "bytes");
    json::push_integer(out, value.bytes);
    if let Some(held0) = &value.archive {
        json::member(out, "archive");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.archive_sha256 {
        json::member(out, "archive_sha256");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.UiBuildResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_build_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiBuildResult, json::DecodeError> {
    json::closed_at(value, at, &["out", "files", "bytes", "archive", "archive_sha256"])?;
    Ok(agenticspec_types::cli::UiBuildResult {
        out: {
            let at0 = json::nested(at, "out");
            let member0 = json::member_at(value, at, "out")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        files: {
            let at1 = json::nested(at, "files");
            let member1 = json::member_at(value, at, "files")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        bytes: {
            let at2 = json::nested(at, "bytes");
            let member2 = json::member_at(value, at, "bytes")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        archive: match value.member("archive") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "archive");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        archive_sha256: match value.member("archive_sha256") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "archive_sha256");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.UiConformCheck` as JSON.
pub fn encode_agenticspec_cli_ui_conform_check(value: &agenticspec_types::cli::UiConformCheck, out: &mut String) {
    out.push('{');
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "status");
    json::push_text(out, &value.status);
    json::member(out, "detail");
    json::push_text(out, &value.detail);
    out.push('}');
}

/// Reads `agenticspec.cli.UiConformCheck` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_conform_check(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiConformCheck, json::DecodeError> {
    json::closed_at(value, at, &["name", "status", "detail"])?;
    Ok(agenticspec_types::cli::UiConformCheck {
        name: {
            let at0 = json::nested(at, "name");
            let member0 = json::member_at(value, at, "name")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        status: {
            let at1 = json::nested(at, "status");
            let member1 = json::member_at(value, at, "status")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        detail: {
            let at2 = json::nested(at, "detail");
            let member2 = json::member_at(value, at, "detail")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.UiConformInput` as JSON.
pub fn encode_agenticspec_cli_ui_conform_input(value: &agenticspec_types::cli::UiConformInput, out: &mut String) {
    out.push('{');
    if let Some(held0) = &value.source {
        json::member(out, "source");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.plan {
        json::member(out, "plan");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.root {
        json::member(out, "root");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.architecture {
        json::member(out, "architecture");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.contract {
        json::member(out, "contract");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.origin {
        json::member(out, "origin");
        json::push_text(out, &*held0);
    }
    json::member(out, "header");
    out.push('[');
    for (index0, item0) in value.header.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "json");
    json::push_bool(out, value.json);
    out.push('}');
}

/// Reads `agenticspec.cli.UiConformInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_conform_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiConformInput, json::DecodeError> {
    json::closed_at(value, at, &["source", "plan", "root", "architecture", "contract", "origin", "header", "json"])?;
    Ok(agenticspec_types::cli::UiConformInput {
        source: match value.member("source") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "source");
                Some(json::text_at(member0, &at0, "a string")?.to_owned())
            }
        },
        plan: match value.member("plan") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "plan");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
        root: match value.member("root") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "root");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
        architecture: match value.member("architecture") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "architecture");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        contract: match value.member("contract") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "contract");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
        origin: match value.member("origin") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "origin");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
        header: {
            let at6 = json::nested(at, "header");
            let member6 = json::member_at(value, at, "header")?;
            {
                let mut items6 = Vec::new();
                for (index6, element6) in json::items_at(member6, &at6, "an array")?.iter().enumerate() {
                    let nested6 = json::nested(&at6, &index6.to_string());
                    items6.push(json::text_at(element6, &nested6, "a string")?.to_owned());
                }
                items6
            }
        },
        json: {
            let at7 = json::nested(at, "json");
            let member7 = json::member_at(value, at, "json")?;
            json::bool_at(member7, &at7, "a boolean")?
        },
    })
}

/// Writes `agenticspec.cli.UiConformResult` as JSON.
pub fn encode_agenticspec_cli_ui_conform_result(value: &agenticspec_types::cli::UiConformResult, out: &mut String) {
    out.push('{');
    json::member(out, "source");
    json::push_text(out, &value.source);
    json::member(out, "passed");
    json::push_integer(out, value.passed);
    json::member(out, "failed");
    json::push_integer(out, value.failed);
    json::member(out, "skipped");
    json::push_integer(out, value.skipped);
    json::member(out, "checks");
    out.push('[');
    for (index0, item0) in value.checks.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_cli_ui_conform_check(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.UiConformResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_conform_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiConformResult, json::DecodeError> {
    json::closed_at(value, at, &["source", "passed", "failed", "skipped", "checks"])?;
    Ok(agenticspec_types::cli::UiConformResult {
        source: {
            let at0 = json::nested(at, "source");
            let member0 = json::member_at(value, at, "source")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        passed: {
            let at1 = json::nested(at, "passed");
            let member1 = json::member_at(value, at, "passed")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        failed: {
            let at2 = json::nested(at, "failed");
            let member2 = json::member_at(value, at, "failed")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        skipped: {
            let at3 = json::nested(at, "skipped");
            let member3 = json::member_at(value, at, "skipped")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        checks: {
            let at4 = json::nested(at, "checks");
            let member4 = json::member_at(value, at, "checks")?;
            {
                let mut items4 = Vec::new();
                for (index4, element4) in json::items_at(member4, &at4, "an array")?.iter().enumerate() {
                    let nested4 = json::nested(&at4, &index4.to_string());
                    items4.push(decode_agenticspec_cli_ui_conform_check(element4, &nested4)?);
                }
                items4
            }
        },
    })
}

/// Writes `agenticspec.cli.UiDeriveInput` as JSON.
pub fn encode_agenticspec_cli_ui_derive_input(value: &agenticspec_types::cli::UiDeriveInput, out: &mut String) {
    out.push('{');
    json::member(out, "path");
    json::push_text(out, &value.path);
    if let Some(held0) = &value.contract {
        json::member(out, "contract");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.ports {
        json::member(out, "ports");
        json::push_text(out, &*held0);
    }
    json::member(out, "out");
    json::push_text(out, &value.out);
    if let Some(held0) = &value.root {
        json::member(out, "root");
        json::push_text(out, &*held0);
    }
    json::member(out, "check");
    json::push_bool(out, value.check);
    json::member(out, "control_plane");
    json::push_bool(out, value.control_plane);
    if let Some(held0) = &value.prediction {
        json::member(out, "prediction");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.UiDeriveInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_derive_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiDeriveInput, json::DecodeError> {
    json::closed_at(value, at, &["path", "contract", "ports", "out", "root", "check", "control_plane", "prediction"])?;
    Ok(agenticspec_types::cli::UiDeriveInput {
        path: {
            let at0 = json::nested(at, "path");
            let member0 = json::member_at(value, at, "path")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        contract: match value.member("contract") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "contract");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
        ports: match value.member("ports") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "ports");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
        out: {
            let at3 = json::nested(at, "out");
            let member3 = json::member_at(value, at, "out")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        root: match value.member("root") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "root");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
        check: {
            let at5 = json::nested(at, "check");
            let member5 = json::member_at(value, at, "check")?;
            json::bool_at(member5, &at5, "a boolean")?
        },
        control_plane: {
            let at6 = json::nested(at, "control_plane");
            let member6 = json::member_at(value, at, "control_plane")?;
            json::bool_at(member6, &at6, "a boolean")?
        },
        prediction: match value.member("prediction") {
            None | Some(json::Value::Null) => None,
            Some(member7) => {
                let at7 = json::nested(at, "prediction");
                Some(json::text_at(member7, &at7, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.UiDeriveResult` as JSON.
pub fn encode_agenticspec_cli_ui_derive_result(value: &agenticspec_types::cli::UiDeriveResult, out: &mut String) {
    out.push('{');
    json::member(out, "out");
    json::push_text(out, &value.out);
    json::member(out, "pages");
    json::push_integer(out, value.pages);
    json::member(out, "actions");
    json::push_integer(out, value.actions);
    json::member(out, "channels");
    json::push_integer(out, value.channels);
    json::member(out, "files");
    json::push_integer(out, value.files);
    json::member(out, "written");
    json::push_bool(out, value.written);
    if let Some(held0) = &value.ingresses {
        json::member(out, "ingresses");
        out.push('[');
        for (index1, item1) in (*held0).iter().enumerate() {
            if index1 > 0 {
                out.push(',');
            }
            encode_agenticspec_cli_ui_ingress_derived(&*item1, out);
        }
        out.push(']');
    }
    out.push('}');
}

/// Reads `agenticspec.cli.UiDeriveResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_derive_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiDeriveResult, json::DecodeError> {
    json::closed_at(value, at, &["out", "pages", "actions", "channels", "files", "written", "ingresses"])?;
    Ok(agenticspec_types::cli::UiDeriveResult {
        out: {
            let at0 = json::nested(at, "out");
            let member0 = json::member_at(value, at, "out")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        pages: {
            let at1 = json::nested(at, "pages");
            let member1 = json::member_at(value, at, "pages")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        actions: {
            let at2 = json::nested(at, "actions");
            let member2 = json::member_at(value, at, "actions")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        channels: {
            let at3 = json::nested(at, "channels");
            let member3 = json::member_at(value, at, "channels")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        files: {
            let at4 = json::nested(at, "files");
            let member4 = json::member_at(value, at, "files")?;
            json::integer_at(member4, &at4, "an integer")?
        },
        written: {
            let at5 = json::nested(at, "written");
            let member5 = json::member_at(value, at, "written")?;
            json::bool_at(member5, &at5, "a boolean")?
        },
        ingresses: match value.member("ingresses") {
            None | Some(json::Value::Null) => None,
            Some(member6) => {
                let at6 = json::nested(at, "ingresses");
                Some({
                let mut items6 = Vec::new();
                for (index6, element6) in json::items_at(member6, &at6, "an array")?.iter().enumerate() {
                    let nested6 = json::nested(&at6, &index6.to_string());
                    items6.push(decode_agenticspec_cli_ui_ingress_derived(element6, &nested6)?);
                }
                items6
            })
            }
        },
    })
}

/// Writes `agenticspec.cli.UiGenerateInput` as JSON.
pub fn encode_agenticspec_cli_ui_generate_input(value: &agenticspec_types::cli::UiGenerateInput, out: &mut String) {
    out.push('{');
    json::member(out, "path");
    json::push_text(out, &value.path);
    json::member(out, "out");
    json::push_text(out, &value.out);
    json::member(out, "project");
    json::push_bool(out, value.project);
    out.push('}');
}

/// Reads `agenticspec.cli.UiGenerateInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_generate_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiGenerateInput, json::DecodeError> {
    json::closed_at(value, at, &["path", "out", "project"])?;
    Ok(agenticspec_types::cli::UiGenerateInput {
        path: {
            let at0 = json::nested(at, "path");
            let member0 = json::member_at(value, at, "path")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        out: {
            let at1 = json::nested(at, "out");
            let member1 = json::member_at(value, at, "out")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        project: {
            let at2 = json::nested(at, "project");
            let member2 = json::member_at(value, at, "project")?;
            json::bool_at(member2, &at2, "a boolean")?
        },
    })
}

/// Writes `agenticspec.cli.UiGenerateResult` as JSON.
pub fn encode_agenticspec_cli_ui_generate_result(value: &agenticspec_types::cli::UiGenerateResult, out: &mut String) {
    out.push('{');
    json::member(out, "out");
    json::push_text(out, &value.out);
    json::member(out, "files");
    out.push('[');
    for (index0, item0) in value.files.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.cli.UiGenerateResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_generate_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiGenerateResult, json::DecodeError> {
    json::closed_at(value, at, &["out", "files"])?;
    Ok(agenticspec_types::cli::UiGenerateResult {
        out: {
            let at0 = json::nested(at, "out");
            let member0 = json::member_at(value, at, "out")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        files: {
            let at1 = json::nested(at, "files");
            let member1 = json::member_at(value, at, "files")?;
            {
                let mut items1 = Vec::new();
                for (index1, element1) in json::items_at(member1, &at1, "an array")?.iter().enumerate() {
                    let nested1 = json::nested(&at1, &index1.to_string());
                    items1.push(json::text_at(element1, &nested1, "a string")?.to_owned());
                }
                items1
            }
        },
    })
}

/// Writes `agenticspec.cli.UiIngressDerived` as JSON.
pub fn encode_agenticspec_cli_ui_ingress_derived(value: &agenticspec_types::cli::UiIngressDerived, out: &mut String) {
    out.push('{');
    json::member(out, "ingress");
    json::push_text(out, &value.ingress);
    json::member(out, "out");
    json::push_text(out, &value.out);
    json::member(out, "pages");
    json::push_integer(out, value.pages);
    json::member(out, "actions");
    json::push_integer(out, value.actions);
    json::member(out, "channels");
    json::push_integer(out, value.channels);
    out.push('}');
}

/// Reads `agenticspec.cli.UiIngressDerived` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_ingress_derived(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiIngressDerived, json::DecodeError> {
    json::closed_at(value, at, &["ingress", "out", "pages", "actions", "channels"])?;
    Ok(agenticspec_types::cli::UiIngressDerived {
        ingress: {
            let at0 = json::nested(at, "ingress");
            let member0 = json::member_at(value, at, "ingress")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        out: {
            let at1 = json::nested(at, "out");
            let member1 = json::member_at(value, at, "out")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        pages: {
            let at2 = json::nested(at, "pages");
            let member2 = json::member_at(value, at, "pages")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        actions: {
            let at3 = json::nested(at, "actions");
            let member3 = json::member_at(value, at, "actions")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        channels: {
            let at4 = json::nested(at, "channels");
            let member4 = json::member_at(value, at, "channels")?;
            json::integer_at(member4, &at4, "an integer")?
        },
    })
}

/// Writes `agenticspec.cli.UiIngressServed` as JSON.
pub fn encode_agenticspec_cli_ui_ingress_served(value: &agenticspec_types::cli::UiIngressServed, out: &mut String) {
    out.push('{');
    json::member(out, "ingress");
    json::push_text(out, &value.ingress);
    json::member(out, "url");
    json::push_text(out, &value.url);
    json::member(out, "interface");
    json::push_text(out, &value.interface);
    out.push('}');
}

/// Reads `agenticspec.cli.UiIngressServed` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_ingress_served(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiIngressServed, json::DecodeError> {
    json::closed_at(value, at, &["ingress", "url", "interface"])?;
    Ok(agenticspec_types::cli::UiIngressServed {
        ingress: {
            let at0 = json::nested(at, "ingress");
            let member0 = json::member_at(value, at, "ingress")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        url: {
            let at1 = json::nested(at, "url");
            let member1 = json::member_at(value, at, "url")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        interface: {
            let at2 = json::nested(at, "interface");
            let member2 = json::member_at(value, at, "interface")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
    })
}

/// Writes `agenticspec.cli.UiPanelsInput` as JSON.
pub fn encode_agenticspec_cli_ui_panels_input(value: &agenticspec_types::cli::UiPanelsInput, out: &mut String) {
    out.push('{');
    if let Some(held0) = &value.root {
        json::member(out, "root");
        json::push_text(out, &*held0);
    }
    json::member(out, "check");
    json::push_bool(out, value.check);
    out.push('}');
}

/// Reads `agenticspec.cli.UiPanelsInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_panels_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiPanelsInput, json::DecodeError> {
    json::closed_at(value, at, &["root", "check"])?;
    Ok(agenticspec_types::cli::UiPanelsInput {
        root: match value.member("root") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "root");
                Some(json::text_at(member0, &at0, "a string")?.to_owned())
            }
        },
        check: {
            let at1 = json::nested(at, "check");
            let member1 = json::member_at(value, at, "check")?;
            json::bool_at(member1, &at1, "a boolean")?
        },
    })
}

/// Writes `agenticspec.cli.UiPanelsResult` as JSON.
pub fn encode_agenticspec_cli_ui_panels_result(value: &agenticspec_types::cli::UiPanelsResult, out: &mut String) {
    out.push('{');
    json::member(out, "out");
    json::push_text(out, &value.out);
    json::member(out, "panels");
    json::push_integer(out, value.panels);
    json::member(out, "files");
    json::push_integer(out, value.files);
    json::member(out, "written");
    json::push_bool(out, value.written);
    out.push('}');
}

/// Reads `agenticspec.cli.UiPanelsResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_panels_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiPanelsResult, json::DecodeError> {
    json::closed_at(value, at, &["out", "panels", "files", "written"])?;
    Ok(agenticspec_types::cli::UiPanelsResult {
        out: {
            let at0 = json::nested(at, "out");
            let member0 = json::member_at(value, at, "out")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        panels: {
            let at1 = json::nested(at, "panels");
            let member1 = json::member_at(value, at, "panels")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        files: {
            let at2 = json::nested(at, "files");
            let member2 = json::member_at(value, at, "files")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        written: {
            let at3 = json::nested(at, "written");
            let member3 = json::member_at(value, at, "written")?;
            json::bool_at(member3, &at3, "a boolean")?
        },
    })
}

/// Writes `agenticspec.cli.UiServeInput` as JSON.
pub fn encode_agenticspec_cli_ui_serve_input(value: &agenticspec_types::cli::UiServeInput, out: &mut String) {
    out.push('{');
    json::member(out, "architecture");
    json::push_text(out, &value.architecture);
    if let Some(held0) = &value.contract {
        json::member(out, "contract");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.port {
        json::member(out, "port");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.fixtures {
        json::member(out, "fixtures");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.sim {
        json::member(out, "sim");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agenticspec.cli.UiServeInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_serve_input(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiServeInput, json::DecodeError> {
    json::closed_at(value, at, &["architecture", "contract", "port", "fixtures", "sim"])?;
    Ok(agenticspec_types::cli::UiServeInput {
        architecture: {
            let at0 = json::nested(at, "architecture");
            let member0 = json::member_at(value, at, "architecture")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        contract: match value.member("contract") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "contract");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
        port: match value.member("port") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "port");
                Some(json::integer_at(member2, &at2, "an integer")?)
            }
        },
        fixtures: match value.member("fixtures") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "fixtures");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        sim: match value.member("sim") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "sim");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agenticspec.cli.UiServeResult` as JSON.
pub fn encode_agenticspec_cli_ui_serve_result(value: &agenticspec_types::cli::UiServeResult, out: &mut String) {
    out.push('{');
    json::member(out, "url");
    json::push_text(out, &value.url);
    json::member(out, "control");
    json::push_text(out, &value.control);
    json::member(out, "interface");
    json::push_text(out, &value.interface);
    if let Some(held0) = &value.ingresses {
        json::member(out, "ingresses");
        out.push('[');
        for (index1, item1) in (*held0).iter().enumerate() {
            if index1 > 0 {
                out.push(',');
            }
            encode_agenticspec_cli_ui_ingress_served(&*item1, out);
        }
        out.push(']');
    }
    out.push('}');
}

/// Reads `agenticspec.cli.UiServeResult` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_cli_ui_serve_result(value: &json::Value, at: &str) -> Result<agenticspec_types::cli::UiServeResult, json::DecodeError> {
    json::closed_at(value, at, &["url", "control", "interface", "ingresses"])?;
    Ok(agenticspec_types::cli::UiServeResult {
        url: {
            let at0 = json::nested(at, "url");
            let member0 = json::member_at(value, at, "url")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        control: {
            let at1 = json::nested(at, "control");
            let member1 = json::member_at(value, at, "control")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        interface: {
            let at2 = json::nested(at, "interface");
            let member2 = json::member_at(value, at, "interface")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        ingresses: match value.member("ingresses") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "ingresses");
                Some({
                let mut items3 = Vec::new();
                for (index3, element3) in json::items_at(member3, &at3, "an array")?.iter().enumerate() {
                    let nested3 = json::nested(&at3, &index3.to_string());
                    items3.push(decode_agenticspec_cli_ui_ingress_served(element3, &nested3)?);
                }
                items3
            })
            }
        },
    })
}

/// Writes `agenticspec.session.Agent.State` as JSON.
pub fn encode_agenticspec_session_agent_state(value: &agenticspec_types::session::AgentState, out: &mut String) {
    match value {
        agenticspec_types::session::AgentState::Ended => json::push_text(out, "Ended"),
        agenticspec_types::session::AgentState::Idle => json::push_text(out, "Idle"),
        agenticspec_types::session::AgentState::Working => json::push_text(out, "Working"),
    }
}

/// Reads `agenticspec.session.Agent.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_agent_state(value: &json::Value, at: &str) -> Result<agenticspec_types::session::AgentState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Ended`, `Idle`, `Working`")? {
        "Ended" => agenticspec_types::session::AgentState::Ended,
        "Idle" => agenticspec_types::session::AgentState::Idle,
        "Working" => agenticspec_types::session::AgentState::Working,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Ended`, `Idle`, `Working`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.AgentId` as JSON.
pub fn encode_agenticspec_session_agent_id(value: &agenticspec_types::session::AgentId, out: &mut String) {
    json::push_text(out, &value.0);
}

/// Reads `agenticspec.session.AgentId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_agent_id(value: &json::Value, at: &str) -> Result<agenticspec_types::session::AgentId, json::DecodeError> {
    Ok(agenticspec_types::session::AgentId(json::text_at(value, at, "a string")?.to_owned()))
}

/// Writes `agenticspec.session.AgentRole` as JSON.
pub fn encode_agenticspec_session_agent_role(value: &agenticspec_types::session::AgentRole, out: &mut String) {
    match value {
        agenticspec_types::session::AgentRole::Main => json::push_text(out, "Main"),
        agenticspec_types::session::AgentRole::SubAgent => json::push_text(out, "SubAgent"),
        agenticspec_types::session::AgentRole::SideCall => json::push_text(out, "SideCall"),
    }
}

/// Reads `agenticspec.session.AgentRole` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_agent_role(value: &json::Value, at: &str) -> Result<agenticspec_types::session::AgentRole, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Main`, `SubAgent`, `SideCall`")? {
        "Main" => agenticspec_types::session::AgentRole::Main,
        "SubAgent" => agenticspec_types::session::AgentRole::SubAgent,
        "SideCall" => agenticspec_types::session::AgentRole::SideCall,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Main`, `SubAgent`, `SideCall`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.Attribution` as JSON.
pub fn encode_agenticspec_session_attribution(value: &agenticspec_types::session::Attribution, out: &mut String) {
    match value {
        agenticspec_types::session::Attribution::Known => json::push_text(out, "Known"),
        agenticspec_types::session::Attribution::Inferred => json::push_text(out, "Inferred"),
        agenticspec_types::session::Attribution::Unknown => json::push_text(out, "Unknown"),
    }
}

/// Reads `agenticspec.session.Attribution` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_attribution(value: &json::Value, at: &str) -> Result<agenticspec_types::session::Attribution, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Known`, `Inferred`, `Unknown`")? {
        "Known" => agenticspec_types::session::Attribution::Known,
        "Inferred" => agenticspec_types::session::Attribution::Inferred,
        "Unknown" => agenticspec_types::session::Attribution::Unknown,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Known`, `Inferred`, `Unknown`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.Content` as JSON.
pub fn encode_agenticspec_session_content(value: &agenticspec_types::session::Content, out: &mut String) {
    out.push('{');
    json::member(out, "digest");
    json::push_text(out, &value.digest);
    json::member(out, "bytes");
    json::push_integer(out, value.bytes);
    out.push('}');
}

/// Reads `agenticspec.session.Content` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_content(value: &json::Value, at: &str) -> Result<agenticspec_types::session::Content, json::DecodeError> {
    json::closed_at(value, at, &["digest", "bytes"])?;
    Ok(agenticspec_types::session::Content {
        digest: {
            let at0 = json::nested(at, "digest");
            let member0 = json::member_at(value, at, "digest")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        bytes: {
            let at1 = json::nested(at, "bytes");
            let member1 = json::member_at(value, at, "bytes")?;
            json::integer_at(member1, &at1, "an integer")?
        },
    })
}

/// Writes `agenticspec.session.Decider` as JSON.
pub fn encode_agenticspec_session_decider(value: &agenticspec_types::session::Decider, out: &mut String) {
    match value {
        agenticspec_types::session::Decider::PermissionMode => json::push_text(out, "PermissionMode"),
        agenticspec_types::session::Decider::Classifier => json::push_text(out, "Classifier"),
        agenticspec_types::session::Decider::Hook => json::push_text(out, "Hook"),
        agenticspec_types::session::Decider::Human => json::push_text(out, "Human"),
        agenticspec_types::session::Decider::SafetyCheck => json::push_text(out, "SafetyCheck"),
        agenticspec_types::session::Decider::Rule => json::push_text(out, "Rule"),
        agenticspec_types::session::Decider::NotRecorded => json::push_text(out, "NotRecorded"),
    }
}

/// Reads `agenticspec.session.Decider` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_decider(value: &json::Value, at: &str) -> Result<agenticspec_types::session::Decider, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `PermissionMode`, `Classifier`, `Hook`, `Human`, `SafetyCheck`, `Rule`, `NotRecorded`")? {
        "PermissionMode" => agenticspec_types::session::Decider::PermissionMode,
        "Classifier" => agenticspec_types::session::Decider::Classifier,
        "Hook" => agenticspec_types::session::Decider::Hook,
        "Human" => agenticspec_types::session::Decider::Human,
        "SafetyCheck" => agenticspec_types::session::Decider::SafetyCheck,
        "Rule" => agenticspec_types::session::Decider::Rule,
        "NotRecorded" => agenticspec_types::session::Decider::NotRecorded,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `PermissionMode`, `Classifier`, `Hook`, `Human`, `SafetyCheck`, `Rule`, `NotRecorded`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.Decision` as JSON.
pub fn encode_agenticspec_session_decision(value: &agenticspec_types::session::Decision, out: &mut String) {
    match value {
        agenticspec_types::session::Decision::Accept => json::push_text(out, "Accept"),
        agenticspec_types::session::Decision::Refuse => json::push_text(out, "Refuse"),
    }
}

/// Reads `agenticspec.session.Decision` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_decision(value: &json::Value, at: &str) -> Result<agenticspec_types::session::Decision, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Accept`, `Refuse`")? {
        "Accept" => agenticspec_types::session::Decision::Accept,
        "Refuse" => agenticspec_types::session::Decision::Refuse,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Accept`, `Refuse`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.Exposure.State` as JSON.
pub fn encode_agenticspec_session_exposure_state(value: &agenticspec_types::session::ExposureState, out: &mut String) {
    match value {
        agenticspec_types::session::ExposureState::Recorded => json::push_text(out, "Recorded"),
    }
}

/// Reads `agenticspec.session.Exposure.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_exposure_state(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ExposureState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Recorded`")? {
        "Recorded" => agenticspec_types::session::ExposureState::Recorded,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Recorded`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.ExposureChannel` as JSON.
pub fn encode_agenticspec_session_exposure_channel(value: &agenticspec_types::session::ExposureChannel, out: &mut String) {
    match value {
        agenticspec_types::session::ExposureChannel::Prompt => json::push_text(out, "Prompt"),
        agenticspec_types::session::ExposureChannel::ToolInput => json::push_text(out, "ToolInput"),
        agenticspec_types::session::ExposureChannel::ToolOutput => json::push_text(out, "ToolOutput"),
        agenticspec_types::session::ExposureChannel::AgentText => json::push_text(out, "AgentText"),
    }
}

/// Reads `agenticspec.session.ExposureChannel` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_exposure_channel(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ExposureChannel, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Prompt`, `ToolInput`, `ToolOutput`, `AgentText`")? {
        "Prompt" => agenticspec_types::session::ExposureChannel::Prompt,
        "ToolInput" => agenticspec_types::session::ExposureChannel::ToolInput,
        "ToolOutput" => agenticspec_types::session::ExposureChannel::ToolOutput,
        "AgentText" => agenticspec_types::session::ExposureChannel::AgentText,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Prompt`, `ToolInput`, `ToolOutput`, `AgentText`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.ExposureClass` as JSON.
pub fn encode_agenticspec_session_exposure_class(value: &agenticspec_types::session::ExposureClass, out: &mut String) {
    match value {
        agenticspec_types::session::ExposureClass::Secret => json::push_text(out, "Secret"),
        agenticspec_types::session::ExposureClass::PersonalData => json::push_text(out, "PersonalData"),
    }
}

/// Reads `agenticspec.session.ExposureClass` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_exposure_class(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ExposureClass, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Secret`, `PersonalData`")? {
        "Secret" => agenticspec_types::session::ExposureClass::Secret,
        "PersonalData" => agenticspec_types::session::ExposureClass::PersonalData,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Secret`, `PersonalData`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.ExposureId` as JSON.
pub fn encode_agenticspec_session_exposure_id(value: &agenticspec_types::session::ExposureId, out: &mut String) {
    json::push_text(out, &value.0);
}

/// Reads `agenticspec.session.ExposureId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_exposure_id(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ExposureId, json::DecodeError> {
    Ok(agenticspec_types::session::ExposureId(json::text_at(value, at, "a string")?.to_owned()))
}

/// Writes `agenticspec.session.Harness` as JSON.
pub fn encode_agenticspec_session_harness(value: &agenticspec_types::session::Harness, out: &mut String) {
    match value {
        agenticspec_types::session::Harness::ClaudeCode => json::push_text(out, "ClaudeCode"),
        agenticspec_types::session::Harness::Codex => json::push_text(out, "Codex"),
        agenticspec_types::session::Harness::Other => json::push_text(out, "Other"),
    }
}

/// Reads `agenticspec.session.Harness` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_harness(value: &json::Value, at: &str) -> Result<agenticspec_types::session::Harness, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `ClaudeCode`, `Codex`, `Other`")? {
        "ClaudeCode" => agenticspec_types::session::Harness::ClaudeCode,
        "Codex" => agenticspec_types::session::Harness::Codex,
        "Other" => agenticspec_types::session::Harness::Other,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `ClaudeCode`, `Codex`, `Other`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.IncompleteReason` as JSON.
pub fn encode_agenticspec_session_incomplete_reason(value: &agenticspec_types::session::IncompleteReason, out: &mut String) {
    match value {
        agenticspec_types::session::IncompleteReason::ClientDisconnected => json::push_text(out, "client_disconnected"),
        agenticspec_types::session::IncompleteReason::UpstreamCut => json::push_text(out, "upstream_cut"),
        agenticspec_types::session::IncompleteReason::WireError => json::push_text(out, "wire_error"),
        agenticspec_types::session::IncompleteReason::Refused => json::push_text(out, "refused"),
    }
}

/// Reads `agenticspec.session.IncompleteReason` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_incomplete_reason(value: &json::Value, at: &str) -> Result<agenticspec_types::session::IncompleteReason, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `client_disconnected`, `upstream_cut`, `wire_error`, `refused`")? {
        "client_disconnected" => agenticspec_types::session::IncompleteReason::ClientDisconnected,
        "upstream_cut" => agenticspec_types::session::IncompleteReason::UpstreamCut,
        "wire_error" => agenticspec_types::session::IncompleteReason::WireError,
        "refused" => agenticspec_types::session::IncompleteReason::Refused,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `client_disconnected`, `upstream_cut`, `wire_error`, `refused`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.PromptAuthor` as JSON.
pub fn encode_agenticspec_session_prompt_author(value: &agenticspec_types::session::PromptAuthor, out: &mut String) {
    match value {
        agenticspec_types::session::PromptAuthor::Human => json::push_text(out, "Human"),
        agenticspec_types::session::PromptAuthor::Delegate => json::push_text(out, "Delegate"),
        agenticspec_types::session::PromptAuthor::PeerSession => json::push_text(out, "PeerSession"),
        agenticspec_types::session::PromptAuthor::ParentAgent => json::push_text(out, "ParentAgent"),
        agenticspec_types::session::PromptAuthor::Harness => json::push_text(out, "Harness"),
        agenticspec_types::session::PromptAuthor::Hook => json::push_text(out, "Hook"),
    }
}

/// Reads `agenticspec.session.PromptAuthor` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_prompt_author(value: &json::Value, at: &str) -> Result<agenticspec_types::session::PromptAuthor, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Human`, `Delegate`, `PeerSession`, `ParentAgent`, `Harness`, `Hook`")? {
        "Human" => agenticspec_types::session::PromptAuthor::Human,
        "Delegate" => agenticspec_types::session::PromptAuthor::Delegate,
        "PeerSession" => agenticspec_types::session::PromptAuthor::PeerSession,
        "ParentAgent" => agenticspec_types::session::PromptAuthor::ParentAgent,
        "Harness" => agenticspec_types::session::PromptAuthor::Harness,
        "Hook" => agenticspec_types::session::PromptAuthor::Hook,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Human`, `Delegate`, `PeerSession`, `ParentAgent`, `Harness`, `Hook`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.PromptClass` as JSON.
pub fn encode_agenticspec_session_prompt_class(value: &agenticspec_types::session::PromptClass, out: &mut String) {
    match value {
        agenticspec_types::session::PromptClass::Task => json::push_text(out, "Task"),
        agenticspec_types::session::PromptClass::Correction => json::push_text(out, "Correction"),
        agenticspec_types::session::PromptClass::Approval => json::push_text(out, "Approval"),
        agenticspec_types::session::PromptClass::Question => json::push_text(out, "Question"),
        agenticspec_types::session::PromptClass::Continue => json::push_text(out, "Continue"),
        agenticspec_types::session::PromptClass::Unknown => json::push_text(out, "Unknown"),
    }
}

/// Reads `agenticspec.session.PromptClass` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_prompt_class(value: &json::Value, at: &str) -> Result<agenticspec_types::session::PromptClass, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Task`, `Correction`, `Approval`, `Question`, `Continue`, `Unknown`")? {
        "Task" => agenticspec_types::session::PromptClass::Task,
        "Correction" => agenticspec_types::session::PromptClass::Correction,
        "Approval" => agenticspec_types::session::PromptClass::Approval,
        "Question" => agenticspec_types::session::PromptClass::Question,
        "Continue" => agenticspec_types::session::PromptClass::Continue,
        "Unknown" => agenticspec_types::session::PromptClass::Unknown,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Task`, `Correction`, `Approval`, `Question`, `Continue`, `Unknown`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.Session.State` as JSON.
pub fn encode_agenticspec_session_session_state(value: &agenticspec_types::session::SessionState, out: &mut String) {
    match value {
        agenticspec_types::session::SessionState::Ended => json::push_text(out, "Ended"),
        agenticspec_types::session::SessionState::Open => json::push_text(out, "Open"),
    }
}

/// Reads `agenticspec.session.Session.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_session_state(value: &json::Value, at: &str) -> Result<agenticspec_types::session::SessionState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Ended`, `Open`")? {
        "Ended" => agenticspec_types::session::SessionState::Ended,
        "Open" => agenticspec_types::session::SessionState::Open,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Ended`, `Open`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.SessionId` as JSON.
pub fn encode_agenticspec_session_session_id(value: &agenticspec_types::session::SessionId, out: &mut String) {
    json::push_text(out, &value.0);
}

/// Reads `agenticspec.session.SessionId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_session_id(value: &json::Value, at: &str) -> Result<agenticspec_types::session::SessionId, json::DecodeError> {
    Ok(agenticspec_types::session::SessionId(json::text_at(value, at, "a string")?.to_owned()))
}

/// Writes `agenticspec.session.Source` as JSON.
pub fn encode_agenticspec_session_source(value: &agenticspec_types::session::Source, out: &mut String) {
    match value {
        agenticspec_types::session::Source::Disk => json::push_text(out, "Disk"),
        agenticspec_types::session::Source::Wire => json::push_text(out, "Wire"),
    }
}

/// Reads `agenticspec.session.Source` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_source(value: &json::Value, at: &str) -> Result<agenticspec_types::session::Source, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Disk`, `Wire`")? {
        "Disk" => agenticspec_types::session::Source::Disk,
        "Wire" => agenticspec_types::session::Source::Wire,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Disk`, `Wire`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.ToolCall.State` as JSON.
pub fn encode_agenticspec_session_tool_call_state(value: &agenticspec_types::session::ToolCallState, out: &mut String) {
    match value {
        agenticspec_types::session::ToolCallState::Accepted => json::push_text(out, "Accepted"),
        agenticspec_types::session::ToolCallState::Failed => json::push_text(out, "Failed"),
        agenticspec_types::session::ToolCallState::Refused => json::push_text(out, "Refused"),
        agenticspec_types::session::ToolCallState::Requested => json::push_text(out, "Requested"),
        agenticspec_types::session::ToolCallState::Succeeded => json::push_text(out, "Succeeded"),
    }
}

/// Reads `agenticspec.session.ToolCall.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_tool_call_state(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ToolCallState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Accepted`, `Failed`, `Refused`, `Requested`, `Succeeded`")? {
        "Accepted" => agenticspec_types::session::ToolCallState::Accepted,
        "Failed" => agenticspec_types::session::ToolCallState::Failed,
        "Refused" => agenticspec_types::session::ToolCallState::Refused,
        "Requested" => agenticspec_types::session::ToolCallState::Requested,
        "Succeeded" => agenticspec_types::session::ToolCallState::Succeeded,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Accepted`, `Failed`, `Refused`, `Requested`, `Succeeded`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.ToolCallId` as JSON.
pub fn encode_agenticspec_session_tool_call_id(value: &agenticspec_types::session::ToolCallId, out: &mut String) {
    json::push_text(out, &value.0);
}

/// Reads `agenticspec.session.ToolCallId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_tool_call_id(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ToolCallId, json::DecodeError> {
    Ok(agenticspec_types::session::ToolCallId(json::text_at(value, at, "a string")?.to_owned()))
}

/// Writes `agenticspec.session.ToolCategory` as JSON.
pub fn encode_agenticspec_session_tool_category(value: &agenticspec_types::session::ToolCategory, out: &mut String) {
    match value {
        agenticspec_types::session::ToolCategory::Read => json::push_text(out, "Read"),
        agenticspec_types::session::ToolCategory::Search => json::push_text(out, "Search"),
        agenticspec_types::session::ToolCategory::Edit => json::push_text(out, "Edit"),
        agenticspec_types::session::ToolCategory::Execute => json::push_text(out, "Execute"),
        agenticspec_types::session::ToolCategory::Network => json::push_text(out, "Network"),
        agenticspec_types::session::ToolCategory::Spawn => json::push_text(out, "Spawn"),
        agenticspec_types::session::ToolCategory::Message => json::push_text(out, "Message"),
        agenticspec_types::session::ToolCategory::Plan => json::push_text(out, "Plan"),
        agenticspec_types::session::ToolCategory::Other => json::push_text(out, "Other"),
    }
}

/// Reads `agenticspec.session.ToolCategory` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_tool_category(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ToolCategory, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Read`, `Search`, `Edit`, `Execute`, `Network`, `Spawn`, `Message`, `Plan`, `Other`")? {
        "Read" => agenticspec_types::session::ToolCategory::Read,
        "Search" => agenticspec_types::session::ToolCategory::Search,
        "Edit" => agenticspec_types::session::ToolCategory::Edit,
        "Execute" => agenticspec_types::session::ToolCategory::Execute,
        "Network" => agenticspec_types::session::ToolCategory::Network,
        "Spawn" => agenticspec_types::session::ToolCategory::Spawn,
        "Message" => agenticspec_types::session::ToolCategory::Message,
        "Plan" => agenticspec_types::session::ToolCategory::Plan,
        "Other" => agenticspec_types::session::ToolCategory::Other,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Read`, `Search`, `Edit`, `Execute`, `Network`, `Spawn`, `Message`, `Plan`, `Other`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.session.Usage` as JSON.
pub fn encode_agenticspec_session_usage(value: &agenticspec_types::session::Usage, out: &mut String) {
    out.push('{');
    json::member(out, "uncached_input");
    json::push_integer(out, value.uncached_input);
    json::member(out, "cache_read");
    json::push_integer(out, value.cache_read);
    json::member(out, "cache_write_5m");
    json::push_integer(out, value.cache_write_5m);
    json::member(out, "cache_write_1h");
    json::push_integer(out, value.cache_write_1h);
    json::member(out, "output");
    json::push_integer(out, value.output);
    out.push('}');
}

/// Reads `agenticspec.session.Usage` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_usage(value: &json::Value, at: &str) -> Result<agenticspec_types::session::Usage, json::DecodeError> {
    json::closed_at(value, at, &["uncached_input", "cache_read", "cache_write_5m", "cache_write_1h", "output"])?;
    Ok(agenticspec_types::session::Usage {
        uncached_input: {
            let at0 = json::nested(at, "uncached_input");
            let member0 = json::member_at(value, at, "uncached_input")?;
            json::integer_at(member0, &at0, "an integer")?
        },
        cache_read: {
            let at1 = json::nested(at, "cache_read");
            let member1 = json::member_at(value, at, "cache_read")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        cache_write_5m: {
            let at2 = json::nested(at, "cache_write_5m");
            let member2 = json::member_at(value, at, "cache_write_5m")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        cache_write_1h: {
            let at3 = json::nested(at, "cache_write_1h");
            let member3 = json::member_at(value, at, "cache_write_1h")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        output: {
            let at4 = json::nested(at, "output");
            let member4 = json::member_at(value, at, "output")?;
            json::integer_at(member4, &at4, "an integer")?
        },
    })
}

/// Writes `agenticspec.session.Verdict` as JSON.
pub fn encode_agenticspec_session_verdict(value: &agenticspec_types::session::Verdict, out: &mut String) {
    match value {
        agenticspec_types::session::Verdict::Ok => json::push_text(out, "Ok"),
        agenticspec_types::session::Verdict::Gap => json::push_text(out, "Gap"),
        agenticspec_types::session::Verdict::Unk => json::push_text(out, "Unk"),
    }
}

/// Reads `agenticspec.session.Verdict` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_session_verdict(value: &json::Value, at: &str) -> Result<agenticspec_types::session::Verdict, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Ok`, `Gap`, `Unk`")? {
        "Ok" => agenticspec_types::session::Verdict::Ok,
        "Gap" => agenticspec_types::session::Verdict::Gap,
        "Unk" => agenticspec_types::session::Verdict::Unk,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Ok`, `Gap`, `Unk`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.view.Agent` as JSON.
pub fn encode_agenticspec_view_agent(value: &agenticspec_types::view::Agent, out: &mut String) {
    out.push('{');
    json::member(out, "id");
    json::push_text(out, &value.id);
    if let Some(held0) = &value.parent {
        json::member(out, "parent");
        json::push_text(out, &*held0);
    }
    json::member(out, "role");
    json::push_text(out, &value.role);
    if let Some(held0) = &value.spawned_by {
        json::member(out, "spawned_by");
        json::push_text(out, &*held0);
    }
    json::member(out, "started_at");
    json::push_integer(out, value.started_at);
    out.push('}');
}

/// Reads `agenticspec.view.Agent` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_agent(value: &json::Value, at: &str) -> Result<agenticspec_types::view::Agent, json::DecodeError> {
    json::closed_at(value, at, &["id", "parent", "role", "spawned_by", "started_at"])?;
    Ok(agenticspec_types::view::Agent {
        id: {
            let at0 = json::nested(at, "id");
            let member0 = json::member_at(value, at, "id")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        parent: match value.member("parent") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "parent");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
        role: {
            let at2 = json::nested(at, "role");
            let member2 = json::member_at(value, at, "role")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        spawned_by: match value.member("spawned_by") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "spawned_by");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        started_at: {
            let at4 = json::nested(at, "started_at");
            let member4 = json::member_at(value, at, "started_at")?;
            json::integer_at(member4, &at4, "an integer")?
        },
    })
}

/// Writes `agenticspec.view.AgentListing.State` as JSON.
pub fn encode_agenticspec_view_agent_listing_state(value: &agenticspec_types::view::AgentListingState, out: &mut String) {
    match value {
        agenticspec_types::view::AgentListingState::Listed => json::push_text(out, "Listed"),
    }
}

/// Reads `agenticspec.view.AgentListing.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_agent_listing_state(value: &json::Value, at: &str) -> Result<agenticspec_types::view::AgentListingState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Listed`")? {
        "Listed" => agenticspec_types::view::AgentListingState::Listed,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Listed`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.view.BucketId` as JSON.
pub fn encode_agenticspec_view_bucket_id(value: &agenticspec_types::view::BucketId, out: &mut String) {
    json::push_text(out, &value.0);
}

/// Reads `agenticspec.view.BucketId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_bucket_id(value: &json::Value, at: &str) -> Result<agenticspec_types::view::BucketId, json::DecodeError> {
    Ok(agenticspec_types::view::BucketId(json::text_at(value, at, "a string")?.to_owned()))
}

/// Writes `agenticspec.view.CheckId` as JSON.
pub fn encode_agenticspec_view_check_id(value: &agenticspec_types::view::CheckId, out: &mut String) {
    json::push_text(out, &value.0);
}

/// Reads `agenticspec.view.CheckId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_check_id(value: &json::Value, at: &str) -> Result<agenticspec_types::view::CheckId, json::DecodeError> {
    Ok(agenticspec_types::view::CheckId(json::text_at(value, at, "a string")?.to_owned()))
}

/// Writes `agenticspec.view.CountId` as JSON.
pub fn encode_agenticspec_view_count_id(value: &agenticspec_types::view::CountId, out: &mut String) {
    json::push_text(out, &value.0);
}

/// Reads `agenticspec.view.CountId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_count_id(value: &json::Value, at: &str) -> Result<agenticspec_types::view::CountId, json::DecodeError> {
    Ok(agenticspec_types::view::CountId(json::text_at(value, at, "a string")?.to_owned()))
}

/// Writes `agenticspec.view.Document` as JSON.
pub fn encode_agenticspec_view_document(value: &agenticspec_types::view::Document, out: &mut String) {
    out.push('{');
    json::member(out, "format");
    json::push_text(out, &value.format);
    json::member(out, "session_id");
    json::push_text(out, &value.session_id);
    if let Some(held0) = &value.title {
        json::member(out, "title");
        json::push_text(out, &*held0);
    }
    json::member(out, "project");
    json::push_text(out, &value.project);
    json::member(out, "harness");
    json::push_text(out, &value.harness);
    json::member(out, "agents");
    out.push('[');
    for (index0, item0) in value.agents.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_view_agent(&*item0, out);
    }
    out.push(']');
    json::member(out, "events");
    out.push('[');
    for (index0, item0) in value.events.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_view_event(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.view.Document` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_document(value: &json::Value, at: &str) -> Result<agenticspec_types::view::Document, json::DecodeError> {
    json::closed_at(value, at, &["format", "session_id", "title", "project", "harness", "agents", "events"])?;
    Ok(agenticspec_types::view::Document {
        format: {
            let at0 = json::nested(at, "format");
            let member0 = json::member_at(value, at, "format")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        session_id: {
            let at1 = json::nested(at, "session_id");
            let member1 = json::member_at(value, at, "session_id")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        title: match value.member("title") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "title");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
        project: {
            let at3 = json::nested(at, "project");
            let member3 = json::member_at(value, at, "project")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        harness: {
            let at4 = json::nested(at, "harness");
            let member4 = json::member_at(value, at, "harness")?;
            json::text_at(member4, &at4, "a string")?.to_owned()
        },
        agents: {
            let at5 = json::nested(at, "agents");
            let member5 = json::member_at(value, at, "agents")?;
            {
                let mut items5 = Vec::new();
                for (index5, element5) in json::items_at(member5, &at5, "an array")?.iter().enumerate() {
                    let nested5 = json::nested(&at5, &index5.to_string());
                    items5.push(decode_agenticspec_view_agent(element5, &nested5)?);
                }
                items5
            }
        },
        events: {
            let at6 = json::nested(at, "events");
            let member6 = json::member_at(value, at, "events")?;
            {
                let mut items6 = Vec::new();
                for (index6, element6) in json::items_at(member6, &at6, "an array")?.iter().enumerate() {
                    let nested6 = json::nested(&at6, &index6.to_string());
                    items6.push(decode_agenticspec_view_event(element6, &nested6)?);
                }
                items6
            }
        },
    })
}

/// Writes `agenticspec.view.Event` as JSON.
pub fn encode_agenticspec_view_event(value: &agenticspec_types::view::Event, out: &mut String) {
    out.push('{');
    json::member(out, "seq");
    json::push_integer(out, value.seq);
    json::member(out, "at");
    json::push_integer(out, value.at);
    json::member(out, "agent");
    json::push_text(out, &value.agent);
    json::member(out, "kind");
    encode_agenticspec_view_event_kind(&value.kind, out);
    json::member(out, "label");
    json::push_text(out, &value.label);
    if let Some(held0) = &value.detail {
        json::member(out, "detail");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.call {
        json::member(out, "call");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.ok {
        json::member(out, "ok");
        json::push_bool(out, *held0);
    }
    if let Some(held0) = &value.context_tokens {
        json::member(out, "context_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.output_tokens {
        json::member(out, "output_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cache_read_tokens {
        json::member(out, "cache_read_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cache_write_tokens {
        json::member(out, "cache_write_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cost_microdollars {
        json::member(out, "cost_microdollars");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.bytes {
        json::member(out, "bytes");
        json::push_integer(out, *held0);
    }
    out.push('}');
}

/// Reads `agenticspec.view.Event` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_event(value: &json::Value, at: &str) -> Result<agenticspec_types::view::Event, json::DecodeError> {
    json::closed_at(value, at, &["seq", "at", "agent", "kind", "label", "detail", "call", "ok", "context_tokens", "output_tokens", "cache_read_tokens", "cache_write_tokens", "cost_microdollars", "bytes"])?;
    Ok(agenticspec_types::view::Event {
        seq: {
            let at0 = json::nested(at, "seq");
            let member0 = json::member_at(value, at, "seq")?;
            json::integer_at(member0, &at0, "an integer")?
        },
        at: {
            let at1 = json::nested(at, "at");
            let member1 = json::member_at(value, at, "at")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        agent: {
            let at2 = json::nested(at, "agent");
            let member2 = json::member_at(value, at, "agent")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        kind: {
            let at3 = json::nested(at, "kind");
            let member3 = json::member_at(value, at, "kind")?;
            decode_agenticspec_view_event_kind(member3, &at3)?
        },
        label: {
            let at4 = json::nested(at, "label");
            let member4 = json::member_at(value, at, "label")?;
            json::text_at(member4, &at4, "a string")?.to_owned()
        },
        detail: match value.member("detail") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "detail");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
        call: match value.member("call") {
            None | Some(json::Value::Null) => None,
            Some(member6) => {
                let at6 = json::nested(at, "call");
                Some(json::text_at(member6, &at6, "a string")?.to_owned())
            }
        },
        ok: match value.member("ok") {
            None | Some(json::Value::Null) => None,
            Some(member7) => {
                let at7 = json::nested(at, "ok");
                Some(json::bool_at(member7, &at7, "a boolean")?)
            }
        },
        context_tokens: match value.member("context_tokens") {
            None | Some(json::Value::Null) => None,
            Some(member8) => {
                let at8 = json::nested(at, "context_tokens");
                Some(json::integer_at(member8, &at8, "an integer")?)
            }
        },
        output_tokens: match value.member("output_tokens") {
            None | Some(json::Value::Null) => None,
            Some(member9) => {
                let at9 = json::nested(at, "output_tokens");
                Some(json::integer_at(member9, &at9, "an integer")?)
            }
        },
        cache_read_tokens: match value.member("cache_read_tokens") {
            None | Some(json::Value::Null) => None,
            Some(member10) => {
                let at10 = json::nested(at, "cache_read_tokens");
                Some(json::integer_at(member10, &at10, "an integer")?)
            }
        },
        cache_write_tokens: match value.member("cache_write_tokens") {
            None | Some(json::Value::Null) => None,
            Some(member11) => {
                let at11 = json::nested(at, "cache_write_tokens");
                Some(json::integer_at(member11, &at11, "an integer")?)
            }
        },
        cost_microdollars: match value.member("cost_microdollars") {
            None | Some(json::Value::Null) => None,
            Some(member12) => {
                let at12 = json::nested(at, "cost_microdollars");
                Some(json::integer_at(member12, &at12, "an integer")?)
            }
        },
        bytes: match value.member("bytes") {
            None | Some(json::Value::Null) => None,
            Some(member13) => {
                let at13 = json::nested(at, "bytes");
                Some(json::integer_at(member13, &at13, "an integer")?)
            }
        },
    })
}

/// Writes `agenticspec.view.EventCount.State` as JSON.
pub fn encode_agenticspec_view_event_count_state(value: &agenticspec_types::view::EventCountState, out: &mut String) {
    match value {
        agenticspec_types::view::EventCountState::Counted => json::push_text(out, "Counted"),
    }
}

/// Reads `agenticspec.view.EventCount.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_event_count_state(value: &json::Value, at: &str) -> Result<agenticspec_types::view::EventCountState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Counted`")? {
        "Counted" => agenticspec_types::view::EventCountState::Counted,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Counted`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.view.EventId` as JSON.
pub fn encode_agenticspec_view_event_id(value: &agenticspec_types::view::EventId, out: &mut String) {
    json::push_text(out, &value.0);
}

/// Reads `agenticspec.view.EventId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_event_id(value: &json::Value, at: &str) -> Result<agenticspec_types::view::EventId, json::DecodeError> {
    Ok(agenticspec_types::view::EventId(json::text_at(value, at, "a string")?.to_owned()))
}

/// Writes `agenticspec.view.EventKind` as JSON.
pub fn encode_agenticspec_view_event_kind(value: &agenticspec_types::view::EventKind, out: &mut String) {
    match value {
        agenticspec_types::view::EventKind::AgentStarted => json::push_text(out, "agent_started"),
        agenticspec_types::view::EventKind::Prompt => json::push_text(out, "prompt"),
        agenticspec_types::view::EventKind::ModelTurn => json::push_text(out, "model_turn"),
        agenticspec_types::view::EventKind::ToolCall => json::push_text(out, "tool_call"),
        agenticspec_types::view::EventKind::Decision => json::push_text(out, "decision"),
        agenticspec_types::view::EventKind::ToolResult => json::push_text(out, "tool_result"),
        agenticspec_types::view::EventKind::TurnEnd => json::push_text(out, "turn_end"),
        agenticspec_types::view::EventKind::Compaction => json::push_text(out, "compaction"),
        agenticspec_types::view::EventKind::Exposure => json::push_text(out, "exposure"),
        agenticspec_types::view::EventKind::CacheBust => json::push_text(out, "cache_bust"),
    }
}

/// Reads `agenticspec.view.EventKind` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_event_kind(value: &json::Value, at: &str) -> Result<agenticspec_types::view::EventKind, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `agent_started`, `prompt`, `model_turn`, `tool_call`, `decision`, `tool_result`, `turn_end`, `compaction`, `exposure`, `cache_bust`")? {
        "agent_started" => agenticspec_types::view::EventKind::AgentStarted,
        "prompt" => agenticspec_types::view::EventKind::Prompt,
        "model_turn" => agenticspec_types::view::EventKind::ModelTurn,
        "tool_call" => agenticspec_types::view::EventKind::ToolCall,
        "decision" => agenticspec_types::view::EventKind::Decision,
        "tool_result" => agenticspec_types::view::EventKind::ToolResult,
        "turn_end" => agenticspec_types::view::EventKind::TurnEnd,
        "compaction" => agenticspec_types::view::EventKind::Compaction,
        "exposure" => agenticspec_types::view::EventKind::Exposure,
        "cache_bust" => agenticspec_types::view::EventKind::CacheBust,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `agent_started`, `prompt`, `model_turn`, `tool_call`, `decision`, `tool_result`, `turn_end`, `compaction`, `exposure`, `cache_bust`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.view.EventRow.State` as JSON.
pub fn encode_agenticspec_view_event_row_state(value: &agenticspec_types::view::EventRowState, out: &mut String) {
    match value {
        agenticspec_types::view::EventRowState::Logged => json::push_text(out, "Logged"),
    }
}

/// Reads `agenticspec.view.EventRow.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_event_row_state(value: &json::Value, at: &str) -> Result<agenticspec_types::view::EventRowState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Logged`")? {
        "Logged" => agenticspec_types::view::EventRowState::Logged,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Logged`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.view.FigureBucket.State` as JSON.
pub fn encode_agenticspec_view_figure_bucket_state(value: &agenticspec_types::view::FigureBucketState, out: &mut String) {
    match value {
        agenticspec_types::view::FigureBucketState::Counted => json::push_text(out, "Counted"),
    }
}

/// Reads `agenticspec.view.FigureBucket.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_figure_bucket_state(value: &json::Value, at: &str) -> Result<agenticspec_types::view::FigureBucketState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Counted`")? {
        "Counted" => agenticspec_types::view::FigureBucketState::Counted,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Counted`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.view.SessionCheck.State` as JSON.
pub fn encode_agenticspec_view_session_check_state(value: &agenticspec_types::view::SessionCheckState, out: &mut String) {
    match value {
        agenticspec_types::view::SessionCheckState::Judged => json::push_text(out, "Judged"),
    }
}

/// Reads `agenticspec.view.SessionCheck.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_session_check_state(value: &json::Value, at: &str) -> Result<agenticspec_types::view::SessionCheckState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Judged`")? {
        "Judged" => agenticspec_types::view::SessionCheckState::Judged,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Judged`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.view.SessionListing.State` as JSON.
pub fn encode_agenticspec_view_session_listing_state(value: &agenticspec_types::view::SessionListingState, out: &mut String) {
    match value {
        agenticspec_types::view::SessionListingState::Listed => json::push_text(out, "Listed"),
    }
}

/// Reads `agenticspec.view.SessionListing.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_session_listing_state(value: &json::Value, at: &str) -> Result<agenticspec_types::view::SessionListingState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Listed`")? {
        "Listed" => agenticspec_types::view::SessionListingState::Listed,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Listed`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.view.SlotId` as JSON.
pub fn encode_agenticspec_view_slot_id(value: &agenticspec_types::view::SlotId, out: &mut String) {
    json::push_text(out, &value.0);
}

/// Reads `agenticspec.view.SlotId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_slot_id(value: &json::Value, at: &str) -> Result<agenticspec_types::view::SlotId, json::DecodeError> {
    Ok(agenticspec_types::view::SlotId(json::text_at(value, at, "a string")?.to_owned()))
}

/// Writes `agenticspec.view.TimelineSlot.State` as JSON.
pub fn encode_agenticspec_view_timeline_slot_state(value: &agenticspec_types::view::TimelineSlotState, out: &mut String) {
    match value {
        agenticspec_types::view::TimelineSlotState::Counted => json::push_text(out, "Counted"),
    }
}

/// Reads `agenticspec.view.TimelineSlot.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_timeline_slot_state(value: &json::Value, at: &str) -> Result<agenticspec_types::view::TimelineSlotState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Counted`")? {
        "Counted" => agenticspec_types::view::TimelineSlotState::Counted,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Counted`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agenticspec.view.Update` as JSON.
pub fn encode_agenticspec_view_update(value: &agenticspec_types::view::Update, out: &mut String) {
    out.push('{');
    json::member(out, "format");
    json::push_text(out, &value.format);
    json::member(out, "session_id");
    json::push_text(out, &value.session_id);
    json::member(out, "reset");
    json::push_bool(out, value.reset);
    json::member(out, "from_seq");
    json::push_integer(out, value.from_seq);
    json::member(out, "agents");
    out.push('[');
    for (index0, item0) in value.agents.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_view_agent(&*item0, out);
    }
    out.push(']');
    json::member(out, "events");
    out.push('[');
    for (index0, item0) in value.events.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agenticspec_view_event(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agenticspec.view.Update` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agenticspec_view_update(value: &json::Value, at: &str) -> Result<agenticspec_types::view::Update, json::DecodeError> {
    json::closed_at(value, at, &["format", "session_id", "reset", "from_seq", "agents", "events"])?;
    Ok(agenticspec_types::view::Update {
        format: {
            let at0 = json::nested(at, "format");
            let member0 = json::member_at(value, at, "format")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        session_id: {
            let at1 = json::nested(at, "session_id");
            let member1 = json::member_at(value, at, "session_id")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        reset: {
            let at2 = json::nested(at, "reset");
            let member2 = json::member_at(value, at, "reset")?;
            json::bool_at(member2, &at2, "a boolean")?
        },
        from_seq: {
            let at3 = json::nested(at, "from_seq");
            let member3 = json::member_at(value, at, "from_seq")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        agents: {
            let at4 = json::nested(at, "agents");
            let member4 = json::member_at(value, at, "agents")?;
            {
                let mut items4 = Vec::new();
                for (index4, element4) in json::items_at(member4, &at4, "an array")?.iter().enumerate() {
                    let nested4 = json::nested(&at4, &index4.to_string());
                    items4.push(decode_agenticspec_view_agent(element4, &nested4)?);
                }
                items4
            }
        },
        events: {
            let at5 = json::nested(at, "events");
            let member5 = json::member_at(value, at, "events")?;
            {
                let mut items5 = Vec::new();
                for (index5, element5) in json::items_at(member5, &at5, "an array")?.iter().enumerate() {
                    let nested5 = json::nested(&at5, &index5.to_string());
                    items5.push(decode_agenticspec_view_event(element5, &nested5)?);
                }
                items5
            }
        },
    })
}

/// Writes the event `agenticspec.session.AgentEnded` as JSON.
pub fn encode_event_agenticspec_session_agent_ended(value: &agenticspec_types::session::AgentEnded, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the event `agenticspec.session.AgentEnded` from JSON, the payload [`encode_event_agenticspec_session_agent_ended`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_session_agent_ended(value: &json::Value, at: &str) -> Result<agenticspec_types::session::AgentEnded, json::DecodeError> {
    Ok(agenticspec_types::session::AgentEnded {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        at: {
            let at1 = json::nested(at, "at");
            let member1 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member1, &at1, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the event `agenticspec.session.AgentStarted` as JSON.
pub fn encode_event_agenticspec_session_agent_started(value: &agenticspec_types::session::AgentStarted, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    if let Some(held0) = &value.parent_agent_id {
        json::member(out, "parent_agent_id");
        encode_agenticspec_session_agent_id(&*held0, out);
    }
    json::member(out, "role");
    encode_agenticspec_session_agent_role(&value.role, out);
    if let Some(held0) = &value.spawned_by {
        json::member(out, "spawned_by");
        encode_agenticspec_session_tool_call_id(&*held0, out);
    }
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the event `agenticspec.session.AgentStarted` from JSON, the payload [`encode_event_agenticspec_session_agent_started`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_session_agent_started(value: &json::Value, at: &str) -> Result<agenticspec_types::session::AgentStarted, json::DecodeError> {
    Ok(agenticspec_types::session::AgentStarted {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        session_id: {
            let at1 = json::nested(at, "session_id");
            let member1 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member1, &at1)?
        },
        parent_agent_id: match value.member("parent_agent_id") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "parent_agent_id");
                Some(decode_agenticspec_session_agent_id(member2, &at2)?)
            }
        },
        role: {
            let at3 = json::nested(at, "role");
            let member3 = json::member_at(value, at, "role")?;
            decode_agenticspec_session_agent_role(member3, &at3)?
        },
        spawned_by: match value.member("spawned_by") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "spawned_by");
                Some(decode_agenticspec_session_tool_call_id(member4, &at4)?)
            }
        },
        at: {
            let at5 = json::nested(at, "at");
            let member5 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member5, &at5, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the event `agenticspec.session.ContextCompacted` as JSON.
pub fn encode_event_agenticspec_session_context_compacted(value: &agenticspec_types::session::ContextCompacted, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "context_tokens_before");
    json::push_integer(out, value.context_tokens_before);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the event `agenticspec.session.ContextCompacted` from JSON, the payload [`encode_event_agenticspec_session_context_compacted`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_session_context_compacted(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ContextCompacted, json::DecodeError> {
    Ok(agenticspec_types::session::ContextCompacted {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        context_tokens_before: {
            let at1 = json::nested(at, "context_tokens_before");
            let member1 = json::member_at(value, at, "context_tokens_before")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        at: {
            let at2 = json::nested(at, "at");
            let member2 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member2, &at2, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the event `agenticspec.session.ExposureDetected` as JSON.
pub fn encode_event_agenticspec_session_exposure_detected(value: &agenticspec_types::session::ExposureDetected, out: &mut String) {
    out.push('{');
    json::member(out, "exposure_id");
    encode_agenticspec_session_exposure_id(&value.exposure_id, out);
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "channel");
    encode_agenticspec_session_exposure_channel(&value.channel, out);
    json::member(out, "class");
    encode_agenticspec_session_exposure_class(&value.class, out);
    json::member(out, "kind");
    json::push_text(out, &value.kind);
    json::member(out, "alias");
    json::push_text(out, &value.alias);
    json::member(out, "verdict");
    encode_agenticspec_session_verdict(&value.verdict, out);
    if let Some(held0) = &value.tool_call_id {
        json::member(out, "tool_call_id");
        encode_agenticspec_session_tool_call_id(&*held0, out);
    }
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the event `agenticspec.session.ExposureDetected` from JSON, the payload [`encode_event_agenticspec_session_exposure_detected`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_session_exposure_detected(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ExposureDetected, json::DecodeError> {
    Ok(agenticspec_types::session::ExposureDetected {
        exposure_id: {
            let at0 = json::nested(at, "exposure_id");
            let member0 = json::member_at(value, at, "exposure_id")?;
            decode_agenticspec_session_exposure_id(member0, &at0)?
        },
        agent_id: {
            let at1 = json::nested(at, "agent_id");
            let member1 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member1, &at1)?
        },
        channel: {
            let at2 = json::nested(at, "channel");
            let member2 = json::member_at(value, at, "channel")?;
            decode_agenticspec_session_exposure_channel(member2, &at2)?
        },
        class: {
            let at3 = json::nested(at, "class");
            let member3 = json::member_at(value, at, "class")?;
            decode_agenticspec_session_exposure_class(member3, &at3)?
        },
        kind: {
            let at4 = json::nested(at, "kind");
            let member4 = json::member_at(value, at, "kind")?;
            json::text_at(member4, &at4, "a string")?.to_owned()
        },
        alias: {
            let at5 = json::nested(at, "alias");
            let member5 = json::member_at(value, at, "alias")?;
            json::text_at(member5, &at5, "a string")?.to_owned()
        },
        verdict: {
            let at6 = json::nested(at, "verdict");
            let member6 = json::member_at(value, at, "verdict")?;
            decode_agenticspec_session_verdict(member6, &at6)?
        },
        tool_call_id: match value.member("tool_call_id") {
            None | Some(json::Value::Null) => None,
            Some(member7) => {
                let at7 = json::nested(at, "tool_call_id");
                Some(decode_agenticspec_session_tool_call_id(member7, &at7)?)
            }
        },
        at: {
            let at8 = json::nested(at, "at");
            let member8 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member8, &at8, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the event `agenticspec.session.ModelTurnRecorded` as JSON.
pub fn encode_event_agenticspec_session_model_turn_recorded(value: &agenticspec_types::session::ModelTurnRecorded, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "model");
    json::push_text(out, &value.model);
    json::member(out, "usage");
    encode_agenticspec_session_usage(&value.usage, out);
    json::member(out, "context_tokens");
    json::push_integer(out, value.context_tokens);
    if let Some(held0) = &value.message_id {
        json::member(out, "message_id");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.call_id {
        json::member(out, "call_id");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.route {
        json::member(out, "route");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.upstream {
        json::member(out, "upstream");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.attempts {
        json::member(out, "attempts");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.queue_wait_ms {
        json::member(out, "queue_wait_ms");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.ttft_ms {
        json::member(out, "ttft_ms");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.declared_cost_microdollars {
        json::member(out, "declared_cost_microdollars");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.incomplete {
        json::member(out, "incomplete");
        encode_agenticspec_session_incomplete_reason(&*held0, out);
    }
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the event `agenticspec.session.ModelTurnRecorded` from JSON, the payload [`encode_event_agenticspec_session_model_turn_recorded`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_session_model_turn_recorded(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ModelTurnRecorded, json::DecodeError> {
    Ok(agenticspec_types::session::ModelTurnRecorded {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        model: {
            let at1 = json::nested(at, "model");
            let member1 = json::member_at(value, at, "model")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        usage: {
            let at2 = json::nested(at, "usage");
            let member2 = json::member_at(value, at, "usage")?;
            decode_agenticspec_session_usage(member2, &at2)?
        },
        context_tokens: {
            let at3 = json::nested(at, "context_tokens");
            let member3 = json::member_at(value, at, "context_tokens")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        message_id: match value.member("message_id") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "message_id");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
        call_id: match value.member("call_id") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "call_id");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
        route: match value.member("route") {
            None | Some(json::Value::Null) => None,
            Some(member6) => {
                let at6 = json::nested(at, "route");
                Some(json::text_at(member6, &at6, "a string")?.to_owned())
            }
        },
        upstream: match value.member("upstream") {
            None | Some(json::Value::Null) => None,
            Some(member7) => {
                let at7 = json::nested(at, "upstream");
                Some(json::text_at(member7, &at7, "a string")?.to_owned())
            }
        },
        attempts: match value.member("attempts") {
            None | Some(json::Value::Null) => None,
            Some(member8) => {
                let at8 = json::nested(at, "attempts");
                Some(json::integer_at(member8, &at8, "an integer")?)
            }
        },
        queue_wait_ms: match value.member("queue_wait_ms") {
            None | Some(json::Value::Null) => None,
            Some(member9) => {
                let at9 = json::nested(at, "queue_wait_ms");
                Some(json::integer_at(member9, &at9, "an integer")?)
            }
        },
        ttft_ms: match value.member("ttft_ms") {
            None | Some(json::Value::Null) => None,
            Some(member10) => {
                let at10 = json::nested(at, "ttft_ms");
                Some(json::integer_at(member10, &at10, "an integer")?)
            }
        },
        declared_cost_microdollars: match value.member("declared_cost_microdollars") {
            None | Some(json::Value::Null) => None,
            Some(member11) => {
                let at11 = json::nested(at, "declared_cost_microdollars");
                Some(json::integer_at(member11, &at11, "an integer")?)
            }
        },
        incomplete: match value.member("incomplete") {
            None | Some(json::Value::Null) => None,
            Some(member12) => {
                let at12 = json::nested(at, "incomplete");
                Some(decode_agenticspec_session_incomplete_reason(member12, &at12)?)
            }
        },
        at: {
            let at13 = json::nested(at, "at");
            let member13 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member13, &at13, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the event `agenticspec.session.PromptSubmitted` as JSON.
pub fn encode_event_agenticspec_session_prompt_submitted(value: &agenticspec_types::session::PromptSubmitted, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "author");
    encode_agenticspec_session_prompt_author(&value.author, out);
    json::member(out, "attribution");
    encode_agenticspec_session_attribution(&value.attribution, out);
    json::member(out, "class");
    encode_agenticspec_session_prompt_class(&value.class, out);
    json::member(out, "content");
    encode_agenticspec_session_content(&value.content, out);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the event `agenticspec.session.PromptSubmitted` from JSON, the payload [`encode_event_agenticspec_session_prompt_submitted`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_session_prompt_submitted(value: &json::Value, at: &str) -> Result<agenticspec_types::session::PromptSubmitted, json::DecodeError> {
    Ok(agenticspec_types::session::PromptSubmitted {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        author: {
            let at1 = json::nested(at, "author");
            let member1 = json::member_at(value, at, "author")?;
            decode_agenticspec_session_prompt_author(member1, &at1)?
        },
        attribution: {
            let at2 = json::nested(at, "attribution");
            let member2 = json::member_at(value, at, "attribution")?;
            decode_agenticspec_session_attribution(member2, &at2)?
        },
        class: {
            let at3 = json::nested(at, "class");
            let member3 = json::member_at(value, at, "class")?;
            decode_agenticspec_session_prompt_class(member3, &at3)?
        },
        content: {
            let at4 = json::nested(at, "content");
            let member4 = json::member_at(value, at, "content")?;
            decode_agenticspec_session_content(member4, &at4)?
        },
        at: {
            let at5 = json::nested(at, "at");
            let member5 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member5, &at5, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the event `agenticspec.session.SessionEnded` as JSON.
pub fn encode_event_agenticspec_session_session_ended(value: &agenticspec_types::session::SessionEnded, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the event `agenticspec.session.SessionEnded` from JSON, the payload [`encode_event_agenticspec_session_session_ended`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_session_session_ended(value: &json::Value, at: &str) -> Result<agenticspec_types::session::SessionEnded, json::DecodeError> {
    Ok(agenticspec_types::session::SessionEnded {
        session_id: {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
        at: {
            let at1 = json::nested(at, "at");
            let member1 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member1, &at1, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the event `agenticspec.session.SessionOpened` as JSON.
pub fn encode_event_agenticspec_session_session_opened(value: &agenticspec_types::session::SessionOpened, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "harness");
    encode_agenticspec_session_harness(&value.harness, out);
    if let Some(held0) = &value.source {
        json::member(out, "source");
        encode_agenticspec_session_source(&*held0, out);
    }
    if let Some(held0) = &value.session_hash {
        json::member(out, "session_hash");
        json::push_text(out, &*held0);
    }
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the event `agenticspec.session.SessionOpened` from JSON, the payload [`encode_event_agenticspec_session_session_opened`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_session_session_opened(value: &json::Value, at: &str) -> Result<agenticspec_types::session::SessionOpened, json::DecodeError> {
    Ok(agenticspec_types::session::SessionOpened {
        session_id: {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
        harness: {
            let at1 = json::nested(at, "harness");
            let member1 = json::member_at(value, at, "harness")?;
            decode_agenticspec_session_harness(member1, &at1)?
        },
        source: match value.member("source") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "source");
                Some(decode_agenticspec_session_source(member2, &at2)?)
            }
        },
        session_hash: match value.member("session_hash") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "session_hash");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        at: {
            let at4 = json::nested(at, "at");
            let member4 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member4, &at4, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the event `agenticspec.session.ToolCallCompleted` as JSON.
pub fn encode_event_agenticspec_session_tool_call_completed(value: &agenticspec_types::session::ToolCallCompleted, out: &mut String) {
    out.push('{');
    json::member(out, "tool_call_id");
    encode_agenticspec_session_tool_call_id(&value.tool_call_id, out);
    json::member(out, "ok");
    json::push_bool(out, value.ok);
    json::member(out, "output");
    encode_agenticspec_session_content(&value.output, out);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the event `agenticspec.session.ToolCallCompleted` from JSON, the payload [`encode_event_agenticspec_session_tool_call_completed`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_session_tool_call_completed(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ToolCallCompleted, json::DecodeError> {
    Ok(agenticspec_types::session::ToolCallCompleted {
        tool_call_id: {
            let at0 = json::nested(at, "tool_call_id");
            let member0 = json::member_at(value, at, "tool_call_id")?;
            decode_agenticspec_session_tool_call_id(member0, &at0)?
        },
        ok: {
            let at1 = json::nested(at, "ok");
            let member1 = json::member_at(value, at, "ok")?;
            json::bool_at(member1, &at1, "a boolean")?
        },
        output: {
            let at2 = json::nested(at, "output");
            let member2 = json::member_at(value, at, "output")?;
            decode_agenticspec_session_content(member2, &at2)?
        },
        at: {
            let at3 = json::nested(at, "at");
            let member3 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member3, &at3, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the event `agenticspec.session.ToolCallDecided` as JSON.
pub fn encode_event_agenticspec_session_tool_call_decided(value: &agenticspec_types::session::ToolCallDecided, out: &mut String) {
    out.push('{');
    json::member(out, "tool_call_id");
    encode_agenticspec_session_tool_call_id(&value.tool_call_id, out);
    json::member(out, "decision");
    encode_agenticspec_session_decision(&value.decision, out);
    json::member(out, "decider");
    encode_agenticspec_session_decider(&value.decider, out);
    if let Some(held0) = &value.reason {
        json::member(out, "reason");
        json::push_text(out, &*held0);
    }
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the event `agenticspec.session.ToolCallDecided` from JSON, the payload [`encode_event_agenticspec_session_tool_call_decided`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_session_tool_call_decided(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ToolCallDecided, json::DecodeError> {
    Ok(agenticspec_types::session::ToolCallDecided {
        tool_call_id: {
            let at0 = json::nested(at, "tool_call_id");
            let member0 = json::member_at(value, at, "tool_call_id")?;
            decode_agenticspec_session_tool_call_id(member0, &at0)?
        },
        decision: {
            let at1 = json::nested(at, "decision");
            let member1 = json::member_at(value, at, "decision")?;
            decode_agenticspec_session_decision(member1, &at1)?
        },
        decider: {
            let at2 = json::nested(at, "decider");
            let member2 = json::member_at(value, at, "decider")?;
            decode_agenticspec_session_decider(member2, &at2)?
        },
        reason: match value.member("reason") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "reason");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        at: {
            let at4 = json::nested(at, "at");
            let member4 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member4, &at4, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the event `agenticspec.session.ToolCallRequested` as JSON.
pub fn encode_event_agenticspec_session_tool_call_requested(value: &agenticspec_types::session::ToolCallRequested, out: &mut String) {
    out.push('{');
    json::member(out, "tool_call_id");
    encode_agenticspec_session_tool_call_id(&value.tool_call_id, out);
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "tool");
    json::push_text(out, &value.tool);
    json::member(out, "category");
    encode_agenticspec_session_tool_category(&value.category, out);
    json::member(out, "input");
    encode_agenticspec_session_content(&value.input, out);
    if let Some(held0) = &value.cwd {
        json::member(out, "cwd");
        json::push_text(out, &*held0);
    }
    json::member(out, "background");
    json::push_bool(out, value.background);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the event `agenticspec.session.ToolCallRequested` from JSON, the payload [`encode_event_agenticspec_session_tool_call_requested`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_session_tool_call_requested(value: &json::Value, at: &str) -> Result<agenticspec_types::session::ToolCallRequested, json::DecodeError> {
    Ok(agenticspec_types::session::ToolCallRequested {
        tool_call_id: {
            let at0 = json::nested(at, "tool_call_id");
            let member0 = json::member_at(value, at, "tool_call_id")?;
            decode_agenticspec_session_tool_call_id(member0, &at0)?
        },
        agent_id: {
            let at1 = json::nested(at, "agent_id");
            let member1 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member1, &at1)?
        },
        tool: {
            let at2 = json::nested(at, "tool");
            let member2 = json::member_at(value, at, "tool")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        category: {
            let at3 = json::nested(at, "category");
            let member3 = json::member_at(value, at, "category")?;
            decode_agenticspec_session_tool_category(member3, &at3)?
        },
        input: {
            let at4 = json::nested(at, "input");
            let member4 = json::member_at(value, at, "input")?;
            decode_agenticspec_session_content(member4, &at4)?
        },
        cwd: match value.member("cwd") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "cwd");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
        background: {
            let at6 = json::nested(at, "background");
            let member6 = json::member_at(value, at, "background")?;
            json::bool_at(member6, &at6, "a boolean")?
        },
        at: {
            let at7 = json::nested(at, "at");
            let member7 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member7, &at7, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the event `agenticspec.session.TurnEnded` as JSON.
pub fn encode_event_agenticspec_session_turn_ended(value: &agenticspec_types::session::TurnEnded, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the event `agenticspec.session.TurnEnded` from JSON, the payload [`encode_event_agenticspec_session_turn_ended`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_session_turn_ended(value: &json::Value, at: &str) -> Result<agenticspec_types::session::TurnEnded, json::DecodeError> {
    Ok(agenticspec_types::session::TurnEnded {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        at: {
            let at1 = json::nested(at, "at");
            let member1 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member1, &at1, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the event `agenticspec.view.AgentUpdated` as JSON.
pub fn encode_event_agenticspec_view_agent_updated(value: &agenticspec_types::view::AgentUpdated, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    if let Some(held0) = &value.parent {
        json::member(out, "parent");
        encode_agenticspec_session_agent_id(&*held0, out);
    }
    json::member(out, "role");
    encode_agenticspec_session_agent_role(&value.role, out);
    if let Some(held0) = &value.model {
        json::member(out, "model");
        json::push_text(out, &*held0);
    }
    json::member(out, "started_at");
    json::push_text(out, &value.started_at.0);
    json::member(out, "turns");
    json::push_integer(out, value.turns);
    json::member(out, "context_tokens");
    json::push_integer(out, value.context_tokens);
    json::member(out, "context_max");
    json::push_integer(out, value.context_max);
    json::member(out, "output_tokens");
    json::push_integer(out, value.output_tokens);
    json::member(out, "cost_usd");
    out.push_str(&value.cost_usd.token());
    json::member(out, "tool_calls");
    json::push_integer(out, value.tool_calls);
    json::member(out, "busts");
    json::push_integer(out, value.busts);
    out.push('}');
}

/// Reads the event `agenticspec.view.AgentUpdated` from JSON, the payload [`encode_event_agenticspec_view_agent_updated`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_view_agent_updated(value: &json::Value, at: &str) -> Result<agenticspec_types::view::AgentUpdated, json::DecodeError> {
    Ok(agenticspec_types::view::AgentUpdated {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        session_id: {
            let at1 = json::nested(at, "session_id");
            let member1 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member1, &at1)?
        },
        parent: match value.member("parent") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "parent");
                Some(decode_agenticspec_session_agent_id(member2, &at2)?)
            }
        },
        role: {
            let at3 = json::nested(at, "role");
            let member3 = json::member_at(value, at, "role")?;
            decode_agenticspec_session_agent_role(member3, &at3)?
        },
        model: match value.member("model") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "model");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
        started_at: {
            let at5 = json::nested(at, "started_at");
            let member5 = json::member_at(value, at, "started_at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member5, &at5, "an RFC 3339 instant")?.to_owned())
        },
        turns: {
            let at6 = json::nested(at, "turns");
            let member6 = json::member_at(value, at, "turns")?;
            json::integer_at(member6, &at6, "an integer")?
        },
        context_tokens: {
            let at7 = json::nested(at, "context_tokens");
            let member7 = json::member_at(value, at, "context_tokens")?;
            json::integer_at(member7, &at7, "an integer")?
        },
        context_max: {
            let at8 = json::nested(at, "context_max");
            let member8 = json::member_at(value, at, "context_max")?;
            json::integer_at(member8, &at8, "an integer")?
        },
        output_tokens: {
            let at9 = json::nested(at, "output_tokens");
            let member9 = json::member_at(value, at, "output_tokens")?;
            json::integer_at(member9, &at9, "an integer")?
        },
        cost_usd: {
            let at10 = json::nested(at, "cost_usd");
            let member10 = json::member_at(value, at, "cost_usd")?;
            binary64_at(member10, &at10)?
        },
        tool_calls: {
            let at11 = json::nested(at, "tool_calls");
            let member11 = json::member_at(value, at, "tool_calls")?;
            json::integer_at(member11, &at11, "an integer")?
        },
        busts: {
            let at12 = json::nested(at, "busts");
            let member12 = json::member_at(value, at, "busts")?;
            json::integer_at(member12, &at12, "an integer")?
        },
    })
}

/// Writes the event `agenticspec.view.EventAppended` as JSON.
pub fn encode_event_agenticspec_view_event_appended(value: &agenticspec_types::view::EventAppended, out: &mut String) {
    out.push('{');
    json::member(out, "event_id");
    encode_agenticspec_view_event_id(&value.event_id, out);
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "seq");
    json::push_integer(out, value.seq);
    json::member(out, "ord");
    json::push_integer(out, value.ord);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    json::member(out, "agent");
    encode_agenticspec_session_agent_id(&value.agent, out);
    json::member(out, "kind");
    encode_agenticspec_view_event_kind(&value.kind, out);
    json::member(out, "notable");
    json::push_bool(out, value.notable);
    json::member(out, "label");
    json::push_text(out, &value.label);
    if let Some(held0) = &value.detail {
        json::member(out, "detail");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.call {
        json::member(out, "call");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.ok {
        json::member(out, "ok");
        json::push_bool(out, *held0);
    }
    if let Some(held0) = &value.context_tokens {
        json::member(out, "context_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.output_tokens {
        json::member(out, "output_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cache_read_tokens {
        json::member(out, "cache_read_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cache_write_tokens {
        json::member(out, "cache_write_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cost_microdollars {
        json::member(out, "cost_microdollars");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.bytes {
        json::member(out, "bytes");
        json::push_integer(out, *held0);
    }
    json::member(out, "agent_cost_usd");
    out.push_str(&value.agent_cost_usd.token());
    json::member(out, "session_cost_usd");
    out.push_str(&value.session_cost_usd.token());
    out.push('}');
}

/// Reads the event `agenticspec.view.EventAppended` from JSON, the payload [`encode_event_agenticspec_view_event_appended`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_view_event_appended(value: &json::Value, at: &str) -> Result<agenticspec_types::view::EventAppended, json::DecodeError> {
    Ok(agenticspec_types::view::EventAppended {
        event_id: {
            let at0 = json::nested(at, "event_id");
            let member0 = json::member_at(value, at, "event_id")?;
            decode_agenticspec_view_event_id(member0, &at0)?
        },
        session_id: {
            let at1 = json::nested(at, "session_id");
            let member1 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member1, &at1)?
        },
        seq: {
            let at2 = json::nested(at, "seq");
            let member2 = json::member_at(value, at, "seq")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        ord: {
            let at3 = json::nested(at, "ord");
            let member3 = json::member_at(value, at, "ord")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        at: {
            let at4 = json::nested(at, "at");
            let member4 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member4, &at4, "an RFC 3339 instant")?.to_owned())
        },
        agent: {
            let at5 = json::nested(at, "agent");
            let member5 = json::member_at(value, at, "agent")?;
            decode_agenticspec_session_agent_id(member5, &at5)?
        },
        kind: {
            let at6 = json::nested(at, "kind");
            let member6 = json::member_at(value, at, "kind")?;
            decode_agenticspec_view_event_kind(member6, &at6)?
        },
        notable: {
            let at7 = json::nested(at, "notable");
            let member7 = json::member_at(value, at, "notable")?;
            json::bool_at(member7, &at7, "a boolean")?
        },
        label: {
            let at8 = json::nested(at, "label");
            let member8 = json::member_at(value, at, "label")?;
            json::text_at(member8, &at8, "a string")?.to_owned()
        },
        detail: match value.member("detail") {
            None | Some(json::Value::Null) => None,
            Some(member9) => {
                let at9 = json::nested(at, "detail");
                Some(json::text_at(member9, &at9, "a string")?.to_owned())
            }
        },
        call: match value.member("call") {
            None | Some(json::Value::Null) => None,
            Some(member10) => {
                let at10 = json::nested(at, "call");
                Some(json::text_at(member10, &at10, "a string")?.to_owned())
            }
        },
        ok: match value.member("ok") {
            None | Some(json::Value::Null) => None,
            Some(member11) => {
                let at11 = json::nested(at, "ok");
                Some(json::bool_at(member11, &at11, "a boolean")?)
            }
        },
        context_tokens: match value.member("context_tokens") {
            None | Some(json::Value::Null) => None,
            Some(member12) => {
                let at12 = json::nested(at, "context_tokens");
                Some(json::integer_at(member12, &at12, "an integer")?)
            }
        },
        output_tokens: match value.member("output_tokens") {
            None | Some(json::Value::Null) => None,
            Some(member13) => {
                let at13 = json::nested(at, "output_tokens");
                Some(json::integer_at(member13, &at13, "an integer")?)
            }
        },
        cache_read_tokens: match value.member("cache_read_tokens") {
            None | Some(json::Value::Null) => None,
            Some(member14) => {
                let at14 = json::nested(at, "cache_read_tokens");
                Some(json::integer_at(member14, &at14, "an integer")?)
            }
        },
        cache_write_tokens: match value.member("cache_write_tokens") {
            None | Some(json::Value::Null) => None,
            Some(member15) => {
                let at15 = json::nested(at, "cache_write_tokens");
                Some(json::integer_at(member15, &at15, "an integer")?)
            }
        },
        cost_microdollars: match value.member("cost_microdollars") {
            None | Some(json::Value::Null) => None,
            Some(member16) => {
                let at16 = json::nested(at, "cost_microdollars");
                Some(json::integer_at(member16, &at16, "an integer")?)
            }
        },
        bytes: match value.member("bytes") {
            None | Some(json::Value::Null) => None,
            Some(member17) => {
                let at17 = json::nested(at, "bytes");
                Some(json::integer_at(member17, &at17, "an integer")?)
            }
        },
        agent_cost_usd: {
            let at18 = json::nested(at, "agent_cost_usd");
            let member18 = json::member_at(value, at, "agent_cost_usd")?;
            binary64_at(member18, &at18)?
        },
        session_cost_usd: {
            let at19 = json::nested(at, "session_cost_usd");
            let member19 = json::member_at(value, at, "session_cost_usd")?;
            binary64_at(member19, &at19)?
        },
    })
}

/// Writes the event `agenticspec.view.SessionUpdated` as JSON.
pub fn encode_event_agenticspec_view_session_updated(value: &agenticspec_types::view::SessionUpdated, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "project");
    json::push_text(out, &value.project);
    if let Some(held0) = &value.title {
        json::member(out, "title");
        json::push_text(out, &*held0);
    }
    json::member(out, "names");
    json::push_text(out, &value.names);
    json::member(out, "rank");
    json::push_integer(out, value.rank);
    json::member(out, "harness");
    encode_agenticspec_session_harness(&value.harness, out);
    json::member(out, "first_at");
    json::push_text(out, &value.first_at.0);
    json::member(out, "last_at");
    json::push_text(out, &value.last_at.0);
    json::member(out, "agents");
    json::push_integer(out, value.agents);
    json::member(out, "cost_usd");
    out.push_str(&value.cost_usd.token());
    json::member(out, "output_tokens");
    json::push_integer(out, value.output_tokens);
    json::member(out, "context_max");
    json::push_integer(out, value.context_max);
    json::member(out, "cache_hit_share");
    out.push_str(&value.cache_hit_share.token());
    json::member(out, "busts");
    json::push_integer(out, value.busts);
    json::member(out, "human_prompts");
    json::push_integer(out, value.human_prompts);
    json::member(out, "tool_calls");
    json::push_integer(out, value.tool_calls);
    json::member(out, "refused_tool_calls");
    json::push_integer(out, value.refused_tool_calls);
    json::member(out, "exposures");
    json::push_integer(out, value.exposures);
    json::member(out, "working_seconds");
    json::push_integer(out, value.working_seconds);
    json::member(out, "waiting_on_human_seconds");
    json::push_integer(out, value.waiting_on_human_seconds);
    json::member(out, "waiting_share");
    out.push_str(&value.waiting_share.token());
    out.push('}');
}

/// Reads the event `agenticspec.view.SessionUpdated` from JSON, the payload [`encode_event_agenticspec_view_session_updated`] writes.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_event_agenticspec_view_session_updated(value: &json::Value, at: &str) -> Result<agenticspec_types::view::SessionUpdated, json::DecodeError> {
    Ok(agenticspec_types::view::SessionUpdated {
        session_id: {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
        project: {
            let at1 = json::nested(at, "project");
            let member1 = json::member_at(value, at, "project")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        title: match value.member("title") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "title");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
        names: {
            let at3 = json::nested(at, "names");
            let member3 = json::member_at(value, at, "names")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        rank: {
            let at4 = json::nested(at, "rank");
            let member4 = json::member_at(value, at, "rank")?;
            json::integer_at(member4, &at4, "an integer")?
        },
        harness: {
            let at5 = json::nested(at, "harness");
            let member5 = json::member_at(value, at, "harness")?;
            decode_agenticspec_session_harness(member5, &at5)?
        },
        first_at: {
            let at6 = json::nested(at, "first_at");
            let member6 = json::member_at(value, at, "first_at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member6, &at6, "an RFC 3339 instant")?.to_owned())
        },
        last_at: {
            let at7 = json::nested(at, "last_at");
            let member7 = json::member_at(value, at, "last_at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member7, &at7, "an RFC 3339 instant")?.to_owned())
        },
        agents: {
            let at8 = json::nested(at, "agents");
            let member8 = json::member_at(value, at, "agents")?;
            json::integer_at(member8, &at8, "an integer")?
        },
        cost_usd: {
            let at9 = json::nested(at, "cost_usd");
            let member9 = json::member_at(value, at, "cost_usd")?;
            binary64_at(member9, &at9)?
        },
        output_tokens: {
            let at10 = json::nested(at, "output_tokens");
            let member10 = json::member_at(value, at, "output_tokens")?;
            json::integer_at(member10, &at10, "an integer")?
        },
        context_max: {
            let at11 = json::nested(at, "context_max");
            let member11 = json::member_at(value, at, "context_max")?;
            json::integer_at(member11, &at11, "an integer")?
        },
        cache_hit_share: {
            let at12 = json::nested(at, "cache_hit_share");
            let member12 = json::member_at(value, at, "cache_hit_share")?;
            binary64_at(member12, &at12)?
        },
        busts: {
            let at13 = json::nested(at, "busts");
            let member13 = json::member_at(value, at, "busts")?;
            json::integer_at(member13, &at13, "an integer")?
        },
        human_prompts: {
            let at14 = json::nested(at, "human_prompts");
            let member14 = json::member_at(value, at, "human_prompts")?;
            json::integer_at(member14, &at14, "an integer")?
        },
        tool_calls: {
            let at15 = json::nested(at, "tool_calls");
            let member15 = json::member_at(value, at, "tool_calls")?;
            json::integer_at(member15, &at15, "an integer")?
        },
        refused_tool_calls: {
            let at16 = json::nested(at, "refused_tool_calls");
            let member16 = json::member_at(value, at, "refused_tool_calls")?;
            json::integer_at(member16, &at16, "an integer")?
        },
        exposures: {
            let at17 = json::nested(at, "exposures");
            let member17 = json::member_at(value, at, "exposures")?;
            json::integer_at(member17, &at17, "an integer")?
        },
        working_seconds: {
            let at18 = json::nested(at, "working_seconds");
            let member18 = json::member_at(value, at, "working_seconds")?;
            json::integer_at(member18, &at18, "an integer")?
        },
        waiting_on_human_seconds: {
            let at19 = json::nested(at, "waiting_on_human_seconds");
            let member19 = json::member_at(value, at, "waiting_on_human_seconds")?;
            json::integer_at(member19, &at19, "an integer")?
        },
        waiting_share: {
            let at20 = json::nested(at, "waiting_share");
            let member20 = json::member_at(value, at, "waiting_share")?;
            binary64_at(member20, &at20)?
        },
    })
}

/// Writes the declared error `agenticspec.session.AgentStateConflict` as JSON.
pub fn encode_error_agenticspec_session_agent_state_conflict(value: &agenticspec_types::session::AgentStateConflict, out: &mut String) {
    out.push('{');
    json::member(out, "state");
    encode_agenticspec_session_agent_state(&value.state, out);
    out.push('}');
}

/// Writes the declared error `agenticspec.session.SessionStateConflict` as JSON.
pub fn encode_error_agenticspec_session_session_state_conflict(value: &agenticspec_types::session::SessionStateConflict, out: &mut String) {
    out.push('{');
    json::member(out, "state");
    encode_agenticspec_session_session_state(&value.state, out);
    out.push('}');
}

/// Writes the declared error `agenticspec.session.ToolCallStateConflict` as JSON.
pub fn encode_error_agenticspec_session_tool_call_state_conflict(value: &agenticspec_types::session::ToolCallStateConflict, out: &mut String) {
    out.push('{');
    json::member(out, "state");
    encode_agenticspec_session_tool_call_state(&value.state, out);
    out.push('}');
}

/// Writes the declared error `agenticspec.session.UnknownAgent` as JSON.
pub fn encode_error_agenticspec_session_unknown_agent(value: &agenticspec_types::session::UnknownAgent, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    out.push('}');
}

/// Writes one row of the view `agenticspec.session.Agents` as JSON.
pub fn encode_view_agenticspec_session_agents(value: &agenticspec_types::session::Agents, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    if let Some(held0) = &value.parent_agent_id {
        json::member(out, "parent_agent_id");
        encode_agenticspec_session_agent_id(&*held0, out);
    }
    json::member(out, "role");
    encode_agenticspec_session_agent_role(&value.role, out);
    if let Some(held0) = &value.model {
        json::member(out, "model");
        json::push_text(out, &*held0);
    }
    json::member(out, "context_tokens");
    json::push_integer(out, value.context_tokens);
    json::member(out, "state");
    encode_agenticspec_session_agent_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agenticspec.session.Exposures` as JSON.
pub fn encode_view_agenticspec_session_exposures(value: &agenticspec_types::session::Exposures, out: &mut String) {
    out.push('{');
    json::member(out, "exposure_id");
    encode_agenticspec_session_exposure_id(&value.exposure_id, out);
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "channel");
    encode_agenticspec_session_exposure_channel(&value.channel, out);
    json::member(out, "class");
    encode_agenticspec_session_exposure_class(&value.class, out);
    json::member(out, "kind");
    json::push_text(out, &value.kind);
    json::member(out, "alias");
    json::push_text(out, &value.alias);
    json::member(out, "verdict");
    encode_agenticspec_session_verdict(&value.verdict, out);
    json::member(out, "state");
    encode_agenticspec_session_exposure_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agenticspec.session.Sessions` as JSON.
pub fn encode_view_agenticspec_session_sessions(value: &agenticspec_types::session::Sessions, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "harness");
    encode_agenticspec_session_harness(&value.harness, out);
    json::member(out, "opened_at");
    json::push_text(out, &value.opened_at.0);
    json::member(out, "state");
    encode_agenticspec_session_session_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agenticspec.session.ToolCalls` as JSON.
pub fn encode_view_agenticspec_session_tool_calls(value: &agenticspec_types::session::ToolCalls, out: &mut String) {
    out.push('{');
    json::member(out, "tool_call_id");
    encode_agenticspec_session_tool_call_id(&value.tool_call_id, out);
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "tool");
    json::push_text(out, &value.tool);
    json::member(out, "category");
    encode_agenticspec_session_tool_category(&value.category, out);
    json::member(out, "state");
    encode_agenticspec_session_tool_call_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agenticspec.view.AgentTurns` as JSON.
pub fn encode_view_agenticspec_view_agent_turns(value: &agenticspec_types::view::AgentTurns, out: &mut String) {
    out.push('{');
    json::member(out, "event_id");
    encode_agenticspec_view_event_id(&value.event_id, out);
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "seq");
    json::push_integer(out, value.seq);
    json::member(out, "ord");
    json::push_integer(out, value.ord);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    json::member(out, "agent");
    encode_agenticspec_session_agent_id(&value.agent, out);
    json::member(out, "kind");
    encode_agenticspec_view_event_kind(&value.kind, out);
    json::member(out, "notable");
    json::push_bool(out, value.notable);
    json::member(out, "label");
    json::push_text(out, &value.label);
    if let Some(held0) = &value.detail {
        json::member(out, "detail");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.call {
        json::member(out, "call");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.ok {
        json::member(out, "ok");
        json::push_bool(out, *held0);
    }
    if let Some(held0) = &value.context_tokens {
        json::member(out, "context_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.output_tokens {
        json::member(out, "output_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cache_read_tokens {
        json::member(out, "cache_read_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cache_write_tokens {
        json::member(out, "cache_write_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cost_microdollars {
        json::member(out, "cost_microdollars");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.bytes {
        json::member(out, "bytes");
        json::push_integer(out, *held0);
    }
    json::member(out, "agent_cost_usd");
    out.push_str(&value.agent_cost_usd.token());
    json::member(out, "session_cost_usd");
    out.push_str(&value.session_cost_usd.token());
    out.push('}');
}

/// Reads the parameters of `agenticspec.view.AgentTurns` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agenticspec_view_agent_turns(value: &json::Value, at: &str) -> Result<(agenticspec_types::session::SessionId, agenticspec_types::session::AgentId,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
        {
            let at1 = json::nested(at, "agent");
            let member1 = json::member_at(value, at, "agent")?;
            decode_agenticspec_session_agent_id(member1, &at1)?
        },
    ))
}

/// Writes one row of the view `agenticspec.view.OverviewFigures` as JSON.
pub fn encode_view_agenticspec_view_overview_figures(value: &agenticspec_types::view::OverviewFigures, out: &mut String) {
    out.push('{');
    json::member(out, "bucket_id");
    encode_agenticspec_view_bucket_id(&value.bucket_id, out);
    json::member(out, "start");
    json::push_text(out, &value.start.0);
    json::member(out, "bucket_seconds");
    json::push_integer(out, value.bucket_seconds);
    json::member(out, "sessions");
    json::push_integer(out, value.sessions);
    json::member(out, "cost_usd");
    out.push_str(&value.cost_usd.token());
    json::member(out, "working_seconds");
    json::push_integer(out, value.working_seconds);
    json::member(out, "waiting_share");
    out.push_str(&value.waiting_share.token());
    json::member(out, "cache_hit_share");
    out.push_str(&value.cache_hit_share.token());
    json::member(out, "tool_calls");
    json::push_integer(out, value.tool_calls);
    json::member(out, "refused_tool_calls");
    json::push_integer(out, value.refused_tool_calls);
    out.push('}');
}

/// Reads the parameters of `agenticspec.view.OverviewFigures` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agenticspec_view_overview_figures(value: &json::Value, at: &str) -> Result<(agenticspec_types::primitives::Timestamp, i64,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "since");
            let member0 = json::member_at(value, at, "since")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member0, &at0, "an RFC 3339 instant")?.to_owned())
        },
        {
            let at1 = json::nested(at, "bucket_seconds");
            let member1 = json::member_at(value, at, "bucket_seconds")?;
            json::integer_at(member1, &at1, "an integer")?
        },
    ))
}

/// Writes one row of the view `agenticspec.view.SessionAgents` as JSON.
pub fn encode_view_agenticspec_view_session_agents(value: &agenticspec_types::view::SessionAgents, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    if let Some(held0) = &value.parent {
        json::member(out, "parent");
        encode_agenticspec_session_agent_id(&*held0, out);
    }
    json::member(out, "role");
    encode_agenticspec_session_agent_role(&value.role, out);
    if let Some(held0) = &value.model {
        json::member(out, "model");
        json::push_text(out, &*held0);
    }
    json::member(out, "started_at");
    json::push_text(out, &value.started_at.0);
    json::member(out, "turns");
    json::push_integer(out, value.turns);
    json::member(out, "context_tokens");
    json::push_integer(out, value.context_tokens);
    json::member(out, "context_max");
    json::push_integer(out, value.context_max);
    json::member(out, "output_tokens");
    json::push_integer(out, value.output_tokens);
    json::member(out, "cost_usd");
    out.push_str(&value.cost_usd.token());
    json::member(out, "tool_calls");
    json::push_integer(out, value.tool_calls);
    json::member(out, "busts");
    json::push_integer(out, value.busts);
    out.push('}');
}

/// Reads the parameters of `agenticspec.view.SessionAgents` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agenticspec_view_session_agents(value: &json::Value, at: &str) -> Result<(agenticspec_types::session::SessionId,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
    ))
}

/// Writes one row of the view `agenticspec.view.SessionById` as JSON.
pub fn encode_view_agenticspec_view_session_by_id(value: &agenticspec_types::view::SessionById, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "project");
    json::push_text(out, &value.project);
    if let Some(held0) = &value.title {
        json::member(out, "title");
        json::push_text(out, &*held0);
    }
    json::member(out, "harness");
    encode_agenticspec_session_harness(&value.harness, out);
    json::member(out, "first_at");
    json::push_text(out, &value.first_at.0);
    json::member(out, "last_at");
    json::push_text(out, &value.last_at.0);
    json::member(out, "agents");
    json::push_integer(out, value.agents);
    json::member(out, "cost_usd");
    out.push_str(&value.cost_usd.token());
    json::member(out, "output_tokens");
    json::push_integer(out, value.output_tokens);
    json::member(out, "context_max");
    json::push_integer(out, value.context_max);
    json::member(out, "cache_hit_share");
    out.push_str(&value.cache_hit_share.token());
    json::member(out, "busts");
    json::push_integer(out, value.busts);
    json::member(out, "human_prompts");
    json::push_integer(out, value.human_prompts);
    json::member(out, "tool_calls");
    json::push_integer(out, value.tool_calls);
    json::member(out, "refused_tool_calls");
    json::push_integer(out, value.refused_tool_calls);
    json::member(out, "exposures");
    json::push_integer(out, value.exposures);
    json::member(out, "working_seconds");
    json::push_integer(out, value.working_seconds);
    json::member(out, "waiting_on_human_seconds");
    json::push_integer(out, value.waiting_on_human_seconds);
    json::member(out, "waiting_share");
    out.push_str(&value.waiting_share.token());
    json::member(out, "kpis");
    encode_agenticspec_cli_kpi_report(&value.kpis, out);
    out.push('}');
}

/// Reads the parameters of `agenticspec.view.SessionById` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agenticspec_view_session_by_id(value: &json::Value, at: &str) -> Result<(agenticspec_types::session::SessionId,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
    ))
}

/// Writes one row of the view `agenticspec.view.SessionChecks` as JSON.
pub fn encode_view_agenticspec_view_session_checks(value: &agenticspec_types::view::SessionChecks, out: &mut String) {
    out.push('{');
    json::member(out, "check_id");
    encode_agenticspec_view_check_id(&value.check_id, out);
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "check");
    json::push_text(out, &value.check);
    json::member(out, "about");
    json::push_text(out, &value.about);
    json::member(out, "severity");
    json::push_text(out, &value.severity);
    json::member(out, "verdict");
    encode_agenticspec_session_verdict(&value.verdict, out);
    json::member(out, "rows");
    json::push_integer(out, value.rows);
    json::member(out, "possible");
    json::push_integer(out, value.possible);
    out.push('}');
}

/// Reads the parameters of `agenticspec.view.SessionChecks` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agenticspec_view_session_checks(value: &json::Value, at: &str) -> Result<(agenticspec_types::session::SessionId,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
    ))
}

/// Writes one row of the view `agenticspec.view.SessionEventCounts` as JSON.
pub fn encode_view_agenticspec_view_session_event_counts(value: &agenticspec_types::view::SessionEventCounts, out: &mut String) {
    out.push('{');
    json::member(out, "count_id");
    encode_agenticspec_view_count_id(&value.count_id, out);
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    if let Some(held0) = &value.agent {
        json::member(out, "agent");
        encode_agenticspec_session_agent_id(&*held0, out);
    }
    json::member(out, "kind");
    encode_agenticspec_view_event_kind(&value.kind, out);
    json::member(out, "count");
    json::push_integer(out, value.count);
    out.push('}');
}

/// Reads the parameters of `agenticspec.view.SessionEventCounts` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agenticspec_view_session_event_counts(value: &json::Value, at: &str) -> Result<(agenticspec_types::session::SessionId, Option<agenticspec_types::session::AgentId>,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
        match value.member("agent") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "agent");
                Some(decode_agenticspec_session_agent_id(member1, &at1)?)
            }
        },
    ))
}

/// Writes one row of the view `agenticspec.view.SessionEvents` as JSON.
pub fn encode_view_agenticspec_view_session_events(value: &agenticspec_types::view::SessionEvents, out: &mut String) {
    out.push('{');
    json::member(out, "event_id");
    encode_agenticspec_view_event_id(&value.event_id, out);
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "seq");
    json::push_integer(out, value.seq);
    json::member(out, "ord");
    json::push_integer(out, value.ord);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    json::member(out, "agent");
    encode_agenticspec_session_agent_id(&value.agent, out);
    json::member(out, "kind");
    encode_agenticspec_view_event_kind(&value.kind, out);
    json::member(out, "notable");
    json::push_bool(out, value.notable);
    json::member(out, "label");
    json::push_text(out, &value.label);
    if let Some(held0) = &value.detail {
        json::member(out, "detail");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.call {
        json::member(out, "call");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.ok {
        json::member(out, "ok");
        json::push_bool(out, *held0);
    }
    if let Some(held0) = &value.context_tokens {
        json::member(out, "context_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.output_tokens {
        json::member(out, "output_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cache_read_tokens {
        json::member(out, "cache_read_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cache_write_tokens {
        json::member(out, "cache_write_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cost_microdollars {
        json::member(out, "cost_microdollars");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.bytes {
        json::member(out, "bytes");
        json::push_integer(out, *held0);
    }
    json::member(out, "agent_cost_usd");
    out.push_str(&value.agent_cost_usd.token());
    json::member(out, "session_cost_usd");
    out.push_str(&value.session_cost_usd.token());
    out.push('}');
}

/// Reads the parameters of `agenticspec.view.SessionEvents` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agenticspec_view_session_events(value: &json::Value, at: &str) -> Result<(agenticspec_types::session::SessionId, Option<agenticspec_types::session::AgentId>, Option<agenticspec_types::view::EventKind>, Option<bool>, Option<i64>,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
        match value.member("agent") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "agent");
                Some(decode_agenticspec_session_agent_id(member1, &at1)?)
            }
        },
        match value.member("kind") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "kind");
                Some(decode_agenticspec_view_event_kind(member2, &at2)?)
            }
        },
        match value.member("notable") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "notable");
                Some(json::bool_at(member3, &at3, "a boolean")?)
            }
        },
        match value.member("before") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "before");
                Some(json::integer_at(member4, &at4, "an integer")?)
            }
        },
    ))
}

/// Writes one row of the view `agenticspec.view.SessionList` as JSON.
pub fn encode_view_agenticspec_view_session_list(value: &agenticspec_types::view::SessionList, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "project");
    json::push_text(out, &value.project);
    if let Some(held0) = &value.title {
        json::member(out, "title");
        json::push_text(out, &*held0);
    }
    json::member(out, "names");
    json::push_text(out, &value.names);
    json::member(out, "rank");
    json::push_integer(out, value.rank);
    json::member(out, "harness");
    encode_agenticspec_session_harness(&value.harness, out);
    json::member(out, "first_at");
    json::push_text(out, &value.first_at.0);
    json::member(out, "last_at");
    json::push_text(out, &value.last_at.0);
    json::member(out, "agents");
    json::push_integer(out, value.agents);
    json::member(out, "cost_usd");
    out.push_str(&value.cost_usd.token());
    json::member(out, "output_tokens");
    json::push_integer(out, value.output_tokens);
    json::member(out, "context_max");
    json::push_integer(out, value.context_max);
    json::member(out, "cache_hit_share");
    out.push_str(&value.cache_hit_share.token());
    json::member(out, "busts");
    json::push_integer(out, value.busts);
    json::member(out, "human_prompts");
    json::push_integer(out, value.human_prompts);
    json::member(out, "tool_calls");
    json::push_integer(out, value.tool_calls);
    json::member(out, "refused_tool_calls");
    json::push_integer(out, value.refused_tool_calls);
    json::member(out, "exposures");
    json::push_integer(out, value.exposures);
    json::member(out, "working_seconds");
    json::push_integer(out, value.working_seconds);
    json::member(out, "waiting_on_human_seconds");
    json::push_integer(out, value.waiting_on_human_seconds);
    json::member(out, "waiting_share");
    out.push_str(&value.waiting_share.token());
    out.push('}');
}

/// Reads the parameters of `agenticspec.view.SessionList` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agenticspec_view_session_list(value: &json::Value, at: &str) -> Result<(Option<agenticspec_types::primitives::Timestamp>, Option<String>, Option<i64>, Option<i64>,), json::DecodeError> {
    Ok((
        match value.member("since") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "since");
                Some(agenticspec_types::primitives::Timestamp(json::timestamp_at(member0, &at0, "an RFC 3339 instant")?.to_owned()))
            }
        },
        match value.member("search") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "search");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
        match value.member("limit") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "limit");
                Some(json::integer_at(member2, &at2, "an integer")?)
            }
        },
        match value.member("offset") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "offset");
                Some(json::integer_at(member3, &at3, "an integer")?)
            }
        },
    ))
}

/// Writes one row of the view `agenticspec.view.SessionTimeline` as JSON.
pub fn encode_view_agenticspec_view_session_timeline(value: &agenticspec_types::view::SessionTimeline, out: &mut String) {
    out.push('{');
    json::member(out, "slot_id");
    encode_agenticspec_view_slot_id(&value.slot_id, out);
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "agent");
    encode_agenticspec_session_agent_id(&value.agent, out);
    json::member(out, "slot_seconds");
    json::push_integer(out, value.slot_seconds);
    json::member(out, "start");
    json::push_text(out, &value.start.0);
    json::member(out, "end");
    json::push_text(out, &value.end.0);
    json::member(out, "events");
    json::push_integer(out, value.events);
    json::member(out, "read_calls");
    json::push_integer(out, value.read_calls);
    json::member(out, "search_calls");
    json::push_integer(out, value.search_calls);
    json::member(out, "edit_calls");
    json::push_integer(out, value.edit_calls);
    json::member(out, "execute_calls");
    json::push_integer(out, value.execute_calls);
    json::member(out, "network_calls");
    json::push_integer(out, value.network_calls);
    json::member(out, "spawn_calls");
    json::push_integer(out, value.spawn_calls);
    json::member(out, "message_calls");
    json::push_integer(out, value.message_calls);
    json::member(out, "plan_calls");
    json::push_integer(out, value.plan_calls);
    json::member(out, "other_calls");
    json::push_integer(out, value.other_calls);
    json::member(out, "human_prompts");
    json::push_integer(out, value.human_prompts);
    json::member(out, "relayed_prompts");
    json::push_integer(out, value.relayed_prompts);
    json::member(out, "refusals");
    json::push_integer(out, value.refusals);
    json::member(out, "busts");
    json::push_integer(out, value.busts);
    json::member(out, "compactions");
    json::push_integer(out, value.compactions);
    if let Some(held0) = &value.context_max {
        json::member(out, "context_max");
        json::push_integer(out, *held0);
    }
    json::member(out, "agent_cost_usd");
    out.push_str(&value.agent_cost_usd.token());
    json::member(out, "session_cost_usd");
    out.push_str(&value.session_cost_usd.token());
    out.push('}');
}

/// Reads the parameters of `agenticspec.view.SessionTimeline` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agenticspec_view_session_timeline(value: &json::Value, at: &str) -> Result<(agenticspec_types::session::SessionId, i64,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
        {
            let at1 = json::nested(at, "slot_seconds");
            let member1 = json::member_at(value, at, "slot_seconds")?;
            json::integer_at(member1, &at1, "an integer")?
        },
    ))
}

/// Writes one row of the view `agenticspec.view.SessionTurns` as JSON.
pub fn encode_view_agenticspec_view_session_turns(value: &agenticspec_types::view::SessionTurns, out: &mut String) {
    out.push('{');
    json::member(out, "event_id");
    encode_agenticspec_view_event_id(&value.event_id, out);
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "seq");
    json::push_integer(out, value.seq);
    json::member(out, "ord");
    json::push_integer(out, value.ord);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    json::member(out, "agent");
    encode_agenticspec_session_agent_id(&value.agent, out);
    json::member(out, "kind");
    encode_agenticspec_view_event_kind(&value.kind, out);
    json::member(out, "notable");
    json::push_bool(out, value.notable);
    json::member(out, "label");
    json::push_text(out, &value.label);
    if let Some(held0) = &value.detail {
        json::member(out, "detail");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.call {
        json::member(out, "call");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.ok {
        json::member(out, "ok");
        json::push_bool(out, *held0);
    }
    if let Some(held0) = &value.context_tokens {
        json::member(out, "context_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.output_tokens {
        json::member(out, "output_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cache_read_tokens {
        json::member(out, "cache_read_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cache_write_tokens {
        json::member(out, "cache_write_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.cost_microdollars {
        json::member(out, "cost_microdollars");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.bytes {
        json::member(out, "bytes");
        json::push_integer(out, *held0);
    }
    json::member(out, "agent_cost_usd");
    out.push_str(&value.agent_cost_usd.token());
    json::member(out, "session_cost_usd");
    out.push_str(&value.session_cost_usd.token());
    out.push('}');
}

/// Reads the parameters of `agenticspec.view.SessionTurns` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agenticspec_view_session_turns(value: &json::Value, at: &str) -> Result<(agenticspec_types::session::SessionId,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
    ))
}

/// Writes the input of `agenticspec.session.CompactContext` as JSON.
pub fn encode_command_agenticspec_session_compact_context(value: &agenticspec_types::session::CompactContext, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "context_tokens_before");
    json::push_integer(out, value.context_tokens_before);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the input of `agenticspec.session.CompactContext` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agenticspec_session_compact_context(value: &json::Value, at: &str) -> Result<agenticspec_types::session::CompactContext, json::DecodeError> {
    json::closed_at(value, at, &["agent_id", "context_tokens_before", "at"])?;
    Ok(agenticspec_types::session::CompactContext {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        context_tokens_before: {
            let at1 = json::nested(at, "context_tokens_before");
            let member1 = json::member_at(value, at, "context_tokens_before")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        at: {
            let at2 = json::nested(at, "at");
            let member2 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member2, &at2, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the outcome of `agenticspec.session.CompactContext` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agenticspec_session_compact_context(value: &agenticspec_types::session::CompactContextOutcome, out: &mut String) {
    out.push('{');
    match value {
        agenticspec_types::session::CompactContextOutcome::Compacted { context_compacted } => {
            json::member(out, "outcome");
            json::push_text(out, "compacted");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.ContextCompacted");
            json::member(out, "payload");
            encode_event_agenticspec_session_context_compacted(context_compacted, out);
            out.push('}');
            out.push(']');
        }
        agenticspec_types::session::CompactContextOutcome::NoSuchAgent { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-agent");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.UnknownAgent");
            json::member(out, "payload");
            encode_error_agenticspec_session_unknown_agent(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agenticspec.session.CompleteToolCall` as JSON.
pub fn encode_command_agenticspec_session_complete_tool_call(value: &agenticspec_types::session::CompleteToolCall, out: &mut String) {
    out.push('{');
    json::member(out, "tool_call_id");
    encode_agenticspec_session_tool_call_id(&value.tool_call_id, out);
    json::member(out, "ok");
    json::push_bool(out, value.ok);
    json::member(out, "output");
    encode_agenticspec_session_content(&value.output, out);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the input of `agenticspec.session.CompleteToolCall` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agenticspec_session_complete_tool_call(value: &json::Value, at: &str) -> Result<agenticspec_types::session::CompleteToolCall, json::DecodeError> {
    json::closed_at(value, at, &["tool_call_id", "ok", "output", "at"])?;
    Ok(agenticspec_types::session::CompleteToolCall {
        tool_call_id: {
            let at0 = json::nested(at, "tool_call_id");
            let member0 = json::member_at(value, at, "tool_call_id")?;
            decode_agenticspec_session_tool_call_id(member0, &at0)?
        },
        ok: {
            let at1 = json::nested(at, "ok");
            let member1 = json::member_at(value, at, "ok")?;
            json::bool_at(member1, &at1, "a boolean")?
        },
        output: {
            let at2 = json::nested(at, "output");
            let member2 = json::member_at(value, at, "output")?;
            decode_agenticspec_session_content(member2, &at2)?
        },
        at: {
            let at3 = json::nested(at, "at");
            let member3 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member3, &at3, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the outcome of `agenticspec.session.CompleteToolCall` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agenticspec_session_complete_tool_call(value: &agenticspec_types::session::CompleteToolCallOutcome, out: &mut String) {
    out.push('{');
    match value {
        agenticspec_types::session::CompleteToolCallOutcome::Succeeded { tool_call_completed } => {
            json::member(out, "outcome");
            json::push_text(out, "succeeded");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.ToolCallCompleted");
            json::member(out, "payload");
            encode_event_agenticspec_session_tool_call_completed(tool_call_completed, out);
            out.push('}');
            out.push(']');
        }
        agenticspec_types::session::CompleteToolCallOutcome::Failed { tool_call_completed } => {
            json::member(out, "outcome");
            json::push_text(out, "failed");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.ToolCallCompleted");
            json::member(out, "payload");
            encode_event_agenticspec_session_tool_call_completed(tool_call_completed, out);
            out.push('}');
            out.push(']');
        }
        agenticspec_types::session::CompleteToolCallOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.ToolCallStateConflict");
            json::member(out, "payload");
            encode_error_agenticspec_session_tool_call_state_conflict(error, out);
            out.push('}');
        }
        agenticspec_types::session::CompleteToolCallOutcome::WrongStateUnknownInstance => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.ToolCallStateConflict");
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agenticspec.session.DecideToolCall` as JSON.
pub fn encode_command_agenticspec_session_decide_tool_call(value: &agenticspec_types::session::DecideToolCall, out: &mut String) {
    out.push('{');
    json::member(out, "tool_call_id");
    encode_agenticspec_session_tool_call_id(&value.tool_call_id, out);
    json::member(out, "decision");
    encode_agenticspec_session_decision(&value.decision, out);
    json::member(out, "decider");
    encode_agenticspec_session_decider(&value.decider, out);
    if let Some(held0) = &value.reason {
        json::member(out, "reason");
        json::push_text(out, &*held0);
    }
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the input of `agenticspec.session.DecideToolCall` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agenticspec_session_decide_tool_call(value: &json::Value, at: &str) -> Result<agenticspec_types::session::DecideToolCall, json::DecodeError> {
    json::closed_at(value, at, &["tool_call_id", "decision", "decider", "reason", "at"])?;
    Ok(agenticspec_types::session::DecideToolCall {
        tool_call_id: {
            let at0 = json::nested(at, "tool_call_id");
            let member0 = json::member_at(value, at, "tool_call_id")?;
            decode_agenticspec_session_tool_call_id(member0, &at0)?
        },
        decision: {
            let at1 = json::nested(at, "decision");
            let member1 = json::member_at(value, at, "decision")?;
            decode_agenticspec_session_decision(member1, &at1)?
        },
        decider: {
            let at2 = json::nested(at, "decider");
            let member2 = json::member_at(value, at, "decider")?;
            decode_agenticspec_session_decider(member2, &at2)?
        },
        reason: match value.member("reason") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "reason");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        at: {
            let at4 = json::nested(at, "at");
            let member4 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member4, &at4, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the outcome of `agenticspec.session.DecideToolCall` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agenticspec_session_decide_tool_call(value: &agenticspec_types::session::DecideToolCallOutcome, out: &mut String) {
    out.push('{');
    match value {
        agenticspec_types::session::DecideToolCallOutcome::Accepted { tool_call_decided } => {
            json::member(out, "outcome");
            json::push_text(out, "accepted");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.ToolCallDecided");
            json::member(out, "payload");
            encode_event_agenticspec_session_tool_call_decided(tool_call_decided, out);
            out.push('}');
            out.push(']');
        }
        agenticspec_types::session::DecideToolCallOutcome::Refused { tool_call_decided } => {
            json::member(out, "outcome");
            json::push_text(out, "refused");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.ToolCallDecided");
            json::member(out, "payload");
            encode_event_agenticspec_session_tool_call_decided(tool_call_decided, out);
            out.push('}');
            out.push(']');
        }
        agenticspec_types::session::DecideToolCallOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.ToolCallStateConflict");
            json::member(out, "payload");
            encode_error_agenticspec_session_tool_call_state_conflict(error, out);
            out.push('}');
        }
        agenticspec_types::session::DecideToolCallOutcome::WrongStateUnknownInstance => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.ToolCallStateConflict");
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agenticspec.session.EndAgent` as JSON.
pub fn encode_command_agenticspec_session_end_agent(value: &agenticspec_types::session::EndAgent, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the input of `agenticspec.session.EndAgent` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agenticspec_session_end_agent(value: &json::Value, at: &str) -> Result<agenticspec_types::session::EndAgent, json::DecodeError> {
    json::closed_at(value, at, &["agent_id", "at"])?;
    Ok(agenticspec_types::session::EndAgent {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        at: {
            let at1 = json::nested(at, "at");
            let member1 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member1, &at1, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the outcome of `agenticspec.session.EndAgent` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agenticspec_session_end_agent(value: &agenticspec_types::session::EndAgentOutcome, out: &mut String) {
    out.push('{');
    match value {
        agenticspec_types::session::EndAgentOutcome::Ended { agent_ended } => {
            json::member(out, "outcome");
            json::push_text(out, "ended");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.AgentEnded");
            json::member(out, "payload");
            encode_event_agenticspec_session_agent_ended(agent_ended, out);
            out.push('}');
            out.push(']');
        }
        agenticspec_types::session::EndAgentOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.AgentStateConflict");
            json::member(out, "payload");
            encode_error_agenticspec_session_agent_state_conflict(error, out);
            out.push('}');
        }
        agenticspec_types::session::EndAgentOutcome::WrongStateUnknownInstance => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.AgentStateConflict");
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agenticspec.session.EndSession` as JSON.
pub fn encode_command_agenticspec_session_end_session(value: &agenticspec_types::session::EndSession, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the input of `agenticspec.session.EndSession` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agenticspec_session_end_session(value: &json::Value, at: &str) -> Result<agenticspec_types::session::EndSession, json::DecodeError> {
    json::closed_at(value, at, &["session_id", "at"])?;
    Ok(agenticspec_types::session::EndSession {
        session_id: {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
        at: {
            let at1 = json::nested(at, "at");
            let member1 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member1, &at1, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the outcome of `agenticspec.session.EndSession` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agenticspec_session_end_session(value: &agenticspec_types::session::EndSessionOutcome, out: &mut String) {
    out.push('{');
    match value {
        agenticspec_types::session::EndSessionOutcome::Ended { session_ended } => {
            json::member(out, "outcome");
            json::push_text(out, "ended");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.SessionEnded");
            json::member(out, "payload");
            encode_event_agenticspec_session_session_ended(session_ended, out);
            out.push('}');
            out.push(']');
        }
        agenticspec_types::session::EndSessionOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.SessionStateConflict");
            json::member(out, "payload");
            encode_error_agenticspec_session_session_state_conflict(error, out);
            out.push('}');
        }
        agenticspec_types::session::EndSessionOutcome::WrongStateUnknownInstance => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.SessionStateConflict");
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agenticspec.session.EndTurn` as JSON.
pub fn encode_command_agenticspec_session_end_turn(value: &agenticspec_types::session::EndTurn, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the input of `agenticspec.session.EndTurn` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agenticspec_session_end_turn(value: &json::Value, at: &str) -> Result<agenticspec_types::session::EndTurn, json::DecodeError> {
    json::closed_at(value, at, &["agent_id", "at"])?;
    Ok(agenticspec_types::session::EndTurn {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        at: {
            let at1 = json::nested(at, "at");
            let member1 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member1, &at1, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the outcome of `agenticspec.session.EndTurn` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agenticspec_session_end_turn(value: &agenticspec_types::session::EndTurnOutcome, out: &mut String) {
    out.push('{');
    match value {
        agenticspec_types::session::EndTurnOutcome::Ended { turn_ended } => {
            json::member(out, "outcome");
            json::push_text(out, "ended");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.TurnEnded");
            json::member(out, "payload");
            encode_event_agenticspec_session_turn_ended(turn_ended, out);
            out.push('}');
            out.push(']');
        }
        agenticspec_types::session::EndTurnOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.AgentStateConflict");
            json::member(out, "payload");
            encode_error_agenticspec_session_agent_state_conflict(error, out);
            out.push('}');
        }
        agenticspec_types::session::EndTurnOutcome::WrongStateUnknownInstance => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.AgentStateConflict");
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agenticspec.session.OpenSession` as JSON.
pub fn encode_command_agenticspec_session_open_session(value: &agenticspec_types::session::OpenSession, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    json::member(out, "harness");
    encode_agenticspec_session_harness(&value.harness, out);
    if let Some(held0) = &value.source {
        json::member(out, "source");
        encode_agenticspec_session_source(&*held0, out);
    }
    if let Some(held0) = &value.session_hash {
        json::member(out, "session_hash");
        json::push_text(out, &*held0);
    }
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the input of `agenticspec.session.OpenSession` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agenticspec_session_open_session(value: &json::Value, at: &str) -> Result<agenticspec_types::session::OpenSession, json::DecodeError> {
    json::closed_at(value, at, &["session_id", "harness", "source", "session_hash", "at"])?;
    Ok(agenticspec_types::session::OpenSession {
        session_id: {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member0, &at0)?
        },
        harness: {
            let at1 = json::nested(at, "harness");
            let member1 = json::member_at(value, at, "harness")?;
            decode_agenticspec_session_harness(member1, &at1)?
        },
        source: match value.member("source") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "source");
                Some(decode_agenticspec_session_source(member2, &at2)?)
            }
        },
        session_hash: match value.member("session_hash") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "session_hash");
                Some(json::text_at(member3, &at3, "a string")?.to_owned())
            }
        },
        at: {
            let at4 = json::nested(at, "at");
            let member4 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member4, &at4, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the outcome of `agenticspec.session.OpenSession` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agenticspec_session_open_session(value: &agenticspec_types::session::OpenSessionOutcome, out: &mut String) {
    out.push('{');
    match value {
        agenticspec_types::session::OpenSessionOutcome::Opened { session_opened } => {
            json::member(out, "outcome");
            json::push_text(out, "opened");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.SessionOpened");
            json::member(out, "payload");
            encode_event_agenticspec_session_session_opened(session_opened, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agenticspec.session.RecordExposure` as JSON.
pub fn encode_command_agenticspec_session_record_exposure(value: &agenticspec_types::session::RecordExposure, out: &mut String) {
    out.push('{');
    json::member(out, "exposure_id");
    encode_agenticspec_session_exposure_id(&value.exposure_id, out);
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "channel");
    encode_agenticspec_session_exposure_channel(&value.channel, out);
    json::member(out, "class");
    encode_agenticspec_session_exposure_class(&value.class, out);
    json::member(out, "kind");
    json::push_text(out, &value.kind);
    json::member(out, "alias");
    json::push_text(out, &value.alias);
    json::member(out, "verdict");
    encode_agenticspec_session_verdict(&value.verdict, out);
    if let Some(held0) = &value.tool_call_id {
        json::member(out, "tool_call_id");
        encode_agenticspec_session_tool_call_id(&*held0, out);
    }
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the input of `agenticspec.session.RecordExposure` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agenticspec_session_record_exposure(value: &json::Value, at: &str) -> Result<agenticspec_types::session::RecordExposure, json::DecodeError> {
    json::closed_at(value, at, &["exposure_id", "agent_id", "channel", "class", "kind", "alias", "verdict", "tool_call_id", "at"])?;
    Ok(agenticspec_types::session::RecordExposure {
        exposure_id: {
            let at0 = json::nested(at, "exposure_id");
            let member0 = json::member_at(value, at, "exposure_id")?;
            decode_agenticspec_session_exposure_id(member0, &at0)?
        },
        agent_id: {
            let at1 = json::nested(at, "agent_id");
            let member1 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member1, &at1)?
        },
        channel: {
            let at2 = json::nested(at, "channel");
            let member2 = json::member_at(value, at, "channel")?;
            decode_agenticspec_session_exposure_channel(member2, &at2)?
        },
        class: {
            let at3 = json::nested(at, "class");
            let member3 = json::member_at(value, at, "class")?;
            decode_agenticspec_session_exposure_class(member3, &at3)?
        },
        kind: {
            let at4 = json::nested(at, "kind");
            let member4 = json::member_at(value, at, "kind")?;
            json::text_at(member4, &at4, "a string")?.to_owned()
        },
        alias: {
            let at5 = json::nested(at, "alias");
            let member5 = json::member_at(value, at, "alias")?;
            json::text_at(member5, &at5, "a string")?.to_owned()
        },
        verdict: {
            let at6 = json::nested(at, "verdict");
            let member6 = json::member_at(value, at, "verdict")?;
            decode_agenticspec_session_verdict(member6, &at6)?
        },
        tool_call_id: match value.member("tool_call_id") {
            None | Some(json::Value::Null) => None,
            Some(member7) => {
                let at7 = json::nested(at, "tool_call_id");
                Some(decode_agenticspec_session_tool_call_id(member7, &at7)?)
            }
        },
        at: {
            let at8 = json::nested(at, "at");
            let member8 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member8, &at8, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the outcome of `agenticspec.session.RecordExposure` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agenticspec_session_record_exposure(value: &agenticspec_types::session::RecordExposureOutcome, out: &mut String) {
    out.push('{');
    match value {
        agenticspec_types::session::RecordExposureOutcome::Recorded { exposure_detected } => {
            json::member(out, "outcome");
            json::push_text(out, "recorded");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.ExposureDetected");
            json::member(out, "payload");
            encode_event_agenticspec_session_exposure_detected(exposure_detected, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agenticspec.session.RecordModelTurn` as JSON.
pub fn encode_command_agenticspec_session_record_model_turn(value: &agenticspec_types::session::RecordModelTurn, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "model");
    json::push_text(out, &value.model);
    json::member(out, "usage");
    encode_agenticspec_session_usage(&value.usage, out);
    json::member(out, "context_tokens");
    json::push_integer(out, value.context_tokens);
    if let Some(held0) = &value.message_id {
        json::member(out, "message_id");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.call_id {
        json::member(out, "call_id");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.route {
        json::member(out, "route");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.upstream {
        json::member(out, "upstream");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.attempts {
        json::member(out, "attempts");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.queue_wait_ms {
        json::member(out, "queue_wait_ms");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.ttft_ms {
        json::member(out, "ttft_ms");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.declared_cost_microdollars {
        json::member(out, "declared_cost_microdollars");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.incomplete {
        json::member(out, "incomplete");
        encode_agenticspec_session_incomplete_reason(&*held0, out);
    }
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the input of `agenticspec.session.RecordModelTurn` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agenticspec_session_record_model_turn(value: &json::Value, at: &str) -> Result<agenticspec_types::session::RecordModelTurn, json::DecodeError> {
    json::closed_at(value, at, &["agent_id", "model", "usage", "context_tokens", "message_id", "call_id", "route", "upstream", "attempts", "queue_wait_ms", "ttft_ms", "declared_cost_microdollars", "incomplete", "at"])?;
    Ok(agenticspec_types::session::RecordModelTurn {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        model: {
            let at1 = json::nested(at, "model");
            let member1 = json::member_at(value, at, "model")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        usage: {
            let at2 = json::nested(at, "usage");
            let member2 = json::member_at(value, at, "usage")?;
            decode_agenticspec_session_usage(member2, &at2)?
        },
        context_tokens: {
            let at3 = json::nested(at, "context_tokens");
            let member3 = json::member_at(value, at, "context_tokens")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        message_id: match value.member("message_id") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "message_id");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
        call_id: match value.member("call_id") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "call_id");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
        route: match value.member("route") {
            None | Some(json::Value::Null) => None,
            Some(member6) => {
                let at6 = json::nested(at, "route");
                Some(json::text_at(member6, &at6, "a string")?.to_owned())
            }
        },
        upstream: match value.member("upstream") {
            None | Some(json::Value::Null) => None,
            Some(member7) => {
                let at7 = json::nested(at, "upstream");
                Some(json::text_at(member7, &at7, "a string")?.to_owned())
            }
        },
        attempts: match value.member("attempts") {
            None | Some(json::Value::Null) => None,
            Some(member8) => {
                let at8 = json::nested(at, "attempts");
                Some(json::integer_at(member8, &at8, "an integer")?)
            }
        },
        queue_wait_ms: match value.member("queue_wait_ms") {
            None | Some(json::Value::Null) => None,
            Some(member9) => {
                let at9 = json::nested(at, "queue_wait_ms");
                Some(json::integer_at(member9, &at9, "an integer")?)
            }
        },
        ttft_ms: match value.member("ttft_ms") {
            None | Some(json::Value::Null) => None,
            Some(member10) => {
                let at10 = json::nested(at, "ttft_ms");
                Some(json::integer_at(member10, &at10, "an integer")?)
            }
        },
        declared_cost_microdollars: match value.member("declared_cost_microdollars") {
            None | Some(json::Value::Null) => None,
            Some(member11) => {
                let at11 = json::nested(at, "declared_cost_microdollars");
                Some(json::integer_at(member11, &at11, "an integer")?)
            }
        },
        incomplete: match value.member("incomplete") {
            None | Some(json::Value::Null) => None,
            Some(member12) => {
                let at12 = json::nested(at, "incomplete");
                Some(decode_agenticspec_session_incomplete_reason(member12, &at12)?)
            }
        },
        at: {
            let at13 = json::nested(at, "at");
            let member13 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member13, &at13, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the outcome of `agenticspec.session.RecordModelTurn` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agenticspec_session_record_model_turn(value: &agenticspec_types::session::RecordModelTurnOutcome, out: &mut String) {
    out.push('{');
    match value {
        agenticspec_types::session::RecordModelTurnOutcome::Recorded { model_turn_recorded } => {
            json::member(out, "outcome");
            json::push_text(out, "recorded");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.ModelTurnRecorded");
            json::member(out, "payload");
            encode_event_agenticspec_session_model_turn_recorded(model_turn_recorded, out);
            out.push('}');
            out.push(']');
        }
        agenticspec_types::session::RecordModelTurnOutcome::NoSuchAgent { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-agent");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.UnknownAgent");
            json::member(out, "payload");
            encode_error_agenticspec_session_unknown_agent(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agenticspec.session.RequestToolCall` as JSON.
pub fn encode_command_agenticspec_session_request_tool_call(value: &agenticspec_types::session::RequestToolCall, out: &mut String) {
    out.push('{');
    json::member(out, "tool_call_id");
    encode_agenticspec_session_tool_call_id(&value.tool_call_id, out);
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "tool");
    json::push_text(out, &value.tool);
    json::member(out, "category");
    encode_agenticspec_session_tool_category(&value.category, out);
    json::member(out, "input");
    encode_agenticspec_session_content(&value.input, out);
    if let Some(held0) = &value.cwd {
        json::member(out, "cwd");
        json::push_text(out, &*held0);
    }
    json::member(out, "background");
    json::push_bool(out, value.background);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the input of `agenticspec.session.RequestToolCall` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agenticspec_session_request_tool_call(value: &json::Value, at: &str) -> Result<agenticspec_types::session::RequestToolCall, json::DecodeError> {
    json::closed_at(value, at, &["tool_call_id", "agent_id", "tool", "category", "input", "cwd", "background", "at"])?;
    Ok(agenticspec_types::session::RequestToolCall {
        tool_call_id: {
            let at0 = json::nested(at, "tool_call_id");
            let member0 = json::member_at(value, at, "tool_call_id")?;
            decode_agenticspec_session_tool_call_id(member0, &at0)?
        },
        agent_id: {
            let at1 = json::nested(at, "agent_id");
            let member1 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member1, &at1)?
        },
        tool: {
            let at2 = json::nested(at, "tool");
            let member2 = json::member_at(value, at, "tool")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        category: {
            let at3 = json::nested(at, "category");
            let member3 = json::member_at(value, at, "category")?;
            decode_agenticspec_session_tool_category(member3, &at3)?
        },
        input: {
            let at4 = json::nested(at, "input");
            let member4 = json::member_at(value, at, "input")?;
            decode_agenticspec_session_content(member4, &at4)?
        },
        cwd: match value.member("cwd") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "cwd");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
        background: {
            let at6 = json::nested(at, "background");
            let member6 = json::member_at(value, at, "background")?;
            json::bool_at(member6, &at6, "a boolean")?
        },
        at: {
            let at7 = json::nested(at, "at");
            let member7 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member7, &at7, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the outcome of `agenticspec.session.RequestToolCall` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agenticspec_session_request_tool_call(value: &agenticspec_types::session::RequestToolCallOutcome, out: &mut String) {
    out.push('{');
    match value {
        agenticspec_types::session::RequestToolCallOutcome::Requested { tool_call_requested } => {
            json::member(out, "outcome");
            json::push_text(out, "requested");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.ToolCallRequested");
            json::member(out, "payload");
            encode_event_agenticspec_session_tool_call_requested(tool_call_requested, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agenticspec.session.StartAgent` as JSON.
pub fn encode_command_agenticspec_session_start_agent(value: &agenticspec_types::session::StartAgent, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "session_id");
    encode_agenticspec_session_session_id(&value.session_id, out);
    if let Some(held0) = &value.parent_agent_id {
        json::member(out, "parent_agent_id");
        encode_agenticspec_session_agent_id(&*held0, out);
    }
    json::member(out, "role");
    encode_agenticspec_session_agent_role(&value.role, out);
    if let Some(held0) = &value.spawned_by {
        json::member(out, "spawned_by");
        encode_agenticspec_session_tool_call_id(&*held0, out);
    }
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the input of `agenticspec.session.StartAgent` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agenticspec_session_start_agent(value: &json::Value, at: &str) -> Result<agenticspec_types::session::StartAgent, json::DecodeError> {
    json::closed_at(value, at, &["agent_id", "session_id", "parent_agent_id", "role", "spawned_by", "at"])?;
    Ok(agenticspec_types::session::StartAgent {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        session_id: {
            let at1 = json::nested(at, "session_id");
            let member1 = json::member_at(value, at, "session_id")?;
            decode_agenticspec_session_session_id(member1, &at1)?
        },
        parent_agent_id: match value.member("parent_agent_id") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "parent_agent_id");
                Some(decode_agenticspec_session_agent_id(member2, &at2)?)
            }
        },
        role: {
            let at3 = json::nested(at, "role");
            let member3 = json::member_at(value, at, "role")?;
            decode_agenticspec_session_agent_role(member3, &at3)?
        },
        spawned_by: match value.member("spawned_by") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "spawned_by");
                Some(decode_agenticspec_session_tool_call_id(member4, &at4)?)
            }
        },
        at: {
            let at5 = json::nested(at, "at");
            let member5 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member5, &at5, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the outcome of `agenticspec.session.StartAgent` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agenticspec_session_start_agent(value: &agenticspec_types::session::StartAgentOutcome, out: &mut String) {
    out.push('{');
    match value {
        agenticspec_types::session::StartAgentOutcome::Started { agent_started } => {
            json::member(out, "outcome");
            json::push_text(out, "started");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.AgentStarted");
            json::member(out, "payload");
            encode_event_agenticspec_session_agent_started(agent_started, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agenticspec.session.SubmitPrompt` as JSON.
pub fn encode_command_agenticspec_session_submit_prompt(value: &agenticspec_types::session::SubmitPrompt, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agenticspec_session_agent_id(&value.agent_id, out);
    json::member(out, "author");
    encode_agenticspec_session_prompt_author(&value.author, out);
    json::member(out, "attribution");
    encode_agenticspec_session_attribution(&value.attribution, out);
    json::member(out, "class");
    encode_agenticspec_session_prompt_class(&value.class, out);
    json::member(out, "content");
    encode_agenticspec_session_content(&value.content, out);
    json::member(out, "at");
    json::push_text(out, &value.at.0);
    out.push('}');
}

/// Reads the input of `agenticspec.session.SubmitPrompt` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agenticspec_session_submit_prompt(value: &json::Value, at: &str) -> Result<agenticspec_types::session::SubmitPrompt, json::DecodeError> {
    json::closed_at(value, at, &["agent_id", "author", "attribution", "class", "content", "at"])?;
    Ok(agenticspec_types::session::SubmitPrompt {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agenticspec_session_agent_id(member0, &at0)?
        },
        author: {
            let at1 = json::nested(at, "author");
            let member1 = json::member_at(value, at, "author")?;
            decode_agenticspec_session_prompt_author(member1, &at1)?
        },
        attribution: {
            let at2 = json::nested(at, "attribution");
            let member2 = json::member_at(value, at, "attribution")?;
            decode_agenticspec_session_attribution(member2, &at2)?
        },
        class: {
            let at3 = json::nested(at, "class");
            let member3 = json::member_at(value, at, "class")?;
            decode_agenticspec_session_prompt_class(member3, &at3)?
        },
        content: {
            let at4 = json::nested(at, "content");
            let member4 = json::member_at(value, at, "content")?;
            decode_agenticspec_session_content(member4, &at4)?
        },
        at: {
            let at5 = json::nested(at, "at");
            let member5 = json::member_at(value, at, "at")?;
            agenticspec_types::primitives::Timestamp(json::timestamp_at(member5, &at5, "an RFC 3339 instant")?.to_owned())
        },
    })
}

/// Writes the outcome of `agenticspec.session.SubmitPrompt` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agenticspec_session_submit_prompt(value: &agenticspec_types::session::SubmitPromptOutcome, out: &mut String) {
    out.push('{');
    match value {
        agenticspec_types::session::SubmitPromptOutcome::Submitted { prompt_submitted } => {
            json::member(out, "outcome");
            json::push_text(out, "submitted");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agenticspec.session.PromptSubmitted");
            json::member(out, "payload");
            encode_event_agenticspec_session_prompt_submitted(prompt_submitted, out);
            out.push('}');
            out.push(']');
        }
        agenticspec_types::session::SubmitPromptOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.AgentStateConflict");
            json::member(out, "payload");
            encode_error_agenticspec_session_agent_state_conflict(error, out);
            out.push('}');
        }
        agenticspec_types::session::SubmitPromptOutcome::WrongStateUnknownInstance => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agenticspec.session.AgentStateConflict");
            out.push('}');
        }
    }
    out.push('}');
}

/// The finite `Binary64` at this path: a JSON number, rounded once to the nearest binary64,
/// and refused where it rounds to an infinity.
#[allow(dead_code)]
fn binary64_at(value: &json::Value, at: &str) -> Result<agenticspec_types::primitives::Binary64, json::DecodeError> {
    match value {
        json::Value::Number(token) => agenticspec_types::primitives::Binary64::from_token(token).ok_or_else(|| json::DecodeError {
            at: at.to_owned(),
            expected: "a finite Binary64 number".to_owned(),
            found: format!("the number {token}"),
        }),
        other => Err(json::DecodeError::of(at, "a finite Binary64 number", other)),
    }
}

/// Writes any event on the system's log as JSON: its qualified name and its payload,
/// `{"event": …, "payload": {…}}`, the envelope a command's answer lists it in.
pub fn encode_system_event(value: &agenticspec_system::SystemEvent) -> String {
    let mut out = String::from("{");
    json::member(&mut out, "event");
    json::push_text(&mut out, value.name());
    json::member(&mut out, "payload");
    match value {
        agenticspec_system::SystemEvent::AgentEnded(event) => encode_event_agenticspec_session_agent_ended(event, &mut out),
        agenticspec_system::SystemEvent::AgentStarted(event) => encode_event_agenticspec_session_agent_started(event, &mut out),
        agenticspec_system::SystemEvent::ContextCompacted(event) => encode_event_agenticspec_session_context_compacted(event, &mut out),
        agenticspec_system::SystemEvent::ExposureDetected(event) => encode_event_agenticspec_session_exposure_detected(event, &mut out),
        agenticspec_system::SystemEvent::ModelTurnRecorded(event) => encode_event_agenticspec_session_model_turn_recorded(event, &mut out),
        agenticspec_system::SystemEvent::PromptSubmitted(event) => encode_event_agenticspec_session_prompt_submitted(event, &mut out),
        agenticspec_system::SystemEvent::SessionEnded(event) => encode_event_agenticspec_session_session_ended(event, &mut out),
        agenticspec_system::SystemEvent::SessionOpened(event) => encode_event_agenticspec_session_session_opened(event, &mut out),
        agenticspec_system::SystemEvent::ToolCallCompleted(event) => encode_event_agenticspec_session_tool_call_completed(event, &mut out),
        agenticspec_system::SystemEvent::ToolCallDecided(event) => encode_event_agenticspec_session_tool_call_decided(event, &mut out),
        agenticspec_system::SystemEvent::ToolCallRequested(event) => encode_event_agenticspec_session_tool_call_requested(event, &mut out),
        agenticspec_system::SystemEvent::TurnEnded(event) => encode_event_agenticspec_session_turn_ended(event, &mut out),
        agenticspec_system::SystemEvent::AgentUpdated(event) => encode_event_agenticspec_view_agent_updated(event, &mut out),
        agenticspec_system::SystemEvent::EventAppended(event) => encode_event_agenticspec_view_event_appended(event, &mut out),
        agenticspec_system::SystemEvent::SessionUpdated(event) => encode_event_agenticspec_view_session_updated(event, &mut out),
    }
    out.push('}');
    out
}

/// Reads any event on the system's log back from the envelope [`encode_system_event`] writes,
/// `{"event": …, "payload": {…}}`: its qualified name selects the event, and its payload is read
/// under the wire names that event declares.
///
/// # Errors
///
/// [`json::DecodeError`] at `event` when the envelope names no event this system publishes,
/// naming the one it carried, and at the payload's own path when the payload is not that
/// event's.
pub fn decode_system_event(value: &json::Value) -> Result<agenticspec_system::SystemEvent, json::DecodeError> {
    let event = json::text_at(json::member_at(value, "", "event")?, "event", "a qualified event name")?;
    Ok(match event {
        "agenticspec.session.AgentEnded" => agenticspec_system::SystemEvent::AgentEnded(decode_event_agenticspec_session_agent_ended(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.session.AgentStarted" => agenticspec_system::SystemEvent::AgentStarted(decode_event_agenticspec_session_agent_started(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.session.ContextCompacted" => agenticspec_system::SystemEvent::ContextCompacted(decode_event_agenticspec_session_context_compacted(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.session.ExposureDetected" => agenticspec_system::SystemEvent::ExposureDetected(decode_event_agenticspec_session_exposure_detected(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.session.ModelTurnRecorded" => agenticspec_system::SystemEvent::ModelTurnRecorded(decode_event_agenticspec_session_model_turn_recorded(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.session.PromptSubmitted" => agenticspec_system::SystemEvent::PromptSubmitted(decode_event_agenticspec_session_prompt_submitted(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.session.SessionEnded" => agenticspec_system::SystemEvent::SessionEnded(decode_event_agenticspec_session_session_ended(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.session.SessionOpened" => agenticspec_system::SystemEvent::SessionOpened(decode_event_agenticspec_session_session_opened(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.session.ToolCallCompleted" => agenticspec_system::SystemEvent::ToolCallCompleted(decode_event_agenticspec_session_tool_call_completed(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.session.ToolCallDecided" => agenticspec_system::SystemEvent::ToolCallDecided(decode_event_agenticspec_session_tool_call_decided(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.session.ToolCallRequested" => agenticspec_system::SystemEvent::ToolCallRequested(decode_event_agenticspec_session_tool_call_requested(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.session.TurnEnded" => agenticspec_system::SystemEvent::TurnEnded(decode_event_agenticspec_session_turn_ended(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.view.AgentUpdated" => agenticspec_system::SystemEvent::AgentUpdated(decode_event_agenticspec_view_agent_updated(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.view.EventAppended" => agenticspec_system::SystemEvent::EventAppended(decode_event_agenticspec_view_event_appended(json::member_at(value, "", "payload")?, "payload")?),
        "agenticspec.view.SessionUpdated" => agenticspec_system::SystemEvent::SessionUpdated(decode_event_agenticspec_view_session_updated(json::member_at(value, "", "payload")?, "payload")?),
        other => {
            return Err(json::DecodeError { at: "event".to_owned(), expected: "one of `agenticspec.session.AgentEnded`, `agenticspec.session.AgentStarted`, `agenticspec.session.ContextCompacted`, `agenticspec.session.ExposureDetected`, `agenticspec.session.ModelTurnRecorded`, `agenticspec.session.PromptSubmitted`, `agenticspec.session.SessionEnded`, `agenticspec.session.SessionOpened`, `agenticspec.session.ToolCallCompleted`, `agenticspec.session.ToolCallDecided`, `agenticspec.session.ToolCallRequested`, `agenticspec.session.TurnEnded`, `agenticspec.view.AgentUpdated`, `agenticspec.view.EventAppended`, `agenticspec.view.SessionUpdated`".to_owned(), found: format!("`{other}`") })
        }
    })
}
