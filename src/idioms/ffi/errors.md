# FFIでのエラーハンドリング

## 説明

Cのような外部言語では、エラーはリターンコードで表現されます。しかし、
Rustの型システムは、はるかに豊かなエラー情報をキャプチャし、
完全な型を通じて伝播することを可能にします。

このベストプラクティスは、異なる種類のエラーコードと、それらを
使いやすい方法で公開する方法を示します：

1. フラットなEnumは整数に変換し、コードとして返すべきです。
2. 構造化されたEnumは、詳細のための文字列エラーメッセージを伴う
   整数コードに変換すべきです。
3. カスタムエラー型は「透明」にし、C表現を持つべきです。

## Code Example

### Flat Enums

```rust,ignore
enum DatabaseError {
    IsReadOnly = 1,    // ユーザーが書き込み操作を試行
    IOError = 2,       // ユーザーはそれが何であったかをCのerrno()で読むべき
    FileCorrupted = 3, // ユーザーは修復ツールを実行して復旧すべき
}

impl From<DatabaseError> for libc::c_int {
    fn from(e: DatabaseError) -> libc::c_int {
        (e as i8).into()
    }
}
```

### Structured Enums

```rust,ignore
pub mod errors {
    enum DatabaseError {
        IsReadOnly,
        IOError(std::io::Error),
        FileCorrupted(String), // 問題を説明するメッセージ
    }

    impl From<DatabaseError> for libc::c_int {
        fn from(e: DatabaseError) -> libc::c_int {
            match e {
                DatabaseError::IsReadOnly => 1,
                DatabaseError::IOError(_) => 2,
                DatabaseError::FileCorrupted(_) => 3,
            }
        }
    }
}

pub mod c_api {
    use super::errors::DatabaseError;
    use core::ptr;

    #[no_mangle]
    pub extern "C" fn db_error_description(
        e: Option<ptr::NonNull<DatabaseError>>,
    ) -> Option<ptr::NonNull<libc::c_char>> {
        // SAFETY: `e`のライフタイムが現在のスタックフレームよりも
        // 長いことを仮定しています。
        let error = unsafe { e?.as_ref() };

        let error_str: String = match error {
            DatabaseError::IsReadOnly => {
                format!("読み取り専用データベースに書き込めません")
            }
            DatabaseError::IOError(e) => {
                format!("I/O Error: {e}")
            }
            DatabaseError::FileCorrupted(s) => {
                format!("ファイルが破損しています、修復を実行してください: {}", &s)
            }
        };

        let error_bytes = error_str.as_bytes();

        let c_error = unsafe {
            // SAFETY: error_bytesを末尾に'\0'バイトを持つ
            // 割り当てられたバッファにコピー。
            let buffer = ptr::NonNull::<u8>::new(libc::malloc(error_bytes.len() + 1).cast())?;

            buffer
                .as_ptr()
                .copy_from_nonoverlapping(error_bytes.as_ptr(), error_bytes.len());
            buffer.as_ptr().add(error_bytes.len()).write(0_u8);
            buffer
        };

        Some(c_error.cast())
    }
}
```

### Custom Error Types

```rust,ignore
struct ParseError {
    expected: char,
    line: u32,
    ch: u16,
}

impl ParseError {
    /* ... */
}

/* C構造体として公開される2番目のバージョンを作成 */
#[repr(C)]
pub struct parse_error {
    pub expected: libc::c_char,
    pub line: u32,
    pub ch: u16,
}

impl From<ParseError> for parse_error {
    fn from(e: ParseError) -> parse_error {
        let ParseError { expected, line, ch } = e;
        parse_error { expected, line, ch }
    }
}
```

## 利点

これはRustコードのAPIを全く損なうことなく、外部言語がエラー情報に
明確にアクセスできることを保証します。

## 欠点

多くのタイピングが必要であり、一部の型はCに簡単に変換できない可能性が
あります。
