use diesel::RunQueryDsl;
use diesel::dsl::insert_into;
use orm_module::establish_connection;
use orm_module::model::graph::{Entity, Relationship};
use orm_module::schema::graph_schema::{entities, relationships};

fn main() {
    let mut connection = establish_connection();
    let entities_vals = [
        (
            "account_1",
            "github_account",
            "Account",
            r#"{"name": "Account 1"}"#,
        ),
        (
            "account_2",
            "github_account",
            "Account",
            r#"{"name": "Account 2"}"#,
        ),
        ("user_1", "github_user", "User", r#"{"name": "User 1"}"#),
        ("user_2", "github_user", "User", r#"{"name": "User 2"}"#),
        ("user_3", "github_user", "User", r#"{"name": "User 3"}"#),
        (
            "repository_1",
            "github_repository",
            "CodeRepo",
            r#"{"name": "Repository 1"}"#,
        ),
        (
            "repository_2",
            "github_repository",
            "CodeRepo",
            r#"{"name": "Repository 2"}"#,
        ),
        (
            "repository_3",
            "github_repository",
            "CodeRepo",
            r#"{"name": "Repository 3"}"#,
        ),
        (
            "pr_1",
            "github_pullRequest",
            "PullRequest",
            r#"{"name": "Pull Request 1"}"#,
        ),
        (
            "pr_2",
            "github_pullRequest",
            "PullRequest",
            r#"{"name": "Pull Request 2"}"#,
        ),
        (
            "pr_3",
            "github_pullRequest",
            "PullRequest",
            r#"{"name": "Pull Request 3"}"#,
        ),
    ]
    .map(|(id, _type, _class, metadata)| Entity {
        id: id.to_string(),
        _type: _type.to_string(),
        _class: _class.to_string(),
        metadata: Some(
            serde_json::from_str(metadata).expect("Failed to parse metadata JSON for seed entity"),
        ),
    });
    let entities_vals = insert_into(entities::table)
        .values(&entities_vals)
        .on_conflict(entities::id)
        .do_nothing()
        .get_results::<Entity>(&mut connection)
        .expect("Error saving entities");

    let relationships_in = [
        ("account_1", "user_1", "HAS"),
        ("account_1", "user_2", "HAS"),
        ("account_2", "user_3", "HAS"),
        ("account_1", "repository_1", "HAS"),
        ("account_1", "repository_2", "HAS"),
        ("account_2", "repository_3", "HAS"),
        ("repository_1", "pr_2", "HAS"),
        ("repository_2", "pr_1", "HAS"),
        ("repository_3", "pr_3", "HAS"),
        ("user_2", "pr_1", "APPROVED"),
        ("user_2", "pr_2", "OPENED"),
    ]
    .map(|(source, target, class)| Relationship {
        source_entity_id: source.to_string(),
        target_entity_id: target.to_string(),
        _class: Some(class.to_string()),
    });

    let relationships_in = insert_into(relationships::table)
        .values(&relationships_in)
        .on_conflict((
            relationships::source_entity_id,
            relationships::target_entity_id,
        ))
        .do_nothing()
        .get_results::<Relationship>(&mut connection)
        .expect("Error saving relationships");

    println!("New Entities: {:?}", entities_vals);
    println!("New Relationships: {:?}", relationships_in);
}
