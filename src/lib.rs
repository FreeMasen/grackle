use serde::{Deserialize, Serialize};
use serde_yml::Value;
use std::collections::BTreeMap;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "kebab-case")]
#[builder(mutators(
    fn on_pull_request(&mut self, pull: PullRequest) {
        self.on.pull_request = Some(pull);
    }
    fn on_pull_request_target(&mut self, pull: PullRequest) {
        self.on.pull_request_target = Some(pull);
    }
    fn on_push(&mut self, push: Push) {
        self.on.push = Some(push);
    }
    fn on_schedule(&mut self, sched: Schedule) {
        self.on.schedule = Some(sched);
    }
    fn on_workflow_call(&mut self, call: WorkflowCall) {
        self.on.workflow_call = Some(call);
    }
    fn on_workflow_dispatch(&mut self, disp: WorkflowDispatch) {
        self.on.workflow_dispatch = Some(disp);
    }
    fn actions_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.actions = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().actions(PermissionValue::Write).build())
        }
    }
    fn actions_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.actions = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().actions(PermissionValue::Read).build())
        }
    }
    fn attestations_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.attestations = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().attestations(PermissionValue::Write).build())
        }
    }
    fn attestations_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.attestations = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().attestations(PermissionValue::Read).build())
        }
    }
    fn checks_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.checks = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().checks(PermissionValue::Write).build())
        }
    }
    fn checks_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.checks = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().checks(PermissionValue::Read).build())
        }
    }
    fn contents_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.contents = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().contents(PermissionValue::Write).build())
        }
    }
    fn contents_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.contents = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().contents(PermissionValue::Read).build())
        }
    }
    fn deployments_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.deployments = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().deployments(PermissionValue::Write).build())
        }
    }
    fn deployments_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.deployments = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().deployments(PermissionValue::Read).build())
        }
    }
    fn id_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.id = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().id(PermissionValue::Write).build())
        }
    }
    fn id_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.id = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().id(PermissionValue::Read).build())
        }
    }
    fn issues_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.issues = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().issues(PermissionValue::Write).build())
        }
    }
    fn issues_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.issues = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().issues(PermissionValue::Read).build())
        }
    }
    fn discussions_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.discussions = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().discussions(PermissionValue::Write).build())
        }
    }
    fn discussions_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.discussions = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().discussions(PermissionValue::Read).build())
        }
    }
    fn packages_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.packages = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().packages(PermissionValue::Write).build())
        }
    }
    fn packages_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.packages = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().packages(PermissionValue::Read).build())
        }
    }
    fn pages_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.pages = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().pages(PermissionValue::Write).build())
        }
    }
    fn pages_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.pages = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().pages(PermissionValue::Read).build())
        }
    }
    fn pull_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.pull = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().pull(PermissionValue::Write).build())
        }
    }
    fn pull_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.pull = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().pull(PermissionValue::Read).build())
        }
    }
    fn repository_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.repository = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().repository(PermissionValue::Write).build())
        }
    }
    fn repository_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.repository = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().repository(PermissionValue::Read).build())
        }
    }
    fn security_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.security = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().security(PermissionValue::Write).build())
        }
    }
    fn security_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.security = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().security(PermissionValue::Read).build())
        }
    }
    fn statuses_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.statuses = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().statuses(PermissionValue::Write).build())
        }
    }
    fn statuses_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.statuses = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().statuses(PermissionValue::Read).build())
        }
    }

    fn env_var(&mut self, key: impl ToString, value: impl ToString) {
        self.env.insert(key.to_string(), value.to_string());
    }

    fn default_run_shell(&mut self, shell: impl ToString) {
        self.defaults.run.shell = Some(shell.to_string());
    }

    fn default_run_cwd(&mut self, cwd: impl ToString) {
        self.defaults.run.working_directory = Some(cwd.to_string());
    }

    fn concurrency_group(&mut self, group: impl Into<Value>) {
        self.concurrency.group = Some(group.into());
    }

    fn concurrency_cancel_in_progress(&mut self) {
        self.concurrency.cancel_in_progress = Some(true.into());
    }

    fn add_job(&mut self, name: impl ToString, job: Job) {
        self.jobs.insert(name.to_string(), job);
    }
))]
pub struct Workflow {
    #[builder(setter(into))]
    pub name: String,
    #[builder(default, setter(into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,
    #[builder(via_mutators(init = Default::default()))]
    #[serde(default, skip_serializing_if = "Triggers::is_empty")]
    pub on: Triggers,
    #[builder(via_mutators(init = Default::default()))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[builder(via_mutators(init = Default::default()))]
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub env: BTreeMap<String, String>,
    #[builder(via_mutators(init = Default::default()))]
    #[serde(default, skip_serializing_if = "Defaults::is_empty")]
    pub defaults: Defaults,
    #[serde(default, skip_serializing_if = "Concurrency::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub concurrency: Concurrency,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub jobs: BTreeMap<String, Job>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "snake_case")]
pub struct Triggers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request_target: Option<PullRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<Push>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_call: Option<WorkflowCall>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_dispatch: Option<WorkflowDispatch>,
}

