# Cálculo de Preço de Pedido — Especificação

> Documento que descreve em linguagem humana como o sistema Chickie calcula o preço total de um pedido, do momento em que o cliente envia a requisição até o valor persistido no banco de dados.

---

## Visão Geral do Fluxo

```
Cliente envia POST /api/pedidos/criar
        ↓
[Handler] Lê a requisição e delega ao Usecase
        ↓
[Usecase] Valida produtos e monta os itens
        ↓
[Service] Busca configuração da loja
        ↓
[Service] Calcula Subtotal
        ↓
[Service] Calcula Descontos (promoção e/ou cupom)
        ↓
[Service] Define Total Final e salva no banco
```

---

## Passo 1 — Recepção da Requisição

O cliente envia uma requisição com os seguintes dados relevantes para o cálculo:

| Campo | Tipo | Descrição |
|-------|------|-----------|
| `taxa_entrega` | Decimal | Valor do frete, informado pelo frontend |
| `itens` | Lista | Cada item tem quantidade + lista de partes (sabores) |
| `itens[].partes[].produto_uuid` | UUID | Identifica qual produto (sabor) está sendo pedido |
| `codigo_cupom` | String (opcional) | Código de desconto |

> **Nota:** A `taxa_entrega` é enviada pelo frontend — ela é exibida ao cliente antes da confirmação do pedido e enviada como parte da requisição. O servidor a usa diretamente no cálculo sem recalcular.

---

## Passo 2 — Validação e Snapshot de Preços dos Produtos

Para cada parte (sabor) de cada item, o sistema:

1. Busca o produto no banco de dados pelo `produto_uuid`
2. Captura o **nome** e o **preço atual** do produto como snapshot

> **Por que snapshot?** O preço do produto pode mudar no futuro. O pedido guarda o preço no momento em que foi feito, não o preço atual do catálogo.

Nenhum cálculo de valor acontece ainda — apenas a captura dos dados.

---

## Passo 3 — Busca da Configuração da Loja

O sistema busca a `ConfiguracaoDePedidosLoja`, que define **como calcular o preço quando um item tem múltiplos sabores**:

| Tipo de Cálculo | Como funciona |
|-----------------|---------------|
| `mais_caro` | O preço do item é o preço do sabor mais caro entre todas as partes |
| `media_ponderada` | O preço do item é a média aritmética dos preços de todos os sabores |

**Exemplo prático** — item com 2 sabores: Pizza Portuguesa (R$ 49,90) e Pizza Mussarela (R$ 39,90):

- Com `mais_caro`: preço = **R$ 49,90**
- Com `media_ponderada`: preço = **(49,90 + 39,90) / 2 = R$ 44,90**

> Se a loja não tiver configuração cadastrada, o pedido é recusado com erro.

---

## Passo 4 — Cálculo do Subtotal

O subtotal é calculado somando a contribuição de cada item. Para cada item:

### 4a. Preço base do item

Aplica a regra `mais_caro` ou `media_ponderada` sobre os preços das partes:

```
mais_caro:        preco_base = max(preco_parte_1, preco_parte_2, ...)
media_ponderada:  preco_base = (preco_parte_1 + preco_parte_2 + ...) / total_partes
```

### 4b. Soma dos adicionais do item

Soma o preço de todos os adicionais vinculados a **qualquer parte** do item:

```
total_adicionais = soma(adicional.preco) para todos adicionais de todas as partes
```

### 4c. Multiplicação pela quantidade

```
contribuicao_item = (preco_base + total_adicionais) × quantidade
```

### 4d. Acumulação no subtotal

```
subtotal = soma(contribuicao_item) para todos os itens
```

**Exemplo completo:**

| Item | Sabores | Preço Base | Adicionais | Quantidade | Contribuição |
|------|---------|-----------|------------|------------|-------------|
| Pizza | Portuguesa (49,90) + Mussarela (39,90) | 49,90 (mais_caro) | Queijo Extra (3,50) | 2 | (49,90 + 3,50) × 2 = **R$ 106,80** |
| Coca-Cola | Coca 2L (12,00) | 12,00 | — | 1 | 12,00 × 1 = **R$ 12,00** |

**Subtotal = R$ 118,80**

---

## Passo 5 — Cálculo de Descontos

O sistema verifica dois tipos de desconto de forma independente e depois decide qual aplicar.

### 5a. Promoções Automáticas da Loja

O sistema busca todas as promoções cadastradas da loja e, para cada uma, verifica se é aplicável ao pedido atual:

**Critérios de elegibilidade:**
- Status da promoção = `ativo`
- Data/hora atual está entre `data_inicio` e `data_fim`
- Subtotal do pedido >= `valor_minimo` da promoção (se definido)
- Dia da semana atual está na lista `dias_semana_validos` (se definida)

**Cálculo do desconto da promoção:**

| Tipo | Fórmula |
|------|---------|
| `percentual` | `subtotal × (percentagem / 100)` |
| `valor_fixo` | valor fixo em reais definido na promoção |
| `frete_gratis` | desconto = valor da `taxa_entrega` |

Se houver múltiplas promoções elegíveis, o sistema escolhe **a que gera o maior desconto**.

### 5b. Cupom de Desconto

Se o cliente informou um `codigo_cupom`, o sistema:

1. Busca o cupom pelo código vinculado à loja do pedido
2. Valida se é utilizável:
   - Cupom pertence à mesma loja do pedido
   - Status = `ativo`
   - Data atual < `data_validade`
   - Subtotal >= `valor_minimo` do cupom (se definido)
3. Calcula o desconto:

