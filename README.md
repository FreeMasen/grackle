# Grackle

A collection of structs for generating/parsing github-actions workflows.


## Usage

```rust
Workflow::builder()
    .name("test-workflow")
    .on_push(
        Push::builder()
            .branch("main")
            .tag("v?*\\.[0-9]+\\.[0-9]+\\.[0-9]+")
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
                    .with("complex", serde_yml::Mapping::from_iter([
                        (Value::from("inner-key"), Value::from("inner-value")),
                        (Value::from("inner-key2"), Value::from(1)),
                    ].into_iter()))
                    .build(),
            )
            .build(),
    )
    .build()
```

Generates

```yml
name: test-workflow
"on":
  pull_request:
    branches-ignore:
      - test
  push:
    branches:
      - main
    tags:
      - "v?.[0-9]+.[0-9]+.[0-9]+"
  workflow_dispatch: {}
jobs:
  some-job:
    steps:
      - uses: some-action-user/some-action
        with:
          complex:
            inner-key: inner-value
            inner-key2: 1
          key: value
```