impl Triggers {
    pub fn is_empty(&self) -> bool {
        self.pull_request.is_none()
            && self.pull_request_target.is_none()
            && self.push.is_none()
            && self.schedule.is_none()
            && self.workflow_call.is_none()
            && self.workflow_dispatch.is_none()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "kebab-case")]
#[builder(mutators(
    fn branch(&mut self, branch: impl ToString) {
        self.branches.push(branch.to_string());
    }
    fn ignore_branch(&mut self, branch: impl ToString) {
        self.branches_ignore.push(branch.to_string());
    }
    fn path(&mut self, path: impl ToString) {
        self.paths.push(path.to_string());
    }
    fn ignore_path(&mut self, path: impl ToString) {
        self.paths_ignore.push(path.to_string());
    }
    fn r#type(&mut self, value: impl ToString) {
        self.types.push(value.to_string());
    }
))]
pub struct PullRequest {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub branches: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub branches_ignore: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub paths: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub paths_ignore: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub types: Vec<String>,
}

impl PullRequest {
    pub fn is_empty(&self) -> bool {
        self.branches.is_empty()
            && self.branches_ignore.is_empty()
            && self.paths.is_empty()
            && self.paths_ignore.is_empty()
            && self.types.is_empty()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "kebab-case")]
#[builder(mutators(
    fn branch(&mut self, branch: impl ToString) {
        self.branches.push(branch.to_string());
    }
    fn ignore_branch(&mut self, branch: impl ToString) {
        self.branches_ignore.push(branch.to_string());
    }
    fn tag(&mut self, tag: impl ToString) {
        self.tags.push(tag.to_string());
    }
    fn ignore_tag(&mut self, tag: impl ToString) {
        self.tags_ignore.push(tag.to_string());
    }
    fn path(&mut self, path: impl ToString) {
        self.paths.push(path.to_string());
    }
    fn ignore_path(&mut self, path: impl ToString) {
        self.paths_ignore.push(path.to_string());
    }
    fn r#type(&mut self, value: impl ToString) {
        self.types.push(value.to_string());
    }
))]
pub struct Push {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    branches: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    branches_ignore: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    tags_ignore: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    paths: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    paths_ignore: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    types: Vec<String>,
}

impl Push {
    pub fn is_empty(&self) -> bool {
        self.branches.is_empty()
            && self.tags.is_empty()
            && self.branches_ignore.is_empty()
            && self.tags_ignore.is_empty()
            && self.paths.is_empty()
            && self.paths_ignore.is_empty()
            && self.types.is_empty()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "kebab-case")]
#[builder(mutators(
    fn cron(&mut self, value: impl ToString) {
        self.cron.push(value.to_string());
    }
    fn r#type(&mut self, value: impl ToString) {
        self.types.push(value.to_string());
    }
))]
pub struct Schedule {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    cron: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    types: Vec<String>,
}

impl Schedule {
    pub fn is_empty(&self) -> bool {
        self.cron.is_empty() && self.types.is_empty()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "kebab-case")]
