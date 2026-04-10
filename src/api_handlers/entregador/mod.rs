mod atualizar;
mod atualizar_disponibilidade;
mod listar_entregadores;
mod trocar_email_senha;

pub use atualizar::atualizar_entregador;
pub use atualizar_disponibilidade::atualizar_disponibilidade_entregador;
pub use listar_entregadores::listar_entregadores;
pub use trocar_email_senha::entregador_trocar_email_senha;