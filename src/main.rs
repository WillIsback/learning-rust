use anyhow::{Context, Ok, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
const FILE_PATH: &str = "users.json";

#[derive(Parser)]
#[command(name = "User Management CLI")]
#[command(about = "A simple CLI for managing users")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    firstname: String,
    lastname: String,
    email: String,
    burge: String,
    si: String,
    login: String,
    password: String,
}

#[derive(Subcommand)]
enum Commands {
    GenerateUser {
        firstname: String,
        lastname: String,
        email: String,
        burge: String,
        si: String,
    },
    EditUser {
        firstname: String,
        lastname: String,
        email: String,
        burge: String,
        si: String,
        login: String,
        password: String,
    },
    DeleteUser {
        login: String,
    },
    ListUsers,
}

fn load_users() -> Result<Vec<User>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(vec![]);
    }
    let data: String = fs::read_to_string(FILE_PATH).context("Failed to read file ")?;
    let users: Vec<User> = serde_json::from_str(&data).context("Failed to parse JSON:")?;
    Ok(users)
}

fn save_users(users: &Vec<User>) -> Result<()> {
    let data: String = serde_json::to_string(users).context("Failed to serialize users")?;
    fs::write(FILE_PATH, data).context("Failed to write file")?;
    Ok(())
}

fn generate_user(
    firstname: String,
    lastname: String,
    email: String,
    burge: String,
    si: String,
) -> Result<()> {
    let mut registered_users = load_users()?;

    // Check if the user already exists
    if registered_users
        .iter()
        .any(|user| user.firstname == firstname && user.lastname == lastname)
    {
        return Err(anyhow::anyhow!("User already exists"));
    }

    // Create the new user
    let new_user = User {
        firstname,
        lastname,
        email,
        burge,
        si,
        login: "login".to_string(),
        password: "password".to_string(),
    };

    // Add the new user and save
    registered_users.push(new_user);
    save_users(&registered_users)?;
    println!("User created successfully");

    Ok(())
}

fn edit_user(
    firstname: String,
    lastname: String,
    email: String,
    burge: String,
    si: String,
    login: String,
    password: String,
) -> Result<()> {
    let mut registered_users = load_users()?;

    // Find the user to edit
    if let Some(user) = registered_users
        .iter_mut()
        .find(|user| user.firstname == firstname && user.lastname == lastname)
    {
        println!("User found. Updating...");

        // Update the user fields
        user.firstname = firstname;
        user.lastname = lastname;
        user.email = email;
        user.burge = burge;
        user.si = si;
        user.login = login;
        user.password = password;

        // Save the updated user list
        save_users(&registered_users)?;
        println!("User updated successfully");
        Ok(())
    } else {
        // User not found
        Err(anyhow::anyhow!("User not found"))
    }
}

fn delete_user(login: String) -> Result<()> {
    let mut registered_users = load_users()?;

    // Find the user to delete
    if let Some(pos) = registered_users.iter().position(|user| user.login == login) {
        registered_users.remove(pos);
        save_users(&registered_users)?;
        println!("User deleted successfully");
        Ok(())
    } else {
        Err(anyhow::anyhow!("User not found"))
    }
}
fn main() -> Result<()> {
    println!("Hello, to my Login backend program manager!");
    let cli = Cli::parse();
    let user = load_users()?;
    match cli.command {
        Commands::GenerateUser {
            firstname,
            lastname,
            email,
            burge,
            si,
        } => {
            generate_user(firstname, lastname, email, burge, si)?;
        }
        Commands::EditUser {
            firstname,
            lastname,
            email,
            burge,
            si,
            login,
            password,
        } => {
            edit_user(firstname, lastname, email, burge, si, login, password)?;
        }
        Commands::DeleteUser { login } => {
            delete_user(login)?;
        }
        Commands::ListUsers => {
            if user.is_empty() {
                println!("No users found");
            } else {
                for u in &user {
                    println!("User: {} {} - Email: {} - Login {}", u.firstname, u.lastname, u.email, u.login);
                }
            }
        }
    }
    Ok(())
}
