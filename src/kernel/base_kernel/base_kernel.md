# BaseKernel

`BaseKernel`包含了最基本的`几何元素`所需要实现的`trait`，包括:

-   BaseVector2
-   BasePoint2
-   BaseLine2
    -   BaseSegment2
-   BaseCircle2
    -   BaseArc2

# BaseVector2

`BaseVector2`是`2维向量`，实现了`点乘(dot)`、`叉乘(cross)`，`归一化(normalize)`、`长度/模长(length)`等方法，通过`x`和`y`方法获取对应位置的数值。