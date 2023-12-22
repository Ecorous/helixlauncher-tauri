// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::process::Child;

use helixlauncher_core::auth::account::Account;
use helixlauncher_core::auth::account::AccountConfig;
use helixlauncher_core::auth::MinecraftAuthenticator;
use helixlauncher_core::auth::DEFAULT_ACCOUNT_JSON;
use helixlauncher_core::config::Config;
use helixlauncher_core::launch;
use helixlauncher_core::launch::asset::merge_components;
use helixlauncher_core::launch::instance;
use helixlauncher_core::launch::instance::Instance;
use helixlauncher_core::launch::instance::InstanceLaunchConfig;
use helixlauncher_core::launch::instance::Modloader;
use helixlauncher_core::launch::prepared::{prepare_launch, LaunchOptions};
use serde::Serialize;
use tauri::async_runtime::spawn_blocking;
use tauri::command::CommandArg;
use tauri::command::CommandItem;
use tauri::window;
use tauri::Event;
use tauri::Manager;
use tauri::Window;

fn config() -> Config {
    Config::new("dev.helixlauncher.HelixLauncher", "HelixLauncher").unwrap()
}

fn account_config() -> AccountConfig {
    let config = config();
    AccountConfig::new(config.get_base_path().as_path().join(DEFAULT_ACCOUNT_JSON)).unwrap()
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
struct ResponseInstance {
    name: String,
}

#[derive(Serialize)]
struct ResponseAccount {
    username: String,
    uuid: String,
}

#[tauri::command]
fn get_instances() -> Vec<ResponseInstance> {
    let config = config();
    let i = Instance::list_instances(config.get_instances_path()).unwrap();
    let mut ins: Vec<ResponseInstance> = vec![];
    for a in i {
        ins.push(ResponseInstance {
            name: a.config.name,
        });
    }
    ins
}

#[tauri::command]
fn get_accounts() -> Vec<ResponseAccount> {
    let account_config = account_config();
    let mut accounts: Vec<ResponseAccount> = vec![];
    for a in account_config.accounts {
        accounts.push(ResponseAccount {
            username: a.username,
            uuid: a.uuid,
        })
    }
    accounts
}

#[tauri::command]
fn is_account_selected(uuid: String) -> bool {
    let account_config = account_config();
    println!("baba");
    if let Some(selected) = account_config.selected() {
        println!("wawa: {}", selected.uuid == uuid);
        println!("awaw: {}", selected.uuid);
        println!("wawaawaw: {}", uuid);
        return selected.uuid == uuid;
    }
    println!("abab");
    false
}

#[tauri::command]
fn select_account(uuid: String) {
    let config = config();
    let mut account_config =
        AccountConfig::new(config.get_base_path().as_path().join(DEFAULT_ACCOUNT_JSON)).unwrap();
    account_config.default = Some(uuid);
    let _ = account_config.save();
}

#[derive(Clone, Debug, Serialize)]
struct LaunchedEvent {
    name: String,
    id: u32,
}

#[derive(Clone, Debug, Serialize)]
struct ClosedEvent {
    name: String,
    id: i32,
}

#[derive(Clone, Debug, Serialize)]
struct LaunchErroredEvent {
    name: String,
}

#[tauri::command]
async fn launch(name: String, window: Window) {
    let config = config();
    let instance = Instance::from_path(config.get_instances_path().join(name.clone())).unwrap();
    let components = merge_components(&config, &instance.config.components)
        .await
        .unwrap();
    let mut account_config =
        AccountConfig::new(config.get_base_path().as_path().join(DEFAULT_ACCOUNT_JSON)).unwrap();
    let prepared = prepare_launch(
        &config,
        &instance,
        &components,
        LaunchOptions::default().account(account_config.selected()),
    )
    .await
    .unwrap();
    let mut launch = prepared.launch(true).await.unwrap();
    
    println!("me!!!");
    let id = launch.id();
    if id.is_none() {
        window.emit("launch-error", LaunchErroredEvent { name: name.clone() }).unwrap();
        return;
    }
    assert!(id.is_some());
    window.emit("started", LaunchedEvent { name: name.clone(), id: id.clone().unwrap() }).unwrap();
    let x = launch.wait().await;
    if x.is_ok() {
        window
            .emit(
                "closed",
                ClosedEvent {
                    name,
                    id: x.unwrap().code().unwrap(),
                },
            )
            .unwrap();
    }
    Window::listen(&window, "kill-game", move |event| {
        if id.clone().unwrap() == event.payload().unwrap().parse::<u32>().unwrap() {
            //launch.kill(); //FIXME
        }
    });
}

#[tauri::command]
fn create_instance(
    name: String,
    mc_version: String,
    modloader: String,
    modloader_version: Option<String>,
) {
    let config = config();
    println!(
        "{0}, {1}. {2}, {3:#?}",
        name, mc_version, modloader, modloader_version
    );
    let modloader_real = match modloader.as_str() {
        "Vanilla" => Modloader::Vanilla,
        "Quilt" => Modloader::Quilt,
        "Fabric" => Modloader::Fabric,
        "Forge" => Modloader::Forge,
        _ => panic!(),
    };
    Instance::new(
        name,
        mc_version,
        InstanceLaunchConfig::default(),
        &config.get_instances_path(),
        modloader_real,
        modloader_version,
    )
    .unwrap();
}

#[derive(Serialize, Clone)]
struct AuthCallback {
    code: String,
    url: String,
    message: String,
}

fn initial_auth_callback(code: String, url: String, message: String, window: Window) {
    println!("reached callback: {0}, {1}, {2}", code, url, message);
    window
        .emit("login-phase", AuthCallback { code, url, message })
        .unwrap();
}

#[tauri::command]
async fn login_phase_one(window: Window) {
    println!("reached stage one");
    let minecraft_authenticator: MinecraftAuthenticator =
        MinecraftAuthenticator::new("1d644380-5a23-4a84-89c3-5d29615fbac2");
    println!("created auth");
    let cw = window.clone();
    let account = minecraft_authenticator
        .initial_auth(move |code, url, message| {
            initial_auth_callback(code, url, message, window.clone())
        })
        .await
        .unwrap();
    let username = account.username.clone();
    println!("finished phase 2 auth, user is {}", username);
    let mut account_config = account_config();
    let stored_account = account_config
        .accounts
        .iter_mut()
        .find(|it| it.uuid == account.uuid);
    match stored_account {
        None => {
            if account_config.accounts.len() == 0 {
                account_config.default = Some(account.uuid.clone())
            }
            account_config.accounts.push(account);
        }
        Some(stored_account) => {
            stored_account.refresh_token = account.refresh_token;
            stored_account.username = account.username;
            stored_account.token = account.token;
        }
    }
    account_config.save().unwrap();
    println!("Welcome! You are logged in as: {}", username);
    cw.emit(
        "auth-finished",
        format!("Welcome! You are logged in as: {}", username),
    )
    .unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_instances,
            get_accounts,
            launch,
            select_account,
            is_account_selected,
            create_instance,
            login_phase_one
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
