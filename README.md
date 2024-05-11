# iriomote-mountaincat
このプロジェクトはAIを使ってイリオモテヤマネコかじゃないかを判断するAIの周辺回りです。

## 利用方法
```py
from iriomote_py import IriomotePy


with open("./sample.jpg", "rb") as f:
    data = f.read()


async def main() -> None:
    ai = IriomotePy()
    result = await ai.execute(data)
    print(f"イリオモテヤマネコの可能性は{round((1 - result) * 100, 2)}%です。")


if __name__ == "__main__":
    import asyncio
    asyncio.run(main())
```

## License
MITで公開しています。