#[builder(mutators(
    fn input(&mut self,key: impl ToString, value: Input) {
        self.inputs.insert(key.to_string(), value);
    }
    fn output(&mut self,key: impl ToString, value: Output) {
        self.outputs.insert(key.to_string(), value);
    }
    fn secret(&mut self,key: impl ToString, value: Secret) {
        self.secrets.insert(key.to_string(), value);
    }
    fn branch(&mut self, branch: impl ToString) {
        self.branches.push(branch.to_string());
    }
    fn ignore_branch(&mut self, branch: impl ToString) {
        self.branches_ignore.push(branch.to_string());
    }
    fn r#type(&mut self, value: impl ToString) {
        self.types.push(value.to_string());
    }
))]
pub struct WorkflowCall {
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub inputs: BTreeMap<String, Input>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub outputs: BTreeMap<String, Output>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub secrets: BTreeMap<String, Secret>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub branches: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub branches_ignore: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub types: Vec<String>,
}

impl WorkflowCall {
    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty()
            && self.outputs.is_empty()
            && self.secrets.is_empty()
            && self.branches.is_empty()
            && self.branches_ignore.is_empty()
            && self.types.is_empty()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "kebab-case")]
#[builder(mutators(
    fn input(&mut self,key: impl ToString, value: Input) {
        self.inputs.insert(key.to_string(), value);
    }
    fn r#type(&mut self, value: impl ToString) {
        self.types.push(value.to_string());
    }
))]
pub struct WorkflowDispatch {
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub inputs: BTreeMap<String, Input>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub types: Vec<String>,
}

impl WorkflowDispatch {
    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty() && self.types.is_empty()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct Input {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    pub default: Option<Value>,
    #[serde(default)]
    pub required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    #[builder(setter(strip_option))]
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct Output {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    pub default: Option<Value>,
    #[serde(default)]
    pub required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    #[builder(setter(strip_option))]
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option, into)))]
pub struct Secret {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<Value>,
}

