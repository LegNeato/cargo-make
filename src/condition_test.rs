use super::*;
use crate::types::{
    Config, ConfigSection, EnvInfo, FlowInfo, RustVersionCondition, Step, Task, TaskCondition,
};
use indexmap::IndexMap;
use rust_info::types::{RustChannel, RustInfo};

#[test]
fn validate_env_set_empty() {
    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    };

    let enabled = validate_env_set(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_set_valid() {
    env::set_var("ENV_SET1", "");
    env::set_var("ENV_SET2", "value");

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: Some(vec!["ENV_SET1".to_string(), "ENV_SET2".to_string()]),
        env_not_set: None,
        env: None,
        rust_version: None,
    };

    let enabled = validate_env_set(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_set_invalid() {
    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: Some(vec!["BAD_ENV_SET1".to_string(), "BAD_ENV_SET2".to_string()]),
        env_not_set: None,
        env: None,
        rust_version: None,
    };

    let enabled = validate_env_set(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_set_invalid_partial_found() {
    env::set_var("ENV_SET1", "");
    env::set_var("ENV_SET2", "value");

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: Some(vec![
            "ENV_SET1".to_string(),
            "ENV_SET2".to_string(),
            "BAD_ENV_SET1".to_string(),
        ]),
        env_not_set: None,
        env: None,
        rust_version: None,
    };

    let enabled = validate_env_set(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_not_set_empty() {
    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    };

    let enabled = validate_env_not_set(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_not_set_valid() {
    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: Some(vec!["BAD_ENV_SET1".to_string(), "BAD_ENV_SET2".to_string()]),
        env: None,
        rust_version: None,
    };

    let enabled = validate_env_not_set(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_not_set_invalid() {
    env::set_var("ENV_SET1", "");
    env::set_var("ENV_SET2", "value");

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: Some(vec!["ENV_SET1".to_string(), "ENV_SET2".to_string()]),
        env: None,
        rust_version: None,
    };

    let enabled = validate_env_not_set(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_not_set_invalid_partial_found() {
    env::set_var("ENV_SET1", "");
    env::set_var("ENV_SET2", "value");

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: Some(vec![
            "ENV_SET1".to_string(),
            "ENV_SET2".to_string(),
            "BAD_ENV_SET1".to_string(),
        ]),
        env: None,
        rust_version: None,
    };

    let enabled = validate_env_not_set(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_empty() {
    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    };

    let enabled = validate_env(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_valid() {
    env::set_var("ENV_SET1", "");
    env::set_var("ENV_SET2", "value");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "".to_string());
    env_values.insert("ENV_SET2".to_string(), "value".to_string());

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
        rust_version: None,
    };

    let enabled = validate_env(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_invalid_not_found() {
    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("BAD_ENV_SET1".to_string(), "".to_string());
    env_values.insert("BAD_ENV_SET2".to_string(), "value".to_string());

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
        rust_version: None,
    };

    let enabled = validate_env(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_invalid_not_equal() {
    env::set_var("ENV_SET2", "value");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET2".to_string(), "value2".to_string());

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
        rust_version: None,
    };

    let enabled = validate_env(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_invalid_partial_found() {
    env::set_var("ENV_SET1", "good");
    env::set_var("ENV_SET2", "good");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good".to_string());
    env_values.insert("ENV_SET2".to_string(), "bad".to_string());

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
        rust_version: None,
    };

    let enabled = validate_env(&condition);

    assert!(!enabled);
}

#[test]
fn validate_script_empty() {
    let task = Task::new();
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    let enabled = validate_script(&step);

    assert!(enabled);
}

#[test]
fn validate_script_valid() {
    let mut task = Task::new();
    task.condition_script = Some(vec!["exit 0".to_string()]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    let enabled = validate_script(&step);

    assert!(enabled);
}

#[test]
fn validate_script_invalid() {
    let mut task = Task::new();
    task.condition_script = Some(vec!["exit 1".to_string()]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    let enabled = validate_script(&step);

    assert!(!enabled);
}

#[test]
fn validate_platform_valid() {
    let condition = TaskCondition {
        platforms: Some(vec![
            "bad1".to_string(),
            types::get_platform_name(),
            "bad2".to_string(),
        ]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    };

    let enabled = validate_platform(&condition);

    assert!(enabled);
}

#[test]
fn validate_platform_invalid() {
    let condition = TaskCondition {
        platforms: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    };

    let enabled = validate_platform(&condition);

    assert!(!enabled);
}

#[test]
fn validate_channel_valid() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    let mut condition = TaskCondition {
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "stable".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    };
    let mut enabled = validate_channel(&condition, &flow_info);
    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Beta);
    condition = TaskCondition {
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "beta".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    };
    enabled = validate_channel(&condition, &flow_info);

    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Nightly);
    condition = TaskCondition {
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "nightly".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    };
    enabled = validate_channel(&condition, &flow_info);

    assert!(enabled);
}

#[test]
fn validate_channel_invalid() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    let condition = TaskCondition {
        platforms: None,
        channels: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    };
    let enabled = validate_channel(&condition, &flow_info);

    assert!(!enabled);
}

#[test]
fn validate_criteria_empty() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    });

    let enabled = validate_criteria(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_criteria_valid_platform() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec![
            "bad1".to_string(),
            types::get_platform_name(),
            "bad2".to_string(),
        ]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    });

    let enabled = validate_criteria(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_criteria_invalid_platform() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    });

    let enabled = validate_criteria(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_valid_channel() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "stable".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    });
    let mut enabled = validate_criteria(&flow_info, &step);

    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Beta);
    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "beta".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    });
    enabled = validate_criteria(&flow_info, &step);

    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Nightly);
    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "nightly".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    });
    enabled = validate_criteria(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_criteria_invalid_channel() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    });
    let enabled = validate_criteria(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_condition_both_valid() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec![
            "bad1".to_string(),
            types::get_platform_name(),
            "bad2".to_string(),
        ]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_criteria_valid_script_invalid() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec![
            "bad1".to_string(),
            types::get_platform_name(),
            "bad2".to_string(),
        ]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    });
    step.config.condition_script = Some(vec!["exit 1".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_invalid_script_valid() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_invalid_env_set() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: Some(vec!["BAD_ENV_SET1".to_string()]),
        env_not_set: None,
        env: None,
        rust_version: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_invalid_env_not_set() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    env::set_var("ENV_SET1", "bad");

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: Some(vec!["ENV_SET1".to_string()]),
        env: None,
        rust_version: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_valid_env() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    env::set_var("ENV_SET1", "good1");
    env::set_var("ENV_SET2", "good2");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good1".to_string());
    env_values.insert("ENV_SET2".to_string(), "good2".to_string());

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
        rust_version: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_criteria_invalid_env_not_found() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("BAD_ENV_SET1".to_string(), "good".to_string());
    env_values.insert("BAD_ENV_SET2".to_string(), "bad".to_string());

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
        rust_version: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_invalid_env_not_equal() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    env::set_var("ENV_SET1", "good");
    env::set_var("ENV_SET2", "good");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good".to_string());
    env_values.insert("ENV_SET2".to_string(), "bad".to_string());

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
        rust_version: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_valid_rust_version() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let rustinfo = rust_info::get();
    let version = rustinfo.version.unwrap();

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: Some(RustVersionCondition {
            min: None,
            max: None,
            equal: Some(version),
        }),
    });

    let enabled = validate_condition(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_criteria_invalid_rust_version() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo::default(),
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let rustinfo = rust_info::get();
    let mut version = rustinfo.version.unwrap();
    version.push_str("1");

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: Some(RustVersionCondition {
            min: None,
            max: None,
            equal: Some(version),
        }),
    });

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_no_rustinfo() {
    let rustinfo = RustInfo::new();

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("2.0.0".to_string()),
            max: Some("1.0.0".to_string()),
            equal: Some("3.0.0".to_string()),
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_empty_condition() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: None,
            equal: None,
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_min_enabled() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("1.9.9".to_string()),
            max: None,
            equal: None,
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_min_same() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("2.0.0".to_string()),
            max: None,
            equal: None,
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_min_disabled_major() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("3.0.0".to_string()),
            max: None,
            equal: None,
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_min_disabled_minor() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("2.1.0".to_string()),
            max: None,
            equal: None,
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_min_disabled_patch() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("2.0.1".to_string()),
            max: None,
            equal: None,
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_max_enabled() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("1.9.9".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: Some("2.0.0".to_string()),
            equal: None,
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_max_same() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: Some("2.0.0".to_string()),
            equal: None,
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_max_disabled_major() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("3.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: Some("2.0.0".to_string()),
            equal: None,
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_max_disabled_minor() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.1.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: Some("2.0.0".to_string()),
            equal: None,
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_max_disabled_patch() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.1".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: Some("2.0.0".to_string()),
            equal: None,
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_equal_same() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: None,
            equal: Some("2.0.0".to_string()),
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_equal_not_same() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: None,
            equal: Some("3.0.0".to_string()),
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_all_enabled() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("1.0.0".to_string()),
            max: Some("3.0.0".to_string()),
            equal: Some("2.0.0".to_string()),
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_no_condition() {
    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: None,
    };

    let enabled = validate_rust_version(&condition);

    assert!(enabled);
}

#[test]
fn validate_rust_version_with_valid_condition() {
    let rustinfo = rust_info::get();
    let version = rustinfo.version.unwrap();

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: Some(RustVersionCondition {
            min: Some(version.clone()),
            max: Some(version.clone()),
            equal: Some(version.clone()),
        }),
    };

    let enabled = validate_rust_version(&condition);

    assert!(enabled);
}

#[test]
fn validate_rust_version_with_invalid_condition() {
    let rustinfo = rust_info::get();
    let mut version = rustinfo.version.unwrap();
    version.push_str("1");

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
        rust_version: Some(RustVersionCondition {
            min: None,
            max: None,
            equal: Some(version),
        }),
    };

    let enabled = validate_rust_version(&condition);

    assert!(!enabled);
}
