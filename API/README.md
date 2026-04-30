# API Specification — Chickie

> Índice da especificação completa. Cada arquivo cobre um grupo de endpoints.

---

## Interactive API Documentation (Swagger/OpenAPI)

| Resource | URL |
|----------|-----|
| **Swagger UI** | `http://localhost:3000/api/docs/swagger-ui` |
| **OpenAPI JSON Spec** | `http://localhost:3000/api/docs/openapi.json` |

---

## Informações Gerais

| Item | Valor |
|------|-------|
| **Base URL** | `http://localhost:3000/api` |
| **Protocolo** | HTTP/1.1 |
| **Formato** | JSON (`application/json`) |
| **Autenticação** | JWT Bearer Token via header `Authorization: Bearer <token>` |
| **Charset** | UTF-8 |

### Autenticação

Endpoints marcados com 🔒 exigem:
```
Authorization: Bearer <JWT_TOKEN>
```

Endpoints marcados com 👑 exigem além do JWT que o usuário tenha `classe = "administrador"` ou seja o owner da plataforma.

### Classes de Usuário

| Classe | Descrição |
|--------|-----------|
| `cliente` | Padrão. Faz pedidos e avalia. |
| `administrador` | Cria e gerencia lojas, funcionários e entregadores. |
| `funcionario` | Funcionário de uma loja (vinculado via `usuario_uuid`). |
| `entregador` | Entregador de uma loja (vinculado via `usuario_uuid`). |
| `owner` | Dono da plataforma. Acesso total. |

### Erros

```json
{ "error": "Mensagem de erro descritiva" }
```

| Status | Significado |
|--------|-------------|
| `400` | Bad Request — dados inválidos ou transição de estado inválida |
| `403` | Forbidden — sem permissão |
| `404` | Not Found — recurso não encontrado |
| `500` | Internal Server Error |
| `201` | Created — recurso criado com sucesso |
| `204` | No Content — operação bem-sucedida sem body |

---

## Máquina de Estados do Pedido

```
criado → aguardando_confirmacao_de_loja → confirmado_pela_loja
       → em_preparo → pronto → saiu_para_entrega → entregue (terminal)
```

| Estado Atual | Próximo Válido |
|--------------|----------------|
| `criado` | `aguardando_confirmacao_de_loja` |
| `aguardando_confirmacao_de_loja` | `confirmado_pela_loja` ou `criado` |
| `confirmado_pela_loja` | `em_preparo` ou `aguardando_confirmacao_de_loja` |
| `em_preparo` | `pronto` ou `confirmado_pela_loja` |
| `pronto` | `saiu_para_entrega` ou `em_preparo` |
| `saiu_para_entrega` | `entregue` ou `pronto` |
| `entregue` | **(terminal)** |

---

## Índice de Arquivos

| Arquivo | Grupo | Prefixo |
|---------|-------|---------|
| [01-autenticacao.md](./01-autenticacao.md) | Autenticação | `/api/auth/` |
| [02-lojas.md](./02-lojas.md) | Lojas públicas | `/api/lojas/` |
| [03-usuarios.md](./03-usuarios.md) | Usuários | `/api/usuarios/` |
| [04-admin.md](./04-admin.md) | Administração | `/api/admin/` |
| [05-pedidos.md](./05-pedidos.md) | Pedidos | `/api/pedidos/` |
| [06-cupons.md](./06-cupons.md) | Cupons | `/api/cupons/` |
| [07-marketing.md](./07-marketing.md) | Marketing (avaliações, promoções) | `/api/marketing/` |
| [08-catalogo.md](./08-catalogo.md) | Catálogo (adicionais, categorias) | `/api/catalogo/` |
| [09-enderecos-entrega.md](./09-enderecos-entrega.md) | Endereços de entrega | `/api/enderecos-entrega/` |
| [10-enderecos-loja.md](./10-enderecos-loja.md) | Endereços de loja | `/api/enderecos-loja/` |
| [11-enderecos-usuario.md](./11-enderecos-usuario.md) | Endereços de usuário | `/api/enderecos-usuario/` |
| [12-favoritos.md](./12-favoritos.md) | Lojas favoritas | `/api/favoritos/` |
| [13-produtos.md](./13-produtos.md) | Produtos | `/api/produtos/` |
| [14-horarios.md](./14-horarios.md) | Horários de funcionamento | `/api/horarios/` |
| [15-utilitarios.md](./15-utilitarios.md) | Utilitários (health, wipe) | `/api/ok`, `/api/wipe` |

---

## Sumário Completo de Endpoints

