// use axum::{
//     Extension, Json, extract::{Path, State}, response::IntoResponse
// };
// use serde::Deserialize;
// use uuid::Uuid;
// use std::sync::Arc;

// use crate::{
//     api::{dto::AppError, AppState},
//     models::Usuario,
//     repositories::Repository as _,
//     usecases::PedidoUsecase
// };

// #[derive(Deserialize)]
// pub struct ProcessarPrecosRequest {
//     pub taxa_entrega: f64,
//     pub forma_pagamento: String,
//     pub observacoes: Option<String>,
//     pub itens: Vec<ItemPedidoInput>,
// }

// #[derive(Deserialize)]
// pub struct ItemPedidoInput {
//     pub quantidade: i32,
//     pub observacoes: Option<String>,
//     pub partes: Vec<ParteItemInput>,
// }

// #[derive(Deserialize)]
// pub struct ParteItemInput {
//     pub produto_uuid: Uuid,
//     pub posicao: i32,
// }

// pub async fn processar_e_exibir_precos(
//     State(state): State<Arc<AppState>>,
//     Path(loja_uuid): Path<Uuid>,
//     Extension(usuario): Extension<Usuario>,
//     Json(p): Json<ProcessarPrecosRequest>,
// ) -> Result<impl IntoResponse, AppError> {

//     let usecase = PedidoUsecase::new(
//         state.pedido_service.clone(),
//         Arc::clone(&state.produto_repo),
//         usuario,
//         loja_uuid,
//     );

//     // Monta um pedido mínimo só para cálculo de preços
//     let mut pedido = crate::models::Pedido::new(
//         usecase.usuario.uuid,
//         loja_uuid,
//         0.0,
//         p.taxa_entrega,
//         p.forma_pagamento,
//         p.observacoes.clone(),
//     );

//     for item_req in &p.itens {
//         let mut partes = Vec::new();
//         for parte_req in &item_req.partes {
//             let produto: Option<crate::models::Produto> = state.produto_repo
//                 .buscar_por_uuid(parte_req.produto_uuid)
//                 .await
//                 .map_err(|e| AppError::BadRequest(e.to_string()))?;
//             let produto = produto
//                 .ok_or_else(|| AppError::NotFound(format!("Produto {} não encontrado", parte_req.produto_uuid)))?;
//             partes.push(crate::models::ParteDeItemPedido::new(&produto, parte_req.posicao));
//         }
//         pedido.adicionar_item(item_req.quantidade, item_req.observacoes.clone(), partes);
//     }

//     usecase.processar_e_exibir_precos(&mut pedido).await?;

//     Ok(Json(serde_json::json!({
//         "subtotal": pedido.subtotal,
//         "total": pedido.total,
//         "desconto": pedido.desconto,
//         "itens": pedido.itens.iter().map(|i| serde_json::json!({
//             "quantidade": i.quantidade,
//             "partes": i.partes.len()
//         })).collect::<Vec<_>>()
//     })))
// }
