# 関数の基本
# このファイルを保存すると自動的に実行されます

print("=== 関数の基本 ===")

# 問題1: 基本的な関数定義
def greet():
    """挨拶をする関数"""
    print("こんにちは！")

print("関数を呼び出し:")
greet()

# 問題2: 引数を受け取る関数
def greet_person(name):
    """名前を受け取って挨拶する関数"""
    print(f"こんにちは、{name}さん！")

print("\n引数付き関数:")
greet_person("太郎")
greet_person("花子")

# 問題3: 戻り値のある関数
def add_numbers(a, b):
    """2つの数を足し算する関数"""
    return a + b

result = add_numbers(10, 5)
print(f"\n10 + 5 = {result}")

# 問題4: デフォルト引数
def introduce(name, age=20):
    """自己紹介する関数（年齢はデフォルト値あり）"""
    return f"私の名前は{name}で、{age}歳です"

print("\nデフォルト引数の例:")
print(introduce("太郎"))
print(introduce("花子", 25))

# 問題5: 複数の戻り値
def calculate(x, y):
    """四則演算の結果を返す関数"""
    add = x + y
    sub = x - y
    mul = x * y
    div = x / y if y != 0 else None
    return add, sub, mul, div

a, s, m, d = calculate(10, 3)
print(f"\n10と3の計算結果:")
print(f"足し算: {a}, 引き算: {s}, 掛け算: {m}, 割り算: {d:.2f}")

# 問題6: リストを処理する関数
def find_max(numbers):
    """リストの最大値を見つける関数"""
    if not numbers:
        return None
    
    max_val = numbers[0]
    for num in numbers:
        if num > max_val:
            max_val = num
    return max_val

test_list = [3, 7, 2, 9, 1, 5]
max_value = find_max(test_list)
print(f"\nリスト {test_list} の最大値: {max_value}")

# 問題7: 再帰関数の例
def factorial(n):
    """階乗を計算する再帰関数"""
    if n <= 1:
        return 1
    return n * factorial(n - 1)

print(f"\n5の階乗: {factorial(5)}")

# 問題8: ラムダ関数
print("\n=== ラムダ関数 ===")
square = lambda x: x ** 2
print(f"5の2乗: {square(5)}")

numbers = [1, 2, 3, 4, 5]
squared_numbers = list(map(lambda x: x**2, numbers))
print(f"リスト {numbers} の各要素の2乗: {squared_numbers}")

# 問題9: 関数をまとめて実行
def main():
    """メイン関数"""
    print("\n=== 関数のまとめ実行 ===")
    
    # 各関数を呼び出し
    greet()
    result = add_numbers(7, 3)
    print(f"7 + 3 = {result}")
    
    info = introduce("学習者", 22)
    print(info)

# メイン関数を実行
main()