#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, Symbol, Vec};

#[contract]
pub struct AlumniAlms;

const ALUMNI: Symbol = Symbol::short("ALUMNI");
const DONATIONS: Symbol = Symbol::short("DONATE");
const REQUESTS: Symbol = Symbol::short("REQUEST");
const ALUMNI_CNT: Symbol = Symbol::short("A_CNT");
const DONATION_CNT: Symbol = Symbol::short("D_CNT");
const REQUEST_CNT: Symbol = Symbol::short("R_CNT");

#[contractimpl]
impl AlumniAlms {
    /// Register a graduate as an alumni donor.
    /// Stores the alumnus's address and graduation year so future donations
    /// can be attributed to a specific cohort.
    pub fn register_alumni(env: Env, alumnus: Address, graduation_year: u32) {
        alumnus.require_auth();

        let mut alumni: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&ALUMNI)
            .unwrap_or(Map::new(&env));

        if alumni.get(alumnus.clone()).is_some() {
            panic!("Alumni already registered");
        }

        alumni.set(alumnus.clone(), graduation_year);
        env.storage().instance().set(&ALUMNI, &alumni);

        let count: u32 = env
            .storage()
            .instance()
            .get(&ALUMNI_CNT)
            .unwrap_or(0u32);
        env.storage().instance().set(&ALUMNI_CNT, &(count + 1));
    }

    /// Record a donation made by a registered alumnus.
    /// The donation is stored with a numeric id, the donor's address, the
    /// amount pledged, an optional message, and a `matched` flag (false on
    /// creation). This function does NOT move any XLM; it only records the
    /// pledge in on-chain storage so that students can later request aid.
    pub fn donate(env: Env, donor: Address, amount: u64, message: Symbol) -> u32 {
        donor.require_auth();

        if amount == 0 {
            panic!("Donation amount must be positive");
        }

        let alumni: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&ALUMNI)
            .unwrap_or(Map::new(&env));
        if alumni.get(donor.clone()).is_none() {
            panic!("Donor is not a registered alumnus");
        }

        let mut donations: Map<u32, (Address, u64, Symbol, bool)> = env
            .storage()
            .instance()
            .get(&DONATIONS)
            .unwrap_or(Map::new(&env));

        let next_id: u32 = env
            .storage()
            .instance()
            .get(&DONATION_CNT)
            .unwrap_or(0u32)
            + 1;

        donations.set(next_id, (donor, amount, message, false));
        env.storage().instance().set(&DONATIONS, &donations);
        env.storage().instance().set(&DONATION_CNT, &next_id);

        next_id
    }

    /// Allow a current student to file an aid request.
    /// Each request is stored with a numeric id, the student's address, the
    /// amount requested, a short reason, and a `funded` flag that starts as
    /// false. Requests are publicly visible to all registered alumni.
    pub fn apply_for_aid(env: Env, student: Address, amount: u64, reason: Symbol) -> u32 {
        student.require_auth();

        if amount == 0 {
            panic!("Aid request amount must be positive");
        }

        let mut requests: Map<u32, (Address, u64, Symbol, bool)> = env
            .storage()
            .instance()
            .get(&REQUESTS)
            .unwrap_or(Map::new(&env));

        let next_id: u32 = env
            .storage()
            .instance()
            .get(&REQUEST_CNT)
            .unwrap_or(0u32)
            + 1;

        requests.set(next_id, (student, amount, reason, false));
        env.storage().instance().set(&REQUESTS, &requests);
        env.storage().instance().set(&REQUEST_CNT, &next_id);

        next_id
    }

    /// Match an existing donation with a student's aid request.
    /// The caller (an alumni) selects a donation they made and a request
    /// they want to fund; both records are marked as matched/funded so the
    /// same contribution cannot be re-used. No XLM is actually moved —
    /// matching is a bookkeeping operation that records the pledge.
    pub fn match_aid(
        env: Env,
        matcher: Address,
        donation_id: u32,
        student_request_id: u32,
    ) {
        matcher.require_auth();

        let mut donations: Map<u32, (Address, u64, Symbol, bool)> = env
            .storage()
            .instance()
            .get(&DONATIONS)
            .unwrap_or(Map::new(&env));

        let mut requests: Map<u32, (Address, u64, Symbol, bool)> = env
            .storage()
            .instance()
            .get(&REQUESTS)
            .unwrap_or(Map::new(&env));

        let mut donation = donations
            .get(donation_id)
            .unwrap_or_else(|| panic!("Donation not found"));

        let mut request = requests
            .get(student_request_id)
            .unwrap_or_else(|| panic!("Aid request not found"));

        if donation.0 != matcher {
            panic!("Only the original donor can match this donation");
        }

        if donation.3 {
            panic!("Donation already matched");
        }
        if request.3 {
            panic!("Aid request already funded");
        }

        if donation.1 != request.1 {
            panic!("Donation amount does not match aid request amount");
        }

        donation.3 = true;
        request.3 = true;

        donations.set(donation_id, donation);
        requests.set(student_request_id, request);

        env.storage().instance().set(&DONATIONS, &donations);
        env.storage().instance().set(&REQUESTS, &requests);
    }

    /// Return the total number of registered alumni donors.
    pub fn donor_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&ALUMNI_CNT)
            .unwrap_or(0u32)
    }

    /// Return the total number of donations that have been recorded.
    pub fn donation_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DONATION_CNT)
            .unwrap_or(0u32)
    }

    /// Return the total number of aid requests filed by students.
    pub fn request_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&REQUEST_CNT)
            .unwrap_or(0u32)
    }

    /// Return the graduation year recorded for a given alumnus.
    /// Returns `None` if the address is not a registered alumnus.
    pub fn graduation_year(env: Env, alumnus: Address) -> Option<u32> {
        let alumni: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&ALUMNI)
            .unwrap_or(Map::new(&env));
        alumni.get(alumnus)
    }

    /// Return the details of a single donation as a tuple
    /// `(donor, amount, message, matched)`.
    pub fn get_donation(env: Env, donation_id: u32) -> (Address, u64, Symbol, bool) {
        let donations: Map<u32, (Address, u64, Symbol, bool)> = env
            .storage()
            .instance()
            .get(&DONATIONS)
            .unwrap_or(Map::new(&env));
        donations
            .get(donation_id)
            .unwrap_or_else(|| panic!("Donation not found"))
    }

    /// Return the details of a single aid request as a tuple
    /// `(student, amount, reason, funded)`.
    pub fn get_request(env: Env, request_id: u32) -> (Address, u64, Symbol, bool) {
        let requests: Map<u32, (Address, u64, Symbol, bool)> = env
            .storage()
            .instance()
            .get(&REQUESTS)
            .unwrap_or(Map::new(&env));
        requests
            .get(request_id)
            .unwrap_or_else(|| panic!("Aid request not found"))
    }

    /// Return a `Vec<u32>` listing all currently open (unmatched) donation ids.
    /// Useful for off-chain frontends that want to display available funds.
    pub fn open_donations(env: Env) -> Vec<u32> {
        let donations: Map<u32, (Address, u64, Symbol, bool)> = env
            .storage()
            .instance()
            .get(&DONATIONS)
            .unwrap_or(Map::new(&env));
        let total: u32 = env
            .storage()
            .instance()
            .get(&DONATION_CNT)
            .unwrap_or(0u32);

        let mut open: Vec<u32> = Vec::new(&env);
        for i in 1..=total {
            if let Some(d) = donations.get(i) {
                if !d.3 {
                    open.push_back(i);
                }
            }
        }
        open
    }
}
