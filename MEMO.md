PowerShellで標準入力のリダイレクトは以下のようなものとなる

```powershell
cat .\csv\sample.csv | cargo run # 開発中
cat .\csv\sample.csv | .\target\debug\csv_to_wav.exe # build後
```