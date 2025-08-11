# 社交项目简介

> **社交项目地址链接**
>
> 🔗 合约地址：https://github.com/SoniaChan33/sol-friend
>
> 🔗 客户端地址：https://github.com/SoniaChan33/solana-friend-cli


## 一、项目概述

本项目是基于Solana区块链的去中心化社交应用合约，使用Rust语言开发。合约支持用户账户初始化、关注/取消关注其他用户、发布内容、查询粉丝列表及查询帖子等核心社交功能。通过Solana的Program Derived Address (PDA)机制管理用户数据，使用Borsh进行数据序列化/反序列化，确保链上数据的高效存储与读取。


## 二、核心模块与数据结构

### 1. 指令定义（`instruction.rs`）

定义了合约支持的所有操作指令，通过`SocialInstruction`枚举实现，包含以下指令：

| 指令名           | 功能描述                                | 参数说明                                     |
| ---------------- | --------------------------------------- | -------------------------------------------- |
| `InitializeUser` | 初始化用户账户（ profile 或 post 类型） | `seed_type`: 账户类型（"profile" 或 "post"） |
| `FollowUser`     | 关注指定用户                            | `user_to_follow`: 被关注用户的Pubkey         |
| `UnfollowUser`   | 取消关注指定用户                        | `user_to_unfollow`: 被取消关注用户的Pubkey   |
| `QueryFollowers` | 查询当前用户的粉丝列表                  | 无参数                                       |
| `PostContent`    | 发布内容                                | `content`: 帖子内容字符串                    |
| `QueryPosts`     | 查询用户的帖子信息                      | 无参数                                       |


### 2. 状态数据结构（`state.rs`）

定义了链上存储的数据结构及操作方法，核心结构如下：

#### （1）`UserProfile`

存储用户的粉丝列表信息：

```rust
pub struct UserProfile {
    pub data_len: u16,       // 粉丝数量（与followers.len()一致，用于快速计算存储空间）
    pub followers: Vec<Pubkey>, // 粉丝的Pubkey列表
}
```

**核心方法**：

- `new()`：初始化空的用户资料
- `follow(&mut self, user: Pubkey)`：添加粉丝（去重）
- `unfollow(&mut self, user_to_follow: Pubkey)`：移除粉丝（更新数量）


#### （2）`UserPost`

记录用户发布的帖子数量：

```rust
pub struct UserPost {
    pub post_count: u16, // 帖子总数
}
```

**核心方法**：

- `new()`：初始化帖子计数器（初始为0）
- `add_post(&mut self)`：发布新帖子时递增计数器
- `get_count(&self)`：获取当前帖子总数


#### （3）`Post`

存储单条帖子的具体内容：

```rust
pub struct Post {
    pub content: String,  // 帖子内容
    pub timestamp: u64,   // 发布时间戳（基于Solana网络时钟）
}
```

**核心方法**：

- `new(content: String, timestamp: u64)`：创建新帖子实例


## 三、指令处理逻辑（`processor.rs`）

`Processor`结构体实现了所有指令的具体处理逻辑，核心流程如下：


### 1. `initialize_user`：初始化用户账户

- **功能**：创建用户的PDA账户（分为`profile`和`post`两种类型），用于存储用户资料或帖子计数器。
- **流程**：
  1. 解析账户信息（用户账户、PDA账户、系统程序）；
  2. 基于用户Pubkey和`seed_type`（"profile"或"post"）生成PDA地址，验证传入的PDA账户是否匹配；
  3. 计算账户所需存储空间（`profile`账户根据最大粉丝数计算，`post`账户固定大小）；
  4. 调用系统程序创建PDA账户（预存足够租金以豁免租金）；
  5. 初始化对应的数据结构（`UserProfile`或`UserPost`）并序列化到PDA账户中。


### 2. `follow_user`：关注用户

- **功能**：将被关注用户的Pubkey添加到当前用户的`UserProfile`粉丝列表中。
- **流程**：
  1. 解析PDA账户（存储当前用户的`UserProfile`）；
  2. 读取`data_len`计算当前粉丝列表的存储空间，反序列化`UserProfile`；
  3. 调用`follow`方法添加粉丝（自动去重）；
  4. 将更新后的`UserProfile`重新序列化并写入PDA账户。


### 3. `unfollow_user`：取消关注用户

- **功能**：从当前用户的`UserProfile`粉丝列表中移除指定用户的Pubkey。
- **流程**：
  1. 解析PDA账户（存储当前用户的`UserProfile`）；
  2. 反序列化`UserProfile`；
  3. 调用`unfollow`方法移除粉丝（更新`data_len`）；
  4. 将更新后的`UserProfile`重新序列化并写入PDA账户。


### 4. `query_followers`：查询粉丝列表

- **功能**：读取当前用户的`UserProfile`并输出粉丝列表。
- **流程**：
  1. 解析PDA账户（存储当前用户的`UserProfile`）；
  2. 反序列化`UserProfile`并通过`msg!`打印粉丝列表。


### 5. `post_content`：发布内容

- **功能**：创建新的帖子PDA账户，存储帖子内容和时间戳，并更新用户的帖子计数器。
- **流程**：
  1. 解析账户信息（用户账户、帖子计数器PDA、新帖子PDA、系统程序）；
  2. 读取当前用户的`UserPost`，递增`post_count`并重新序列化；
  3. 基于用户Pubkey、"post"种子和最新`post_count`生成新帖子的PDA地址；
  4. 计算帖子所需存储空间（基于`Post`结构体大小），创建新帖子PDA账户；
  5. 将帖子内容和当前时间戳（通过Solana时钟获取）序列化到新帖子PDA中。


### 6. `query_post`：查询帖子信息

- **功能**：读取用户的帖子计数器和指定帖子的内容。
- **流程**：
  1. 解析账户信息（帖子计数器PDA、目标帖子PDA）；
  2. 分别反序列化`UserPost`和`Post`，通过`msg!`打印帖子数量和内容。




## 五、程序入口（`lib.rs`）

定义了Solana程序的入口点`process_instruction`，将指令处理逻辑委托给`Processor::process_instruction`，符合Solana程序的标准入口规范。


## 六、总结

本合约通过PDA管理用户数据，结合Borsh序列化和Solana系统指令，实现了去中心化社交应用的核心功能。关键设计点包括：用PDA确保数据所有权、通过计数器和种子生成唯一帖子地址、严格遵循Rust内存安全规则处理账户数据。



