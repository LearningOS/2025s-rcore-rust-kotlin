## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 实现功能

重写前面实现的系统调用，以及添加了sys_spawn系统调用和stride调度算法

## 简答作业

### 1

u8容易整数溢出，250 + 10溢出后会重新变成最小值4，所以仍然是p2执行

### 2

最大stride对应最小priority，为最小priority为2，简单放缩知最大stride小于等于`BigStride / 2`，自然最大stride减去最小stride小于等于`BigStride / 2`

### 3

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false // 假设两个 Stride 永远不相等
    }
}

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 计算差值并转换为有符号整数
        let diff = self.0.wrapping_sub(other.0) as i64;
        if diff < 0 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}
```