use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::io::AsyncWriteExt;

use crate::helpers::{get_file_path, load_file, make_nxn_mat};

use super::{
    consts::{app_dir, CELL_N},
    score::Score,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    name: String,
    scores: [[Arc<Score>; CELL_N]; CELL_N],
}

impl User {
    pub fn new(name: &str) -> Self {
        let scores = make_nxn_mat();
        User {
            name: String::from(name),
            scores,
        }
    }
    pub fn iter(&self) -> ScoresIter {
        ScoresIter {
            index_x: 0,
            index_y: 0,
            user: self,
        }
    }

    pub fn get_user_path(name: &str) -> PathBuf {
        get_file_path(name)
    }
    pub fn get_score(&self, x: usize, y: usize) -> Arc<Score> {
        self.scores[x][y].clone()
    }

    pub fn get_mut_score(&mut self, x: usize, y: usize) -> Option<&mut Score> {
        Arc::<Score>::get_mut(&mut self.scores[x][y])
    }

    pub fn get_opt_score(&self, x: usize, y: usize) -> Option<Arc<Score>> {
        Some(self.scores.get(x)?.get(y)?.clone())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, new_name: String) -> String {
        std::mem::replace(&mut self.name, new_name)
    }

    async fn load_from_file_anyhow(name: &str) -> Result<Self, Arc<Error>> {
        async {
            Ok(ron::de::from_bytes(
                &load_file(Self::get_user_path(name)).await?,
            )?)
        }
        .await
        .map_err(Arc::new)
    }
    pub async fn load_user(name: String) -> Box<Self> {
        let loaded = Self::load_from_file_anyhow(&name).await;
        Box::new(if let Ok(u) = loaded {
            u
        } else {
            println!("user not found");
            Self::create_new(&name).await.unwrap()
        })
    }

    pub async fn rename_user_file(&self, new_name: String) -> Result<String, Arc<anyhow::Error>> {
        let mut from = app_dir();
        let mut to = from.clone();
        from.push(&self.name);
        to.push(&new_name);

        tokio::fs::rename(from, to)
            .await
            .map_err(|e| Arc::new(anyhow!(e)))?;
        Ok(new_name)
    }

    pub async fn create_new(name: &str) -> Result<Self, Arc<Error>> {
        let new_user = Self::new(name);
        async {
            let mut file = tokio::fs::File::create(Self::get_user_path(name)).await?;
            println!("created {name}.ron file!");
            file.write_all(ron::ser::to_string(&new_user)?.as_bytes())
                .await?;
            Ok(())
        }
        .await
        .map_err(Arc::new)?;
        Ok(new_user)
    }

    pub async fn update_file(&self) -> Result<(), Arc<Error>> {
        async {
            let name = &self.name;
            let mut file = tokio::fs::File::options()
                .write(true)
                .open(Self::get_user_path(name))
                .await?;
            file.write_all(ron::ser::to_string(self)?.as_bytes())
                .await?;
            Ok(())
        }
        .await
        .map_err(Arc::new)
    }
}

#[derive(PartialEq, Eq)]
pub struct ScoreWithEq(usize, usize, Arc<Score>);

impl PartialOrd for ScoreWithEq {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoreWithEq {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.eq(other) {
            std::cmp::Ordering::Equal
        } else {
            self.2.cmp(&other.2)
        }
    }
}

impl From<ScoreWithEq> for (u32, u32) {
    fn from(value: ScoreWithEq) -> Self {
        (value.0 as u32, value.1 as u32)
    }
}

pub struct ScoresIter<'user> {
    index_x: usize,
    index_y: usize,
    user: &'user User,
}

impl<'u> Iterator for ScoresIter<'u> {
    type Item = ScoreWithEq;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.user.scores.get(self.index_x)?;
        let val = row.get(self.index_y)?;
        let v = ScoreWithEq(self.index_x + 1, self.index_y + 1, Arc::clone(val));
        if self.index_y == CELL_N - 1 {
            self.index_x += 1;
            self.index_y = 0;
        } else {
            self.index_y += 1;
        }
        Some(v)
    }
}

pub(crate) mod score {}
