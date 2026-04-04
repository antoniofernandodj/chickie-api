use std::sync::Arc;

use uuid::Uuid;

use crate::models::{LojaFavorita, Usuario};
use crate::services::LojaFavoritaService;

pub struct AdicionarLojaFavoritaUsecase {
    pub service: Arc<LojaFavoritaService>,
    pub usuario: Usuario,
    pub loja_uuid: Uuid,
}

impl AdicionarLojaFavoritaUsecase {
    pub fn new(
        service: Arc<LojaFavoritaService>,
        usuario: Usuario,
        loja_uuid: Uuid,
    ) -> Self {
        Self { service, usuario, loja_uuid }
    }

    pub async fn executar(&self) -> Result<LojaFavorita, String> {
        self.service.adicionar_favorita(self.usuario.uuid, self.loja_uuid).await
    }
}

pub struct RemoverLojaFavoritaUsecase {
    pub service: Arc<LojaFavoritaService>,
    pub usuario: Usuario,
    pub loja_uuid: Uuid,
}

impl RemoverLojaFavoritaUsecase {
    pub fn new(
        service: Arc<LojaFavoritaService>,
        usuario: Usuario,
        loja_uuid: Uuid,
    ) -> Self {
        Self { service, usuario, loja_uuid }
    }

    pub async fn executar(&self) -> Result<(), String> {
        self.service.remover_favorita(self.usuario.uuid, self.loja_uuid).await
    }
}

pub struct ListarLojasFavoritasUsecase {
    pub service: Arc<LojaFavoritaService>,
    pub usuario: Usuario,
}

impl ListarLojasFavoritasUsecase {
    pub fn new(
        service: Arc<LojaFavoritaService>,
        usuario: Usuario,
    ) -> Self {
        Self { service, usuario }
    }

    pub async fn executar(&self) -> Result<Vec<LojaFavorita>, String> {
        self.service.listar_favoritas(self.usuario.uuid).await
    }
}

pub struct VerificarLojaFavoritaUsecase {
    pub service: Arc<LojaFavoritaService>,
    pub usuario: Usuario,
    pub loja_uuid: Uuid,
}

impl VerificarLojaFavoritaUsecase {
    pub fn new(
        service: Arc<LojaFavoritaService>,
        usuario: Usuario,
        loja_uuid: Uuid,
    ) -> Self {
        Self { service, usuario, loja_uuid }
    }

    pub async fn executar(&self) -> Result<bool, String> {
        self.service.eh_favorita(self.usuario.uuid, self.loja_uuid).await
    }
}
