# ガード付きRAII

## 説明

[RAII][wikipedia]は「Resource Acquisition is Initialisation（リソース取得は初期化）」
の略で、ひどい名前です。このパターンの本質は、リソースの初期化がオブジェクトの
コンストラクタで行われ、最終化がデストラクタで行われることです。このパターンは
Rustでは、RAIIオブジェクトをリソースのガードとして使用し、型システムに依存して
アクセスが常にガードオブジェクトによって仲介されることを保証することで拡張されています。

## Example

Mutex guards are the classic example of this pattern from the std library (this
is a simplified version of the real implementation):

```rust,ignore
use std::ops::Deref;

struct Foo {}

struct Mutex<T> {
    // ここでデータTへの参照を保持します。
    //..
}

struct MutexGuard<'a, T: 'a> {
    data: &'a T,
    //..
}

// mutexのロックは明示的です。
impl<T> Mutex<T> {
    fn lock(&self) -> MutexGuard<T> {
        // 基盤OSのmutexをロックします。
        //..

        // MutexGuardはselfへの参照を保持します
        MutexGuard {
            data: self,
            //..
        }
    }
}

// mutexのロックを解除するためのデストラクタ。
impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // 基盤OSのmutexのロックを解除します。
        //..
    }
}

// Derefの実装により、MutexGuardをTへのポインターのように扱えます。
impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

fn baz(x: Mutex<Foo>) {
    let xx = x.lock();
    xx.foo(); // fooはFooのメソッドです。
              // 借用チェッカーは、ガードxxよりも長生きする基盤の
              // Fooへの参照を保存できないことを保証します。

    // この関数を抜ける際とxxのデストラクタが実行される際にxはロック解除されます。
}
```

## 動機

使用後にリソースを最終化しなければならない場合、RAIIを使用してこの
最終化を行うことができます。最終化後にそのリソースにアクセスすることがエラーである場合、
このパターンを使用してそのようなエラーを防ぐことができます。

## 利点

リソースが最終化されない場合や、最終化後にリソースが使用される場合の
エラーを防ぎます。

## 議論

RAIIは、リソースが適切に割り当て解除または最終化されることを保証するための
有用なパターンです。Rustでは借用チェッカーを利用して、最終化後にリソースを
使用することによるエラーを静的に防ぐことができます。

借用チェッカーの中核的な目的は、データへの参照がそのデータより長生きしないことを
保証することです。RAIIガードパターンが機能するのは、ガードオブジェクトが
基盤リソースへの参照を含み、そのような参照のみを公開するからです。Rustは、
ガードが基盤リソースより長生きできないこと、およびガードによって仲介されるリソースへの
参照がガードより長生きできないことを保証します。これがどのように機能するかを確認するには、
ライフタイム省略なしの`deref`のシグネチャを調べることが有用です：

```rust,ignore
fn deref<'a>(&'a self) -> &'a T {
    //..
}
```

リソースへの返される参照は`self`と同じライフタイム（`'a`）を持ちます。
したがって借用チェッカーは、`T`への参照のライフタイムが`self`のライフタイムより
短いことを保証します。

`Deref`の実装はこのパターンの中核部分ではなく、ガードオブジェクトの使用を
よりエルゴノミクスにするだけであることに注意してください。ガードに`get`メソッドを
実装することも同様に機能します。

## 関連項目

[デストラクタでの最終処理イディオム](../../idioms/dtor-finally.md)

RAIIはC++での一般的なパターンです：
[cppreference.com](http://en.cppreference.com/w/cpp/language/raii)、
[wikipedia][wikipedia]。

[wikipedia]: https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization

[スタイルガイドエントリ](https://doc.rust-lang.org/1.0.0/style/ownership/raii.html)
（現在はプレースホルダーのみ）。
