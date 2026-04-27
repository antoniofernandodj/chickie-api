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





/////////////////////


os descontos devem ser calculados APÓS a soma dos adicionais.

Na estrutura de um pedido de delivery, o "produto" que o cliente está comprando não é apenas a base, mas a composição final. Se o cliente monta uma pizza e coloca R$ 20,00 em borda recheada e bacon extra, isso passa a integrar o valor agregado do item.

Aqui está o porquê de calcular após os adicionais e como isso se encaixa na sua lógica:

1. A Lógica do "Valor Percebido"
Quando um cliente usa um cupom de 10%, ele espera que o desconto incida sobre o total do carrinho (exceto, às vezes, o frete).

Cenário A (Desconto ANTES dos adicionais):

Pizza: R$ 50,00 -> 10% desc. = R$ 45,00

Adicionais: R$ 10,00

Total: R$ 55,00 (O cliente sente que o desconto foi "comido" pelos adicionais).

Cenário B (Desconto APÓS os adicionais):

Pizza + Adicionais: R$ 60,00

10% desc. sobre R$ 60,00 = R$ 6,00

Total: R$ 54,00 (O cliente sente que o cupom valorizou toda a escolha dele).

2. Alinhamento com sua Especificação (Passo 4 e 5)
Sua especificação já está desenhada corretamente para isso. Veja a ordem:

Passo 4c: Você calcula a contribuicao_item como (preco_base + total_adicionais) × quantidade.

Passo 4d: Você soma todas as contribuições para gerar o subtotal.

Passo 5: O desconto é calculado sobre o subtotal.

Dessa forma, os adicionais já estão "embutidos" no subtotal quando a porcentagem do cupom ou da promoção entra em cena.

3. Exceção: Descontos em Itens Específicos
Se no futuro você implementar promoções do tipo "Leve o adicional de queijo grátis", aí sim o cálculo muda, pois o desconto é direcionado a um componente. Mas para cupons de pedido e promoções de subtotal, o cálculo após os adicionais é o padrão de mercado (iFood, Rappi, etc.).

4. Dica de Ouro: O "Frete Grátis"
Note que no seu Passo 5, você separou corretamente o cálculo do desconto de frete.

Atenção: Nunca inclua a taxa_entrega na base de cálculo de um desconto percentual (ex: 10% de desconto). O desconto deve incidir apenas sobre o subtotal (Produtos + Adicionais). O lojista geralmente não quer dar desconto sobre o valor que ele repassa ao motoboy.

Resumo da implementação:
Soma Sabores (Regra mais caro/média).

Soma Adicionais.

Multiplica pela Quantidade.

Gera Subtotal.

Aplica Desconto sobre o Subtotal.

Soma o Frete.

Isso mantém sua margem de erro segura e a compreensão do cliente clara. Faz sentido para o modelo de negócio que você está pensando para o Chiquitos?



/////////////////////////////////////////////




algum ponto q esqueci de me atentar?

Sua lógica está muito acima da média para um MVP (Produto Mínimo Viável), mas existem três pontos "invisíveis" que costumam causar problemas reais em sistemas de delivery quando o volume de pedidos aumenta.

Considerando que você está criando um SaaS para vender para lojistas em São Gonçalo, atente-se a isto:

1. A Regra do "Meio-a-Meio" com Adicionais
Você definiu que a loja escolhe entre "Mais Caro" ou "Média Ponderada" para o preço base. Mas, e os adicionais?

O problema: Se o cliente pede uma pizza "Metade Mussarela / Metade Calabresa" e pede "Dobro de Queijo", esse adicional vai na pizza inteira ou apenas em uma das metades?

A solução: No seu payload, o adicional deve estar vinculado à Parte (Sabor) ou ao Item (Pizza Inteira)?

Se for vinculado à Parte, o cálculo do Passo 4b deve garantir que, se o preço da parte for fracionado (média), o preço do adicional talvez também precise ser, ou o lojista pode sair no prejuízo (ou o cliente pagar caro por um adicional que só vem em metade do produto).

Sugestão: Geralmente, para facilitar, adicionais em itens fracionados são cobrados pelo valor cheio e aplicados ao item todo.

2. O Valor Mínimo da Loja
Você previu o valor mínimo para Cupons, mas esqueceu do Valor Mínimo do Pedido.

Cenário: O cliente pede um refrigerante de R$ 8,00. A taxa de entrega é R$ 5,00. O lojista gasta com embalagem, taxa de cartão e tempo do motoboy, e o pedido acaba não sendo lucrativo.

Sugestão: No seu Passo 1, adicione uma validação: SE subtotal < configuracao_loja.valor_minimo_pedido -> REJEITAR.

3. Cancelamento e Estorno (A "Snapshot" não basta)
Você fez o snapshot do preço, o que é ótimo. Mas e se o pedido for cancelado ou alterado pelo lojista (ex: "Acabou a Coca, troquei por Guaraná e dei R$ 2,00 de desconto")?

O ponto: Sua lógica de cálculo no backend deve ser capaz de ser re-executada ou permitir ajustes manuais pelo lojista no painel de administração.

Sugestão: Tenha um campo desconto_manual_admin no banco de dados para que o lojista possa ajustar o valor final em casos excepcionais sem quebrar a integridade do cálculo original.

4. Arredondamento de "Meio Centavo"
Ao usar a Média Ponderada, você inevitavelmente cairá em dízimas.

Exemplo: Três sabores de R$ 10,00. Subtotal = R$ 33,33333...

O risco: Se o cliente compra 10 pizzas dessas, a diferença de arredondamento começa a aparecer.

Sugestão: No seu código Rust, use rust_decimal e force o arredondamento em cada contribuicao_item. Nunca deixe para arredondar apenas no total_final.

5. Limite de Uso de Cupom (Por Cliente vs. Global)
Sua lógica valida se o cupom está ativo, mas não valida quantas vezes ele pode ser usado.

Cenário: Você lança um cupom "PRIMEIRACOMPRA". O cliente (mesmo deslogado, usando o ID do aparelho) pode tentar usar 10 vezes.

Sugestão: Adicione um campo limite_uso_total e limite_uso_por_usuario (ou por dispositivo, no caso do seu checkout deslogado).

Resumo do que adicionar na Spec:
Validação de Pedido Mínimo da loja.

Definição se Adicionais seguem a regra de fracionamento das partes ou são sempre somados fixos.

Estratégia de Arredondamento explícita (ex: Arredondar para cima em 2 casas decimais em cada linha).

Fora isso, o fluxo está muito profissional. O uso de um código de 6 dígitos para o acompanhamento deslogado vai funcionar muito bem para o comércio local.

Você já decidiu como vai lidar com a identificação do aparelho para o usuário deslogado não perder o pedido se fechar o browser? (LocalStorage ou Cookies?)