impl Secret {
    pub fn is_empty(&self) -> bool {
        self.description.is_none() && self.required.is_none()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct Permissions {
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub actions: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub attestations: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub checks: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub contents: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub deployments: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub id: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub issues: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub discussions: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub packages: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub pages: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub pull: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub repository: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub security: PermissionValue,
    #[serde(default, skip_serializing_if = "PermissionValue::is_none")]
    pub statuses: PermissionValue,
}

impl Permissions {
    pub fn is_empty(&self) -> bool {
        self.actions.is_none()
            && self.attestations.is_none()
            && self.checks.is_none()
            && self.contents.is_none()
            && self.deployments.is_none()
            && self.id.is_none()
            && self.issues.is_none()
            && self.discussions.is_none()
            && self.packages.is_none()
            && self.pages.is_none()
            && self.pull.is_none()
            && self.repository.is_none()
            && self.security.is_none()
            && self.statuses.is_none()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum PermissionValue {
    Read,
    Write,
    None,
}

impl PermissionValue {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl std::default::Default for PermissionValue {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[builder(mutators(
    fn shell(&mut self, shell: impl ToString) {
        self.run.shell = Some(shell.to_string());
    }
    fn working_directory(&mut self, cwd: impl ToString) {
        self.run.working_directory = Some(cwd.to_string());
    }
))]
pub struct Defaults {
    #[serde(default, skip_serializing_if = "RunDefaults::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub run: RunDefaults,
}

impl Defaults {
    pub fn is_empty(&self) -> bool {
        self.run.is_empty()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RunDefaults {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

impl RunDefaults {
    pub fn is_empty(&self) -> bool {
        self.shell.is_none() && self.working_directory.is_none()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Concurrency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_in_progress: Option<Value>,
}

impl Concurrency {
    pub fn is_empty(&self) -> bool {
        self.group.is_none() && self.cancel_in_progress.is_none()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "kebab-case")]
#[builder(mutators(
    fn actions_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.actions = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().actions(PermissionValue::Write).build())
        }
    }
    fn actions_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.actions = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().actions(PermissionValue::Read).build())
        }
    }
    fn attestations_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.attestations = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().attestations(PermissionValue::Write).build())
        }
    }
    fn attestations_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.attestations = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().attestations(PermissionValue::Read).build())
        }
    }
    fn checks_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.checks = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().checks(PermissionValue::Write).build())
        }
    }
    fn checks_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.checks = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().checks(PermissionValue::Read).build())
        }
    }
    fn contents_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.contents = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().contents(PermissionValue::Write).build())
        }
    }
    fn contents_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.contents = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().contents(PermissionValue::Read).build())
        }
    }
    fn deployments_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.deployments = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().deployments(PermissionValue::Write).build())
        }
    }
    fn deployments_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.deployments = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().deployments(PermissionValue::Read).build())
        }
    }
    fn id_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.id = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().id(PermissionValue::Write).build())
        }
    }
    fn id_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.id = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().id(PermissionValue::Read).build())
        }
    }
    fn issues_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.issues = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().issues(PermissionValue::Write).build())
        }
    }
    fn issues_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.issues = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().issues(PermissionValue::Read).build())
        }
    }
    fn discussions_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.discussions = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().discussions(PermissionValue::Write).build())
        }
    }
    fn discussions_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.discussions = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().discussions(PermissionValue::Read).build())
        }
    }
    fn packages_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.packages = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().packages(PermissionValue::Write).build())
        }
    }
    fn packages_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.packages = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().packages(PermissionValue::Read).build())
        }
    }
    fn pages_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.pages = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().pages(PermissionValue::Write).build())
        }
    }
    fn pages_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.pages = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().pages(PermissionValue::Read).build())
        }
    }
    fn pull_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.pull = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().pull(PermissionValue::Write).build())
        }
    }
    fn pull_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.pull = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().pull(PermissionValue::Read).build())
        }
    }
    fn repository_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.repository = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().repository(PermissionValue::Write).build())
        }
    }
    fn repository_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.repository = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().repository(PermissionValue::Read).build())
        }
    }
    fn security_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.security = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().security(PermissionValue::Write).build())
        }
    }
    fn security_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.security = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().security(PermissionValue::Read).build())
        }
    }
    fn statuses_write_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.statuses = PermissionValue::Write;
        } else {
            self.permissions = Some(Permissions::builder().statuses(PermissionValue::Write).build())
        }
    }
    fn statuses_read_permission(&mut self) {
        if let Some(perms) = self.permissions.as_mut() {
            perms.statuses = PermissionValue::Read;
        } else {
            self.permissions = Some(Permissions::builder().statuses(PermissionValue::Read).build())
        }
    }
    fn needs(&mut self, value: impl ToString) {
        self.needs.push(value.to_string())
    }

    fn output(&mut self, key: impl ToString, value: impl ToString) {
        self.outputs.insert(key.to_string(), value.to_string());
    }
    fn env_var(&mut self, key: impl ToString, value: impl ToString) {
        self.env.insert(key.to_string(), value.to_string());
    }
    fn add_step(&mut self, step: JobStep) {
        self.steps.push(step);
    }
    fn service(&mut self, name: impl ToString, container: Container) {
        self.services.insert(name.to_string(), container);
    }
    fn with(&mut self, key: impl ToString, value: impl Into<Value>) {
        self.with.insert(key.to_string(), value.into());
    }
    fn secret(&mut self, key: impl ToString, value: impl ToString) {
        self.secrets.insert(key.to_string(), value.to_string());
    }
))]
pub struct Job {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub name: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub id: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(via_mutators(init = Default::default()))]
    pub permissions: Option<Permissions>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub needs: Vec<String>,
    #[serde(rename = "if")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub if_cond: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub runs_on: Option<String>,
    #[serde(default, skip_serializing_if = "Environment::is_empty")]
    #[builder(default)]
    pub environment: Environment,
    #[serde(default, skip_serializing_if = "Concurrency::is_empty")]
    #[builder(default)]
    pub concurrency: Concurrency,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub outputs: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub env: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Defaults::is_empty")]
    #[builder(default)]
    pub defaults: Defaults,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub steps: Vec<JobStep>,
    #[serde(default, skip_serializing_if = "Strategy::is_empty")]
    #[builder(default)]
    pub strategy: Strategy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub timeout_minutes: Option<Value>,
    #[serde(default, skip_serializing_if = "Container::is_empty")]
    #[builder(default)]
    pub container: Container,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub services: BTreeMap<String, Container>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub uses: Option<Value>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub with: BTreeMap<String, Value>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub secrets: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option, into)))]
