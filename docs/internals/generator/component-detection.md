<!-- [Doc] | Алгоритм детекции переиспользуемых компонентов: hashing, clustering, similarity, ranking. -->

# Component Detection

## Phase 1: Hashing

- Выполняется DFS обход дерева.
- Для каждого узла считается хеш:
  tag name + sorted attributes + children hashes.

Псевдокод:

```text
fn hash(node):
  child_hashes = [hash(c) for c in node.children]
  sort(child_hashes)
  payload = node.tag + serialize(sorted(node.attrs)) + join(child_hashes)
  return sha256(payload)
```

## Phase 2: Clustering

- Узлы группируются в структуру HashMap<Hash, Vec<NodeRef>>.
- Каждая группа представляет кандидатов на переиспользуемый компонент.

## Phase 3: Similarity

- Для возможных коллизий хеша вычисляется structural similarity.
- Порог принятия: > 95%.
- Метрика: совпадающие nodes / общее количество nodes.

## Phase 4: Ranking

- Кандидаты сортируются по occurrences.
- Применяется min_size фильтр (default 2, то есть >= 2 использования).
- Применяется max_depth фильтр (default 3), чтобы не выделять слишком глубокие поддеревья.

## Результат

Итоговая структура:

```text
Component {
  id,
  template: SVGElement,
  occurrences: Vec<Occurrence>
}
```
