# Análise — Sistema de Indicação e Categorias de Conta

> Documento de análise estratégica e técnica. Não representa decisão implementada.

---

## O que foi proposto

- Administradores (empresas clientes) iniciam na **Categoria A** (taxa: 4% por pedido)
- Cada admin pode gerar até **5 cupons de indicação**
- Um cupom é **confirmado** quando o indicado criar ao menos 1 loja + 1 produto
- Com **5 cupons confirmados** → upgrade para **Categoria B** (taxa: 2% por pedido)

---

## O que está bom

### Incentivo genuíno
Uma redução de 4% → 2% representa **50% de economia** sobre a taxa. Para uma loja com R$ 30.000/mês em pedidos, a diferença é:
- Categoria A: R$ 1.200/mês
- Categoria B: R$ 600/mês

Isso é forte o suficiente para motivar o admin a ativamente indicar a plataforma.

### Crescimento orgânico B2B
Admins indicam outros negócios do mesmo setor (vizinhos, concorrentes, parceiros). Esse canal tende a trazer clientes com perfil semelhante ao que já funciona — menor churn esperado.

### Filtro mínimo de qualidade
Exigir loja + produto afasta cadastros vazios. A indicação só vale quando o indicado deu ao menos os primeiros passos na plataforma.

---

## O que preocupa

### 1. Critério de confirmação fraco

**Problema:** Criar 1 loja e 1 produto custa menos de 2 minutos. Um usuário pode criar 5 contas fictícias, montar 5 lojas vazias e confirmar todos os cupons sem nunca gerar receita real para a Chickie.

**Recomendação:** Mudar o critério de confirmação para:
> A loja indicada precisa ter ao menos **1 pedido pago** processado pela plataforma.

Isso é muito mais difícil de falsificar, alinha o incentivo do indicador com o da Chickie (receita real), e garante que o indicado está de fato ativo.

---

### 2. Troca permanente por evento único

**Problema:** O upgrade para Categoria B é permanente e baseado em um evento que acontece uma vez. Isso significa que um admin pode atingir o upgrade e depois os 5 indicados abandonarem a plataforma — mas ele continua pagando 2%.

**Recomendação:** Avaliar uma das duas abordagens:

| Abordagem | Descrição | Vantagem |
|---|---|---|
| **Upgrade permanente** | Atingiu os 5, ficou em B para sempre | Simplicidade, forte incentivo |
| **Upgrade condicional** | Mantém B enquanto X dos indicados estão ativos | Sustentável, mas mais complexo |

Se a meta é aquisição de clientes, o upgrade permanente é mais agressivo e funciona melhor no início.

---

### 3. Teto de 5 cupons pode frustrar bons indicadores

**Problema:** Um admin com rede grande pode indicar 20 pessoas mas só receber benefício pelas primeiras 5.

**Recomendação:** Permitir continuar gerando cupons além de 5. A recompensa de 2% já está dada, mas o indicador pode receber outros benefícios futuros (categoria C, créditos, etc). Não bloquear quem quer indicar mais.

---

### 4. Impacto na receita precisa ser projetado

**Simulação simples:**

Suponha 100 admins na Categoria A, cada um processando R$ 10.000/mês:
- Receita atual: 100 × R$ 400 = **R$ 40.000/mês**

Se 20% atingirem Categoria B (20 admins), mas cada um trouxe 5 novos admins (100 novos):
- 80 admins em A: R$ 32.000
- 20 admins em B: R$ 4.000
- 100 novos admins em A: R$ 40.000
- **Total: R$ 76.000/mês**

A perda de receita por cliente é mais do que compensada pelo volume de novos clientes — desde que a taxa de conversão das indicações seja real.

---

## Riscos colaterais

| Risco | Probabilidade | Mitigação |
|---|---|---|
| Gaming com contas falsas | Alta (se critério for loja+produto) | Exigir pedido pago |
| Admins em B parando de pagar taxa justa | Média | Aceitar como custo de aquisição calculado |
| Complexidade de código subestimada | Alta | Ver seção técnica abaixo |
| Confusão do usuário sobre o status | Média | Dashboard claro com progresso dos cupons |

---

## Sugestão de ajuste no modelo

```
Admin se cadastra → Categoria A (4%)
        ↓
Gera cupom de indicação (até 5 ativos simultâneos)
        ↓
Indicado usa o cupom no signup
        ↓
Indicado cria loja + produto + tem 1 pedido pago
        ↓
Cupom confirmado (notificação ao indicador)
        ↓
5 cupons confirmados → upgrade para Categoria B (2%)
        ↓
Pode continuar gerando cupons (sem limite), mas sem novo benefício por ora
```

---

## Complexidade técnica

### Novas entidades necessárias

**`categoria_conta`** — campo na tabela `usuarios`:
```
A | B
```

**`cupons_indicacao`** — nova tabela:
```
uuid
gerado_por_uuid       FK → usuarios
usado_por_uuid        FK → usuarios (nullable até ser usado)
codigo                VARCHAR único
status                pendente | usado | confirmado | expirado
criado_em
confirmado_em
```

### Lógica de negócio adicional

- Ao criar usuário admin: verificar se veio com `codigo_indicacao` no signup → vincular cupom
- Ao criar primeiro pedido pago na loja do indicado: verificar se ele tem cupom `usado` pendente → marcar como `confirmado`
- Ao confirmar 5º cupom do indicador: alterar `categoria_conta` de A para B
- Cálculo da taxa no pedido: ler `categoria_conta` do admin dono da loja

### Impacto em código existente

| Arquivo | Mudança necessária |
|---|---|
| `migrations/` | Nova tabela `cupons_indicacao`, coluna `categoria_conta` em `usuarios` |
| `models/usuario.rs` | Campo `categoria_conta` |
| `services/usuario_service.rs` | Verificar cupom no signup |
| `services/pedido_service.rs` | Lógica de confirmar cupom no 1º pedido pago |
| `usecases/pagamento.rs` | Calcular taxa variável (4% ou 2%) |
| `api/auth/` | Campo opcional `codigo_indicacao` no signup |

---

## Opinião final

A estratégia é sólida para o estágio atual da Chickie. O principal risco é o critério de confirmação fraco — **troque loja+produto por pedido pago** antes de lançar, senão o sistema será explorado em dias.

O modelo financeiro fecha desde que a taxa de conversão das indicações seja real: cada indicação bem-sucedida adiciona um novo cliente que paga 4%, compensando a redução do indicador para 2%.

Se for implementar, comece simples: sem expiração de cupons, sem limite além de 5, sem categoria C ainda. Valide se as pessoas realmente usam o sistema antes de adicionar complexidade.

---

*Documento gerado em 2026-04-29 para discussão interna.*
