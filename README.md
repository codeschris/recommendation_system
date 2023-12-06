# recommendation_system

Welcome to your new recommendation_system project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

This code establishes the infrastructure for a recommendation system within the Internet Computer ecosystem using Rust. It's designed to enable machine learning-driven understanding of user preferences and offer personalized recommendations for movies, products, music, and more. CRUD operations are implemented to manage user profiles and preferences effectively.


To learn more before you start working with recommendation_system, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)


### Data Structures and Definitions 

- **Structs Definition:** Defines the data structures (`User`, `Item`, `UserPreference`, `RecommendationSystem`) representing users, items, user preferences, and the recommendation system itself. These structures hold information about their respective entities.

- **Implementation of Storable and BoundedStorable Traits:** Implements serialization and deserialization for the defined data structures, allowing them to be stored and retrieved from the stable memory. It includes functions to convert the structs into bytes and vice versa.
If you want to start working on your project right away, you might want to try the following commands:

### Memory Management and Counters 

- **Thread-local Memory Management:** Sets up a memory manager to handle different data storages (users, items, preferences, recommendation systems) using IC Stable Structures.

- **ID Counters Initialization:** Initializes counters for generating unique IDs for users, items, user preferences, and recommendation systems.

### CRUD operations for Users 

- `get_users()`, `get_user_by_id(id)`, `add_user(payload)`, `update_user(id, payload)`, `delete_user(id)`: These functions handle operations related to users. They allow retrieval of all users, getting a user by ID, adding a new user, updating an existing user, and deleting a user.

### CRUD operations for Items 

- `get_items()`, `get_item_by_id(id)`, `add_item(payload)`, `update_item(id, payload)`, `delete_item(id)`: Similar to user functions but for managing items.

### CRUD operations for User Preferences 

- `get_user_preferences()`, `get_user_preference_by_id(id)`, `add_user_preference(payload)`, `update_user_preference(id, payload)`, `delete_user_preference(id)`: Manage user preferences, including retrieval by ID, addition, update, and deletion.

### CRUD Operations for  Recommendation systems

- `get_recommendation_systems()`, `get_recommendation_system_by_id(id)`, `add_recommendation_system()`, `update_recommendation_system(id)`,` delete_recommendation_system(id)`: Handle recommendation systems, allowing operations such as retrieval by ID, addition, update, and deletion.

### Association Management 

- `add_user_to_recommendation_system(recommendation_system_id, user_id)`, `add_item_to_recommendation_system(recommendation_system_id, item_id)`, `add_user_preference_to_recommendation_system(recommendation_system_id, user_preference_id)`: Functions to associate users, items, and user preferences with a specific recommendation system.

### Query Functions 

- ** Query Functions for Retrieval** : These functions allow querying users, items, user preferences, and recommendation systems based on specific criteria.

### Error Handling 
- `Error` **Enum**: Defines an error enum that encapsulates different error types for handling operations like "Not Found" errors.

### Candid Interface Export 

- `ic_cdk::export_candid!()`; Exports the Candid interface, allowing external interaction with the defined functions.





```bash
cd recommendation_system/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
