generate-parsers:
    rewrite=1 cargo test --package fall_gen --test cli

update-test-data:
    rewrite_test_data=1 cargo test --all

code:
    cd code && npm install && ./node_modules/vsce/out/vsce package
