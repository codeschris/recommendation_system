#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use ic_cdk::api::time;


type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    name: String,
    email: String,
    password: String,
    created_at: u64,
    updated_at: Option<u64>,
}

// struct to represent an item
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Item {
    id: u64,
    name: String,
    category: String,
    description: String,
    created_at: u64,
    updated_at: Option<u64>,
}

// struct to represent User preferences or interactions with items
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct UserPreference {
    id: u64,
    user_id: u64,
    item_id: u64,
    rating: u64,
    created_at: u64,
    updated_at: Option<u64>,
}

// Struct to manage the recommendation system
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecommendationSystem {
    id: u64,
    users : Vec<User>,
    items : Vec<Item>,
    user_preferences : Vec<UserPreference>,
    
}


// Implement Storable and BoundedStorable  traits for User
impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable and BoundedStorable  traits for Item
impl Storable for Item {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Item {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable and BoundedStorable  traits for UserPreference

impl Storable for UserPreference {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for UserPreference {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable and BoundedStorable  traits for RecommendationSystem

impl Storable for RecommendationSystem {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for RecommendationSystem {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// thread memory manager 
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static USER_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static ITEM_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
            .expect("Cannot create a counter")
    );

    static USER_PREFERENCE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 0)
            .expect("Cannot create a counter")
    );

    static RECOMMENDATION_SYSTEM_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))), 0)
            .expect("Cannot create a counter")
    );

    static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    static ITEM_STORAGE: RefCell<StableBTreeMap<u64, Item, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );

    static USER_PREFERENCE_STORAGE: RefCell<StableBTreeMap<u64, UserPreference, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))))
    );

    static RECOMMENDATION_SYSTEM_STORAGE: RefCell<StableBTreeMap<u64, RecommendationSystem, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7))))
    );
}

// user payload
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct UserPayload {
    name: String,
    email: String,
    password: String,
}

// item payload
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct ItemPayload {
    name: String,
    category: String,
    description: String,
}

// user preference payload
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct UserPreferencePayload {
    user_id: u64,
    item_id: u64,
    rating: u64,
}


// function to get all users
#[ic_cdk::query]
fn get_users() -> Result<Vec<User>,Error> {

    let users = USER_STORAGE.with(|m| m.borrow().iter().map(|(_, v)| v.clone()).collect::<Vec<_>>());
    if users.len() == 0 {
        return Err(Error::NotFound { msg: "No users found".to_string() });
    }
    Ok(users)
}

// function to get user by id
#[ic_cdk::query]
fn get_user_by_id(id: u64) -> Result<User,Error> {
    USER_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("player with id={} not found", id),
            })
    })
}

// function to add user
#[ic_cdk::update]
fn add_user(payload: UserPayload) -> Result<User,Error> {

    // validate user payload all fields are required
    if payload.name.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::NotFound { msg: "All fields are required".to_string() });
    }
    let id = USER_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");

    let user = User {
        id,
        name: payload.name,
        email: payload.email,
        password: payload.password,
        created_at: time(),
        updated_at: None,
    };
    USER_STORAGE.with(|m| m.borrow_mut().insert(id, user.clone()));
    Ok(user)
}

// function to update user
#[ic_cdk::update]
fn update_user(id: u64, payload: UserPayload) -> Result<User,Error> {

    // validate user payload all fields are required
    if payload.name.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::NotFound { msg: "All fields are required".to_string() });
    }

    match USER_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut user) => {
            user.name = payload.name;
            user.email = payload.email;
            user.password = payload.password;
            user.updated_at = Some(time());
            USER_STORAGE.with(|m| m.borrow_mut().insert(id, user.clone()));
            Ok(user)
        }
        None => Err(Error::NotFound {
            msg: format!("user with id={} not found", id),
        }),
    }
}

// function to delete user
#[ic_cdk::update]
fn delete_user(id: u64) -> Result<(), Error>{
    USER_STORAGE.with(|service| {
        service
            .borrow_mut()
            .remove(&id)
            .ok_or(Error::NotFound {
                msg: format!("User with id={} not found", id),
            })
    })?;
    remove_user_from_recommendation_system(id);
    Ok(())
}

// remove user from recommendation system
fn remove_user_from_recommendation_system(user_id: u64){
    let recomandations_listing: Vec<(u64, RecommendationSystem)> =
        RECOMMENDATION_SYSTEM_STORAGE.with(|service| service.borrow().iter().collect());
    let mut recommendation_systems: Vec<RecommendationSystem> = recomandations_listing.into_iter().map(|(_, v)| v).collect();
    for recommendation_system in recommendation_systems.iter_mut() {
        let  users = recommendation_system.users.clone();
        recommendation_system.users.retain(|user| user.id != user_id);
        recommendation_system.users = users;
        RECOMMENDATION_SYSTEM_STORAGE.with(|m| m.borrow_mut().insert(recommendation_system.id, recommendation_system.clone()));
    }
}

