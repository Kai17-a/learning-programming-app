# ループ（繰り返し）の練習
# このファイルを保存すると自動的に実行されます

print("=== for ループの練習 ===")

# 問題1: 基本的なfor文
print("1から5まで数える:")
for i in range(1, 6):
    print(f"数字: {i}")

# 問題2: リストの要素を順番に処理
fruits = ["りんご", "バナナ", "オレンジ", "ぶどう"]
print("\n果物リスト:")
for fruit in fruits:
    print(f"- {fruit}")

# 問題3: enumerate を使ってインデックスも取得
print("\n番号付き果物リスト:")
for index, fruit in enumerate(fruits, 1):
    print(f"{index}. {fruit}")

# 問題4: 文字列の各文字を処理
word = "Python"
print(f"\n'{word}'の各文字:")
for char in word:
    print(f"文字: {char}")

print("\n=== while ループの練習 ===")

# 問題5: 基本的なwhile文
count = 1
print("while文で1から5まで:")
while count <= 5:
    print(f"カウント: {count}")
    count += 1

# 問題6: 条件に基づく繰り返し
number = 100
print(f"\n{number}から始めて2で割り続ける:")
while number > 1:
    print(f"現在の値: {number}")
    number = number // 2
print(f"最終値: {number}")

print("\n=== ネストしたループ ===")

# 問題7: 九九の表（一部）
print("3×3の掛け算表:")
for i in range(1, 4):
    for j in range(1, 4):
        result = i * j
        print(f"{i} × {j} = {result:2d}", end="  ")
    print()  # 改行

print("\n=== break と continue ===")

# 問題8: break を使った早期終了
print("1から10まで数えるが、7で止める:")
for i in range(1, 11):
    if i == 7:
        print("7に到達したので終了")
        break
    print(f"数字: {i}")

# 問題9: continue を使ったスキップ
print("\n1から10まで、偶数のみ表示:")
for i in range(1, 11):
    if i % 2 != 0:  # 奇数の場合
        continue    # スキップ
    print(f"偶数: {i}")

# 問題10: リスト内包表記の例
print("\n=== リスト内包表記 ===")
numbers = [1, 2, 3, 4, 5]
squares = [x**2 for x in numbers]
print(f"元のリスト: {numbers}")
print(f"2乗のリスト: {squares}")

even_squares = [x**2 for x in numbers if x % 2 == 0]
print(f"偶数の2乗のみ: {even_squares}")