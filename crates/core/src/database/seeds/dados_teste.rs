use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use serde::Deserialize;
use sqlx::PgPool;

use super::ler_seed_json;

#[derive(Deserialize)]
struct AdminSeed {
    nome: String,
    username: String,
    email: String,
    senha: String,
    celular: String,
    cpf: String,
}

#[derive(Deserialize)]
struct LojaSeed {
    nome: String,
    slug: String,
    descricao: String,
    email: String,
    celular: String,
}

#[derive(Deserialize)]
struct ProdutoSeed {
    nome: String,
    descricao: String,
    preco: String,
    categoria: String,
}

#[derive(Deserialize)]
struct AdicionalSeed {
    nome: String,
    descricao: String,
    preco: String,
}

#[derive(Deserialize)]
struct DadosTesteSeed {
    admin: AdminSeed,
    loja: LojaSeed,
    produtos: Vec<ProdutoSeed>,
    adicionais: Vec<AdicionalSeed>,
}

/// Seeds test data from `data/seed/dados_teste.json`.
///
/// Runs only under `MODE=development`. Idempotent: skips if the admin email already exists.
/// `SEED_ADMIN_EMAIL` and `SEED_ADMIN_SENHA` env vars override the JSON defaults.
pub(in crate::database) async fn seed_dados_teste(pool: &PgPool) -> Result<(), String> {
    let json = ler_seed_json("dados_teste.json")?;
    let mut seed: DadosTesteSeed = serde_json::from_str(&json)
        .map_err(|e| format!("JSON inválido em dados_teste.json: {}", e))?;

    // Env vars override JSON defaults
    if let Ok(v) = std::env::var("SEED_ADMIN_EMAIL") { seed.admin.email = v; }
    if let Ok(v) = std::env::var("SEED_ADMIN_SENHA") { seed.admin.senha = v; }

    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM usuarios WHERE email = $1)")
            .bind(&seed.admin.email)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Falha ao verificar seed admin: {}", e))?;

    if exists {
        tracing::debug!("   ⏭️  Dados de teste já existem, pulando seed");
        return Ok(());
    }

    // Admin user
    let salt = SaltString::generate(&mut OsRng);
    let senha_hash = Argon2::default()
        .hash_password(seed.admin.senha.as_bytes(), &salt)
        .map_err(|e| format!("Falha ao gerar hash de senha no seed: {}", e))?
        .to_string();

    let admin_uuid: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO usuarios (nome, username, email, senha_hash, celular, cpf, classe, modo_de_cadastro)
         VALUES ($1, $2, $3, $4, $5, $6, 'administrador', 'email')
         RETURNING uuid",
    )
    .bind(&seed.admin.nome)
    .bind(&seed.admin.username)
    .bind(&seed.admin.email)
    .bind(&senha_hash)
    .bind(&seed.admin.celular)
    .bind(&seed.admin.cpf)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Falha ao criar usuário admin no seed: {}", e))?;

    tracing::info!("   ✅ Admin criado: {} ({})", seed.admin.email, admin_uuid);

    // Loja
    let loja_uuid: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO lojas (nome, slug, descricao, email, celular, criado_por)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING uuid",
    )
    .bind(&seed.loja.nome)
    .bind(&seed.loja.slug)
    .bind(&seed.loja.descricao)
    .bind(&seed.loja.email)
    .bind(&seed.loja.celular)
    .bind(admin_uuid)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Falha ao criar loja no seed: {}", e))?;

    tracing::info!("   ✅ Loja criada: {} ({})", seed.loja.nome, loja_uuid);

    // Produtos — resolve categoria por nome
    for p in &seed.produtos {
        let categoria_uuid: uuid::Uuid = sqlx::query_scalar(
            "SELECT uuid FROM categorias_produtos WHERE loja_uuid IS NULL AND nome = $1 LIMIT 1",
        )
        .bind(&p.categoria)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Categoria '{}' não encontrada para o produto '{}': {}", p.categoria, p.nome, e))?;

        sqlx::query(
            "INSERT INTO produtos (loja_uuid, categoria_uuid, nome, descricao, preco, disponivel)
             VALUES ($1, $2, $3, $4, $5::NUMERIC, TRUE)",
        )
        .bind(loja_uuid)
        .bind(categoria_uuid)
        .bind(&p.nome)
        .bind(&p.descricao)
        .bind(&p.preco)
        .execute(pool)
        .await
        .map_err(|e| format!("Falha ao criar produto '{}' no seed: {}", p.nome, e))?;

        tracing::info!("   ✅ Produto criado: {}", p.nome);
    }

    // Adicionais
    for a in &seed.adicionais {
        sqlx::query(
            "INSERT INTO adicionais (loja_uuid, nome, descricao, preco, disponivel)
             VALUES ($1, $2, $3, $4::NUMERIC, TRUE)",
        )
        .bind(loja_uuid)
        .bind(&a.nome)
        .bind(&a.descricao)
        .bind(&a.preco)
        .execute(pool)
        .await
        .map_err(|e| format!("Falha ao criar adicional '{}' no seed: {}", a.nome, e))?;

        tracing::info!("   ✅ Adicional criado: {}", a.nome);
    }

    tracing::info!("   🎉 Seed de teste concluído — loja_uuid: {}", loja_uuid);
    Ok(())
}
