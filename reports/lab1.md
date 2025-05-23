## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与**以下各位**就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 无

2. 此外，我也参考了**以下资料**，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 实现的功能

引入`sys_trace`的系统调用，根据传入的`trace_request`实现三个不同的操作

## 简答作业

### 1
> RustSBI-QEMU Version 0.2.0-alpha.2

```bash
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

1. 访问不可访问的内存触发PageFault的trap
2. 在U模式执行S指令，IllegalInstruction的trap
3. 在U模式执行S指令，IllegalInstruction的trap
### 2
#### 2.1
sp为栈顶指针，值为trap的上下文

__restore用于trap后的恢复或是内核启动完成后进入用户态
#### 2.2
`sstatus` `spec` `sscratch`寄存器

`trap前cpu所处特权级` `进入后执行指令地址` `暂存用户栈栈指针`

#### 2.3
`x2`寄存器在后续restore过程还要用到，暂时不能修改
`x4`寄存器无用

#### 2.4
- `sp`: 用户栈栈指针
- `sscratch`: 内核栈栈指针

#### 2.5
`sret`

执行后CPU的特权级变成用户态，并且跳转到恢复后要执行指令的地址

#### 2.6
- `sp`: 内核栈栈指针
- `sscratch`: 用户栈栈指针

#### 2.7
`ecall`