| Tipo | Fórmula |
|------|---------|
| `percentual` | `subtotal × (percentagem / 100)` |
| `valor_fixo` | valor fixo em reais definido no cupom |
| `frete_gratis` | desconto = valor da `taxa_entrega` |

Se o cupom não for encontrado ou não passar nas validações, o desconto do cupom é zero (sem erro para o cliente).

### 5c. Decisão de Qual Desconto Aplicar

Os dois descontos **não são acumulativos**. A lógica de prioridade é:

```
SE desconto_cupom > 0:
    desconto_final = desconto_cupom   ← Cupom tem prioridade sobre promoção
SENÃO SE desconto_promocao > 0:
    desconto_final = desconto_promocao
SENÃO:
    desconto_final = 0
```

> **Regra de negócio:** O cupom sempre tem prioridade. Se um cupom válido for informado, nenhuma promoção automática é aplicada, mesmo que a promoção gerasse desconto maior.

---

## Passo 6 — Cálculo do Total Final

```
total = subtotal + taxa_entrega - desconto_final
```

**Exemplo:**

```
subtotal      = R$ 118,80
taxa_entrega  = R$   5,00
desconto      = R$  10,00  (cupom 10% sobre subtotal: 118,80 × 10% = 11,88 → cupom percentual)
──────────────────────────
total         = R$ 118,80 + R$ 5,00 - R$ 11,88 = R$ 111,92
```

Os valores ficam registrados no pedido:
- `pedido.subtotal` — valor dos produtos antes do desconto e frete
- `pedido.taxa_entrega` — frete cobrado
- `pedido.desconto` — valor total descontado (pode ser null se zero)
- `pedido.total` — valor final que o cliente paga

---

## Passo 7 — Persistência

Após o cálculo, o sistema:

1. Gera um **código único de 6 caracteres** alfanuméricos (ex: `"A1B2C3"`) — tenta até 64 vezes garantindo unicidade no banco
2. Salva o pedido completo com todos os valores calculados
3. Se endereço de entrega foi informado, salva separadamente vinculado ao pedido

O retorno para o cliente é apenas `{ uuid, codigo }`. O cliente usa o código para acompanhar o pedido.

---

## Resumo das Fórmulas

```
preco_base_item   = max(partes.preco)           [se mais_caro]
                    ou soma(partes.preco) / n    [se media_ponderada]

total_adicionais  = soma(adicional.preco para todos adicionais de todas as partes do item)

contribuicao_item = (preco_base_item + total_adicionais) × quantidade

subtotal          = soma(contribuicao_item para todos os itens)

desconto          = desconto_cupom  [se cupom válido]
                    ou desconto_melhor_promocao
                    ou 0

total             = subtotal + taxa_entrega - desconto
```

---

## Casos Especiais

| Situação | Comportamento |
|----------|--------------|
| Item com uma única parte (sabor) | Preço base = preço daquela parte (ambos os modos produzem o mesmo resultado) |
| Item sem adicionais | `total_adicionais = 0` |
| Cupom inválido/expirado/abaixo do mínimo | Ignora silenciosamente; não retorna erro |
| Nenhuma promoção ativa | `desconto_promocao = 0` |
| Cupom `frete_gratis` | Desconto = valor exato da `taxa_entrega` (frete zerado) |
| Cupom percentual com limite | O desconto não ultrapassa o teto definido no cupom |
| Configuração da loja ausente | Pedido rejeitado com erro 500 ("Configuração da loja não encontrada") |



/////////////


Sugestões de Melhoria e Casos de Borda
Abaixo, listo alguns pontos para você "estressar" sua lógica antes de finalizar o código:

1. A Armadilha da taxa_entrega no Frontend
No Passo 1, você diz que o servidor usa a taxa enviada pelo front diretamente. Cuidado aqui.

Risco: Um usuário mal-intencionado pode interceptar a requisição e alterar o JSON para taxa_entrega: 0.00, mesmo morando a 10km de distância.

Recomendação: O backend deve, no mínimo, validar se aquela taxa enviada é condizente com a tabela de entrega da loja para aquela distância/bairro. Se o cálculo de frete for simples (ex: valor fixo por bairro), recalcule no back. Se for complexo (Google Maps API), valide se o valor enviado não é absurdamente menor que o esperado.

2. Precisão Decimal (Arredondamento)
No Passo 4a (media_ponderada), você terá divisões.

Exemplo: Pizza de R$ 49,90 + R$ 39,90 = R$ 89,80. Dividido por 2 = R$ 44,90 (exato).

Problema: E se forem 3 sabores com preços quebrados?

Dica: No Rust, use o crate rust_decimal. Defina uma estratégia de arredondamento fixa (geralmente Bankers Rounding ou para cima nas duas casas decimais) e aplique-a em cada passo intermediário para evitar que o total final divirja por R$ 0,01 do que o cliente vê.

3. Itens com Quantidade Zero ou Negativa
Validação: No Passo 2, adicione uma regra que rejeita o pedido se quantidade <= 0. Parece óbvio, mas se passar, o subtotal pode ser zerado ou até subtraído.

4. O "Teto" do Desconto (Passo 5)
Você mencionou cupons de porcentagem. É muito comum lojistas pedirem um "limite máximo".

Exemplo: "10% de desconto limitado a R$ 20,00".

Sua lógica deve prever um campo valor_maximo_desconto na tabela de cupons/promoções.

5. Disponibilidade de Estoque (ou Itens Desativados)
No Passo 2, além de capturar o preço, verifique se o produto está ativo e disponível. Se o cliente deixou o carrinho aberto por 2 horas e o lojista pausou a "Coca-Cola" nesse meio tempo, o servidor deve recusar o pedido no Passo 2.