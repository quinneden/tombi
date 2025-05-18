# Tombi | New TOML Language Server

ここしばらく [Tombi](https://tombi-toml.github.io/tombi/) と名付けた TOML の Language Server を Rust で開発しています。

@[card](https://github.com/tombi-toml/tombi)

TOML の Language Server として [taplo](https://github.com/tamasfe/taplo) が既に有名ですが、開発者のメンテナンス継続が困難になってしまってから、不完全な構文で保存すると一部のデータが欠損するようになったりと、利用するには難しいツールになっています。

Tombi は既に実際の開発で使える水準の Language Server に成長しました。

より多くの方に Tombi を利用してもらうため、何回かに分けて Tombi の小話を投稿していきます。

## Tombi と言う名称
プロジェクト名を決めることは開発者の一つの楽しみです。

今回は TOML の Language Server と言う利用者が多いツールであることを念頭に、次の制約を置いて考えました。

- 検索しやすく、重複したツールが少ない単語であること（sqlfmt などは重複しやすい）
- TOML から連想しやすい名称であること（black などは連想しにくい）
- 文字数が少なく、語感の良いこと（kubernetes などは覚えにくい）
- イメージと結びつきやすい名前であること

そして Tombi (鳶) が産まれた 🦅
![](https://storage.googleapis.com/zenn-user-upload/8f36511d4972-20250514.jpg)

TOML の公式サイトである [toml.io](https://toml.io) の `io` を左右にひっくり返して `l` と `o` をくっつけると `Tombi` になります。

以前、 [turu-py](https://github.com/yassun7010/turu-py) （鶴）を作っていたので、鳥の名前がマイブームなのでしょうか？ 🦆


## 自動ソート機能

Tombi の最大の特徴は、自動ソートに対応していることでしょう。

taplo も設定ファイルに記述することで自動ソートに対応できましたが、Tombi **は設定ファイルの記載なし**で自動ソートを行えます。

![](https://storage.googleapis.com/zenn-user-upload/a49422b81035-20250514.gif)

自動ソートのルールは [JSON Schema Store](https://www.schemastore.org/json/) から読み取っており、 JSON Schema に `x-tombi-table-keys-order` を指定することで自動ソートの方法を決定します。
詳しくは [ドキュメント](https://tombi-toml.github.io/tombi/docs/json-schema) を参照してください。

自動ソートを実現するためのコメントの扱いは工夫されており、それは別の記事で紹介しようと思います。

## 設定はサービス提供者へ任せよう

設定ファイルを書く動機を少なくすることは Tombi の設計思想の一つです。
JSON Schema を定義するサービス・ライブラリ提供者がルールを決定することで、利用者は何も考えずに最初から設計者の意図したフォーマットで開発できるように作られています。
（**🚀 Zero Configuration** としてドキュメントでもアピールされています）

実際 Tombi は `tombi.toml` 設定ファイルを用意できますが、そのほとんどは JSON Schema をどのように取得するか（どのサービスプロバイダーを選択するか）にあります。


## 設定の余地がないフォーマッタ
ドキュメントにも明記していますが、[black](https://github.com/psf/black) に影響を受け、それをさらに推し進め、フォーマッタに関しては設定項目が一切ありません。

[rubocop](https://github.com/rubocop/rubocop) に苦しんだ経験から、フォーマッタの設定にこだわる時間は無駄だと思っています。
（それに TOML が設定記述言語であることを考えると、「設定ファイルのための設定」なんて、どうかしている 🙄 ）

最近のプログラミング言語がフォーマッタを最初から用意するようになったことと同じく、 TOML ファイルのフォーマット方法もスキーマの提供者側が指定することで、どのプロジェクトも同じレイアウトで TOML が保存されるように設計されています。

代わりに採用したのは black の [The magic trailing comma](https://black.readthedocs.io/en/stable/the_black_code_style/current_style.html#the-magic-trailing-comma) と、 Magic Trigger と名付けた補完機能です。

magic trailing comma は配列やインラインテーブルの最後の要素にカンマがついていた場合、強制的に複数行にフォーマットする機能です。

magic trigger は TOML のテーブルとインラインテーブル、どちらで記述したいかを補完中に選択できる機能です。

![](https://storage.googleapis.com/zenn-user-upload/c42a3f65618b-20250515.png)

- 「=」を選んだ場合はテーブル形式で補完： `author = { name = "taro" }`
- 「.」を選んだ場合はインラインテーブルで補完： `author.name = "taro"`

と言う挙動をします。

背後には **設定ではなく、ユーザが何を書いたかでフォーマットを決める** と言う思想があります。

## 最後に

少々過激な思想ではありますが、妥協できるレベルと判断して、ユーザの反応を伺っています。

今回は述べていない機能がまだまだありますが、X で新機能の紹介などもしているので、ぜひ反応をください。

https://x.com/tombi_toml/status/1920455555576889839

また、 [スポンサーになっていただける方](https://github.com/sponsors/tombi-toml) を募集しているので、応援していただける方は GitHub のスターや寄付をいただけると、新機能や Issue へ注ぐ時間が取れるので、応援よろしくお願いします。
