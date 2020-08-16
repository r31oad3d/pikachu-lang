# pikachu-lang
A small toy that provides pikachu language!

![pikachu](https://timgsa.baidu.com/timg?image&quality=80&size=b9999_10000&sec=1597591532859&di=ef74853c44ce060991e7c506fd972864&imgtype=0&src=http%3A%2F%2Fimages5.fanpop.com%2Fimage%2Fphotos%2F30600000%2FPikachu-pikachu-30613407-500-323.gif)

# inspired by BrainFuck and [the little book of Rust macros]<https://danielkeep.github.io/tlborm/book/index.html>

|  pikachu-lang   |  brainfuck  |   desc   |
|      ----       |    ----     |   ----   |
|pika! pika!      |  >          |    increment the data pointer|
|pika. pika.      |  <          |    decrement the data pointer|
|pika. pika!      |  +          |    increment the byte at pointer|
|pika! pika.      |  -          |    decrement the byte at pointer|
|pikachu!         |  .          |    output the byte at pointer|
|pikachu?         |  ,          |    input of one byte into pointer|
|pika pi?         |  \[          |   Jump forward past the matching \] if the byte at the pointer is zero.|
|pika pi!         |  \]          |    if pointer is nonzero, jump to matching pika|