// function to get all items
#[ic_cdk::query]
fn get_items() -> Result<Vec<Item>,Error> {

    let items = ITEM_STORAGE.with(|m| m.borrow().iter().map(|(_, v)| v.clone()).collect::<Vec<_>>());
    if items.len() == 0 {
        return Err(Error::NotFound { msg: "No items found".to_string() });
    }
    Ok(items)
}

// function to get item by id
#[ic_cdk::query]
fn get_item_by_id(id: u64) -> Result<Item,Error> {
    ITEM_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("item with id={} not found", id),
            })
    })
}

// function to add item
#[ic_cdk::update]
fn add_item(payload: ItemPayload) -> Result<Item,Error> {

    // validate item payload all fields are required
    if payload.name.is_empty() || payload.category.is_empty() || payload.description.is_empty() {
        return Err(Error::NotFound { msg: "All fields are required".to_string() });
    }
    let id = ITEM_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");

    let item = Item {
        id,
        name: payload.name,
        category: payload.category,
        description: payload.description,
        created_at: time(),
        updated_at: None,
    };
    ITEM_STORAGE.with(|m| m.borrow_mut().insert(id, item.clone()));
    Ok(item)
}

// function to update item
#[ic_cdk::update]
fn update_item(id: u64, payload: ItemPayload) -> Result<Item,Error> {

    // validate item payload all fields are required
    if payload.name.is_empty() || payload.category.is_empty() || payload.description.is_empty() {
        return Err(Error::NotFound { msg: "All fields are required".to_string() });
    }

    match ITEM_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut item) => {
            item.name = payload.name;
            item.category = payload.category;
            item.description = payload.description;
            item.updated_at = Some(time());
            ITEM_STORAGE.with(|m| m.borrow_mut().insert(id, item.clone()));
            Ok(item)
        }
        None => Err(Error::NotFound {
            msg: format!("item with id={} not found", id),
        }),
    }
}

// function to delete item
#[ic_cdk::update]

fn delete_item(id: u64) -> Result<(), Error>{
    ITEM_STORAGE.with(|service| {
        service
            .borrow_mut()
            .remove(&id)
            .ok_or(Error::NotFound {
                msg: format!("item with id={} not found", id),
            })
    })?;
    remove_item_from_recommendation_system(id);
    Ok(())
}

// remove item from recommendation system
fn remove_item_from_recommendation_system(item_id: u64){
    let recomandations_listing: Vec<(u64, RecommendationSystem)> =
        RECOMMENDATION_SYSTEM_STORAGE.with(|service| service.borrow().iter().collect());
    let mut recommendation_systems: Vec<RecommendationSystem> = recomandations_listing.into_iter().map(|(_, v)| v).collect();
    for recommendation_system in recommendation_systems.iter_mut() {
        let  items = recommendation_system.items.clone();
        recommendation_system.items.retain(|item| item.id != item_id);
        recommendation_system.items = items;
        RECOMMENDATION_SYSTEM_STORAGE.with(|m| m.borrow_mut().insert(recommendation_system.id, recommendation_system.clone()));
    }
}

// function to get all user preferences
#[ic_cdk::query]
fn get_user_preferences() -> Result<Vec<UserPreference>,Error> {

    let user_preferences = USER_PREFERENCE_STORAGE.with(|m| m.borrow().iter().map(|(_, v)| v.clone()).collect::<Vec<_>>());
    if user_preferences.len() == 0 {
        return Err(Error::NotFound { msg: "No user preferences found".to_string() });
    }
    Ok(user_preferences)
}

// function to get user preference by id
#[ic_cdk::query]
fn get_user_preference_by_id(id: u64) -> Result<UserPreference,Error> {
    USER_PREFERENCE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("user preference with id={} not found", id),
            })
    })
}

// function to add user preference
#[ic_cdk::update]
fn add_user_preference(payload: UserPreferencePayload) -> Result<UserPreference,Error> {

    // validate user preference payload all fields are required
    if payload.rating == 0 {
        return Err(Error::NotFound { msg: "All fields are required".to_string() });
    }
    let id = USER_PREFERENCE_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");

    let user_preference = UserPreference {
        id,
        user_id: payload.user_id,
        item_id: payload.item_id,
        rating: payload.rating,
        created_at: time(),
        updated_at: None,
    };
    USER_PREFERENCE_STORAGE.with(|m| m.borrow_mut().insert(id, user_preference.clone()));
    Ok(user_preference)
}

