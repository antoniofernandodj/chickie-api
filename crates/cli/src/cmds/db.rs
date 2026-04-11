use crate::app_state::AppState;
use crate::helpers::{print_ok, print_err};

pub async fn run_migrate(state: &AppState) {
    println!("📦 Aplicando migrações...");
    match crate::database::aplicar_migrations(&*state.pool).await {
        Ok(()) => print_ok("Migrações aplicadas com sucesso"),
        Err(e) => print_err(&e),
    }
}

pub async fn run_wipe(state: &AppState) {
    println!("🗑️ Limpando banco de dados...");
    match sqlx::query(
        "DO $$ DECLARE r RECORD;
        BEGIN
            FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
                EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
            END LOOP;
        END $$;"
    )
    .execute(&*state.pool)
    .await
    {
        Ok(_) => print_ok("Todas as tabelas removidas"),
        Err(e) => print_err(&format!("Falha ao limpar: {}", e)),
    }
}
