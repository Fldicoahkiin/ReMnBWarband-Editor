# 触发器操作码文档

## 概述

骑砍战团的触发器系统使用数字操作码来定义游戏逻辑。每个操作码对应一个特定的游戏功能，如条件判断、数值操作、游戏状态修改等。魔球编辑器实现了完整的操作码解析和编辑功能。

## 操作码分类

### 控制流操作 (Control Operations)

#### 脚本调用

```vb
Public Const Call_Script = 1                ' (call_script,<script_id>)
```

调用指定的脚本函数。

#### 条件控制

```vb
Public Const try_begin = 4                  ' (try_begin)
Public Const try_end = 3                    ' (try_end)
Public Const else_try = 5                   ' (else_try)
```

基础的条件控制结构，类似于 if-else 语句。

#### 循环控制

```vb
Public Const try_for_range = 6              ' (try_for_range,<destination>,<lower_bound>,<upper_bound>)
Public Const try_for_range_backwards = 7    ' (try_for_range_backwards,<destination>,<upper_bound>,<lower_bound>)
Public Const try_for_parties = 11           ' (try_for_parties,<destination>)
Public Const try_for_agents = 12            ' (try_for_agents,<destination>)
```

各种循环结构，用于遍历数值范围、队伍、代理等。

#### 参数处理

```vb
Public Const store_script_param_1 = 21      ' (store_script_param_1,<destination>)
Public Const store_script_param_2 = 22      ' (store_script_param_2,<destination>)
Public Const store_script_param = 23        ' (store_script_param,<destination>,<script_param_no>)
```

在脚本内部获取传入的参数。

### 条件操作 (Condition Operations)

#### 数值比较

```vb
Public Const ge = 30                        ' (ge,<value>,<value>) - 大于等于
Public Const eq = 31                        ' (eq,<value>,<value>) - 等于
Public Const gt = 32                        ' (gt,<value>,<value>) - 大于
Public Const is_between = 33                ' (is_between,<value>,<lower_bound>,<upper_bound>)
```

#### 游戏状态检查

```vb
Public Const entering_town = 36             ' (entering_town,<town_id>)
Public Const map_free = 37                  ' (map_free)
Public Const encountered_party_is_attacker = 39  ' (encountered_party_is_attacker)
Public Const conversation_screen_is_active = 42  ' (conversation_screen_active)
```

#### 输入检测

```vb
Public Const key_is_down = 70               ' (key_is_down,<key_id>)
Public Const key_clicked = 71               ' (key_clicked,<key_id>)
Public Const game_key_is_down = 72          ' (game_key_is_down,<game_key_id>)
Public Const game_key_clicked = 73          ' (game_key_clicked,<game_key_id>)
```

### 队伍和角色操作

#### 队伍状态

```vb
Public Const hero_can_join = 101            ' (hero_can_join,[party_id])
Public Const hero_can_join_as_prisoner = 102  ' (hero_can_join_as_prisoner,[party_id])
Public Const party_can_join = 103           ' (party_can_join)
Public Const party_can_join_as_prisoner = 104  ' (party_can_join_as_prisoner)
Public Const troops_can_join = 105          ' (troops_can_join,<value>)
Public Const troops_can_join_as_prisoner = 106  ' (troops_can_join_as_prisoner,<value>)
Public Const party_can_join_party = 107     ' (party_can_join_party,<joiner_party_id>,<host_party_id>,[flip_prisoners])
```

#### 队伍检查

```vb
Public Const main_party_has_troop = 110     ' (main_party_has_troop,<troop_id>)
Public Const party_is_in_town = 130         ' (party_is_in_town,<party_id_1>,<party_id_2>)
Public Const party_is_in_any_town = 131     ' (party_is_in_any_town,<party_id>)
Public Const party_is_active = 132          ' (party_is_active,<party_id>)
```

#### 物品和装备

```vb
Public Const player_has_item = 150          ' (player_has_item,<item_id>)
Public Const troop_has_item_equipped = 151  ' (troop_has_item_equipped,<troop_id>,<item_id>)
Public Const troop_is_mounted = 152         ' (troop_is_mounted,<troop_id>)
Public Const troop_is_guarantee_ranged = 153  ' (troop_is_guarantee_ranged,<troop_id>)
Public Const troop_is_guarantee_horse = 154   ' (troop_is_guarantee_horse,<troop_id>)
```

### 任务系统

#### 任务状态检查