// function to update user preference
#[ic_cdk::update]
fn update_user_preference(id: u64, payload: UserPreferencePayload) -> Result<UserPreference,Error> {

    // validate user preference payload all fields are required
    if payload.rating == 0 {
        return Err(Error::NotFound { msg: "All fields are required".to_string() });
    }

    match USER_PREFERENCE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut user_preference) => {
            user_preference.user_id = payload.user_id;
            user_preference.item_id = payload.item_id;
            user_preference.rating = payload.rating;
            user_preference.updated_at = Some(time());
            USER_PREFERENCE_STORAGE.with(|m| m.borrow_mut().insert(id, user_preference.clone()));
            Ok(user_preference)
        }
        None => Err(Error::NotFound {
            msg: format!("user preference with id={} not found", id),
        }),
    }
}

// function to delete user preference

#[ic_cdk::update]
fn delete_user_preference(id: u64) -> Result<(), Error>{
    USER_PREFERENCE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .remove(&id)
            .ok_or(Error::NotFound {
                msg: format!("user preference with id={} not found", id),
            })
    })?;
    remove_user_preference_from_recommendation_system(id);
    Ok(())
}

// remove user preference from recommendation system
fn remove_user_preference_from_recommendation_system(user_preference_id: u64){
    let recomandations_listing: Vec<(u64, RecommendationSystem)> =
        RECOMMENDATION_SYSTEM_STORAGE.with(|service| service.borrow().iter().collect());
    let mut recommendation_systems: Vec<RecommendationSystem> = recomandations_listing.into_iter().map(|(_, v)| v).collect();
    for recommendation_system in recommendation_systems.iter_mut() {
        let  user_preferences = recommendation_system.user_preferences.clone();
        recommendation_system.user_preferences.retain(|user_preference| user_preference.id != user_preference_id);
        recommendation_system.user_preferences = user_preferences;
        RECOMMENDATION_SYSTEM_STORAGE.with(|m| m.borrow_mut().insert(recommendation_system.id, recommendation_system.clone()));
    }
}


// function to get all recommendation systems
#[ic_cdk::query]
fn get_recommendation_systems() -> Result<Vec<RecommendationSystem>,Error> {

    let recommendation_systems = RECOMMENDATION_SYSTEM_STORAGE.with(|m| m.borrow().iter().map(|(_, v)| v.clone()).collect::<Vec<_>>());
    if recommendation_systems.len() == 0 {
        return Err(Error::NotFound { msg: "No recommendation systems found".to_string() });
    }
    Ok(recommendation_systems)
}

// function to get recommendation system by id
#[ic_cdk::query]
fn get_recommendation_system_by_id(id: u64) -> Result<RecommendationSystem,Error> {
    RECOMMENDATION_SYSTEM_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("recommendation system with id={} not found", id),
            })
    })
}

// function to add recommendation system
#[ic_cdk::update]
fn add_recommendation_system() -> Result<RecommendationSystem,Error> {

    let id = RECOMMENDATION_SYSTEM_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");

    let recommendation_system = RecommendationSystem {
        id,
        users: vec![],
        items: vec![],
        user_preferences: vec![],
    };
    RECOMMENDATION_SYSTEM_STORAGE.with(|m| m.borrow_mut().insert(id, recommendation_system.clone()));
    Ok(recommendation_system)
}

// function to update recommendation system
#[ic_cdk::update]
fn update_recommendation_system(id: u64) -> Result<RecommendationSystem,Error> {

    match RECOMMENDATION_SYSTEM_STORAGE.with(|service| service.borrow().get(&id)) {
        Some( recommendation_system) => {
            RECOMMENDATION_SYSTEM_STORAGE.with(|m| m.borrow_mut().insert(id, recommendation_system.clone()));
            Ok(recommendation_system)
        }
        None => Err(Error::NotFound {
            msg: format!("recommendation system with id={} not found", id),
        }),
    }
}

// function to delete recommendation system
#[ic_cdk::update]
fn delete_recommendation_system(id: u64) -> Result<RecommendationSystem, Error>{
    match RECOMMENDATION_SYSTEM_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(recommendation_system) => Ok(recommendation_system),
        None => Err(Error::NotFound {
            msg: format!("recommendation system with id={} not found", id),
        }),
    }
}

