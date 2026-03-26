#![no_std]


// --- ERROR CODES ---
pub const CLAWBACK_DETECTED: u32 = 2001;
pub const ROUND_ALREADY_PAUSED: u32 = 2002;
pub const ROUND_NOT_PAUSED: u32 = 2003;
pub const INSUFFICIENT_RECOVERY_FUNDS: u32 = 2004;
pub const RECOVERY_PLAN_NOT_ACTIVE: u32 = 2005;

// --- DATA STRUCTURES ---

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Circle(u64),
    Member(Address),

}

#[contracttype]
#[derive(Clone)]
pub struct Member {
    pub address: Address,
    pub index: u32,
    pub contribution_count: u32,
    pub last_contribution_time: u64,

}

#[contracttype]
#[derive(Clone)]
pub struct CircleInfo {
    pub id: u64,
    pub creator: Address,

    pub is_active: bool,
    pub token: Address,
    pub deadline_timestamp: u64,
    pub cycle_duration: u64,
    pub contribution_bitmap: u64,


}

#[contracttype]
#[derive(Clone)]

    }

    fn deposit(env: Env, user: Address, circle_id: u64) {
        user.require_auth();