```vb
Public Const check_quest_active = 200       ' (check_quest_active,<quest_id>)
Public Const check_quest_finished = 201     ' (check_quest_finished,<quest_id>)
Public Const check_quest_succeeded = 202    ' (check_quest_succeeded,<quest_id>)
Public Const check_quest_failed = 203       ' (check_quest_failed,<quest_id>)
Public Const check_quest_concluded = 204    ' (check_quest_concluded,<quest_id>)
```

### 环境和效果

#### 天气控制

```vb
Public Const get_global_cloud_amount = 90   ' (get_global_cloud_amount,<destination>)
Public Const set_global_cloud_amount = 91   ' (set_global_cloud_amount,<value>)
Public Const get_global_haze_amount = 92    ' (get_global_haze_amount,<destination>)
Public Const set_global_haze_amount = 93    ' (set_global_haze_amount,<value>)
```

#### 鼠标操作

```vb
Public Const mouse_get_position = 75        ' (mouse_get_position,<position_no>)
Public Const omit_key_once = 77             ' (omit_key_once,<key_id>)
Public Const clear_omitted_keys = 78        ' (clear_omitted_keys)
```

### 多人游戏操作

#### 服务器通信

```vb
Public Const multiplayer_send_message_to_server = 388     ' (multiplayer_send_message_to_server,<message_type>)
Public Const multiplayer_send_int_to_server = 389        ' (multiplayer_send_int_to_server,<message_type>,<value>)
Public Const multiplayer_send_2_int_to_server = 390      ' (multiplayer_send_2_int_to_server,<message_type>,<value>,<value>)
Public Const multiplayer_send_3_int_to_server = 391      ' (multiplayer_send_3_int_to_server,<message_type>,<value>,<value>,<value>)
Public Const multiplayer_send_4_int_to_server = 392      ' (multiplayer_send_4_int_to_server,<message_type>,<value>,<value>,<value>,<value>)
Public Const multiplayer_send_string_to_server = 393     ' (multiplayer_send_string_to_server,<message_type>,<string_id>)
```

#### 玩家通信

```vb
Public Const multiplayer_send_message_to_player = 394    ' (multiplayer_send_message_to_player,<player_id>,<message_type>)
Public Const multiplayer_send_int_to_player = 395       ' (multiplayer_send_int_to_player,<player_id>,<message_type>,<value>)
Public Const multiplayer_send_2_int_to_player = 396     ' (multiplayer_send_2_int_to_player,<player_id>,<message_type>,<value>,<value>)
Public Const multiplayer_send_3_int_to_player = 397     ' (multiplayer_send_3_int_to_player,<player_id>,<message_type>,<value>,<value>,<value>)
Public Const multiplayer_send_4_int_to_player = 398     ' (multiplayer_send_4_int_to_player,<player_id>,<message_type>,<value>,<value>,<value>,<value>)
Public Const multiplayer_send_string_to_player = 399    ' (multiplayer_send_string_to_player,<player_id>,<message_type>,<string_id>)
```

#### 玩家信息

```vb
Public Const get_max_players = 400          ' (get_max_players,<destination>)
Public Const player_is_active = 401         ' (player_is_active,<player_id>)
Public Const player_get_team_no = 402       ' (player_get_team_no,<destination>,<player_id>)
Public Const player_set_team_no = 403       ' (player_set_team_no,<player_id>,<team_no>)
Public Const player_get_troop_id = 404      ' (player_get_troop_id,<destination>,<player_id>)
Public Const player_set_troop_id = 405      ' (player_set_troop_id,<player_id>,<troop_id>)
Public Const player_get_agent_id = 406      ' (player_get_agent_id,<destination>,<player_id>)
Public Const player_get_gold = 407          ' (player_get_gold,<destination>,<player_id>)
```

### 成就系统

```vb
Public Const get_achievement_stat = 370     ' (get_achievement_stat,<destination>,<achievement_id>,<stat_index>)
Public Const set_achievement_stat = 371     ' (set_achievement_stat,<achievement_id>,<stat_index>,<value>)
Public Const unlock_achievement = 372       ' (unlock_achievement,<achievement_id>)
```

### 网络通信

```vb
Public Const send_message_to_url = 380      ' (send_message_to_url,<string_id>)
```

### 用户配置

```vb
Public Const profile_get_banner_id = 350    ' (profile_get_banner_id,<destination>)
Public Const profile_set_banner_id = 351    ' (profile_set_banner_id,<value>)
```

### 版本检查