//add user to recommendation system
#[ic_cdk::update]
fn add_user_to_recommendation_system(recommendation_system_id: u64, user_id: u64) -> Result<RecommendationSystem,Error> {
    let recommendation_system = RECOMMENDATION_SYSTEM_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&recommendation_system_id)
            .ok_or(Error::NotFound {
                msg: format!("recommendation system with id={} not found", recommendation_system_id),
            })
    })?;

    let user = USER_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&user_id)
            .ok_or(Error::NotFound {
                msg: format!("user with id={} not found", user_id),
            })
    })?;

    let mut recommendation_system = recommendation_system.clone();
    recommendation_system.users.push(user);
    RECOMMENDATION_SYSTEM_STORAGE.with(|m| m.borrow_mut().insert(recommendation_system_id, recommendation_system.clone()));
    Ok(recommendation_system)
    
}

//add item to recommendation system
#[ic_cdk::update]
fn add_item_to_recommendation_system(recommendation_system_id: u64, item_id: u64) -> Result<RecommendationSystem,Error> {
    let recommendation_system = RECOMMENDATION_SYSTEM_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&recommendation_system_id)
            .ok_or(Error::NotFound {
                msg: format!("recommendation system with id={} not found", recommendation_system_id),
            })
    })?;

    let item = ITEM_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&item_id)
            .ok_or(Error::NotFound {
                msg: format!("item with id={} not found", item_id),
            })
    })?;

    let mut recommendation_system = recommendation_system.clone();
    recommendation_system.items.push(item);
    RECOMMENDATION_SYSTEM_STORAGE.with(|m| m.borrow_mut().insert(recommendation_system_id, recommendation_system.clone()));
    Ok(recommendation_system)
    
}

//add user preference to recommendation system
#[ic_cdk::update]
fn add_user_preference_to_recommendation_system(recommendation_system_id: u64, user_preference_id: u64) -> Result<RecommendationSystem,Error> {
    let recommendation_system = RECOMMENDATION_SYSTEM_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&recommendation_system_id)
            .ok_or(Error::NotFound {
                msg: format!("recommendation system with id={} not found", recommendation_system_id),
            })
    })?;

    let user_preference = USER_PREFERENCE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&user_preference_id)
            .ok_or(Error::NotFound {
                msg: format!("user preference with id={} not found", user_preference_id),
            })
    })?;

    let mut recommendation_system = recommendation_system.clone();
    recommendation_system.user_preferences.push(user_preference);
    RECOMMENDATION_SYSTEM_STORAGE.with(|m| m.borrow_mut().insert(recommendation_system_id, recommendation_system.clone()));
    Ok(recommendation_system)
    
}

// function to get all users in recommendation system
#[ic_cdk::query]
fn get_users_in_recommendation_system(recommendation_system_id: u64) -> Result<Vec<User>,Error> {

    let recommendation_system = RECOMMENDATION_SYSTEM_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&recommendation_system_id)
            .ok_or(Error::NotFound {
                msg: format!("recommendation system with id={} not found", recommendation_system_id),
            })
    })?;

    let users = recommendation_system.users.clone();
    if users.len() == 0 {
        return Err(Error::NotFound { msg: "No users found".to_string() });
    }
    Ok(users)
}

// function to get all items in recommendation system
#[ic_cdk::query]
fn get_items_in_recommendation_system(recommendation_system_id: u64) -> Result<Vec<Item>,Error> {

    let recommendation_system = RECOMMENDATION_SYSTEM_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&recommendation_system_id)
            .ok_or(Error::NotFound {
                msg: format!("recommendation system with id={} not found", recommendation_system_id),
            })
    })?;

    let items = recommendation_system.items.clone();
    if items.len() == 0 {
        return Err(Error::NotFound { msg: "No items found".to_string() });
    }
    Ok(items)
}

// function to get all user preferences in recommendation system
#[ic_cdk::query]
fn get_user_preferences_in_recommendation_system(recommendation_system_id: u64) -> Result<Vec<UserPreference>,Error> {

    let recommendation_system = RECOMMENDATION_SYSTEM_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&recommendation_system_id)
            .ok_or(Error::NotFound {
                msg: format!("recommendation system with id={} not found", recommendation_system_id),
            })
    })?;

    let user_preferences = recommendation_system.user_preferences.clone();
    if user_preferences.len() == 0 {
        return Err(Error::NotFound { msg: "No user preferences found".to_string() });
    }
    Ok(user_preferences)
}



#[derive(candid::CandidType, Deserialize, Serialize)]
enum  Error {
    NotFound { msg: String },
}

// Export the candid interface
ic_cdk::export_candid!();