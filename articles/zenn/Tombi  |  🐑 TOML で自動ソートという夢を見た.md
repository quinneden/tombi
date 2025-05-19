<!-- Published at https://zenn.dev/yassun7010/articles/2d84fe96a1ae2d -->

TOML Language Server を開発している @yassun7010 です。

https://zenn.dev/yassun7010/articles/6a60d705d0a55c

より多くの方に Tombi を知ってもらうため、いくつかに分けて Tombi の開発の小話を記事にしていきます。

今回は Tombi で最も特徴的な自動ソートについての話をしたいと思います。

## 自動ソートは不可能？

Tombi と同様の TOML Language Server である [taplo](https://taplo.tamasfe.dev/) でも自動ソートの機能があるのですが、部分的なサポートにとどまり、複数の Issues が開かれたまま解決されていない状態です。

taplo のアプローチにはいくつか問題があると考えていますが、[#675](https://github.com/tamasfe/taplo/issues/675) で議論されているように、コメントを考慮すると、自動ソートは簡単に行えないことがわかります。

taplo での議論より難しい問題を考えてみましょう。

```toml:Cargo.toml
[dependencies]
serde = "1.0.0"

# clap = "4.5.37"

ahash = "0.8.11"
```

さて、このようなファイルを自動ソートした場合、`# clap = ...` の扱いはどうすれば良いのでしょう？

1. `serde = ...` に紐づいていると解釈？
2. `ahash = ...` に紐づいていると解釈？
3. どちらにも紐づいていないと解釈？

一見すると 3 に見えますが、だとすると並び替えはどうするのでしょう？
名前順に並び替えると `ahash` -> `serde` ですが、どこにも紐づいていない `# clap` のコメントはどこに配置すれば良いのでしょうか？

並び替え後の形を決定できないので、自動ソートは不可能に見えます。

本当にそうでしょうか？

## 嘘だよ。できるよ
Tombi は次のように自動ソートします 🦅

:::details Tombi の自動ソート結果
```toml:Cargo.toml
[dependencies]
# clap = "4.5.37"
ahash = "0.8.11"
serde = "1.0.0"
```
:::

でも、どうやって？

自動ソートを実現するために、 Tombi は印象的なコメント解釈を行います。

## Tombi のコメント解釈ルール
### 一般的なコメントの分類
Tombi のコメント解釈について説明する前に、プログラミング言語をパースする問題としてみると、一般的にコメントは3種類に分類できます。

```toml:Leading Comment
# 要素の上にくっつくもの
# コメント行は連続していていい
key = "value"
```

```toml:Trailing Comment
key = "value" # 要素の後ろにくっつくもの
```

```toml: Dangling Comment
# 直前または直後に要素がないもの
# コメント行は連続していていい

# 空白行に挟まれているもの
# コメント行は連続していていい

key = "value"
# 要素の次の行にくっつくもの
# コメント行は連続していていい
```

このうち自動ソートを困難にするものは Dangling Comment です。

そのため、自動ソートを行うには、どのように Dangling Comment を扱うかが問題になります。

:::message
要素の次の行を Dangling Comment とするのではなく、 Tailing Comment のような上の要素に紐づくコメントにした方が良いのではないかと考える方がいるかもしれませんが、
次のケースを考えると Dangling Comment として扱い、 Leading Comment は Dangling Comment より優先して扱われると考えた方が良いことがわかります。

```toml
key1 = "value1"
# これは key1 の Dangling Comment？ いいえ、 key2 の Leading Comment！
key2 = "value2"
```
:::

### Dangling Comment なんてなかった
私は Tombi が新規参入であるメリットを活かして解決しようと考えました。

そもそも、新規参入である Tombi は最初から自動ソートを提供できるので、コメントがユーザの意図している通りに自動ソートできなくても「それが仕様です」で押し通すことができます 🙄

つまり、それなりに「筋の通ったルール」で、自動ソートをしてしまえば良かったのです。

:::message
「筋の通ったルール」とは何か？となりますが、ここでは次のようなものです。

- フォーマット前後で、要素が欠損せず、意味が変化しない（等価性）
- 繰り返しフォーマットを繰り返しても、フォーマット後の形が維持される（冪等性）
- フォーマットされた前後の差分について、大きな違和感が発生しない

:::

そこで、「自動ソートに邪魔な Dangling Comment」を「Leading Comment」として扱うという解釈にしました。

```toml:Cargo.toml
[dependencies]
serde = "1.0.0"

# clap = "4.5.37"
# これは Dangling Comment ではなく、 ahash の Leading Comment とみなす。

# これが普通の Leading Comment
ahash = "0.8.11"
```

どこの Dangling Comment が Leading Comment を満たすのかは、[ドキュメント](https://tombi-toml.github.io/tombi/docs/formatter/treatment-of-comments) か、特定のコメント行をエディタで Hover すればわかります。

![](https://storage.googleapis.com/zenn-user-upload/d3e7636ab9f8-20250518.png)

この解釈は、保存したコメントが意図しない行にソートされる可能性がありますが、

- 自動ソートによる大きな変更は最初の一度だけ
- 追加のコードを書いている分には、違和感が発生しない
- Tombi は仕様であると割り切っている

という考えで採用しました。


## 設定ファイルによる自動ソートの問題
さて、 taplo の自動ソートは taplo.toml で次のように設定することで利用できます。

```toml:talpo.toml
[formatting]
reorder_keys = true
```

しかしこの方法には穴があり、自動ソートして欲しいものは TOML ファイルの全てではなく一部なのです。

```toml:Cargo.toml
# このセクションはキー名で並び替えして欲しくない
[package]
name = "my-crate"
version = "1.0.0"
authors = ["taro <taro@gmail.com>"]

# このセクションはキー名で並び替えして欲しい
[dependencies]
anyhow = "^1.0.0"
serde = "^1.0.0"
```

設定ファイルで指定する方式のまま発展させるとすると、コメントで設定を追加する形でしょうか？

```toml:Cargo.toml
# taplo: reorder-key = false
[package]
name = "my-crate"
version = "1.0.0"
authors = ["taro <taro@gmail.com>"]

# taplo: reorder-key = true
[dependencies]
anyhow = "^1.0.0"
serde = "^1.0.0"
```

しかし、これは美しくない！