```vb
Public Const is_trial_version = 250         ' (is_trial_version)
```

## 操作码类型分类

### 操作类型常量

```vb
Public Const OPT_NONE = 0                   ' 无特殊类型
Public Const OPT_Lhs = 1                    ' 左值操作（可赋值）
Public Const OPT_Global_Lhs = 2             ' 全局左值操作
Public Const OPT_Can_Fail = 3               ' 可能失败的操作
```

## 参数类型系统

### 参数结构

```vb
Public Type Type_Para
    Value As String             ' 参数值
    Para_Type As String         ' 参数类型
End Type

Public Type Type_Operation
    OpID As Long                ' 操作码ID
    Op_name As String           ' 操作名称
    Op_CSVname As String        ' CSV名称
    Pseudo As String            ' 伪代码
    ParaNum As Integer          ' 参数数量
    Para() As Type_Para         ' 参数数组
    Type As Integer             ' 操作类型
End Type
```

### 常见参数类型

- `<destination>`: 目标变量，用于存储结果
- `<value>`: 数值参数
- `<string_id>`: 字符串ID
- `<troop_id>`: 兵种ID
- `<item_id>`: 物品ID
- `<party_id>`: 队伍ID
- `<quest_id>`: 任务ID
- `<script_id>`: 脚本ID

## 触发器编译过程

### 1. 词法分析

```vb
Function SplitParam(ByVal CMD As String, Params() As String) As Long
    ' 分割参数字符串
    ' 处理引号内的字符串
    ' 返回参数数量
End Function
```

### 2. 语法分析

```vb
Function PurseParams(strPara As String, Pointer As Long, Params() As String, Param_Start() As Long) As Long
    ' 解析参数列表
    ' 验证参数类型
    ' 构建操作结构
End Function
```

### 3. 语义检查

- 验证操作码是否存在
- 检查参数数量是否正确
- 验证参数类型是否匹配
- 检查引用的ID是否有效

### 4. 代码生成

- 将操作码转换为二进制格式
- 优化操作序列
- 生成最终的触发器数据

## 触发器条件类型

### 触发时机常量

```vb
Public Const ti_on_item_picked_up = 0       ' 物品被拾取时
Public Const ti_on_item_dropped = 1         ' 物品被丢弃时
Public Const ti_on_item_wielded = 2         ' 物品被装备时
Public Const ti_on_item_unwielded = 3       ' 物品被卸下时
```

## 错误处理

### 常见错误类型

```vb
Public Const ERR_SUCCESS = 0                ' 成功
Public Const ERR_FAIL = 1                   ' 失败
Public Const ERR_BAD_QUETO = 2              ' 引号错误
Public Const ERR_BAD_OPERATION = -1         ' 无效操作
Public Const ERR_NULL_STRING = 0            ' 空字符串
Public Const ERR_INCOMPLETE_SOURCE_CODE = 9 ' 不完整的源代码
```

### 错误检查流程

1. 语法错误检查
2. 参数类型验证
3. 引用完整性检查
4. 逻辑一致性验证

## 高级特性

### 变量系统

触发器支持局部变量和全局变量：

- 局部变量: `reg0` - `reg63`
- 全局变量: `$g_variable_name`

### 字符串处理

支持字符串常量和动态字符串操作：

- 字符串常量: `"str_constant"`
- 字符串变量: `s0` - `s63`

### 位置系统

支持3D位置和方向操作：

- 位置变量: `pos0` - `pos63`
- 坐标操作: `(position_set_x, <position>, <value>)`

## 调试和优化

### 调试技巧

1. 使用 `display_message` 输出调试信息
2. 分步测试复杂逻辑
3. 检查变量值的变化
4. 验证条件判断的正确性

### 性能优化

1. 避免不必要的循环
2. 缓存频繁使用的值
3. 合并相似的操作
4. 减少字符串操作

## 常见使用模式

### 条件判断模式

```
(try_begin),
  (eq, <condition>, 1),
  # 执行操作
(try_end),
```

### 循环遍历模式

```
(try_for_range, <iterator>, <start>, <end>),
  # 循环体
(try_end),
```

### 错误处理模式

```
(try_begin),
  (call_script, "script_function"),
  (eq, reg0, 1),  # 检查返回值
  # 成功处理
(else_try),
  # 错误处理
(try_end),
```

---

*本文档基于魔球编辑器源码分析，详细描述了骑砍战团触发器系统的操作码定义和使用方法。*
