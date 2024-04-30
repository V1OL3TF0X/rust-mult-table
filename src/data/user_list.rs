use std::{path::Path, sync::Arc};

use crate::helpers::{get_file_path, load_file};
use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use super::consts::create_app_dir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserList {
    last_user: String,
    all_users: Vec<String>,
}

impl Default for UserList {
    fn default() -> Self {
        Self {
            last_user: String::from("User"),
            all_users: vec![String::from("User")],
        }
    }
}

impl UserList {
    fn get_file_path() -> std::path::PathBuf {
        get_file_path("UserList")
    }

    async fn load_from_file_anyhow(path: impl AsRef<Path>) -> Result<Self, Error> {
        create_app_dir().await;
        Ok(ron::de::from_bytes(&load_file(path).await?)?)
    }
    pub async fn load_from_file() -> Self {
        if let Ok(list) = Self::load_from_file_anyhow(Self::get_file_path()).await {
            list
        } else {
            let new_list = Self::default();
            let mut file = tokio::fs::File::create(Self::get_file_path())
                .await
                .unwrap();

            file.write_all(ron::ser::to_string(&new_list).unwrap().as_bytes())
                .await
                .unwrap();
            new_list
        }
    }

    pub async fn create_new(path: impl AsRef<Path>) -> Result<Self, Arc<Error>> {
        Self::load_from_file_anyhow(path).await.map_err(Arc::new)
    }

    pub fn get_current(&self) -> &str {
        &self.last_user
    }

    pub fn get_all(&self) -> &Vec<String> {
        &self.all_users
    }

    pub fn add_user(&mut self, user_name: &str) {
        self.all_users.push(user_name.to_owned());
        self.last_user = user_name.to_owned();
    }

    pub fn switch_current(&mut self, new_current: String) {
        self.last_user = new_current;
    }

    pub fn update_current_user_after_rename(&mut self, user_name: String) {
        let _ = std::mem::replace(
            self.all_users
                .iter_mut()
                .find(|u| *u == &self.last_user)
                .unwrap(),
            user_name.clone(),
        );
        self.switch_current(user_name);
    }

    pub async fn save_to_file(&self) -> Result<(), Arc<Error>> {
        let mut file = tokio::fs::File::options()
            .write(true)
            .truncate(true)
            .open(Self::get_file_path())
            .await
            .map_err(|e| Arc::new(anyhow!(e)))?;
        file.write_all(ron::ser::to_string(self).unwrap().as_bytes())
            .await
            .map_err(|e| Arc::new(anyhow!(e)))?;
        println!("saved user list successfully \n {self:#?}");
        Ok(())
    }
}
