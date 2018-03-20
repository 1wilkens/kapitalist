# kapitalist - todo

- Error handling
    1. Add proper conversion traits from error sources to rocket responses
        - diesel
        - jwt?
        - std::io?
    2. Replace `unwrap` and `expect` with `?`
- Configuration
    - Think about required config values
    - Think about environment vs config file
    - Properly use environment for..
        - `JWT_SECRET`
        - ..
