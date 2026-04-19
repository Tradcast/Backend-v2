use std::collections::HashMap;

// ====================== Data Structs ======================

#[derive(Debug)]
struct User {
    fid: String,
    total_profit: f64,
    total_pnl: f64,
    total_games: u32,
    energy: u32,
    daily_games: u32,
    streak: u32,
    giveaway_eligible: Option<bool>,
    latest_trades: Vec<String>,
}

#[derive(Debug)]
struct LeaderboardEntry {
    fid: String,
    total_profit: i64,
    weekly_profit: i64,
    daily_profit: i64,
    monthly_profit: i64,
}

#[derive(Debug)]
struct ScoreUpdatePayload {
    fid: String,
    profit: f64,
    final_pnl: f64,
    session_id: Option<String>,
    trade_env_id: Option<String>,
    created_at: Option<f64>,
    secret: String,
}

#[derive(Debug)]
struct DebugStats {
    uptime_seconds: f64,
    uptime_human: String,
    http_requests_total: u64,
    users_in_memory: usize,
    score_updates_received: u64,
    score_updates_rejected: u64,
    energy_lookups_received: u64,
    energy_lookups_rejected: u64,
    trades_db_rows: u64,
    firestore_users_cache_size: usize,
}

// ====================== Root / Health ======================

fn root() -> &'static str {
    "Miniapp backend is running"
}

fn health() -> &'static str {
    "ok"
}

// ====================== Debug ======================

fn debug_info() -> DebugStats {
    DebugStats {
        uptime_seconds: 0.0,
        uptime_human: "0h 0m 0s".to_string(),
        http_requests_total: 0,
        users_in_memory: 0,
        score_updates_received: 0,
        score_updates_rejected: 0,
        energy_lookups_received: 0,
        energy_lookups_rejected: 0,
        trades_db_rows: 0,
        firestore_users_cache_size: 0,
    }
}

// ====================== User Routes ======================

fn get_home(fid: &str) -> User {
    User {
        fid: fid.to_lowercase(),
        total_profit: 0.0,
        total_pnl: 0.0,
        total_games: 0,
        energy: 10,
        daily_games: 0,
        streak: 0,
        giveaway_eligible: None,
        latest_trades: vec![],
    }
}

fn get_profile(fid: &str, _username: Option<&str>, _wallet: Option<&str>) -> User {
    User {
        fid: fid.to_lowercase(),
        total_profit: 0.0,
        total_pnl: 0.0,
        total_games: 0,
        energy: 10,
        daily_games: 0,
        streak: 0,
        giveaway_eligible: None,
        latest_trades: vec!["trade_1".to_string(), "trade_2".to_string()],
    }
}

fn get_leaderboard(_fid: &str, top_n: usize) -> Vec<LeaderboardEntry> {
    (0..top_n)
        .map(|i| LeaderboardEntry {
            fid: format!("user_{}", i),
            total_profit: (top_n - i) as i64 * 100,
            weekly_profit: 0,
            daily_profit: 0,
            monthly_profit: 0,
        })
        .collect()
}

fn get_weekly_leaderboard(_fid: &str, top_n: usize) -> Vec<LeaderboardEntry> {
    (0..top_n)
        .map(|i| LeaderboardEntry {
            fid: format!("user_{}", i),
            total_profit: 0,
            weekly_profit: (top_n - i) as i64 * 50,
            daily_profit: 0,
            monthly_profit: 0,
        })
        .collect()
}

fn get_daily_leaderboard(_fid: &str, top_n: usize) -> Vec<LeaderboardEntry> {
    (0..top_n)
        .map(|i| LeaderboardEntry {
            fid: format!("user_{}", i),
            total_profit: 0,
            weekly_profit: 0,
            daily_profit: (top_n - i) as i64 * 10,
            monthly_profit: 0,
        })
        .collect()
}

fn get_monthly_leaderboard(_fid: &str, top_n: usize) -> Vec<LeaderboardEntry> {
    (0..top_n)
        .map(|i| LeaderboardEntry {
            fid: format!("user_{}", i),
            total_profit: 0,
            weekly_profit: 0,
            daily_profit: 0,
            monthly_profit: (top_n - i) as i64 * 200,
        })
        .collect()
}

// ====================== Internal Endpoints ======================

fn internal_update_score(payload: ScoreUpdatePayload) -> Result<&'static str, &'static str> {
    if payload.secret != "dummy_secret" {
        return Err("forbidden");
    }
    println!(
        "Score update received: fid={} profit={}",
        payload.fid, payload.profit
    );
    Ok("ok")
}

fn internal_user_energy(fid: &str, secret: &str) -> Result<u32, &'static str> {
    if secret != "dummy_secret" {
        return Err("forbidden");
    }
    // Dummy energy value
    Ok(7)
}

fn internal_users_cache(secret: &str) -> Result<HashMap<String, u32>, &'static str> {
    if secret != "dummy_secret" {
        return Err("forbidden");
    }
    let mut cache = HashMap::new();
    cache.insert("user_1".to_string(), 5);
    cache.insert("user_2".to_string(), 10);
    Ok(cache)
}

fn internal_user_cache(fid: &str, secret: &str) -> Result<User, &'static str> {
    if secret != "dummy_secret" {
        return Err("forbidden");
    }
    Ok(User {
        fid: fid.to_lowercase(),
        total_profit: 0.0,
        total_pnl: 0.0,
        total_games: 0,
        energy: 5,
        daily_games: 0,
        streak: 0,
        giveaway_eligible: None,
        latest_trades: vec![],
    })
}

// ====================== Gameplay Tracker ======================

fn increase_tracker(fid: &str) -> &'static str {
    println!("Incrementing tracker for fid={}", fid);
    "ok"
}

fn get_tracker() -> HashMap<String, u32> {
    let mut data = HashMap::new();
    data.insert("alice".to_string(), 4);
    data.insert("bob".to_string(), 6);
    data
}

// ====================== Main ======================

fn main() {
    println!("{}", root());
    println!("{}", health());

    let stats = debug_info();
    println!("{:?}", stats);

    let user = get_home("123");
    println!("{:?}", user);

    let profile = get_profile("456", Some("alice"), Some("0xabc"));
    println!("{:?}", profile);

    let lb = get_leaderboard("123", 5);
    for entry in &lb { println!("{:?}", entry); }

    let weekly = get_weekly_leaderboard("123", 5);
    for entry in &weekly { println!("{:?}", entry); }

    let daily = get_daily_leaderboard("123", 5);
    for entry in &daily { println!("{:?}", entry); }

    let monthly = get_monthly_leaderboard("123", 5);
    for entry in &monthly { println!("{:?}", entry); }

    let payload = ScoreUpdatePayload {
        fid: "123".to_string(),
        profit: 42.0,
        final_pnl: 10.0,
        session_id: Some("sess_1".to_string()),
        trade_env_id: Some("env_1".to_string()),
        created_at: Some(0.0),
        secret: "dummy_secret".to_string(),
    };
    println!("{:?}", internal_update_score(payload));
    println!("{:?}", internal_user_energy("123", "dummy_secret"));
    println!("{:?}", internal_users_cache("dummy_secret"));
    println!("{:?}", internal_user_cache("123", "dummy_secret"));

    increase_tracker("123");
    println!("{:?}", get_tracker());
}
