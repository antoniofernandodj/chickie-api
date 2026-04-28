mod categorias;
mod dados_teste;

pub(in crate::database) use categorias::seed_categorias_globais;
pub(in crate::database) use dados_teste::seed_dados_teste;

/// Reads a file from `data/seed/`, trying multiple base paths to support
/// different working directories (workspace root, crate root, etc.).
pub(super) fn ler_seed_json(filename: &str) -> Result<String, String> {
    let candidates = [
        format!("data/seed/{filename}"),
        format!("../data/seed/{filename}"),
        format!("../../data/seed/{filename}"),
    ];

    for path in &candidates {
        if let Ok(contents) = std::fs::read_to_string(path) {
            return Ok(contents);
        }
    }

    Err(format!(
        "Arquivo de seed '{filename}' não encontrado. Procurei em: {}",
        candidates.join(", ")
    ))
}
