# 簡単なdoc初期化

## 説明

ドキュメントを書く際に構造体の初期化に多大な労力が必要な場合、
構造体を引数として受け取るヘルパー関数で例をラップする方が
早い場合があります。

## 動機

複数のまたは複雑なパラメータといくつかのメソッドを持つ構造体が
あることがあります。これらのメソッドのそれぞれには例が必要です。

例えば：

````rust,ignore
struct Connection {
    name: String,
    stream: TcpStream,
}

impl Connection {
    /// 接続を介してリクエストを送信します。
    ///
    /// # 例
    /// ```no_run
    /// # // 例を動作させるにはボイラープレートが必要です。
    /// # let stream = TcpStream::connect("127.0.0.1:34254");
    /// # let connection = Connection { name: "foo".to_owned(), stream };
    /// # let request = Request::new("RequestId", RequestType::Get, "payload");
    /// let response = connection.send_request(request);
    /// assert!(response.is_ok());
    /// ```
    fn send_request(&self, request: Request) -> Result<Status, SendErr> {
        // ...
    }

    /// うわあ、このボイラープレートをここでも繰り返す必要があります！
    fn check_status(&self) -> Status {
        // ...
    }
}
````

## 例

`Connection`と`Request`を作成するためにこのすべてのボイラープレートを
入力する代わりに、それらを引数として受け取るラッピングヘルパー関数を
作成する方が簡単です：

````rust,ignore
struct Connection {
    name: String,
    stream: TcpStream,
}

impl Connection {
    /// 接続を介してリクエストを送信します。
    ///
    /// # 例
    /// ```
    /// # fn call_send(connection: Connection, request: Request) {
    /// let response = connection.send_request(request);
    /// assert!(response.is_ok());
    /// # }
    /// ```
    fn send_request(&self, request: Request) {
        // ...
    }
}
````

**注意** 上記の例では、行`assert!(response.is_ok());`は
呼び出されることのない関数の内部にあるため、テスト中に実際には実行されません。

## 利点

これははるかに簡潔で、例での反復的なコードを回避します。

## 欠点

例が関数内にあるため、コードはテストされません。ただし、`cargo test`の実行時に
コンパイルされることを確認するためにチェックされます。そのため、このパターンは
`no_run`が必要な場合に最も有用です。これにより、`no_run`を追加する必要がなくなります。

## 議論

アサーションが不要な場合、このパターンはうまく機能します。

必要な場合、代替案として`#[doc(hidden)]`でアノテーションされた
（ユーザーには見えないように）ヘルパーインスタンスを作成するパブリックメソッドを
作成できます。そのメソッドはクレートのパブリックAPIの一部であるため、
rustdoc内で呼び出すことができます。
