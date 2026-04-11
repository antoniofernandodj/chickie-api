use chickie_core::models::{ConfiguracaoDePedidosLoja, TipoCalculoPedido};

use crate::app_state::AppState;
use crate::args::*;
use crate::helpers::{print_ok, print_err, json_print};

pub async fn run_get_config_pedido(state: &AppState, args: GetConfigPedidoArgs) {
    match state
        .config_pedido_service
        .buscar(args.loja_uuid)
        .await
    {
        Ok(Some(c)) => json_print(&c),
        Ok(None) => print_err("Configuração não encontrada"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_save_config_pedido(state: &AppState, args: SaveConfigPedidoArgs) {
    let tipo = match args.tipo_calculo.as_str() {
        "mais_caro" => TipoCalculoPedido::MaisCaro,
        _ => TipoCalculoPedido::MediaPonderada,
    };

    let config = ConfiguracaoDePedidosLoja::new(
        args.loja_uuid,
        args.max_partes,
        tipo,
    );

    match config {
        Ok(c) => {
            match state
                .config_pedido_service
                .salvar(&c)
                .await
            {
                Ok(()) => {
                    print_ok("Configuração salva");
                }
                Err(e) => print_err(&format!("{:?}", e)),
            }
        }
        Err(e) => print_err(&format!("Config inválida: {}", e)),
    }
}