| # | Método | Rota | Auth | Admin |
|---|--------|------|------|-------|
| 1 | `GET` | `/` | — | — |
| 2 | `POST` | `/api/auth/signup` | — | — |
| 2.1 | `GET` | `/api/auth/confirmar-email?token=` | — | — |
| 3 | `POST` | `/api/auth/login` | — | — |
| 3.1 | `GET` | `/api/auth/me` | 🔒 | — |
| 3.2 | `POST` | `/api/auth/verificar-celular` | — | — |
| 5 | `GET` | `/api/lojas/` | — | — |
| 6 | `GET` | `/api/lojas/pesquisar` | — | — |
| 7 | `GET` | `/api/lojas/{uuid}` | — | — |
| 8 | `GET` | `/api/lojas/slug/{slug}` | — | — |
| 9 | `GET` | `/api/lojas/verificar-slug/{slug}` | — | — |
| 9.1 | `PATCH` | `/api/lojas/{uuid}/bloqueado` | 🔒 + 👑 Owner | — |
| 9.2 | `GET` | `/api/usuarios/verificar-celular/{celular}` | — | — |
| 10 | `GET` | `/api/usuarios/?classe=...` | 🔒 + 👑 Owner | — |
| 10.1 | `PATCH` | `/api/usuarios/{uuid}/marcar-remocao` | 🔒 (Self/Owner) | — |
| 10.2 | `PATCH` | `/api/usuarios/{uuid}/desmarcar-remocao` | 🔒 (Self/Owner) | — |
| 10.3 | `PUT` | `/api/usuarios/{uuid}/ativo` | 🔒 + 👑 Owner | — |
| 10.4 | `PATCH` | `/api/usuarios/{uuid}/bloqueado` | 🔒 + 👑 Owner | — |
| 11 | `POST` | `/api/admin/lojas` | 🔒 | 👑 |
| 12 | `GET` | `/api/admin/lojas/listar` | 🔒 | — |
| 13 | `POST` | `/api/admin/lojas/{loja_uuid}/funcionarios` | 🔒 | 👑 |
| 14 | `POST` | `/api/admin/lojas/{loja_uuid}/entregadores` | 🔒 | 👑 |
| 15 | `POST` | `/api/admin/lojas/{loja_uuid}/clientes` | 🔒 | 👑 |
| 15.1 | `POST` | `/api/admin/categorias/globais` | 🔒 | 👑 Owner |
| 15.2 | `PUT` | `/api/admin/categorias/globais/{uuid}` | 🔒 | 👑 Owner |
| 15.3 | `DELETE` | `/api/admin/categorias/globais/{uuid}` | 🔒 | 👑 Owner |
| 16 | `POST` | `/api/pedidos/criar` | — (auth opcional) | — |
| 17 | `GET` | `/api/pedidos/listar` | 🔒 | — |
| 17.1 | `GET` | `/api/pedidos/meus` | 🔒 | — |
| 17.2 | `GET` | `/api/pedidos/codigo/{codigo}` | — | — |
| 18 | `GET` | `/api/pedidos/por-loja/{loja_uuid}` | 🔒 | — |
| 19 | `GET` | `/api/pedidos/{uuid}` | 🔒 | — |
| 20 | `GET` | `/api/pedidos/{uuid}/com-entrega` | 🔒 | — |
| 21 | `PUT` | `/api/pedidos/{uuid}/status` | 🔒 | — |
| 22 | `PUT` | `/api/pedidos/{pedido_uuid}/entregador/{loja_uuid}` | 🔒 | — |
| 23 | `DELETE` | `/api/pedidos/{pedido_uuid}/entregador/{loja_uuid}` | 🔒 | — |
| 24 | `GET` | `/api/pedidos/{pedido_uuid}/com-entregador` | 🔒 | — |
| 25 | `POST` | `/api/cupons/` | 🔒 | — |
| 26 | `GET` | `/api/cupons/` | 🔒 | — |
| 27 | `GET` | `/api/cupons/{uuid}` | 🔒 | — |
| 28 | `PUT` | `/api/cupons/{uuid}` | 🔒 | — |
| 29 | `DELETE` | `/api/cupons/{uuid}` | 🔒 | — |
| 29.1 | `PATCH` | `/api/cupons/{uuid}/status` | 🔒 | — |
| 30 | `POST` | `/api/marketing/{loja_uuid}/cupons` | 🔒 | — |
| 31 | `GET` | `/api/marketing/cupons/{codigo}` | — | — |
| 32 | `GET` | `/api/marketing/cupons` | 🔒 | — |
| 33 | `POST` | `/api/marketing/{loja_uuid}/avaliar-loja` | 🔒 | — |
| 34 | `POST` | `/api/marketing/{loja_uuid}/avaliar-produto` | 🔒 | — |
| 34.1 | `GET` | `/api/marketing/{loja_uuid}/avaliacoes-loja` | 🔒 | — |
| 34.2 | `GET` | `/api/marketing/avaliacoes-loja/{uuid}` | 🔒 | — |
| 34.3 | `PUT` | `/api/marketing/avaliacoes-loja/{uuid}` | 🔒 | — |
| 34.4 | `DELETE` | `/api/marketing/avaliacoes-loja/{uuid}` | 🔒 | — |
| 34.5 | `GET` | `/api/marketing/{loja_uuid}/avaliacoes-produto` | 🔒 | — |
| 34.6 | `GET` | `/api/marketing/avaliacoes-produto/produto/{produto_uuid}` | 🔒 | — |
| 34.7 | `GET` | `/api/marketing/avaliacoes-produto/{uuid}` | 🔒 | — |
| 34.8 | `PUT` | `/api/marketing/avaliacoes-produto/{uuid}` | 🔒 | — |
| 34.9 | `DELETE` | `/api/marketing/avaliacoes-produto/{uuid}` | 🔒 | — |
| 35 | `POST` | `/api/marketing/{loja_uuid}/promocoes` | 🔒 | — |
| 36 | `GET` | `/api/marketing/{loja_uuid}/promocoes` | 🔒 | — |
| 37 | `PUT` | `/api/marketing/{loja_uuid}/promocoes/{uuid}` | 🔒 | — |
| 38 | `DELETE` | `/api/marketing/{loja_uuid}/promocoes/{uuid}` | 🔒 | — |
| 39 | `POST` | `/api/catalogo/{loja_uuid}/adicionais` | 🔒 | — |
| 40 | `GET` | `/api/catalogo/{loja_uuid}/adicionais` | — | — |
| 41 | `GET` | `/api/catalogo/{loja_uuid}/adicionais/disponiveis` | — | — |
| 42 | `PUT` | `/api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}` | 🔒 | — |
| 43 | `PUT` | `/api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}/disponibilidade` | 🔒 | — |
| 44 | `DELETE` | `/api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}` | 🔒 | — |
| 45 | `POST` | `/api/catalogo/{loja_uuid}/categorias` | 🔒 | — |
| 46 | `GET` | `/api/catalogo/{loja_uuid}/categorias` | — | — |
| 46.1 | `GET` | `/api/catalogo/categorias/globais` | — | — |
| 47 | `PUT` | `/api/catalogo/{loja_uuid}/categorias/{uuid}` | 🔒 | — |
| 48 | `DELETE` | `/api/catalogo/{loja_uuid}/categorias/{uuid}` | 🔒 | — |
| 49 | `POST` | `/api/enderecos-entrega/{pedido_uuid}/{loja_uuid}` | 🔒 | — |
| 50 | `GET` | `/api/enderecos-entrega/{pedido_uuid}` | 🔒 | — |
| 51 | `GET` | `/api/enderecos-entrega/{loja_uuid}/loja` | 🔒 | — |
| 52 | `GET` | `/api/enderecos-loja/{loja_uuid}` | 🔒 | — |
| 53 | `POST` | `/api/enderecos-loja/{loja_uuid}` | 🔒 | — |
| 54 | `PUT` | `/api/enderecos-loja/{loja_uuid}/{endereco_uuid}` | 🔒 | — |
| 55 | `DELETE` | `/api/enderecos-loja/{loja_uuid}/{endereco_uuid}` | 🔒 | — |
| 56 | `POST` | `/api/enderecos-usuario/` | 🔒 | — |
| 57 | `GET` | `/api/enderecos-usuario/` | 🔒 | — |
| 58 | `GET` | `/api/enderecos-usuario/{uuid}` | 🔒 | — |
| 59 | `PUT` | `/api/enderecos-usuario/{uuid}` | 🔒 | — |
| 60 | `DELETE` | `/api/enderecos-usuario/{uuid}` | 🔒 | — |
| 61 | `POST` | `/api/favoritos/{loja_uuid}` | 🔒 | — |
| 62 | `DELETE` | `/api/favoritos/{loja_uuid}` | 🔒 | — |
| 63 | `GET` | `/api/favoritos/minhas` | 🔒 | — |
| 64 | `GET` | `/api/favoritos/{loja_uuid}/verificar` | 🔒 | — |
| 65 | `POST` | `/api/produtos/` | 🔒 | — |
| 66 | `GET` | `/api/produtos/listar/{loja_uuid}` | — | — |
| 67 | `GET` | `/api/produtos/categoria/{loja_uuid}/{categoria_uuid}` | — | — |
| 68 | `GET` | `/api/produtos/{uuid}` | — | — |
| 69 | `PUT` | `/api/produtos/{uuid}` | 🔒 | — |
| 70 | `DELETE` | `/api/produtos/{uuid}` | 🔒 | — |
| 71 | `PUT` | `/api/produtos/{loja_uuid}/{produto_uuid}/disponibilidade` | 🔒 | — |
| 72 | `POST` | `/api/produtos/{uuid}/imagem` | 🔒 | — |
| 73 | `GET` | `/api/horarios/{loja_uuid}` | — | — |
| 73.1 | `GET` | `/api/horarios/{loja_uuid}/status` | — | — |
| 74 | `POST` | `/api/horarios/{loja_uuid}` | 🔒 | — |
| 75 | `PUT` | `/api/horarios/{loja_uuid}/dia/{dia_semana}/ativo` | 🔒 | — |
| 76 | `DELETE` | `/api/horarios/{loja_uuid}/dia/{dia_semana}` | 🔒 | — |
| 77 | `GET` | `/api/ok` | — | — |
| 78 | `DELETE` | `/api/wipe` ⚠️ | 🔒 + 👑 Owner | — |

**Total: 98 endpoints**

> **Legenda:** 🔒 = JWT required, 👑 Owner = apenas dono da plataforma (OWNER_EMAIL), 👑 Admin = administrador, (Self/Owner) = próprio usuário ou owner
