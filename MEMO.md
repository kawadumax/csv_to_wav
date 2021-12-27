# PowerShellで標準入力のリダイレクトは以下のようなものとなる

```powershell
cat .\csv\sample.csv | cargo run # 開発中
cat .\csv\sample.csv | .\target\debug\csv_to_wav.exe # build後
```

# データはここから

https://stackoverflow.com/questions/16143266/get-bitcoin-historical-data

```
Actually, you CAN get the whole Bitcoin trades history from Bitcoincharts in CSV format here : http://api.bitcoincharts.com/v1/csv/
it is updated twice a day for active exchanges, and there is a few dead exchanges, too.
EDIT: Since there are no column headers in the CSVs, here's what they are : column 1) the trade's timestamp, column 2) the price, column 3) the volume of the trade
```

1-10までのサンプル
ちなみにヘッダーは無い
```
1315922016,5.800000000000,1.000000000000
1315922024,5.830000000000,3.000000000000
1315922029,5.900000000000,1.000000000000
1315922034,6.000000000000,20.000000000000
1315924373,5.950000000000,12.452100000000
1315924504,5.880000000000,7.458000000000
1315924614,5.880000000000,0.176882380000
1315925663,5.760000000000,2.267000000000
1315927898,5.650000000000,2.542000000000
1315942379,5.920000000000,0.450000000000
```

タイムスタンプ
2011/09/13 22:53:36 ~ 2021/10/15 07:17:29

# 行数
1 - 55866758行 3列
カラム１:タイムスタンプ カラム２:プライス カラム3出来高


# 2021-1111
再開した。過去の自分が何を求めているのかわからん。最初に簡単にでも仕様を書いておけばよかった...
で、今見た感じ、CLIっぽく作ろうとしていたみたい。
どのCSVファイルを読みこむのかの設定ができると便利だろうなぁと思いつつ、
現在はNFTのための音源を作るだけなので、まあいいかな、ハードコーディングで。

本番データがとにかく重たいので、sinを出力できるCSVを作成しようと思います。

# 2021-1224
メリークリスマス。CSVからSINE波の合成は出来ているみたいなので、とりあえずCLI部分は諦めて、bitstampUSD.csvをwavにできるのかを試そうと思う。

# 2021-1227
とりあえず生データのままwavとして書き出すことに成功しました。
データ自体はflacで圧縮したら15Mbぐらいになって経済的。
```powershell
scoop install flac
flac output.wav
# flacにエンコード
flac --best output.wav
# 最高圧縮率でエンコード
flac --delete-input-file abc.wav
# Like above, except abc.wav is deleted if there were no errors.
flac --delete-input-file -w abc.wav
# Like above, except abc.wav is deleted if there were no errors or warnings.
```
波形表示してみたらこうなった
[波形](/img/1227-outputの波形.png)

とりあえず正規化して書き出して、高度にコンセプチュアルなsuper low frequency oscとして使えるってことでNFTに出すか