pub struct Environment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Environment {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.url.is_none()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "kebab-case")]
#[builder(mutators(
    fn with(&mut self, key: impl ToString, value: impl Into<Value>) {
        self.with.insert(key.to_string(), value.into());
    }

    fn env(&mut self, key: impl ToString, value: impl ToString) {
        self.env.insert(key.to_string(), value.to_string());
    }
))]
pub struct JobStep {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub id: Option<String>,
    #[serde(default, rename = "if", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub if_cond: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub uses: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub run: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub working_directory: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub shell: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub with: BTreeMap<String, Value>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub env: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub stragegy: Option<Strategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub continue_on_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub timeout_minutes: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "kebab-case")]
#[builder(mutators(
    fn matrix(&mut self, key: impl ToString, value: impl IntoIterator<Item = Value>) {
        self.matrix.insert(key.to_string(), Vec::from_iter(value.into_iter()));
    }

    fn add_to_matrix(&mut self, key: impl ToString, value: impl Into<Value>) {
        let value = value.into();
        self.matrix.entry(key.to_string())
            .and_modify(|v| v.push(value.clone()))
            .or_insert_with(|| {
                vec![value]
            });
    }
))]
pub struct Strategy {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub matrix: BTreeMap<String, Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option, into))]
    pub fail_fast: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option, into))]
    pub max_parallel: Option<Value>,
}

impl Strategy {
    pub fn is_empty(&self) -> bool {
        self.matrix.is_empty() && self.fail_fast.is_none() && self.max_parallel.is_none()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[builder(mutators(
    fn env_var(&mut self, key: impl ToString, value: impl Into<Value>) {
        self.env.insert(key.to_string(), value.into());
    }

    fn port(&mut self, port: impl Into<Value>) {
        self.ports.push(port.into());
    }

    fn volume(&mut self, volume: impl Into<Value>) {
        self.volumes.push(volume.into());
    }
    fn option(&mut self, option: impl Into<Value>) {
        self.options.push(option.into());
    }
))]
pub struct Container {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option, into))]
    pub image: Option<Value>,
    #[serde(default, skip_serializing_if = "Credentials::is_empty")]
    pub credentials: Credentials,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub env: BTreeMap<String, Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub ports: Vec<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub volumes: Vec<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(via_mutators(init = Default::default()))]
    pub options: Vec<Value>,
}

impl Container {
    pub fn is_empty(&self) -> bool {
        self.image.is_none()
            && self.credentials.is_empty()
            && self.env.is_empty()
            && self.ports.is_empty()
            && self.volumes.is_empty()
            && self.options.is_empty()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct Credentials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl Credentials {
    pub fn is_empty(&self) -> bool {
        self.username.is_none() && self.password.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_builder() {
        let workflow = Workflow::builder()
            .name("test-workflow")
            .on_push(
                Push::builder()
                    .branch("main")
                    .tag(r#"v?.[0-9]+.[0-9]+.[0-9]+"#)
                    .build(),
            )
            .on_pull_request(PullRequest::builder().ignore_branch("test").build())
            .on_workflow_dispatch(WorkflowDispatch::builder().build())
            .add_job(
                "some-job",
                Job::builder()
                    .add_step(
                        JobStep::builder()
                            .uses("some-action-user/some-action")
                            .with("key", "value")
                            .with(
                                "complex",
                                serde_yml::Mapping::from_iter(
                                    [
                                        (Value::from("inner-key"), Value::from("inner-value")),
                                        (Value::from("inner-key2"), Value::from(1)),
                                    ]
                                    .into_iter(),
                                ),
                            )
                            .build(),
                    )
                    .build(),
            )
            .build();
        insta::assert_yaml_snapshot!(workflow);
    }
}
