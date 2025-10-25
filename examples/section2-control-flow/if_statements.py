# 条件分岐（if文）の練習
# このファイルを保存すると自動的に実行されます

# 問題1: 基本的なif文
age = 20
if age >= 18:
    print("成人です")
else:
    print("未成年です")

# 問題2: elif を使った複数条件
score = 85
if score >= 90:
    print("優秀です！")
elif score >= 70:
    print("良い成績です")
elif score >= 60:
    print("合格です")
else:
    print("もう少し頑張りましょう")

# 問題3: 複数の条件を組み合わせ
temperature = 25
weather = "晴れ"

if temperature > 20 and weather == "晴れ":
    print("お出かけ日和です！")
elif temperature > 20 and weather == "雨":
    print("暖かいですが雨です")
else:
    print("寒いか天気が悪いです")

# 問題4: 数値の判定
number = 15
if number > 0:
    print(f"{number}は正の数です")
    if number % 2 == 0:
        print("偶数です")
    else:
        print("奇数です")
elif number < 0:
    print(f"{number}は負の数です")
else:
    print("ゼロです")

# 問題5: リストの要素チェック
fruits = ["りんご", "バナナ", "オレンジ"]
target = "バナナ"

if target in fruits:
    print(f"{target}はリストに含まれています")
else:
    print(f"{target}はリストに含まれていません")

# 問題6: 文字列の判定
password = "python123"
if len(password) >= 8:
    print("パスワードの長さは適切です")
    if password.isalnum():
        print("英数字のみで構成されています")
    else:
        print("特殊文字が含まれています")
else:
    print("パスワードが短すぎます")