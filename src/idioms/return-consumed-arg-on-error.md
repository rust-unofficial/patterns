# エラー時に消費された引数を返す

## 説明

失敗する可能性のある関数が引数を消費（移動）する場合、その引数を
エラーの内部に返します。

## 例

```rust
pub fn send(value: String) -> Result<(), SendError> {
    println!("using {value} in a meaningful way");
    // 非決定論的な失敗可能なアクションをシミュレート。
    use std::time::SystemTime;
    let period = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    if period.subsec_nanos() % 2 == 1 {
        Ok(())
    } else {
        Err(SendError(value))
    }
}

pub struct SendError(String);

fn main() {
    let mut value = "imagine this is very long string".to_string();

    let success = 's: {
        // 値を二回送信してみる。
        for _ in 0..2 {
            value = match send(value) {
                Ok(()) => break 's true,
                Err(SendError(value)) => value,
            }
        }
        false
    };

    println!("success: {success}");
}
```

## 動機

エラーの場合、代替方法を試したり、非決定論的関数の場合にアクションを再試行したい
かもしれません。しかし、引数が常に消費される場合、毎回クローンを作成することを
強制され、これはあまり効率的ではありません。

標準ライブラリでは、例えば`String::from_utf8`メソッドでこのアプローチを使用しています。
有効なUTF-8を含まないベクタが与えられると、`FromUtf8Error`が返されます。
`FromUtf8Error::into_bytes`メソッドを使用して元のベクタを取り戻すことができます。

## 利点

可能な限り引数を移動するため、より良いパフォーマンス。

## 欠点

わずかにより複雑なエラー型。
