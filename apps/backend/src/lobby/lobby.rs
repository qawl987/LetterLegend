use crate::player::Player;
use std::error::Error;
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use super::lobby_player::LobbyPlayer;

#[derive(Debug)]
pub struct Lobby {
    id: u32,
    max_players: u32,
    players: Mutex<HashMap<u32, Arc<LobbyPlayer>>>,
}

impl Lobby {
    pub fn new(id: u32, max_players: u32) -> Self {
        debug_assert!(max_players >= 4, "max_players must be greater than 4");
        debug_assert!(max_players <= 8, "max_players must be less than 8");
        Self {
            id,
            max_players,
            players: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_player(&self, player: Arc<Player>) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut player_lobby_id = player.lobby_id.lock().unwrap();
        if player_lobby_id.is_some() {
            return Err("player already in a lobby".into());
        }
        *player_lobby_id = Some(self.id);
        self.players
            .lock()
            .unwrap()
            .insert(player.id, Arc::new(LobbyPlayer::new(player.clone())));
        Ok(())
    }

    pub fn get_player(&self, id: u32) -> Option<Arc<LobbyPlayer>> {
        Some(self.players.lock().unwrap().get(&id)?.clone())
    }

    pub fn get_players(&self) -> Vec<Arc<LobbyPlayer>> {
        self.players.lock().unwrap().values().cloned().collect()
    }

    pub fn remove_player(&self, id: u32) -> Option<Arc<LobbyPlayer>> {
        let player = self.players.lock().unwrap().remove(&id);

        if let Some(player) = player.clone() {
            *player.player.lobby_id.lock().unwrap() = None;
        }

        player
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_max_players(&self) -> u32 {
        self.max_players
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_id_with_id_0_returns_0() -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        assert_eq!(lobby.get_id(), 0);
        Ok(())
    }

    #[test]
    fn get_max_players_with_max_players_4_returns_4() -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        assert_eq!(lobby.get_max_players(), 4);
        Ok(())
    }

    #[test]
    fn add_player_with_test_player_should_be_added() -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        let player = Arc::new(Player::new(0, "test".to_string()));
        let result = lobby.add_player(player.clone());
        assert!(result.is_ok());
        assert!(lobby.players.lock().unwrap().get(&0).is_some());
        assert_eq!(player.lobby_id.lock().unwrap().unwrap(), 0);
        Ok(())
    }

    #[test]
    fn add_player_with_test_player_already_in_lobby_should_return_error(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        let player = Arc::new(Player::new(0, "test".to_string()));
        *player.lobby_id.lock().unwrap() = Some(1);
        assert!(lobby.add_player(player).is_err());
        Ok(())
    }

    #[test]
    fn get_player_with_test_player_should_return_test_player(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        lobby.players.lock().unwrap().insert(
            0,
            Arc::new(LobbyPlayer::new(Arc::new(Player::new(
                0,
                "test".to_string(),
            )))),
        );
        let player = lobby.get_player(0).unwrap().player.clone();
        assert_eq!(player.id, 0);
        assert_eq!(player.name, "test");
        Ok(())
    }

    #[test]
    fn get_player_with_not_exist_player_should_return_none(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        assert!(lobby.get_player(0).is_none());
        Ok(())
    }

    #[test]
    fn get_players_with_three_players_should_return_players(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        for i in 0..3 {
            lobby.players.lock().unwrap().insert(
                i,
                Arc::new(LobbyPlayer::new(Arc::new(Player::new(
                    i,
                    format!("test{}", i),
                )))),
            );
        }

        for lobby_player in lobby.get_players() {
            assert_eq!(
                lobby_player.player.name,
                format!("test{}", lobby_player.player.id)
            );
        }
        Ok(())
    }

    #[test]
    fn remove_player_with_test_player_should_remove_player(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        lobby.players.lock().unwrap().insert(
            0,
            Arc::new(LobbyPlayer::new(Arc::new(Player::new(
                0,
                format!("test{}", 0),
            )))),
        );

        let player = lobby.remove_player(0);
        assert_eq!(lobby.players.lock().unwrap().len(), 0);
        assert!(player.unwrap().player.lobby_id.lock().unwrap().is_none());
        Ok(())
    }

    #[test]
    fn remove_player_with_not_exist_player_should_return_none(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        assert!(lobby.remove_player(0).is_none());
        Ok(())
    }
}
