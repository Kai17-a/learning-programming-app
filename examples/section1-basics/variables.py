# 変数と基本的なデータ型
# このファイルを保存すると自動的に実行されます

# 問題1: 異なるデータ型の変数を作成してください
name = "Python学習者"        # 文字列
age = 25                    # 整数
height = 170.5              # 浮動小数点数
is_student = True           # ブール値

print(f"名前: {name}")
print(f"年齢: {age}歳")
print(f"身長: {height}cm")
print(f"学生ですか？: {is_student}")

# 問題2: 変数を使って計算してください
width = 10
length = 15
area = width * length
print(f"長方形の面積: {width} × {length} = {area}")

# 問題3: 文字列の操作をしてください
first_name = "太郎"
last_name = "田中"
full_name = last_name + first_name
print(f"フルネーム: {full_name}")

# 問題4: リストを作成して操作してください
fruits = ["りんご", "バナナ", "オレンジ"]
print(f"果物リスト: {fruits}")
print(f"最初の果物: {fruits[0]}")
fruits.append("ぶどう")
print(f"果物を追加後: {fruits}")