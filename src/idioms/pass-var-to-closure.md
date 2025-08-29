# クロージャに変数を渡す

## 説明

デフォルトでは、クロージャは借用によって環境をキャプチャします。または、
`move`クロージャを使用して環境全体を移動できます。しかし、多くの場合、
一部の変数のみをクロージャに移動し、一部のデータのコピーを与え、参照で渡す、
または他の変換を実行したいことがあります。

そのために、別のスコープで変数の再バインディングを使用します。

## 例

次を使用：

```rust
use std::rc::Rc;

let num1 = Rc::new(1);
let num2 = Rc::new(2);
let num3 = Rc::new(3);
let closure = {
    // `num1`は移動される
    let num2 = num2.clone();  // `num2`はクローンされる
    let num3 = num3.as_ref();  // `num3`は借用される
    move || {
        *num1 + *num2 + *num3;
    }
};
```

以下の代わりに：

```rust
use std::rc::Rc;

let num1 = Rc::new(1);
let num2 = Rc::new(2);
let num3 = Rc::new(3);

let num2_cloned = num2.clone();
let num3_borrowed = num3.as_ref();
let closure = move || {
    *num1 + *num2_cloned + *num3_borrowed;
};
```

## 利点

コピーされたデータはクロージャの定義とともにグループ化されるため、その目的が
より明確になり、クロージャによって消費されなくても即座にドロップされます。

データがコピーされるか移動されるかに関わらず、クロージャは周囲のコードと
同じ変数名を使用します。

## 欠点

クロージャ本体の追加のインデント。
