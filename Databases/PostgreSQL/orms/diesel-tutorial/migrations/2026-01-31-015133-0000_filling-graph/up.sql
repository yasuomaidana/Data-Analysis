INSERT INTO entities (id, _type, _class, metadata)
VALUES ('account_1', 'github_account', 'Account', '{"name": "Account 1"}'),
       ('account_2', 'github_account', 'Account', '{"name": "Account 2"}'),
       ('user_1', 'github_user', 'User', '{"name": "User 1"}'),
       ('user_2', 'github_user', 'User', '{"name": "User 2"}'),
       ('user_3', 'github_user', 'User', '{"name": "User 3"}'),
       ('repository_1', 'github_repository', 'CodeRepo', '{"name": "Repository 1"}'),
       ('repository_2', 'github_repository', 'CodeRepo', '{"name": "Repository 2"}'),
       ('repository_3', 'github_repository', 'CodeRepo', '{"name": "Repository 3"}'),
       ('pr_1', 'github_pullRequest', 'PullRequest', '{"name": "Pull Request 1"}'),
       ('pr_2', 'github_pullRequest', 'PullRequest', '{"name": "Pull Request 2"}'),
       ('pr_3', 'github_pullRequest', 'PullRequest', '{"name": "Pull Request 3"}')
ON CONFLICT (id)
    DO NOTHING;

INSERT INTO relationships (source_entity_id, target_entity_id, _class)
VALUES
    ('account_1', 'user_1', 'HAS'),
    ('account_1', 'user_2', 'HAS'),
    ('account_2', 'user_3', 'HAS'),
    ('account_1', 'repository_1', 'HAS'),
    ('account_1', 'repository_2', 'HAS'),
    ('account_2', 'repository_3', 'HAS'),
    ('repository_1', 'pr_2', 'HAS'),
    ('repository_2', 'pr_1', 'HAS'),
    ('repository_3', 'pr_3', 'HAS'),
    ('user_2', 'pr_1', 'APPROVED'),
    ('user_2', 'pr_2', 'OPENED')
ON CONFLICT (source_entity_id, target_entity_id)
    DO NOTHING;