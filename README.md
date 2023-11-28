# maze_generator

迷路を生成するCLIツールです。  
`#`が壁で`.`が道であり、スタートとゴールは設定されていません。  
以下を引数で指定することができます。  
* 高さ ※5以上の奇数
* 幅 ※5以上の奇数
* 生成アルゴリズム ※以下のいずれかを指定
    * laying (棒倒し法)
    * digging (穴掘り法)
    * extending (壁伸ばし法)

![maze](https://github.com/yt2b/maze_generator/assets/76801443/a0460303-543c-426a-8960-f04143bbf2b1)
